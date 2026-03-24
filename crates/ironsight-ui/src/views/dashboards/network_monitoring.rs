use dioxus::prelude::*;
use crate::components::dashboards::*;

struct NetConnection { pid: u32, process: &'static str, local: &'static str, remote: &'static str, state: &'static str, proto: &'static str, flagged: bool }
struct Listener { pid: u32, process: &'static str, address: &'static str, port: u16, proto: &'static str, known: bool }

#[component]
pub fn NetworkMonitoring() -> Element {
    let connections = vec![
        NetConnection { pid: 9901, process: "unknown_agent", local: "192.168.1.5:49211", remote: "45.12.33.1:80", state: "ESTABLISHED", proto: "TCP", flagged: true },
        NetConnection { pid: 4492, process: "chrome.exe", local: "192.168.1.5:55201", remote: "142.250.190.46:443", state: "ESTABLISHED", proto: "TCP", flagged: false },
        NetConnection { pid: 8812, process: "svchost.exe", local: "192.168.1.5:61500", remote: "13.107.4.52:443", state: "ESTABLISHED", proto: "TCP", flagged: false },
        NetConnection { pid: 9901, process: "unknown_agent", local: "192.168.1.5:49500", remote: "185.234.72.10:4444", state: "ESTABLISHED", proto: "TCP", flagged: true },
        NetConnection { pid: 3310, process: "curl", local: "192.168.1.5:55800", remote: "104.21.32.100:443", state: "TIME_WAIT", proto: "TCP", flagged: false },
    ];
    let listeners = vec![
        Listener { pid: 22, process: "sshd", address: "0.0.0.0", port: 22, proto: "TCP", known: true },
        Listener { pid: 80, process: "nginx", address: "0.0.0.0", port: 80, proto: "TCP", known: true },
        Listener { pid: 8812, process: "svchost", address: "0.0.0.0", port: 135, proto: "TCP", known: true },
        Listener { pid: 9901, process: "unknown_agent", address: "0.0.0.0", port: 31337, proto: "TCP", known: false },
    ];

    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                div { style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "🌐 NETWORK INTELLIGENCE" }
                    StatusBadge { label: "2 FLAGS".to_string(), severity: "high".to_string() }
                }
                div { class: "header-actions",
                    span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "Interface: eth0 • 192.168.1.5" }
                }
            }
            div { class: "main-content",
                div { class: "dash-stats-row",
                    DashStatCard { label: "Active Connections".to_string(), value: "5".to_string(), icon: "🔗".to_string(), variant: "blue".to_string() }
                    DashStatCard { label: "Flagged".to_string(), value: "2".to_string(), icon: "🚩".to_string(), variant: "critical".to_string() }
                    DashStatCard { label: "Listeners".to_string(), value: "4".to_string(), icon: "👂".to_string(), variant: "green".to_string() }
                    DashStatCard { label: "Rogue Ports".to_string(), value: "1".to_string(), icon: "⚠️".to_string(), variant: "high".to_string() }
                    DashStatCard { label: "Bandwidth".to_string(), value: "2.4 MB/s".to_string(), icon: "📶".to_string(), variant: "purple".to_string() }
                }
                GlassPanel { style: "padding: 0; margin-bottom: 24px;".to_string(),
                    div { style: "padding: 16px; border-bottom: 1px solid var(--border);",
                        span { style: "font-size: 12px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.2px;", "🔌 External Connections" }
                    }
                    DashTable {
                        DashTableHead { tr { DashTh {"PID"} DashTh {"Process"} DashTh {"Local"} DashTh {"Remote"} DashTh {"State"} DashTh {"Flag"} } }
                        DashTableBody {
                            for conn in connections.iter() {
                                DashTr {
                                    DashTd { style: "color: var(--accent-blue);".to_string(), "{conn.pid}" }
                                    DashTd { style: "color: var(--text-primary); font-weight: 600;".to_string(), "{conn.process}" }
                                    DashTd { "{conn.local}" }
                                    DashTd { style: if conn.flagged { "color: var(--accent-red); font-weight: 700;".to_string() } else { "color: var(--accent-green);".to_string() }, "{conn.remote}" }
                                    DashTd { StatusBadge { label: conn.state.to_string(), severity: if conn.state == "ESTABLISHED" { "success".to_string() } else { "info".to_string() } } }
                                    DashTd {
                                        if conn.flagged { StatusBadge { label: "AUDIT".to_string(), severity: "critical".to_string() } }
                                        else { span { style: "color: var(--text-muted); font-size: 10px;", "—" } }
                                    }
                                }
                            }
                        }
                    }
                }
                GlassPanel { style: "padding: 0;".to_string(),
                    div { style: "padding: 16px; border-bottom: 1px solid var(--border);",
                        span { style: "font-size: 12px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.2px;", "👂 Listening Ports" }
                    }
                    DashTable {
                        DashTableHead { tr { DashTh {"PID"} DashTh {"Process"} DashTh {"Address"} DashTh {"Port"} DashTh {"Status"} } }
                        DashTableBody {
                            for l in listeners.iter() {
                                DashTr {
                                    DashTd { style: "color: var(--accent-blue);".to_string(), "{l.pid}" }
                                    DashTd { style: "font-weight: 600;".to_string(), "{l.process}" }
                                    DashTd { "{l.address}" }
                                    DashTd { style: "font-weight: 700;".to_string(), "{l.port}" }
                                    DashTd {
                                        if l.known { StatusBadge { label: "KNOWN".to_string(), severity: "success".to_string() } }
                                        else { StatusBadge { label: "ROGUE".to_string(), severity: "critical".to_string() } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
