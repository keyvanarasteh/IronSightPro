use dioxus::prelude::*;
use crate::components::dashboards::*;

struct HeuristicSignal { pid: u32, name: &'static str, score: f64, entropy: f64, signed: bool, wx_violations: u32, level: &'static str }

#[component]
pub fn HeuristicMonitor() -> Element {
    let signals = vec![
        HeuristicSignal { pid: 9901, name: "unknown_agent", score: 88.5, entropy: 7.9, signed: false, wx_violations: 4, level: "critical" },
        HeuristicSignal { pid: 8812, name: "svchost.exe", score: 45.2, entropy: 6.8, signed: false, wx_violations: 1, level: "high" },
        HeuristicSignal { pid: 4492, name: "chrome.exe", score: 12.0, entropy: 5.1, signed: true, wx_violations: 0, level: "low" },
        HeuristicSignal { pid: 1204, name: "systemd", score: 5.0, entropy: 4.2, signed: true, wx_violations: 0, level: "low" },
        HeuristicSignal { pid: 3310, name: "temp_installer", score: 62.0, entropy: 7.2, signed: false, wx_violations: 2, level: "high" },
        HeuristicSignal { pid: 7700, name: "hidden_miner", score: 92.0, entropy: 7.8, signed: false, wx_violations: 3, level: "critical" },
    ];

    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                div { style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "⚡ HEURISTIC THREAT MONITOR" }
                    StatusBadge { label: "ELEVATED RISK".to_string(), severity: "high".to_string() }
                }
                div { class: "header-actions",
                    span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "Signal Processing Engine v2.1" }
                }
            }
            div { class: "main-content",
                div { class: "dash-stats-row",
                    DashStatCard { label: "Critical Signals".to_string(), value: "2".to_string(), icon: "🚨".to_string(), variant: "critical".to_string() }
                    DashStatCard { label: "High Risk".to_string(), value: "2".to_string(), icon: "⚠️".to_string(), variant: "high".to_string() }
                    DashStatCard { label: "W^X Violations".to_string(), value: "10".to_string(), icon: "🛡️".to_string(), variant: "warning".to_string() }
                    DashStatCard { label: "Processes Scanned".to_string(), value: "6".to_string(), icon: "🔬".to_string(), variant: "blue".to_string() }
                    DashStatCard { label: "Avg Entropy".to_string(), value: "6.50".to_string(), icon: "📈".to_string(), variant: "purple".to_string() }
                }
                div { class: "dash-grid-2",
                    GlassPanel { style: "padding: 16px;".to_string(),
                        PanelHeader { title: "Scoring Breakdown".to_string(),
                            span { style: "font-size: 10px; color: var(--text-muted); font-family: var(--font-mono);", "weights v2.1" }
                        }
                        DashProgress { label: "Entropy > 7.0 → +30".to_string(), value: 30.0, variant: "critical".to_string() }
                        DashProgress { label: "Unsigned Binary → +20".to_string(), value: 20.0, variant: "warning".to_string() }
                        DashProgress { label: "CPU Spike > 90% → +15".to_string(), value: 15.0, variant: "purple".to_string() }
                        DashProgress { label: "W^X Violation → +40".to_string(), value: 40.0, variant: "danger".to_string() }
                        DashProgress { label: "Suspicious Name → +50".to_string(), value: 50.0, variant: "critical".to_string() }
                    }
                    AlertList { title: "Heuristic Alerts".to_string(),
                        AlertCard { title: "KERNEL".to_string(), description: "VirtualProtect(RX) detected in PID 9901".to_string(), code: "E_MEM_01".to_string(), timestamp: "10:04:22".to_string(), severity: "critical".to_string() }
                        AlertCard { title: "STRINGS".to_string(), description: "Suspicious URL found in PID 9901 heap".to_string(), code: "E_STR_09".to_string(), timestamp: "10:04:25".to_string(), severity: "critical".to_string() }
                        AlertCard { title: "AUTH".to_string(), description: "Unsigned binary started: temp_installer.exe".to_string(), code: "W_SIG_02".to_string(), timestamp: "10:05:01".to_string(), severity: "medium".to_string() }
                    }
                }
                DashTable {
                    DashTableHead { tr { DashTh {"PID"} DashTh {"Process"} DashTh {"Score"} DashTh {"Entropy"} DashTh {"Signed"} DashTh {"W^X"} DashTh {"Level"} } }
                    DashTableBody {
                        for sig in signals.iter() {
                            DashTr {
                                DashTd { style: "color: var(--accent-blue);".to_string(), "{sig.pid}" }
                                DashTd { style: "color: var(--text-primary); font-weight: 600;".to_string(), "{sig.name}" }
                                DashTd { ScoreBar { score: sig.score } }
                                DashTd { style: if sig.entropy > 7.0 { "color: var(--accent-red); font-weight: 700;".to_string() } else { String::new() }, "{sig.entropy:.2}" }
                                DashTd {
                                    if sig.signed { span { style: "color: var(--accent-green); font-weight: 700;", "✓ Trusted" } }
                                    else { span { style: "color: var(--accent-red); font-weight: 700;", "✗ Unsigned" } }
                                }
                                DashTd {
                                    if sig.wx_violations > 0 { StatusBadge { label: format!("{} REGIONS", sig.wx_violations), severity: "critical".to_string() } }
                                    else { span { style: "color: var(--text-muted); font-size: 10px;", "None" } }
                                }
                                DashTd { StatusBadge { label: sig.level.to_uppercase(), severity: sig.level.to_string() } }
                            }
                        }
                    }
                }
            }
        }
    }
}
