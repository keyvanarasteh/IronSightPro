//! Report formatters — JSON and human-readable text output.

use crate::incident::IncidentReport;

/// Format a report as pretty-printed JSON.
pub fn to_json(report: &IncidentReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

/// Format a report as compact JSON (single line, SIEM-ready).
pub fn to_json_compact(report: &IncidentReport) -> Result<String, serde_json::Error> {
    serde_json::to_string(report)
}

/// Format a report as a human-readable text summary.
pub fn to_text(report: &IncidentReport) -> String {
    let mut out = String::new();

    out.push_str("╔══════════════════════════════════════════════════════════════╗\n");
    out.push_str("║                    IRONSIGHT INCIDENT REPORT                ║\n");
    out.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

    // Header
    out.push_str(&format!("  Report ID:  {}\n", report.id));
    out.push_str(&format!("  Timestamp:  {}\n", report.timestamp));
    out.push_str(&format!("  Hostname:   {}\n", report.hostname));
    out.push_str("\n");

    // Process
    out.push_str("── Process ─────────────────────────────────────────────────────\n");
    out.push_str(&format!("  PID:        {}\n", report.process.pid));
    out.push_str(&format!("  Name:       {}\n", report.process.name));
    if let Some(ref path) = report.process.exe_path {
        out.push_str(&format!("  Path:       {path}\n"));
    }
    if let Some(ref cmd) = report.process.cmdline {
        out.push_str(&format!("  Cmdline:    {cmd}\n"));
    }
    out.push_str(&format!("  CPU:        {:.1}%\n", report.process.cpu_percent));
    out.push_str(&format!(
        "  Memory:     {} MiB\n",
        report.process.memory_bytes / (1024 * 1024)
    ));
    out.push_str("\n");

    // Threat
    let level_emoji = match report.threat.level.as_str() {
        "Clean" => "✅",
        "Low" => "🟡",
        "Medium" => "🟠",
        "High" => "🔴",
        "Critical" => "🚨",
        _ => "❓",
    };
    out.push_str("── Threat Assessment ───────────────────────────────────────────\n");
    out.push_str(&format!(
        "  Score:      {:.0}/100 {} {}\n",
        report.threat.score, level_emoji, report.threat.level
    ));
    out.push_str(&format!(
        "  Action:     {}\n",
        report.threat.recommended_action
    ));

    if !report.threat.signals.is_empty() {
        out.push_str("  Signals:\n");
        for signal in &report.threat.signals {
            out.push_str(&format!(
                "    [{:>5.0}] {} — {}\n",
                signal.weight, signal.name, signal.description
            ));
            if let Some(ref evidence) = signal.evidence {
                out.push_str(&format!("            Evidence: {evidence}\n"));
            }
        }
    }
    out.push_str("\n");

    // Security
    out.push_str("── Security Analysis ───────────────────────────────────────────\n");
    if let Some(ref hash) = report.security.sha256 {
        out.push_str(&format!("  SHA-256:    {hash}\n"));
    }
    if let Some(entropy) = report.security.entropy {
        out.push_str(&format!(
            "  Entropy:    {entropy:.4} ({})\n",
            report.security.entropy_risk.as_deref().unwrap_or("N/A")
        ));
    }
    if let Some(signed) = report.security.is_signed {
        out.push_str(&format!(
            "  Signed:     {}\n",
            if signed { "✅ Yes" } else { "❌ No" }
        ));
    }
    for flag in &report.security.flags {
        out.push_str(&format!("  ⚠ {flag}\n"));
    }
    out.push_str("\n");

    // Network
    out.push_str("── Network Posture ────────────────────────────────────────────\n");
    out.push_str(&format!(
        "  Sockets:    {} total, {} listeners\n",
        report.network.total_sockets, report.network.listeners
    ));
    out.push_str(&format!(
        "  External:   {} connections\n",
        report.network.external_connections
    ));
    out.push_str(&format!(
        "  Suspicious: {} connections\n",
        report.network.suspicious_connections
    ));
    for port in &report.network.suspicious_ports {
        out.push_str(&format!("  🔴 {port}\n"));
    }
    out.push_str("\n");

    // Memory
    out.push_str("── Memory Analysis ────────────────────────────────────────────\n");
    out.push_str(&format!(
        "  Regions:    {} total\n",
        report.memory.total_regions
    ));
    out.push_str(&format!(
        "  W^X:        {} violations\n",
        report.memory.wx_violations
    ));
    out.push_str(&format!(
        "  Anon Exec:  {} regions\n",
        report.memory.anonymous_executable
    ));
    out.push_str(&format!(
        "  Patterns:   {} matches\n",
        report.memory.pattern_matches
    ));
    for flag in &report.memory.flags {
        out.push_str(&format!("  ⚠ {flag}\n"));
    }
    out.push_str("\n");

    // Actions
    if !report.actions.is_empty() {
        out.push_str("── Actions Taken ──────────────────────────────────────────────\n");
        for action in &report.actions {
            let status = if action.success { "✅" } else { "❌" };
            out.push_str(&format!(
                "  {status} [{}] {} — {}\n",
                action.action_type, action.message, action.timestamp
            ));
        }
        out.push_str("\n");
    }

    out.push_str("═══════════════════════════════════════════════════════════════\n");
    out
}

/// Save a report to a JSON file.
pub fn save_json(report: &IncidentReport, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_json(report)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incident::*;

    fn sample_report() -> IncidentReport {
        let mut report = IncidentReport::new();
        report.process = ProcessInfo {
            pid: 666,
            name: "evil_payload".into(),
            exe_path: Some("/tmp/evil_payload".into()),
            cmdline: Some("/tmp/evil_payload --reverse-shell".into()),
            parent_pid: Some(1),
            user: Some("nobody".into()),
            cpu_percent: 95.0,
            memory_bytes: 128 * 1024 * 1024,
            start_time: None,
        };
        report.threat = ThreatInfo {
            score: 85.0,
            level: "Critical".into(),
            signals: vec![
                SignalInfo {
                    name: "HIGH_ENTROPY".into(),
                    category: "StaticAnalysis".into(),
                    weight: 30.0,
                    description: "Packed binary".into(),
                    evidence: Some("7.82".into()),
                },
                SignalInfo {
                    name: "SUSPICIOUS_PORT".into(),
                    category: "NetworkBehavior".into(),
                    weight: 25.0,
                    description: "Connection to port 4444".into(),
                    evidence: Some("4444".into()),
                },
            ],
            recommended_action: "SuspendDumpKill".into(),
        };
        report.security = SecurityInfo {
            sha256: Some("abcdef1234567890".into()),
            entropy: Some(7.82),
            entropy_risk: Some("Critical".into()),
            is_signed: Some(false),
            suspicious_path: true,
            flags: vec!["Running from /tmp/".into()],
        };
        report.network = NetworkInfo {
            total_sockets: 3,
            listeners: 1,
            external_connections: 2,
            suspicious_connections: 1,
            suspicious_ports: vec!["4444 (Metasploit)".into()],
        };
        report.memory = MemoryInfo {
            total_regions: 42,
            wx_violations: 1,
            anonymous_executable: 1,
            pattern_matches: 2,
            flags: vec!["W^X violation".into()],
        };
        report
    }

    #[test]
    fn json_roundtrip() {
        let report = sample_report();
        let json = to_json(&report).unwrap();
        let parsed: IncidentReport = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.process.pid, 666);
        assert_eq!(parsed.threat.score, 85.0);
    }

    #[test]
    fn compact_json_is_single_line() {
        let report = sample_report();
        let json = to_json_compact(&report).unwrap();
        assert!(!json.contains('\n'));
    }

    #[test]
    fn text_report_contains_sections() {
        let report = sample_report();
        let text = to_text(&report);
        assert!(text.contains("IRONSIGHT INCIDENT REPORT"));
        assert!(text.contains("evil_payload"));
        assert!(text.contains("85/100"));
        assert!(text.contains("Critical"));
        assert!(text.contains("W^X violation"));
        assert!(text.contains("4444 (Metasploit)"));
    }

    #[test]
    fn text_report_shows_signals() {
        let report = sample_report();
        let text = to_text(&report);
        assert!(text.contains("HIGH_ENTROPY"));
        assert!(text.contains("SUSPICIOUS_PORT"));
    }

    #[test]
    fn save_json_file() {
        let report = sample_report();
        let path = "/tmp/ironsight_test_report.json";
        save_json(&report, path).unwrap();
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("evil_payload"));
        let _ = std::fs::remove_file(path);
    }
}
