use dioxus::prelude::*;
use crate::components::edr_widgets::*;

#[component]
pub fn ResponseCenter() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "🛡️ Response Center" }

                div { class: "dash-stats-row",
                    EdrStatCard { icon: "🔒".to_string(), title: "Suspended".to_string(), value: "12".to_string(), subtitle: "askıya alınan".to_string(), color: "warning".to_string() }
                    EdrStatCard { icon: "❌".to_string(), title: "Killed".to_string(), value: "5".to_string(), subtitle: "sonlandırılan".to_string(), color: "danger".to_string() }
                    EdrStatCard { icon: "📸".to_string(), title: "Dumped".to_string(), value: "8".to_string(), subtitle: "dump alınan".to_string(), color: "info".to_string() }
                    EdrStatCard { icon: "🛡️".to_string(), title: "Excluded".to_string(), value: "34".to_string(), subtitle: "korumalı process".to_string(), color: "success".to_string() }
                }

                // Tabs
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    div { style: "display: flex; gap: 4px; padding: 12px 16px; border-bottom: 1px solid var(--qs-border); background: var(--qs-bg-elevated);",
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg);", "Actions" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Exclusions" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Dumps" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Timeline" }
                    }

                    // Actions table
                    table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                        thead { tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                            th { style: "padding: 8px 14px; text-align: left;", "Time" }
                            th { style: "padding: 8px 14px; text-align: left;", "PID" }
                            th { style: "padding: 8px 14px; text-align: left;", "Name" }
                            th { style: "padding: 8px 14px; text-align: left;", "Action" }
                            th { style: "padding: 8px 14px; text-align: center;", "Result" }
                            th { style: "padding: 8px 14px; text-align: left;", "Detail" }
                            th { style: "padding: 8px 14px; text-align: center;", "Undo" }
                        }}
                        tbody {
                            for (time, pid, name, action, success, detail) in [
                                ("10:05:23", "666", "evil_payload", "Suspend", true, "Process suspended"),
                                ("10:05:24", "666", "evil_payload", "Dump", true, "128 MB dumped → /var/lib/ironsight/dumps/666.bin"),
                                ("10:06:01", "666", "evil_payload", "Kill", true, "Process terminated"),
                                ("10:15:00", "888", "suspicious_net", "Suspend", true, "Process suspended"),
                                ("10:16:30", "888", "suspicious_net", "Resume", true, "False positive — resumed"),
                                ("10:20:00", "1201", "python_miner", "Kill", false, "Permission denied (EPERM)"),
                            ] {
                                {
                                    let result_icon = if success { "✅" } else { "❌" };
                                    let action_color = match action {
                                        "Suspend" => "var(--qs-warning)",
                                        "Kill" => "var(--qs-destructive)",
                                        "Dump" => "var(--qs-info)",
                                        "Resume" => "var(--qs-success)",
                                        _ => "var(--qs-fg-muted)",
                                    };
                                    rsx! {
                                        tr { style: "border-bottom: 1px solid var(--qs-border-subtle);",
                                            td { style: "padding: 8px 14px; font-family: var(--font-mono); font-size: 11px; color: var(--qs-fg-muted);", "{time}" }
                                            td { style: "padding: 8px 14px; font-family: var(--font-mono); color: var(--qs-primary); font-weight: 600;", "{pid}" }
                                            td { style: "padding: 8px 14px; font-weight: 600; color: var(--qs-fg);", "{name}" }
                                            td { style: "padding: 8px 14px; font-weight: 700; color: {action_color};", "{action}" }
                                            td { style: "padding: 8px 14px; text-align: center; font-size: 14px;", "{result_icon}" }
                                            td { style: "padding: 8px 14px; font-size: 11px; color: var(--qs-fg-muted);", "{detail}" }
                                            td { style: "padding: 8px 14px; text-align: center;",
                                                if action == "Suspend" && success { span { style: "cursor: pointer;", "🔄" } }
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
