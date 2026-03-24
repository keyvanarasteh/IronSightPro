use dioxus::prelude::*;
use crate::components::edr_widgets::*;

static CONNECTIONS: &[(&str, &str, &str, &str, &str, &str, &str)] = &[
    ("142", "nginx", "TCP", "0.0.0.0:80", "—", "LISTEN", ""),
    ("789", "chrome", "TCP", "192.168.1.5:54321", "142.250.74.14:443", "ESTABLISHED", "google.com"),
    ("888", "suspicious", "TCP", "192.168.1.5:49999", "93.184.216.34:4444", "ESTABLISHED", "⚠️ C2 Port"),
    ("450", "sshd", "TCP", "0.0.0.0:22", "—", "LISTEN", ""),
    ("666", "evil_payload", "TCP", "192.168.1.5:55555", "185.220.101.1:8080", "ESTABLISHED", "⚠️ Tor Exit"),
    ("1024", "python3", "UDP", "0.0.0.0:5353", "—", "LISTEN", "mDNS"),
];

#[component]
pub fn Network() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "🌐 Network Intelligence" }

                // Row 1: Stat Cards
                div { class: "dash-stats-row",
                    EdrStatCard { icon: "🔌".to_string(), title: "Total Sockets".to_string(), value: "156".to_string(), subtitle: "aktif bağlantı".to_string(), color: "primary".to_string() }
                    EdrStatCard { icon: "📡".to_string(), title: "Listeners".to_string(), value: "12".to_string(), subtitle: "açık port".to_string(), color: "info".to_string() }
                    EdrStatCard { icon: "🌍".to_string(), title: "External".to_string(), value: "34".to_string(), subtitle: "dış bağlantı".to_string(), color: "warning".to_string() }
                    EdrStatCard { icon: "🚨".to_string(), title: "Suspicious".to_string(), value: "3".to_string(), subtitle: "şüpheli bağlantı".to_string(), color: "danger".to_string() }
                }

                // Row 2: Topology
                div { style: "margin-top: 20px;",
                    TopologyGraph { height: "320px".to_string(),
                        TopoEdge { x1: 25.0, y1: 40.0, x2: 50.0, y2: 30.0 }
                        TopoEdge { x1: 25.0, y1: 65.0, x2: 50.0, y2: 70.0 }
                        TopoEdge { x1: 50.0, y1: 30.0, x2: 75.0, y2: 25.0 }
                        TopoEdge { x1: 50.0, y1: 70.0, x2: 75.0, y2: 70.0, suspicious: true, thickness: 2.0 }
                        TopoNode { label: "nginx:142".to_string(), node_type: "process".to_string(), x: 25.0, y: 40.0 }
                        TopoNode { label: "evil:666".to_string(), node_type: "process".to_string(), x: 25.0, y: 65.0 }
                        TopoNode { label: "0.0.0.0:80".to_string(), node_type: "listener".to_string(), x: 50.0, y: 30.0 }
                        TopoNode { label: "185.220.101.1".to_string(), node_type: "suspicious".to_string(), x: 50.0, y: 70.0 }
                        TopoNode { label: "google.com".to_string(), node_type: "external".to_string(), x: 75.0, y: 25.0 }
                        TopoNode { label: "93.184.216.34".to_string(), node_type: "suspicious".to_string(), x: 75.0, y: 70.0 }
                    }
                }

                // Row 3: Tabs + Connection Table
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    div { style: "display: flex; gap: 4px; padding: 12px 16px; border-bottom: 1px solid var(--qs-border); background: var(--qs-bg-elevated);",
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg); cursor: pointer;", "All Connections" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted); cursor: pointer;", "Listeners" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted); cursor: pointer;", "🔴 Suspicious" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted); cursor: pointer;", "DNS" }
                    }
                    table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                        thead {
                            tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                                th { style: "padding: 8px 12px; text-align: left;", "PID" }
                                th { style: "padding: 8px 12px; text-align: left;", "Name" }
                                th { style: "padding: 8px 12px; text-align: left;", "Proto" }
                                th { style: "padding: 8px 12px; text-align: left;", "Local Addr" }
                                th { style: "padding: 8px 12px; text-align: left;", "Remote Addr" }
                                th { style: "padding: 8px 12px; text-align: left;", "State" }
                                th { style: "padding: 8px 12px; text-align: left;", "Intel" }
                            }
                        }
                        tbody {
                            for (pid, name, proto, local, remote, state, intel) in CONNECTIONS.iter() {
                                {
                                    let is_sus = intel.contains("⚠️");
                                    let row_bg = if is_sus { "rgba(239,68,68,0.06)" } else { "transparent" };
                                    rsx! {
                                        tr { style: "border-bottom: 1px solid var(--qs-border-subtle); background: {row_bg};",
                                            td { style: "padding: 8px 12px; font-family: var(--font-mono); color: var(--qs-primary); font-weight: 600;", "{pid}" }
                                            td { style: "padding: 8px 12px; font-weight: 600; color: var(--qs-fg);", "{name}" }
                                            td { style: "padding: 8px 12px; font-size: 10px; color: var(--qs-fg-muted);", "{proto}" }
                                            td { style: "padding: 8px 12px; font-family: var(--font-mono); font-size: 11px; color: var(--qs-fg);", "{local}" }
                                            td { style: "padding: 8px 12px; font-family: var(--font-mono); font-size: 11px; color: var(--qs-fg);", "{remote}" }
                                            td { style: "padding: 8px 12px;",
                                                ThreatBadge { level: if *state == "LISTEN" { "Low".to_string() } else { "Clean".to_string() } }
                                            }
                                            td { style: "padding: 8px 12px; font-size: 11px;",
                                                {
                                                    let intel_color = if is_sus { "var(--qs-destructive)" } else { "var(--qs-fg-muted)" };
                                                    let intel_weight = if is_sus { "700" } else { "400" };
                                                    rsx! { span { style: "color: {intel_color}; font-weight: {intel_weight};", "{intel}" } }
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
        }
    }
}
