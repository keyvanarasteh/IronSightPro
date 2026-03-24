//! Process information — snapshot of a single process.

use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sysinfo::{Process, ProcessStatus};

// ─────────────────────────────────────────────────────────────────────────────
// ProcessInfo — snapshot of one process
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent_pid: Option<u32>,
    pub name: String,
    pub exe: Option<PathBuf>,
    pub cmd: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub status: ProcStatus,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub virtual_memory_bytes: u64,
    pub start_time: u64,
    pub run_time_secs: u64,
    pub threads: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    /// Captured at time of snapshot (now serializable via chrono).
    pub captured_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcStatus {
    Running,
    Sleeping,
    Zombie,
    Stopped,
    Dead,
    Idle,
    Unknown,
}

impl From<ProcessStatus> for ProcStatus {
    fn from(s: ProcessStatus) -> Self {
        match s {
            ProcessStatus::Run => Self::Running,
            ProcessStatus::Sleep => Self::Sleeping,
            ProcessStatus::Zombie => Self::Zombie,
            ProcessStatus::Stop => Self::Stopped,
            ProcessStatus::Dead => Self::Dead,
            ProcessStatus::Idle => Self::Idle,
            _ => Self::Unknown,
        }
    }
}

// ── Display impls (audit Step 10) ────────────────────────────────────────

impl fmt::Display for ProcStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcStatus::Running => write!(f, "Running"),
            ProcStatus::Sleeping => write!(f, "Sleeping"),
            ProcStatus::Zombie => write!(f, "Zombie"),
            ProcStatus::Stopped => write!(f, "Stopped"),
            ProcStatus::Dead => write!(f, "Dead"),
            ProcStatus::Idle => write!(f, "Idle"),
            ProcStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} (PID:{} CPU:{:.1}% MEM:{:.1}MiB {})",
            self.status,
            self.name,
            self.pid,
            self.cpu_percent,
            self.memory_mib(),
            self.exe
                .as_ref()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default(),
        )
    }
}

impl ProcessInfo {
    /// Build a `ProcessInfo` from sysinfo's `Process`.
    pub fn from_sysinfo(pid: u32, p: &Process) -> Self {
        Self {
            pid,
            parent_pid: p.parent().map(|pid| pid.as_u32()),
            name: p.name().to_string_lossy().into_owned(),
            exe: p.exe().map(PathBuf::from),
            cmd: p
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect(),
            cwd: p.cwd().map(PathBuf::from),
            status: p.status().into(),
            cpu_percent: p.cpu_usage(),
            memory_bytes: p.memory(),
            virtual_memory_bytes: p.virtual_memory(),
            start_time: p.start_time(),
            run_time_secs: p.run_time(),
            threads: p.tasks().map(|t| t.len() as u32),
            uid: p.user_id().map(|u| **u),
            gid: p.group_id().map(|g| *g),
            captured_at: Some(Utc::now()),
        }
    }

    /// Resident memory in MiB.
    pub fn memory_mib(&self) -> f64 {
        self.memory_bytes as f64 / 1024.0 / 1024.0
    }

    /// Age of the process.
    pub fn age(&self) -> Duration {
        Duration::from_secs(self.run_time_secs)
    }
}
