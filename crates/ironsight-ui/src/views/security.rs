use dioxus::prelude::*;
use crate::components::edr_widgets::*;

#[component]
pub fn Security() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "🔒 Security Audit" }
                div { class: "dash-stats-row",
                    EdrStatCard { icon: "🔍".to_string(), title: "Audited".to_string(), value: "342".to_string(), color: "primary".to_string() }
                    EdrStatCard { icon: "🚩".to_string(), title: "Flagged".to_string(), value: "18".to_string(), color: "warning".to_string() }
                    EdrStatCard { icon: "❌".to_string(), title: "Unsigned".to_string(), value: "45".to_string(), color: "danger".to_string() }
                    EdrStatCard { icon: "📁".to_string(), title: "Susp. Path".to_string(), value: "12".to_string(), color: "danger".to_string() }
                }
                // Row 2: Charts
                div { class: "dash-grid-2", style: "margin-top: 20px;",
                    // Entropy Distribution
                    div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px;",
                        h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px;", "📊 Entropy Distribution" }
                        div { style: "display: flex; align-items: flex-end; gap: 3px; height: 100px;",
                            for (range, count, color) in [("0-1", 5, "#10b981"), ("1-2", 12, "#10b981"), ("2-3", 25, "#10b981"), ("3-4", 45, "#10b981"), ("4-5", 82, "#10b981"), ("5-6", 120, "#f59e0b"), ("6-7", 35, "#f59e0b"), ("7-7.5", 8, "#f97316"), ("7.5+", 3, "#ef4444")] {
                                {
                                    let h = (count as f64 / 120.0 * 90.0).min(90.0).max(4.0);
                                    rsx! {
                                        div { style: "flex: 1; display: flex; flex-direction: column; align-items: center; gap: 2px;",
                                            div { style: "width: 100%; height: {h}px; background: {color}; border-radius: 3px 3px 0 0; min-width: 12px;" }
                                            span { style: "font-size: 7px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "{range}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Signature Status
                    div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px; display: flex; flex-direction: column; align-items: center; justify-content: center;",
                        h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px; align-self: flex-start;", "🔐 Signature Status" }
                        div { style: "display: flex; gap: 24px; flex-wrap: wrap; justify-content: center;",
                            for (icon, label, count, color) in [("✅", "Signed", "297", "#10b981"), ("❌", "Unsigned", "45", "#ef4444")] {
                                div { style: "text-align: center;",
                                    div { style: "font-size: 28px; font-weight: 800; font-family: var(--font-mono); color: {color};", "{count}" }
                                    div { style: "font-size: 11px; color: var(--qs-fg-muted); margin-top: 4px;", "{icon} {label}" }
                                }
                            }
                        }
                    }
                }
                // Row 3: Audit Table
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                        thead { tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                            th { style: "padding: 8px 12px; text-align: left;", "PID" }
                            th { style: "padding: 8px 12px; text-align: left;", "Name" }
                            th { style: "padding: 8px 12px; text-align: left;", "SHA-256" }
                            th { style: "padding: 8px 12px; text-align: right;", "Entropy" }
                            th { style: "padding: 8px 12px; text-align: center;", "Signed" }
                            th { style: "padding: 8px 12px; text-align: left;", "Flags" }
                        }}
                        tbody {
                            for (pid, name, hash, entropy, signed, flags) in [
                                ("142", "nginx", "a3f8e2..c9d1", "4.2", "✅", "0"),
                                ("666", "evil_payload", "e1d0b7..3f2a", "7.8", "❌", "3"),
                                ("789", "chrome", "b4c9e2..1a3b", "5.1", "✅", "0"),
                                ("888", "suspicious", "f2a1c3..8e7d", "7.3", "❌", "2"),
                                ("1201", "python_miner", "c3d2e4..5f6a", "7.9", "❌", "4"),
                            ] {
                                {
                                    let flag_n: u32 = flags.parse().unwrap_or(0);
                                    let row_bg = if flag_n >= 3 { "rgba(239,68,68,0.06)" } else if flag_n >= 1 { "rgba(245,158,11,0.04)" } else { "transparent" };
                                    let ent: f64 = entropy.parse().unwrap_or(0.0);
                                    let ent_color = if ent > 7.5 { "var(--qs-destructive)" } else if ent > 7.0 { "#f97316" } else if ent > 5.0 { "var(--qs-warning)" } else { "var(--qs-success)" };
                                    rsx! {
                                        tr { style: "border-bottom: 1px solid var(--qs-border-subtle); background: {row_bg};",
                                            td { style: "padding: 8px 12px; font-family: var(--font-mono); color: var(--qs-primary); font-weight: 600;", "{pid}" }
                                            td { style: "padding: 8px 12px; font-weight: 600; color: var(--qs-fg);", "{name}" }
                                            td { style: "padding: 8px 12px; font-family: var(--font-mono); font-size: 10px; color: var(--qs-fg-muted);", "{hash}" }
                                            td { style: "padding: 8px 12px; text-align: right; font-family: var(--font-mono); font-weight: 700; color: {ent_color};", "{entropy}" }
                                            td { style: "padding: 8px 12px; text-align: center; font-size: 14px;", "{signed}" }
                                            td {
                                                style: {
                                                    let flag_color = if flag_n > 0 { "var(--qs-destructive)" } else { "var(--qs-success)" };
                                                    format!("padding: 8px 12px; font-family: var(--font-mono); font-weight: 700; color: {flag_color};")
                                                },
                                                "{flags}"
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
