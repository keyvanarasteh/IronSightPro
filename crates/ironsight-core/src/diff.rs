//! Process diff — what changed between two snapshots.

use crate::{ProcessInfo, ProcessSnapshot};

// ─────────────────────────────────────────────────────────────────────────────
// ProcessDiff
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ProcessDiff {
    /// PIDs that appeared in the new snapshot but not the old.
    pub spawned: Vec<ProcessInfo>,
    /// PIDs that were in the old snapshot but not the new (exited).
    pub exited: Vec<ProcessInfo>,
    /// PIDs present in both but with notable changes.
    pub changed: Vec<ProcessChange>,
}

#[derive(Debug, Clone)]
pub struct ProcessChange {
    pub pid: u32,
    pub old: ProcessInfo,
    pub new: ProcessInfo,
    pub cpu_delta: f32,
    pub memory_delta_bytes: i64,
    pub status_changed: bool,
}

impl ProcessDiff {
    /// Compute diff between `old` and `new` snapshots.
    pub fn compute(old: &ProcessSnapshot, new: &ProcessSnapshot) -> Self {
        let mut spawned = Vec::new();
        let mut exited = Vec::new();
        let mut changed = Vec::new();

        // Spawned: in new but not old
        for (pid, info) in &new.processes {
            if !old.processes.contains_key(pid) {
                spawned.push(info.clone());
            }
        }

        // Exited: in old but not new
        for (pid, info) in &old.processes {
            if !new.processes.contains_key(pid) {
                exited.push(info.clone());
            }
        }

        // Changed: in both
        for (pid, new_info) in &new.processes {
            if let Some(old_info) = old.processes.get(pid) {
                let cpu_delta = new_info.cpu_percent - old_info.cpu_percent;
                let mem_delta =
                    new_info.memory_bytes as i64 - old_info.memory_bytes as i64;
                let status_changed = new_info.status != old_info.status;

                // Only report if something meaningful changed
                if cpu_delta.abs() > 1.0
                    || mem_delta.abs() > 1024 * 1024
                    || status_changed
                {
                    changed.push(ProcessChange {
                        pid: *pid,
                        old: old_info.clone(),
                        new: new_info.clone(),
                        cpu_delta,
                        memory_delta_bytes: mem_delta,
                        status_changed,
                    });
                }
            }
        }

        spawned.sort_by_key(|p| p.pid);
        exited.sort_by_key(|p| p.pid);

        Self {
            spawned,
            exited,
            changed,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.spawned.is_empty() && self.exited.is_empty() && self.changed.is_empty()
    }
}
