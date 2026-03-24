use dioxus::prelude::*;
use crate::components::dashboards::*;

#[component]
pub fn ProcessMonitor() -> Element {
    let procs: &[(u32, &str, f64, &str, &str, &str, u32)] = &[
        (1, "systemd", 0.1, "12 MB", "running", "root", 1),
        (422, "sshd", 0.0, "8 MB", "sleeping", "root", 1),
        (1204, "nginx", 0.3, "32 MB", "running", "www", 4),
        (3310, "dbus-daemon", 0.1, "6 MB", "running", "dbus", 1),
        (4492, "chrome", 4.2, "450 MB", "running", "user", 28),
        (5500, "docker-proxy", 0.2, "18 MB", "running", "root", 3),
        (8812, "svchost.exe", 0.5, "24 MB", "running", "SYSTEM", 6),
        (9901, "unknown_agent", 12.5, "120 MB", "running", "user", 2),
        (10200, "python3", 1.0, "88 MB", "sleeping", "user", 1),
        (12500, "rust-analyzer", 3.0, "200 MB", "running", "user", 12),
    ];
    let running = procs.iter().filter(|p| p.4 == "running").count();

    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                span { style: "font-size: 20px; font-weight: 600;", "📊 PROCESS MONITOR" }
                div { class: "header-actions",
                    StatusBadge { label: format!("{} Running", running), severity: "success".to_string() }
                }
            }
            div { class: "main-content",
                div { class: "dash-stats-row",
                    DashStatCard { label: "Total".to_string(), value: procs.len().to_string(), icon: "⚙️".to_string(), variant: "blue".to_string() }
                    DashStatCard { label: "Running".to_string(), value: running.to_string(), icon: "▶️".to_string(), variant: "green".to_string() }
                    DashStatCard { label: "CPU Usage".to_string(), value: "22.4%".to_string(), icon: "💻".to_string(), variant: "purple".to_string() }
                    DashStatCard { label: "Memory".to_string(), value: "3.2 GB".to_string(), icon: "💾".to_string(), variant: "info".to_string() }
                    DashStatCard { label: "Uptime".to_string(), value: "14d 2h".to_string(), icon: "⏱️".to_string(), variant: "default".to_string() }
                }
                DashTable {
                    DashTableHead {
                        tr { DashTh {"PID"} DashTh {"Name"} DashTh {"CPU %"} DashTh {"Memory"} DashTh {"Status"} DashTh {"User"} DashTh {"Threads"} }
                    }
                    DashTableBody {
                        for p in procs.iter() {
                            DashTr {
                                DashTd { style: "color: var(--text-muted); font-size: 11px;".to_string(), "{p.0}" }
                                DashTd { style: "color: var(--text-primary); font-weight: 500;".to_string(), "{p.1}" }
                                DashTd {
                                    div { style: "display: flex; align-items: center; gap: 6px;",
                                        span { "{p.2}%" }
                                        div { style: "width: 50px; height: 4px; background: var(--bg-hover); border-radius: 2px; overflow: hidden;",
                                            div { style: format!("height:100%; width:{}%; border-radius:2px; background:{};",
                                                (p.2 as f64).min(100.0),
                                                if p.2 > 10.0 { "var(--accent-red)" } else { "var(--accent-blue)" }
                                            ) }
                                        }
                                    }
                                }
                                DashTd { "{p.3}" }
                                DashTd {
                                    StatusBadge {
                                        label: p.4.to_uppercase(),
                                        severity: match p.4 { "running" => "success", "sleeping" => "info", "zombie" => "critical", _ => "warning" }.to_string(),
                                    }
                                }
                                DashTd { style: "opacity: 0.6;".to_string(), "{p.5}" }
                                DashTd { "{p.6}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
