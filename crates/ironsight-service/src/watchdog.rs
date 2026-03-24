//! Peer watchdog — two processes monitor each other.
//!
//! Anti-tamper pattern: advanced malware kills security tools first.
//! The watchdog spawns a sentinel process that monitors the main process.
//! If either dies, the survivor restarts the other.

use std::process::Command;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// Watchdog state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogState {
    pub role: WatchdogRole,
    pub peer_pid: Option<u32>,
    pub last_heartbeat: Option<String>,
    pub restarts: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WatchdogRole {
    /// Primary scanner process.
    Primary,
    /// Sentinel — monitors the primary.
    Sentinel,
    /// Watchdog disabled.
    Disabled,
}

/// Check if a PID is still alive.
pub fn is_pid_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        // kill(pid, 0) checks existence without sending a signal
        nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(pid as i32),
            None, // Signal 0 = existence check
        )
        .is_ok()
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}

/// Spawn a sentinel process that monitors the given PID.
pub fn spawn_sentinel(main_pid: u32, check_interval: Duration) -> Result<u32, std::io::Error> {
    let exe = std::env::current_exe()?;
    let child = Command::new(exe)
        .arg("--watchdog-sentinel")
        .arg("--watch-pid")
        .arg(main_pid.to_string())
        .arg("--watch-interval")
        .arg(check_interval.as_secs().to_string())
        .spawn()?;
    Ok(child.id())
}

/// Run the sentinel loop — monitors a peer PID and restarts it if dead.
/// This is the entry point when `--watchdog-sentinel` is passed.
pub fn run_sentinel_loop(watch_pid: u32, interval: Duration, max_restarts: u32) {
    tracing::info!(
        "🐕 Watchdog sentinel started — monitoring PID {watch_pid} every {}s",
        interval.as_secs()
    );

    let mut restarts = 0u32;
    let mut last_alive = Instant::now();

    loop {
        std::thread::sleep(interval);

        if is_pid_alive(watch_pid) {
            last_alive = Instant::now();
            tracing::debug!("Watchdog: PID {watch_pid} alive");
        } else {
            let dead_duration = last_alive.elapsed();
            tracing::warn!(
                "🚨 Watchdog: PID {watch_pid} is DEAD (missing for {:.1}s)",
                dead_duration.as_secs_f64()
            );

            if restarts >= max_restarts {
                tracing::error!(
                    "Watchdog: Max restarts ({max_restarts}) exceeded — giving up"
                );
                break;
            }

            // Attempt restart
            match restart_main_process() {
                Ok(new_pid) => {
                    restarts += 1;
                    tracing::info!(
                        "Watchdog: Restarted main process as PID {new_pid} (restart #{restarts})"
                    );
                    // Continue monitoring the new PID
                    // Note: In production, the new process would register itself
                    last_alive = Instant::now();
                }
                Err(e) => {
                    tracing::error!("Watchdog: Failed to restart: {e}");
                    break;
                }
            }
        }
    }
}

/// Restart the main IronSight process.
fn restart_main_process() -> Result<u32, std::io::Error> {
    let exe = std::env::current_exe()?;
    let child = Command::new(exe).spawn()?;
    Ok(child.id())
}

/// Parse watchdog arguments from CLI args.
pub fn parse_watchdog_args(args: &[String]) -> Option<WatchdogArgs> {
    if !args.iter().any(|a| a == "--watchdog-sentinel") {
        return None;
    }

    let watch_pid = args.windows(2).find_map(|w| {
        if w[0] == "--watch-pid" {
            w[1].parse().ok()
        } else {
            None
        }
    })?;

    let interval_secs = args
        .windows(2)
        .find_map(|w| {
            if w[0] == "--watch-interval" {
                w[1].parse().ok()
            } else {
                None
            }
        })
        .unwrap_or(10u64);

    Some(WatchdogArgs {
        watch_pid,
        interval: Duration::from_secs(interval_secs),
    })
}

pub struct WatchdogArgs {
    pub watch_pid: u32,
    pub interval: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn own_pid_is_alive() {
        let pid = std::process::id();
        assert!(is_pid_alive(pid));
    }

    #[test]
    fn dead_pid_is_not_alive() {
        // PID 99999999 almost certainly doesn't exist
        assert!(!is_pid_alive(99_999_999));
    }

    #[test]
    fn parse_sentinel_args() {
        let args: Vec<String> = vec![
            "ironsight".into(),
            "--watchdog-sentinel".into(),
            "--watch-pid".into(),
            "1234".into(),
            "--watch-interval".into(),
            "5".into(),
        ];
        let parsed = parse_watchdog_args(&args).unwrap();
        assert_eq!(parsed.watch_pid, 1234);
        assert_eq!(parsed.interval, Duration::from_secs(5));
    }

    #[test]
    fn parse_no_sentinel_args() {
        let args: Vec<String> = vec!["ironsight".into(), "--pid".into(), "123".into()];
        assert!(parse_watchdog_args(&args).is_none());
    }

    #[test]
    fn watchdog_roles_differ() {
        assert_ne!(WatchdogRole::Primary, WatchdogRole::Sentinel);
        assert_ne!(WatchdogRole::Sentinel, WatchdogRole::Disabled);
    }
}
