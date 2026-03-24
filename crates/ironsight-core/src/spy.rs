//! ProcessSpy — the main facade for process monitoring.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use crossbeam_channel::{bounded, Receiver, RecvTimeoutError, Sender};
use sysinfo::{Pid, ProcessRefreshKind, System, UpdateKind};
use tracing::info;

#[cfg(not(target_os = "windows"))]
use sysinfo::Signal;

use crate::diff::ProcessDiff;
use crate::process_info::ProcessInfo;
use crate::snapshot::{build_snapshot, ProcessSnapshot};
use crate::system_info::SystemInfo;
use crate::{Result, SuiteError};

// ─────────────────────────────────────────────────────────────────────────────
// SpyEvent — STEP 6: timestamp added to all variants
// ─────────────────────────────────────────────────────────────────────────────

/// Events emitted during continuous monitoring.
#[derive(Debug, Clone)]
pub enum SpyEvent {
    Snapshot {
        snapshot: ProcessSnapshot,
        timestamp: DateTime<Utc>,
    },
    Diff {
        diff: ProcessDiff,
        timestamp: DateTime<Utc>,
    },
    ProcessSpawned {
        info: ProcessInfo,
        timestamp: DateTime<Utc>,
    },
    ProcessExited {
        info: ProcessInfo,
        timestamp: DateTime<Utc>,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// ProcessSpy — STEP 1: graceful shutdown with AtomicBool + Drop
// ─────────────────────────────────────────────────────────────────────────────

pub struct ProcessSpy {
    system: Arc<Mutex<System>>,
    event_tx: Sender<SpyEvent>,
    event_rx: Receiver<SpyEvent>,
    monitor_handle: Option<thread::JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
}

impl ProcessSpy {
    /// Create a `ProcessSpy`. Does NOT start continuous monitoring.
    pub fn new() -> Self {
        let (tx, rx) = bounded(512);
        Self {
            system: Arc::new(Mutex::new(System::new_all())),
            event_tx: tx,
            event_rx: rx,
            monitor_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start continuous monitoring in the background.
    ///
    /// Returns `Err` if monitoring is already running.
    /// Emits `SpyEvent::Snapshot` every `interval` and `SpyEvent::Diff` when
    /// changes occur.
    pub fn start_monitoring(&mut self, interval: Duration) -> Result<()> {
        if self.monitor_handle.is_some() {
            return Err(SuiteError::ProcessSpy(
                "Monitoring already running; call stop_monitoring() first".into(),
            ));
        }

        self.stop_flag.store(false, Ordering::SeqCst);
        let system = Arc::clone(&self.system);
        let tx = self.event_tx.clone();
        let stop = Arc::clone(&self.stop_flag);

        let handle = thread::spawn(move || {
            let mut prev_snapshot: Option<ProcessSnapshot> = None;
            let refresh_kind = ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .with_exe(UpdateKind::OnlyIfNotSet)
                .with_cmd(UpdateKind::OnlyIfNotSet)
                .with_cwd(UpdateKind::OnlyIfNotSet);

            while !stop.load(Ordering::SeqCst) {
                thread::sleep(interval);
                if stop.load(Ordering::SeqCst) {
                    break;
                }

                let snapshot = {
                    let mut sys = system.lock().unwrap();
                    sys.refresh_processes_specifics(
                        sysinfo::ProcessesToUpdate::All,
                        true,
                        refresh_kind,
                    );
                    build_snapshot(&sys)
                };

                if let Some(prev) = &prev_snapshot {
                    let diff = ProcessDiff::compute(prev, &snapshot);
                    if !diff.is_empty() {
                        let now = Utc::now();
                        for p in &diff.spawned {
                            let _ = tx.send(SpyEvent::ProcessSpawned {
                                info: p.clone(),
                                timestamp: now,
                            });
                        }
                        for p in &diff.exited {
                            let _ = tx.send(SpyEvent::ProcessExited {
                                info: p.clone(),
                                timestamp: now,
                            });
                        }
                        let _ = tx.send(SpyEvent::Diff {
                            diff,
                            timestamp: now,
                        });
                    }
                }

                let _ = tx.send(SpyEvent::Snapshot {
                    snapshot: snapshot.clone(),
                    timestamp: Utc::now(),
                });
                prev_snapshot = Some(snapshot);
            }
        });

        self.monitor_handle = Some(handle);
        info!("ProcessSpy monitoring started (interval={interval:?})");
        Ok(())
    }

    /// Stop continuous monitoring and join the background thread.
    pub fn stop_monitoring(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        if let Some(handle) = self.monitor_handle.take() {
            let _ = handle.join();
            info!("ProcessSpy monitoring stopped");
        }
    }

    /// Returns true if the monitor thread is currently running.
    pub fn is_monitoring(&self) -> bool {
        self.monitor_handle.is_some() && !self.stop_flag.load(Ordering::SeqCst)
    }

    // ── Snapshot ──────────────────────────────────────────────────────────

    /// Take a one-off snapshot of all running processes.
    pub fn snapshot(&self) -> ProcessSnapshot {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_all();
        build_snapshot(&sys)
    }

    // ── System info ───────────────────────────────────────────────────────

    pub fn system_info(&self) -> SystemInfo {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_all();
        SystemInfo {
            hostname: System::host_name(),
            os_name: System::name(),
            os_version: System::os_version(),
            kernel_version: System::kernel_version(),
            cpu_count: sys.cpus().len(),
            total_memory_bytes: sys.total_memory(),
            used_memory_bytes: sys.used_memory(),
            total_swap_bytes: sys.total_swap(),
            used_swap_bytes: sys.used_swap(),
            uptime_secs: System::uptime(),
            load_average: {
                let la = System::load_average();
                [la.one, la.five, la.fifteen]
            },
            process_count: sys.processes().len(),
        }
    }

    // ── Process actions — STEP 4: return value checks ────────────────────

    /// Kill a process by PID (sends SIGKILL / TerminateProcess).
    pub fn kill(&self, pid: u32) -> Result<()> {
        let sys = self.system.lock().unwrap();
        match sys.process(Pid::from(pid as usize)) {
            Some(p) => {
                if p.kill() {
                    info!("Killed process {pid}");
                    Ok(())
                } else {
                    Err(SuiteError::ProcessAction(format!(
                        "kill() returned false for PID {pid}"
                    )))
                }
            }
            None => Err(SuiteError::ProcessNotFound(pid)),
        }
    }

    /// Send a specific signal to a process (POSIX only; no-op on Windows).
    #[cfg(not(target_os = "windows"))]
    pub fn signal(&self, pid: u32, signal: Signal) -> Result<()> {
        let sys = self.system.lock().unwrap();
        match sys.process(Pid::from(pid as usize)) {
            Some(p) => match p.kill_with(signal) {
                Some(true) => {
                    info!("Sent {signal:?} to {pid}");
                    Ok(())
                }
                Some(false) => Err(SuiteError::ProcessAction(format!(
                    "kill_with({signal:?}) returned false for PID {pid}"
                ))),
                None => Err(SuiteError::ProcessAction(format!(
                    "Signal {signal:?} not supported on this platform"
                ))),
            },
            None => Err(SuiteError::ProcessNotFound(pid)),
        }
    }

    // ── STEP 5: wait_for_exit — no deadlock ──────────────────────────────

    /// Wait for a process to exit with a timeout.
    /// Uses short-lived mutex locks to avoid deadlocking the system.
    pub fn wait_for_exit(&self, pid: u32, timeout: Duration) -> Result<()> {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            {
                let sys = self.system.lock().unwrap();
                if sys.process(Pid::from(pid as usize)).is_none() {
                    return Ok(()); // process exited
                }
            } // lock released
            if std::time::Instant::now() > deadline {
                return Err(SuiteError::Timeout(format!(
                    "PID {pid} did not exit within {timeout:?}"
                )));
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    // ── Deep inspection ───────────────────────────────────────────────────

    /// Return environment variables of a process.
    pub fn env_vars(&self, pid: u32) -> Result<Vec<String>> {
        let sys = self.system.lock().unwrap();
        sys.process(Pid::from(pid as usize))
            .map(|p| {
                p.environ()
                    .iter()
                    .map(|s| s.to_string_lossy().into_owned())
                    .collect()
            })
            .ok_or_else(|| SuiteError::ProcessNotFound(pid))
    }

    /// Count open file descriptors for a process (Linux /proc only).
    #[cfg(target_os = "linux")]
    pub fn fd_count(&self, pid: u32) -> Option<usize> {
        std::fs::read_dir(format!("/proc/{pid}/fd"))
            .ok()
            .map(|d| d.count())
    }

    // ── STEP 3: event-based wait_for_spawn ───────────────────────────────

    /// Wait for a process with `name` to appear by listening to events.
    /// Falls back to polling if monitoring is not active.
    pub fn wait_for_spawn(&self, name: &str, timeout: Duration) -> Option<ProcessInfo> {
        let deadline = std::time::Instant::now() + timeout;
        let name_lower = name.to_lowercase();

        // Fast path: already running?
        {
            let snap = self.snapshot();
            if let Some(found) = snap.find_by_name(&name_lower).into_iter().next() {
                return Some(found.clone());
            }
        }

        // If monitoring is active, use events
        if self.is_monitoring() {
            loop {
                let remaining = deadline.saturating_duration_since(std::time::Instant::now());
                if remaining.is_zero() {
                    return None;
                }
                match self.event_rx.recv_timeout(remaining.min(Duration::from_millis(500))) {
                    Ok(SpyEvent::ProcessSpawned { ref info, .. })
                        if info.name.to_lowercase().contains(&name_lower) =>
                    {
                        return Some(info.clone());
                    }
                    Ok(_) => continue,
                    Err(RecvTimeoutError::Timeout) => {
                        if std::time::Instant::now() > deadline {
                            return None;
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => return None,
                }
            }
        }

        // Fallback: polling (less CPU-intensive than before)
        loop {
            if std::time::Instant::now() > deadline {
                return None;
            }
            let snap = self.snapshot();
            if let Some(found) = snap.find_by_name(&name_lower).into_iter().next() {
                return Some(found.clone());
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    // ── Events ────────────────────────────────────────────────────────────

    pub fn subscribe(&self) -> Receiver<SpyEvent> {
        self.event_rx.clone()
    }

    pub fn try_recv(&self) -> Option<SpyEvent> {
        self.event_rx.try_recv().ok()
    }
}

impl Default for ProcessSpy {
    fn default() -> Self {
        Self::new()
    }
}

/// STEP 1: Drop impl joins the monitor thread on cleanup.
impl Drop for ProcessSpy {
    fn drop(&mut self) {
        self.stop_monitoring();
    }
}
