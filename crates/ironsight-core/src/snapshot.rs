//! Process snapshot — collection of all processes at one point in time.

use std::cmp::Reverse;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::ProcessInfo;

// ─────────────────────────────────────────────────────────────────────────────
// ProcessSnapshot — STEP 2: Instant → DateTime<Utc> for serialization
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub processes: HashMap<u32, ProcessInfo>,
    pub taken_at: DateTime<Utc>,
    pub system_total_memory: u64,
    pub system_used_memory: u64,
    pub cpu_count: usize,
}

impl ProcessSnapshot {
    /// How long ago this snapshot was taken.
    pub fn elapsed(&self) -> chrono::Duration {
        Utc::now() - self.taken_at
    }

    /// List processes sorted by CPU descending.
    pub fn by_cpu(&self) -> Vec<&ProcessInfo> {
        let mut v: Vec<&ProcessInfo> = self.processes.values().collect();
        v.sort_by(|a, b| {
            b.cpu_percent
                .partial_cmp(&a.cpu_percent)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        v
    }

    /// List processes sorted by memory descending.
    pub fn by_memory(&self) -> Vec<&ProcessInfo> {
        let mut v: Vec<&ProcessInfo> = self.processes.values().collect();
        v.sort_by_key(|p| Reverse(p.memory_bytes));
        v
    }

    /// Find all processes whose name contains `substring` (case-insensitive).
    pub fn find_by_name(&self, substring: &str) -> Vec<&ProcessInfo> {
        let lower = substring.to_lowercase();
        self.processes
            .values()
            .filter(|p| p.name.to_lowercase().contains(&lower))
            .collect()
    }

    /// Find by exact PID.
    pub fn find_by_pid(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.get(&pid)
    }

    /// Find all processes whose exe path contains `substring`.
    pub fn find_by_path(&self, substring: &str) -> Vec<&ProcessInfo> {
        let lower = substring.to_lowercase();
        self.processes
            .values()
            .filter(|p| {
                p.exe
                    .as_ref()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase().contains(&lower))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Build a parent → children tree.
    pub fn tree(&self) -> HashMap<u32, Vec<u32>> {
        let mut tree: HashMap<u32, Vec<u32>> = HashMap::new();
        for p in self.processes.values() {
            if let Some(ppid) = p.parent_pid {
                tree.entry(ppid).or_default().push(p.pid);
            }
        }
        tree
    }

    /// Return children of a given pid.
    pub fn children_of(&self, pid: u32) -> Vec<&ProcessInfo> {
        self.processes
            .values()
            .filter(|p| p.parent_pid == Some(pid))
            .collect()
    }

    /// Total CPU across all processes.
    pub fn total_cpu(&self) -> f32 {
        self.processes.values().map(|p| p.cpu_percent).sum()
    }

    /// Total resident memory across all processes.
    pub fn total_memory_bytes(&self) -> u64 {
        self.processes.values().map(|p| p.memory_bytes).sum()
    }
}

/// Build a snapshot from the current system state.
pub fn build_snapshot(sys: &System) -> ProcessSnapshot {
    let processes = sys
        .processes()
        .iter()
        .map(|(pid, proc_)| (pid.as_u32(), ProcessInfo::from_sysinfo(pid.as_u32(), proc_)))
        .collect();

    ProcessSnapshot {
        processes,
        taken_at: Utc::now(),
        system_total_memory: sys.total_memory(),
        system_used_memory: sys.used_memory(),
        cpu_count: sys.cpus().len(),
    }
}
