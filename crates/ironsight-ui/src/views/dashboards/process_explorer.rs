use dioxus::prelude::*;
use crate::components::dashboards::*;

/// Process Explorer — Security Overview (from Process.tsx)
#[component]
pub fn ProcessExplorer() -> Element {
    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                div { style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: 600;", "🔬 PROCESS EXPLORER" }
                    StatusBadge { label: "6 MONITORED".to_string(), severity: "info".to_string() }
                }
            }
            div { class: "main-content",
                div { class: "dash-stats-row",
                    DashStatCard { label: "Total Processes".to_string(), value: "6".to_string(), icon: "⚙️".to_string(), variant: "blue".to_string() }
                    DashStatCard { label: "Threats Detected".to_string(), value: "2".to_string(), icon: "🚨".to_string(), variant: "critical".to_string() }
                    DashStatCard { label: "Avg Heuristic".to_string(), value: "51.5".to_string(), icon: "📊".to_string(), variant: "high".to_string() }
                    DashStatCard { label: "Unsigned".to_string(), value: "3".to_string(), icon: "✗".to_string(), variant: "warning".to_string() }
                }
                div { class: "dash-grid-2",
                    GlassPanel { style: "padding: 20px;".to_string(),
                        PanelHeader { title: "Heuristic Score Distribution".to_string(), span {} }
                        DashProgress { label: "unknown_agent".to_string(), value: 88.5, value_text: "88.5".to_string(), variant: "critical".to_string() }
                        DashProgress { label: "hidden_miner".to_string(), value: 92.0, value_text: "92.0".to_string(), variant: "danger".to_string() }
                        DashProgress { label: "svchost.exe".to_string(), value: 45.2, value_text: "45.2".to_string(), variant: "warning".to_string() }
                        DashProgress { label: "temp_installer".to_string(), value: 62.0, value_text: "62.0".to_string(), variant: "warning".to_string() }
                        DashProgress { label: "chrome.exe".to_string(), value: 12.0, value_text: "12.0".to_string(), variant: "success".to_string() }
                        DashProgress { label: "systemd".to_string(), value: 5.0, value_text: "5.0".to_string(), variant: "success".to_string() }
                    }
                    AlertList { title: "Threat Intelligence Feed".to_string(),
                        AlertCard { title: "CRITICAL".to_string(), description: "Reverse shell signature matched in PID 9901".to_string(), severity: "critical".to_string(), timestamp: "10:04:22".to_string() }
                        AlertCard { title: "HIGH".to_string(), description: "Cryptominer heuristic pattern in PID 7700".to_string(), severity: "high".to_string(), timestamp: "10:05:01".to_string() }
                        AlertCard { title: "WARNING".to_string(), description: "Entropy anomaly detected in PID 8812".to_string(), severity: "medium".to_string(), timestamp: "10:03:55".to_string() }
                    }
                }
                DashTable {
                    DashTableHead { tr { DashTh {"PID"} DashTh {"Process"} DashTh {"CPU %"} DashTh {"Memory"} DashTh {"Score"} DashTh {"Signed"} DashTh {"Status"} } }
                    DashTableBody {
                        for (pid, name, cpu, mem, score, signed, status) in [
                            (9901u32, "unknown_agent", "12.5%", "120 MB", 88.5f64, false, "critical"),
                            (7700, "hidden_miner", "95.0%", "80 MB", 92.0, false, "critical"),
                            (8812, "svchost.exe", "0.5%", "24 MB", 45.2, false, "high"),
                            (3310, "temp_installer", "3.0%", "32 MB", 62.0, false, "high"),
                            (4492, "chrome.exe", "4.2%", "450 MB", 12.0, true, "low"),
                            (1204, "systemd", "0.1%", "12 MB", 5.0, true, "low"),
                        ] {
                            DashTr {
                                DashTd { style: "color: var(--accent-blue);".to_string(), "{pid}" }
                                DashTd { style: "font-weight: 600; color: var(--text-primary);".to_string(), "{name}" }
                                DashTd { "{cpu}" }
                                DashTd { "{mem}" }
                                DashTd { ScoreBar { score: score } }
                                DashTd {
                                    if signed { span { style: "color: var(--accent-green); font-weight: 700;", "✓" } }
                                    else { span { style: "color: var(--accent-red); font-weight: 700;", "✗" } }
                                }
                                DashTd { StatusBadge { label: status.to_uppercase(), severity: status.to_string() } }
                            }
                        }
                    }
                }
            }
        }
    }
}
