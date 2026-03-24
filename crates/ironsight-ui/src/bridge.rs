//! Backend ↔ UI channel bridge.
//!
//! Runs the scanning pipeline on a background thread and sends results
//! to the UI via a channel.

// use std::sync::{Arc, Mutex};

use sysinfo::{ProcessRefreshKind, System, UpdateKind};

use ironsight_core::snapshot::build_snapshot;
use ironsight_core::ProcessInfo;
use ironsight_heuristic::signals;
use ironsight_heuristic::{HeuristicEngine, Signal, ThreatAssessment, ThreatLevel};

/// Snapshot of the current system state for the UI.
#[derive(Debug, Clone, PartialEq)]
pub struct UiSnapshot {
    pub assessments: Vec<ProcessAssessment>,
    pub total_processes: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub clean_count: usize,
    pub risk_index: f64,
    pub scan_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessAssessment {
    pub pid: u32,
    pub name: String,
    pub exe_path: String,
    pub cpu_percent: f32,
    pub memory_mb: f64,
    pub score: f64,
    pub level: ThreatLevel,
    pub signal_names: Vec<String>,
    pub signal_count: usize,
}

/// Run a full scan and return a UI-ready snapshot.
pub fn run_scan() -> UiSnapshot {
    let start = std::time::Instant::now();

    let mut sys = System::new();
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_cpu()
            .with_memory()
            .with_exe(UpdateKind::OnlyIfNotSet)
            .with_cmd(UpdateKind::OnlyIfNotSet),
    );

    let snapshot = build_snapshot(&sys);
    let engine = HeuristicEngine::new();

    let mut assessments: Vec<(ThreatAssessment, &ProcessInfo)> = Vec::new();

    for proc_info in snapshot.processes.values() {
        let mut sigs: Vec<Signal> = Vec::new();

        // Security signals
        if let Some(ref exe_path) = proc_info.exe {
            let path = std::path::Path::new(exe_path);

            if let Ok(result) = ironsight_security::entropy::compute_entropy(path) {
                if result.entropy > 7.0 {
                    sigs.push(signals::high_entropy(result.entropy));
                }
            }

            let sig_result = ironsight_security::signature::verify_signature(path);
            if sig_result.is_signed == Some(false) {
                sigs.push(signals::unsigned_binary());
            }

            let path_result = ironsight_security::path_analysis::analyze_path(Some(path));
            if path_result.is_suspicious {
                let reason = path_result.reason.as_deref().unwrap_or("suspicious");
                sigs.push(signals::suspicious_path(&path.to_string_lossy(), reason));
            }
        } else {
            sigs.push(signals::fileless_process());
        }

        // CPU spike
        if proc_info.cpu_percent > 90.0 {
            sigs.push(signals::cpu_spike(proc_info.cpu_percent));
        }

        // Memory analysis
        if let Ok(regions) = ironsight_memory::maps::read_maps(proc_info.pid) {
            let summary = ironsight_memory::maps::summarize(proc_info.pid, &regions);
            if summary.writable_executable_regions > 0 {
                sigs.push(signals::wx_violation(summary.writable_executable_regions));
            }
            if summary.anonymous_executable_regions > 0 {
                sigs.push(signals::anonymous_executable(summary.anonymous_executable_regions));
            }
        }

        let assessment = engine.assess(proc_info.pid, &proc_info.name, sigs);
        assessments.push((assessment, proc_info));
    }

    // Sort by score descending
    assessments.sort_by(|a, b| {
        b.0.score
            .partial_cmp(&a.0.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let total = assessments.len();
    let critical = assessments.iter().filter(|(a, _)| a.level == ThreatLevel::Critical).count();
    let high = assessments.iter().filter(|(a, _)| a.level == ThreatLevel::High).count();
    let medium = assessments.iter().filter(|(a, _)| a.level == ThreatLevel::Medium).count();
    let low = assessments.iter().filter(|(a, _)| a.level == ThreatLevel::Low).count();
    let clean = assessments.iter().filter(|(a, _)| a.level == ThreatLevel::Clean).count();

    // Compute risk index
    let risk_sum: f64 = assessments.iter().map(|(a, _)| a.score).sum();
    let risk_index = if total > 0 { risk_sum / total as f64 } else { 0.0 };

    let ui_assessments: Vec<ProcessAssessment> = assessments
        .into_iter()
        .map(|(a, p)| ProcessAssessment {
            pid: a.pid,
            name: a.name.clone(),
            exe_path: p.exe.as_ref().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default(),
            cpu_percent: p.cpu_percent,
            memory_mb: p.memory_bytes as f64 / (1024.0 * 1024.0),
            score: a.score,
            level: a.level,
            signal_names: a.signals.iter().map(|s| s.name.clone()).collect(),
            signal_count: a.signals.len(),
        })
        .collect();

    UiSnapshot {
        assessments: ui_assessments,
        total_processes: total,
        critical_count: critical,
        high_count: high,
        medium_count: medium,
        low_count: low,
        clean_count: clean,
        risk_index,
        scan_time_ms: start.elapsed().as_millis() as u64,
    }
}
