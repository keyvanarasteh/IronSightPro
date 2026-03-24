//! Process filter — chainable query builder for process snapshots.

use std::cmp::Reverse;

use crate::{ProcStatus, ProcessInfo, ProcessSnapshot};

// ─────────────────────────────────────────────────────────────────────────────
// SortBy
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    Cpu,
    Memory,
    Pid,
    Name,
}

// ─────────────────────────────────────────────────────────────────────────────
// ProcessFilter — STEP 8: sort + limit support
// ─────────────────────────────────────────────────────────────────────────────

/// Chainable filter builder for querying snapshots.
///
/// ```rust,ignore
/// let results = ProcessFilter::new(&snapshot)
///     .name_contains("chrome")
///     .cpu_above(10.0)
///     .memory_above_mib(100.0)
///     .sort_by_cpu()
///     .limit(10)
///     .collect();
/// ```
pub struct ProcessFilter<'a> {
    snapshot: &'a ProcessSnapshot,
    predicates: Vec<Box<dyn Fn(&ProcessInfo) -> bool + 'a>>,
    sort: Option<SortBy>,
    result_limit: Option<usize>,
}

impl<'a> ProcessFilter<'a> {
    pub fn new(snapshot: &'a ProcessSnapshot) -> Self {
        Self {
            snapshot,
            predicates: Vec::new(),
            sort: None,
            result_limit: None,
        }
    }

    pub fn name_contains(mut self, s: &'a str) -> Self {
        self.predicates.push(Box::new(move |p| {
            p.name.to_lowercase().contains(&s.to_lowercase())
        }));
        self
    }

    pub fn cpu_above(mut self, pct: f32) -> Self {
        self.predicates
            .push(Box::new(move |p| p.cpu_percent > pct));
        self
    }

    pub fn memory_above_mib(mut self, mib: f64) -> Self {
        self.predicates
            .push(Box::new(move |p| p.memory_mib() > mib));
        self
    }

    pub fn status(mut self, status: ProcStatus) -> Self {
        self.predicates
            .push(Box::new(move |p| p.status == status));
        self
    }

    pub fn parent_pid(mut self, ppid: u32) -> Self {
        self.predicates
            .push(Box::new(move |p| p.parent_pid == Some(ppid)));
        self
    }

    // ── STEP 8: sort + limit ─────────────────────────────────────────────

    pub fn sort_by_cpu(mut self) -> Self {
        self.sort = Some(SortBy::Cpu);
        self
    }

    pub fn sort_by_memory(mut self) -> Self {
        self.sort = Some(SortBy::Memory);
        self
    }

    pub fn sort_by_pid(mut self) -> Self {
        self.sort = Some(SortBy::Pid);
        self
    }

    pub fn sort_by_name(mut self) -> Self {
        self.sort = Some(SortBy::Name);
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.result_limit = Some(n);
        self
    }

    pub fn collect(self) -> Vec<&'a ProcessInfo> {
        let mut results: Vec<&ProcessInfo> = self
            .snapshot
            .processes
            .values()
            .filter(|p| self.predicates.iter().all(|pred| pred(p)))
            .collect();

        // Apply sort
        if let Some(sort) = self.sort {
            match sort {
                SortBy::Cpu => {
                    results.sort_by(|a, b| {
                        b.cpu_percent
                            .partial_cmp(&a.cpu_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
                SortBy::Memory => {
                    results.sort_by_key(|p| Reverse(p.memory_bytes));
                }
                SortBy::Pid => {
                    results.sort_by_key(|p| p.pid);
                }
                SortBy::Name => {
                    results.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                }
            }
        }

        // Apply limit
        if let Some(limit) = self.result_limit {
            results.truncate(limit);
        }

        results
    }
}
