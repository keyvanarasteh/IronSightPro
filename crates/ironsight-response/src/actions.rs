//! Response actions — the core operations that can be performed on a process.
//!
//! Follows the forensic order: **Suspend → Dump → Kill**.

use serde::{Deserialize, Serialize};

/// Result of an attempted response action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub pid: u32,
    pub action: ActionType,
    pub success: bool,
    pub message: String,
    pub timestamp: String,
}

/// Types of automated response actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// SIGSTOP — freeze the process.
    Suspend,
    /// SIGCONT — resume a suspended process.
    Resume,
    /// Dump process memory to disk for forensics.
    MemoryDump,
    /// SIGKILL — terminate the process.
    Kill,
    /// Log the event for later review.
    LogOnly,
}

/// Suspend a process (SIGSTOP).
#[cfg(unix)]
pub fn suspend(pid: u32) -> ActionResult {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    let result = kill(Pid::from_raw(pid as i32), Signal::SIGSTOP);
    let now = timestamp_now();

    match result {
        Ok(()) => ActionResult {
            pid,
            action: ActionType::Suspend,
            success: true,
            message: format!("Process {pid} suspended (SIGSTOP)"),
            timestamp: now,
        },
        Err(e) => ActionResult {
            pid,
            action: ActionType::Suspend,
            success: false,
            message: format!("Failed to suspend PID {pid}: {e}"),
            timestamp: now,
        },
    }
}

/// Resume a suspended process (SIGCONT).
#[cfg(unix)]
pub fn resume(pid: u32) -> ActionResult {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    let result = kill(Pid::from_raw(pid as i32), Signal::SIGCONT);
    let now = timestamp_now();

    match result {
        Ok(()) => ActionResult {
            pid,
            action: ActionType::Resume,
            success: true,
            message: format!("Process {pid} resumed (SIGCONT)"),
            timestamp: now,
        },
        Err(e) => ActionResult {
            pid,
            action: ActionType::Resume,
            success: false,
            message: format!("Failed to resume PID {pid}: {e}"),
            timestamp: now,
        },
    }
}

/// Kill a process (SIGKILL).
#[cfg(unix)]
pub fn kill_process(pid: u32) -> ActionResult {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    let result = kill(Pid::from_raw(pid as i32), Signal::SIGKILL);
    let now = timestamp_now();

    match result {
        Ok(()) => ActionResult {
            pid,
            action: ActionType::Kill,
            success: true,
            message: format!("Process {pid} killed (SIGKILL)"),
            timestamp: now,
        },
        Err(e) => ActionResult {
            pid,
            action: ActionType::Kill,
            success: false,
            message: format!("Failed to kill PID {pid}: {e}"),
            timestamp: now,
        },
    }
}

/// Dump process memory to a file for forensic analysis.
#[cfg(target_os = "linux")]
pub fn dump_memory(pid: u32, output_dir: &str) -> ActionResult {
    use std::io::{Read, Seek, SeekFrom, Write};

    let now = timestamp_now();
    let maps_path = format!("/proc/{pid}/maps");
    let mem_path = format!("/proc/{pid}/mem");
    let dump_path = format!("{output_dir}/memdump_{pid}_{}.bin", now.replace(':', "-"));

    // Read maps to know which regions to dump
    let maps_content = match std::fs::read_to_string(&maps_path) {
        Ok(c) => c,
        Err(e) => {
            return ActionResult {
                pid,
                action: ActionType::MemoryDump,
                success: false,
                message: format!("Cannot read /proc/{pid}/maps: {e}"),
                timestamp: now,
            };
        }
    };

    let mut mem_file = match std::fs::File::open(&mem_path) {
        Ok(f) => f,
        Err(e) => {
            return ActionResult {
                pid,
                action: ActionType::MemoryDump,
                success: false,
                message: format!("Cannot open /proc/{pid}/mem: {e}"),
                timestamp: now,
            };
        }
    };

    // Create output directory if needed
    let _ = std::fs::create_dir_all(output_dir);

    let mut output = match std::fs::File::create(&dump_path) {
        Ok(f) => f,
        Err(e) => {
            return ActionResult {
                pid,
                action: ActionType::MemoryDump,
                success: false,
                message: format!("Cannot create dump file: {e}"),
                timestamp: now,
            };
        }
    };

    let mut total_bytes: u64 = 0;
    let mut regions_dumped = 0u32;

    for line in maps_content.lines() {
        let parts: Vec<&str> = line.splitn(6, char::is_whitespace).collect();
        if parts.len() < 2 {
            continue;
        }

        // Only dump readable regions
        if !parts[1].starts_with('r') {
            continue;
        }

        let addrs: Vec<&str> = parts[0].split('-').collect();
        if addrs.len() != 2 {
            continue;
        }

        let start = u64::from_str_radix(addrs[0], 16).unwrap_or(0);
        let end = u64::from_str_radix(addrs[1], 16).unwrap_or(0);
        let size = end - start;

        // Skip very large regions (>32 MiB)
        if size > 32 * 1024 * 1024 {
            continue;
        }

        if mem_file.seek(SeekFrom::Start(start)).is_err() {
            continue;
        }

        let mut buf = vec![0u8; size as usize];
        if mem_file.read_exact(&mut buf).is_ok() {
            let _ = output.write_all(&buf);
            total_bytes += size;
            regions_dumped += 1;
        }
    }

    ActionResult {
        pid,
        action: ActionType::MemoryDump,
        success: true,
        message: format!(
            "Dumped {regions_dumped} regions ({} bytes) to {dump_path}",
            total_bytes
        ),
        timestamp: now,
    }
}

#[cfg(not(target_os = "linux"))]
pub fn dump_memory(pid: u32, _output_dir: &str) -> ActionResult {
    ActionResult {
        pid,
        action: ActionType::MemoryDump,
        success: false,
        message: "Memory dump is only supported on Linux".to_string(),
        timestamp: timestamp_now(),
    }
}

#[cfg(not(unix))]
pub fn suspend(pid: u32) -> ActionResult {
    ActionResult {
        pid,
        action: ActionType::Suspend,
        success: false,
        message: "Suspend is only supported on Unix".to_string(),
        timestamp: timestamp_now(),
    }
}

#[cfg(not(unix))]
pub fn resume(pid: u32) -> ActionResult {
    ActionResult {
        pid,
        action: ActionType::Resume,
        success: false,
        message: "Resume is only supported on Unix".to_string(),
        timestamp: timestamp_now(),
    }
}

#[cfg(not(unix))]
pub fn kill_process(pid: u32) -> ActionResult {
    ActionResult {
        pid,
        action: ActionType::Kill,
        success: false,
        message: "Kill is only supported on Unix".to_string(),
        timestamp: timestamp_now(),
    }
}

fn timestamp_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}
