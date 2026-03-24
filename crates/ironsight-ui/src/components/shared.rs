use dioxus::prelude::*;
use ironsight_heuristic::ThreatLevel;
use crate::bridge::ProcessAssessment;

#[component]
pub fn ProcessTable(assessments: Vec<ProcessAssessment>) -> Element {
    let visible: Vec<&ProcessAssessment> = assessments.iter().filter(|a| a.score > 0.0).take(100).collect();
    rsx! {
        div { class: "process-panel",
            div { class: "panel-header",
                span { "⚡ Threat Monitor — Top {visible.len()} Active Threats" }
                span { style: "color: var(--text-muted); font-size: 10px; text-transform: none;", "{assessments.len()} total processes" }
            }
            div { class: "process-table-wrap",
                table { class: "proc",
                    thead {
                        tr {
                            th { "PID" }
                            th { "PROCESS" }
                            th { "SCORE" }
                            th { "LEVEL" }
                            th { "CPU%" }
                            th { "RAM" }
                            th { "SIGNALS" }
                        }
                    }
                    tbody {
                        for proc in visible {
                            ProcessRow { proc: proc.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProcessRow(proc: ProcessAssessment) -> Element {
    let level_class = match proc.level {
        ThreatLevel::Clean => "level-clean",
        ThreatLevel::Low => "level-low",
        ThreatLevel::Medium => "level-medium",
        ThreatLevel::High => "level-high",
        ThreatLevel::Critical => "level-critical",
    };

    let bar_color = match proc.level {
        ThreatLevel::Clean => "var(--accent-green)",
        ThreatLevel::Low => "var(--accent-yellow)",
        ThreatLevel::Medium => "var(--accent-orange)",
        ThreatLevel::High => "var(--accent-red)",
        ThreatLevel::Critical => "#ff0000",
    };

    let bar_width = (proc.score.min(100.0) / 100.0 * 100.0) as u32;

    rsx! {
        tr {
            td { style: "color: var(--text-muted);", "{proc.pid}" }
            td {
                div { style: "font-weight: 600;", "{proc.name}" }
                if !proc.exe_path.is_empty() {
                    div { style: "color: var(--text-muted); font-size: 10px; max-width: 200px; overflow: hidden; text-overflow: ellipsis;", "{proc.exe_path}" }
                }
            }
            td {
                span { class: "score-bar",
                    span { class: "score-bar-fill", style: format!("width: {bar_width}%; background: {bar_color};") }
                }
                span { style: "font-weight: 700;", "{proc.score:.0}" }
            }
            td { span { class: format!("level-badge {level_class}"), "{proc.level.emoji()} {proc.level:?}" } }
            td { style: if proc.cpu_percent > 50.0 { "color: var(--accent-orange);" } else { "" }, "{proc.cpu_percent:.1}%" }
            td { "{proc.memory_mb:.1} MB" }
            td {
                for sig in &proc.signal_names {
                    span { class: "signal-tag", "{sig}" }
                }
            }
        }
    }
}
