//! IronSight — Professional SecOps & Reverse Engineering Toolkit
//!
//! Full pipeline: Config → Privilege → Snapshot → Security → Network → Memory → Heuristic → Report
//!
//! Usage:
//!   ironsight                        Full system scan (auto-discovers config)
//!   ironsight --pid <PID>            Scan specific process
//!   ironsight --top <N>              Show top N threats (default: 10)
//!   ironsight --config <PATH>        Use specific config file
//!   ironsight --generate-config      Print default TOML config
//!   ironsight --check-privileges     Show privilege status and exit
//!   ironsight --watchdog-sentinel    Run as watchdog (internal)

mod config;
mod privilege;
mod watchdog;

use std::path::Path;

use sysinfo::{ProcessRefreshKind, System, UpdateKind};

use ironsight_core::snapshot::build_snapshot;
use ironsight_core::ProcessInfo;
use ironsight_heuristic::signals;
use ironsight_heuristic::{HeuristicEngine, Signal, ThreatLevel};
use ironsight_report::incident;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // ── Watchdog sentinel mode (early exit) ──
    if let Some(wd_args) = watchdog::parse_watchdog_args(&args) {
        tracing_subscriber::fmt()
            .with_env_filter("ironsight=info")
            .init();
        watchdog::run_sentinel_loop(wd_args.watch_pid, wd_args.interval, 5);
        return;
    }

    // ── Generate config (early exit) ──
    if args.iter().any(|a| a == "--generate-config") {
        print!("{}", config::Config::default_toml());
        return;
    }

    // ── Load configuration ──
    let cfg = if let Some(path) = parse_config_arg(&args) {
        match config::Config::from_file(Path::new(&path)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Config error: {e}");
                return;
            }
        }
    } else {
        config::Config::discover()
    };

    // ── Initialize logging from config ──
    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(
            format!("ironsight={}", cfg.general.log_level)
                .parse()
                .unwrap_or_else(|_| "ironsight=info".parse().unwrap()),
        );
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    // ── Banner ──
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔬 IRONSIGHT — SecOps Forensics Toolkit           ║");
    println!("║          v0.1.0 · by Keyvan Arasteh                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // ── Privilege check ──
    let priv_report = privilege::PrivilegeReport::check();
    priv_report.display();

    if args.iter().any(|a| a == "--check-privileges") {
        return;
    }

    if priv_report.overall_level == privilege::PrivilegeLevel::Limited {
        tracing::warn!("Running with limited privileges — some features may be degraded");
    }

    // ── Watchdog spawn ──
    if cfg.watchdog.enabled {
        let my_pid = std::process::id();
        match watchdog::spawn_sentinel(
            my_pid,
            std::time::Duration::from_secs(cfg.watchdog.interval_secs),
        ) {
            Ok(sentinel_pid) => {
                tracing::info!("🐕 Watchdog sentinel spawned as PID {sentinel_pid}");
            }
            Err(e) => {
                tracing::warn!("Failed to spawn watchdog: {e}");
            }
        }
    }

    // ── Parse scan arguments ──
    let target_pid = parse_pid_arg(&args);
    let top_n = parse_top_arg(&args).unwrap_or(cfg.scan.top_n);

    // ─────────────────────────────────────────────────────────────────
    // Stage 1: Process Snapshot
    // ─────────────────────────────────────────────────────────────────
    println!("📸 Stage 1: Capturing process snapshot...");
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_cpu()
            .with_memory()
            .with_exe(UpdateKind::OnlyIfNotSet)
            .with_cmd(UpdateKind::OnlyIfNotSet)
            .with_cwd(UpdateKind::OnlyIfNotSet)
            .with_user(UpdateKind::OnlyIfNotSet),
    );
    let snapshot = build_snapshot(&sys);
    println!("   Found {} processes", snapshot.processes.len());

    let processes: Vec<&ProcessInfo> = if let Some(pid) = target_pid {
        match snapshot.find_by_pid(pid) {
            Some(p) => vec![p],
            None => {
                eprintln!("❌ PID {pid} not found");
                return;
            }
        }
    } else {
        snapshot.processes.values().collect()
    };

    // ─────────────────────────────────────────────────────────────────
    // Stage 2–5: Analyze each process
    // ─────────────────────────────────────────────────────────────────
    let engine = HeuristicEngine::new();
    let mut assessments: Vec<(ironsight_heuristic::ThreatAssessment, incident::IncidentReport)> =
        Vec::new();

    for proc_info in &processes {
        // Check exclusions
        if is_excluded(proc_info, &cfg.exclusions) {
            continue;
        }

        let mut sigs: Vec<Signal> = Vec::new();
        let mut report = ironsight_report::IncidentReport::new();

        // Fill process info
        report.process = incident::ProcessInfo {
            pid: proc_info.pid,
            name: proc_info.name.clone(),
            exe_path: proc_info
                .exe
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            cmdline: if proc_info.cmd.is_empty() {
                None
            } else {
                Some(proc_info.cmd.join(" "))
            },
            parent_pid: proc_info.parent_pid,
            user: proc_info.uid.map(|u| format!("uid:{u}")),
            cpu_percent: proc_info.cpu_percent,
            memory_bytes: proc_info.memory_bytes,
            start_time: None,
        };

        // ── Security Analysis ──
        if cfg.scan.security {
            if let Some(ref exe_path) = proc_info.exe {
                let path = Path::new(exe_path);

                // Entropy
                if let Ok(result) = ironsight_security::entropy::compute_entropy(path) {
                    report.security.entropy = Some(result.entropy);
                    report.security.entropy_risk = Some(format!("{:?}", result.risk_level));
                    if result.entropy > cfg.thresholds.entropy_alert {
                        sigs.push(signals::high_entropy(result.entropy));
                    }
                }

                // Hash
                if let Ok(hash_result) = ironsight_security::hash::compute_sha256(path) {
                    report.security.sha256 = Some(hash_result.sha256);
                }

                // Signature
                let sig_result = ironsight_security::signature::verify_signature(path);
                report.security.is_signed = sig_result.is_signed;
                if sig_result.is_signed == Some(false) {
                    sigs.push(signals::unsigned_binary());
                }

                // Path analysis
                let path_result = ironsight_security::path_analysis::analyze_path(Some(path));
                report.security.suspicious_path = path_result.is_suspicious;
                if let Some(ref reason) = path_result.reason {
                    report.security.flags.push(reason.clone());
                }
                if path_result.is_suspicious {
                    let reason = path_result.reason.as_deref().unwrap_or("unknown");
                    sigs.push(signals::suspicious_path(&path.to_string_lossy(), reason));
                }
            } else {
                sigs.push(signals::fileless_process());
            }
        }

        // CPU spike
        if proc_info.cpu_percent > cfg.thresholds.cpu_spike {
            sigs.push(signals::cpu_spike(proc_info.cpu_percent));
        }

        // ── Network Analysis ──
        if cfg.scan.network {
            let net_audit = ironsight_network::audit::NetworkAudit::scan_pid(proc_info.pid);
            report.network = incident::NetworkInfo {
                total_sockets: net_audit.total_sockets,
                listeners: net_audit.listeners.len(),
                external_connections: net_audit.external_connections.len(),
                suspicious_connections: net_audit.suspicious_connections.len(),
                suspicious_ports: net_audit
                    .suspicious_connections
                    .iter()
                    .map(|s| format!("{} ({})", s.socket.remote_port, s.intel.service))
                    .collect(),
            };

            for conn in &net_audit.suspicious_connections {
                sigs.push(signals::suspicious_port(
                    conn.socket.remote_port,
                    &conn.intel.service,
                ));
            }

            if net_audit.external_connections.len() > 5 {
                sigs.push(signals::external_connections(
                    net_audit.external_connections.len(),
                ));
            }
        }

        // ── Memory Analysis ──
        if cfg.scan.memory {
            if let Ok(regions) = ironsight_memory::maps::read_maps(proc_info.pid) {
                let summary = ironsight_memory::maps::summarize(proc_info.pid, &regions);
                report.memory = incident::MemoryInfo {
                    total_regions: summary.total_regions,
                    wx_violations: summary.writable_executable_regions,
                    anonymous_executable: summary.anonymous_executable_regions,
                    pattern_matches: 0,
                    flags: summary.flags.clone(),
                };

                if summary.writable_executable_regions > 0 {
                    sigs.push(signals::wx_violation(
                        summary.writable_executable_regions,
                    ));
                }
                if summary.anonymous_executable_regions > 0 {
                    sigs.push(signals::anonymous_executable(
                        summary.anonymous_executable_regions,
                    ));
                }
            }
        }

        // ── Heuristic Scoring ──
        let assessment = engine.assess(proc_info.pid, &proc_info.name, sigs);

        report.threat = incident::ThreatInfo {
            score: assessment.score,
            level: format!("{:?}", assessment.level),
            signals: assessment
                .signals
                .iter()
                .map(|s| incident::SignalInfo {
                    name: s.name.clone(),
                    category: format!("{:?}", s.category),
                    weight: s.weight,
                    description: s.description.clone(),
                    evidence: s.evidence.clone(),
                })
                .collect(),
            recommended_action: format!("{:?}", assessment.recommended_action),
        };

        assessments.push((assessment, report));
    }

    // ─────────────────────────────────────────────────────────────────
    // Stage 6: Display results
    // ─────────────────────────────────────────────────────────────────
    // Sort by threat score (highest first)
    assessments.sort_by(|a, b| {
        b.0.score
            .partial_cmp(&a.0.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Summary
    let critical = assessments
        .iter()
        .filter(|(a, _)| a.level == ThreatLevel::Critical)
        .count();
    let high = assessments
        .iter()
        .filter(|(a, _)| a.level == ThreatLevel::High)
        .count();
    let medium = assessments
        .iter()
        .filter(|(a, _)| a.level == ThreatLevel::Medium)
        .count();

    println!();
    println!("── Scan Summary ───────────────────────────────────────────────");
    println!("  Total Processes: {}", assessments.len());
    println!("  🚨 Critical:     {critical}");
    println!("  🔴 High:         {high}");
    println!("  🟠 Medium:       {medium}");
    println!();

    // Top threats
    println!(
        "── Top {top_n} Threats ─────────────────────────────────────────────"
    );
    println!(
        "  {:<8} {:<20} {:>6} {:<10} {}",
        "PID", "NAME", "SCORE", "LEVEL", "SIGNALS"
    );
    println!("  {}", "─".repeat(70));

    for (assessment, _report) in assessments.iter().take(top_n) {
        if assessment.score < 1.0 {
            continue;
        }

        let signal_names: Vec<&str> = assessment
            .signals
            .iter()
            .map(|s| s.name.as_str())
            .collect();

        println!(
            "  {:<8} {:<20} {:>5.0} {} {:<10} {}",
            assessment.pid,
            truncate(&assessment.name, 20),
            assessment.score,
            assessment.level.emoji(),
            format!("{:?}", assessment.level),
            signal_names.join(", ")
        );
    }

    // Full reports for Critical/High threats
    let serious: Vec<&(ironsight_heuristic::ThreatAssessment, incident::IncidentReport)> =
        assessments
            .iter()
            .filter(|(a, _)| a.level >= ThreatLevel::High)
            .collect();

    if !serious.is_empty() {
        println!();
        println!("── Detailed Reports (High/Critical) ───────────────────────────\n");
        for (_, report) in serious {
            println!("{}", ironsight_report::to_text(report));
        }
    }

    // Save JSON reports
    let report_dir = &cfg.general.report_dir;
    let _ = std::fs::create_dir_all(report_dir);
    let mut saved = 0;
    for (assessment, report) in &assessments {
        if assessment.score >= cfg.thresholds.export_min_score {
            let path = format!("{report_dir}/incident_{}.json", report.process.pid);
            if ironsight_report::save_json(report, &path).is_ok() {
                saved += 1;
            }
        }
    }
    if saved > 0 {
        println!("  💾 Saved {saved} incident reports to {report_dir}/");
    }

    println!();
    println!("Done. IronSight scan complete.");
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn is_excluded(proc: &ProcessInfo, exclusions: &config::ExclusionConfig) -> bool {
    if exclusions.pids.contains(&proc.pid) {
        return true;
    }
    if exclusions
        .names
        .iter()
        .any(|n| proc.name.to_lowercase() == n.to_lowercase())
    {
        return true;
    }
    if let Some(ref exe) = proc.exe {
        let exe_str = exe.to_string_lossy();
        if exclusions.paths.iter().any(|p| exe_str.starts_with(p)) {
            return true;
        }
    }
    false
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max - 1])
    } else {
        s.to_string()
    }
}

fn parse_pid_arg(args: &[String]) -> Option<u32> {
    args.windows(2).find_map(|w| {
        if w[0] == "--pid" {
            w[1].parse().ok()
        } else {
            None
        }
    })
}

fn parse_top_arg(args: &[String]) -> Option<usize> {
    args.windows(2).find_map(|w| {
        if w[0] == "--top" {
            w[1].parse().ok()
        } else {
            None
        }
    })
}

fn parse_config_arg(args: &[String]) -> Option<String> {
    args.windows(2).find_map(|w| {
        if w[0] == "--config" {
            Some(w[1].clone())
        } else {
            None
        }
    })
}
