use dioxus::prelude::*;
use crate::components::edr_widgets::*;

#[component]
pub fn Reports() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "📊 Reports" }
                div { class: "dash-stats-row",
                    EdrStatCard { icon: "📋".to_string(), title: "Total".to_string(), value: "156".to_string(), color: "primary".to_string() }
                    EdrStatCard { icon: "📅".to_string(), title: "Today".to_string(), value: "12".to_string(), color: "info".to_string() }
                    EdrStatCard { icon: "🔴".to_string(), title: "Critical".to_string(), value: "3".to_string(), color: "danger".to_string() }
                    EdrStatCard { icon: "📤".to_string(), title: "Export".to_string(), value: "2".to_string(), color: "success".to_string() }
                }
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    div { style: "display: flex; gap: 4px; padding: 12px 16px; border-bottom: 1px solid var(--qs-border); background: var(--qs-bg-elevated);",
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg);", "Reports" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Create" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Export Config" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Scheduled" }
                    }
                    table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                        thead { tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                            th { style: "padding: 8px 14px; text-align: left;", "ID" }
                            th { style: "padding: 8px 14px; text-align: left;", "Date" }
                            th { style: "padding: 8px 14px; text-align: left;", "PID" }
                            th { style: "padding: 8px 14px; text-align: left;", "Level" }
                            th { style: "padding: 8px 14px; text-align: right;", "Score" }
                            th { style: "padding: 8px 14px; text-align: center;", "⬇️" }
                        }}
                        tbody {
                            for (id, date, pid, level, score) in [("a1b2c3", "2026-03-24 10:06", "666", "Critical", "85"), ("d4e5f6", "2026-03-24 10:15", "888", "High", "62"), ("g7h8i9", "2026-03-24 09:30", "1201", "High", "78"), ("j0k1l2", "2026-03-23 18:00", "—", "Low", "—")] {
                                tr { style: "border-bottom: 1px solid var(--qs-border-subtle);",
                                    td { style: "padding: 8px 14px; font-family: var(--font-mono); font-size: 10px; color: var(--qs-primary);", "{id}" }
                                    td { style: "padding: 8px 14px; font-size: 11px; color: var(--qs-fg-muted);", "{date}" }
                                    td { style: "padding: 8px 14px; font-family: var(--font-mono); font-weight: 600;", "{pid}" }
                                    td { style: "padding: 8px 14px;", ThreatBadge { level: level.to_string() } }
                                    td { style: "padding: 8px 14px; text-align: right; font-family: var(--font-mono); font-weight: 700;", "{score}" }
                                    td { style: "padding: 8px 14px; text-align: center; cursor: pointer;", "⬇️" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
