use dioxus::prelude::*;
use crate::components::edr_widgets::*;

static PROCESSES: &[(&str, &str, &str, &str, &str, u32, &str)] = &[
    ("142", "nginx", "root", "2.1", "120 MB", 0, "Clean"),
    ("666", "evil_payload", "nobody", "95.0", "128 MB", 85, "Critical"),
    ("667", "evil_child", "nobody", "12.0", "32 MB", 40, "Medium"),
    ("789", "chrome", "drvoid", "45.0", "512 MB", 10, "Clean"),
    ("1024", "python3", "drvoid", "8.3", "64 MB", 5, "Clean"),
    ("450", "sshd", "root", "0.5", "12 MB", 0, "Clean"),
    ("1200", "bash", "drvoid", "0.1", "8 MB", 0, "Clean"),
    ("300", "cron", "root", "0.0", "4 MB", 0, "Clean"),
    ("888", "suspicious_net", "www", "34.2", "48 MB", 62, "High"),
    ("1201", "python_miner", "nobody", "99.0", "256 MB", 78, "High"),
];

#[component]
pub fn Processes() -> Element {
    rsx! {
        div { class: "dash-page",
            // Toolbar
            div { style: "display: flex; align-items: center; gap: 12px; padding: 16px 24px; background: var(--qs-card); border-bottom: 1px solid var(--qs-border); flex-wrap: wrap;",
                div { style: "flex: 1; min-width: 200px;",
                    input { style: "width: 100%; padding: 8px 14px; border-radius: 8px; border: 1px solid var(--qs-border); background: var(--qs-input-bg); color: var(--qs-input-fg); font-size: 13px; outline: none;",
                        placeholder: "🔍 Search by PID, name, path...",
                    }
                }
                div { style: "display: flex; gap: 6px; align-items: center;",
                    span { style: "font-size: 11px; color: var(--qs-fg-muted); font-weight: 600;", "View:" }
                    span { style: "padding: 4px 12px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg); cursor: pointer;", "List" }
                    span { style: "padding: 4px 12px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted); cursor: pointer;", "Tree" }
                }
                span { style: "font-size: 10px; color: var(--qs-success); font-weight: 700;", "⟳ Auto: 5s" }
            }

            // Content
            div { style: "padding: 20px 24px;",
                // Process Table
                div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    table { style: "width: 100%; border-collapse: collapse; font-size: 13px;",
                        thead {
                            tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                                th { style: "padding: 10px 14px; text-align: left; font-weight: 600;", "☐" }
                                th { style: "padding: 10px 14px; text-align: left; font-weight: 600;", "PID ↕" }
                                th { style: "padding: 10px 14px; text-align: left; font-weight: 600;", "Name ↕" }
                                th { style: "padding: 10px 14px; text-align: left; font-weight: 600;", "User" }
                                th { style: "padding: 10px 14px; text-align: right; font-weight: 600;", "CPU % ↕" }
                                th { style: "padding: 10px 14px; text-align: right; font-weight: 600;", "Memory ↕" }
                                th { style: "padding: 10px 14px; text-align: right; font-weight: 600;", "Score ↕" }
                                th { style: "padding: 10px 14px; text-align: left; font-weight: 600;", "Level" }
                                th { style: "padding: 10px 14px; text-align: center; font-weight: 600;", "⚡" }
                            }
                        }
                        tbody {
                            for (pid, name, user, cpu, mem, score, level) in PROCESSES.iter() {
                                {
                                    let score_color = match *score {
                                        0..=10 => "var(--qs-success)",
                                        11..=30 => "var(--qs-info)",
                                        31..=50 => "var(--qs-warning)",
                                        51..=70 => "#f97316",
                                        _ => "var(--qs-destructive)",
                                    };
                                    let row_bg = if *score > 70 { "rgba(239,68,68,0.06)" } else if *score > 30 { "rgba(245,158,11,0.04)" } else { "transparent" };
                                    rsx! {
                                        tr { style: "border-bottom: 1px solid var(--qs-border-subtle); background: {row_bg}; transition: background 0.1s;",
                                            td { style: "padding: 10px 14px;", "☐" }
                                            td { style: "padding: 10px 14px; font-family: var(--font-mono); color: var(--qs-primary); font-weight: 600;", "{pid}" }
                                            td { style: "padding: 10px 14px; font-weight: 600; color: var(--qs-fg);", "{name}" }
                                            td { style: "padding: 10px 14px; font-size: 11px; color: var(--qs-fg-muted);", "{user}" }
                                            td { style: "padding: 10px 14px; text-align: right; font-family: var(--font-mono); font-size: 12px;", "{cpu}%" }
                                            td { style: "padding: 10px 14px; text-align: right; font-family: var(--font-mono); font-size: 12px; color: var(--qs-fg-muted);", "{mem}" }
                                            td { style: "padding: 10px 14px; text-align: right;",
                                                span { style: "font-family: var(--font-mono); font-weight: 800; color: {score_color};", "{score}" }
                                            }
                                            td { style: "padding: 10px 14px;", ThreatBadge { level: level.to_string() } }
                                            td { style: "padding: 10px 14px; text-align: center; font-size: 14px; cursor: pointer;", "🔍" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Status Bar
                div { style: "display: flex; align-items: center; gap: 16px; padding: 12px 0; font-size: 11px; color: var(--qs-fg-muted);",
                    span { "📋 {PROCESSES.len()} processes" }
                    span { "⏱️ Last scan: 2s ago" }
                    span { style: "color: var(--qs-success);", "● Auto: ON" }
                }
            }
        }
    }
}
