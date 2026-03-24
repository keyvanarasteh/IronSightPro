use dioxus::prelude::*;
use crate::components::edr_widgets::*;

static SIGNALS: &[(&str, &str, u32, u32)] = &[
    ("HIGH_ENTROPY", "StaticAnalysis", 30, 3),
    ("SUSPICIOUS_PORT", "NetworkAnomaly", 25, 1),
    ("WX_VIOLATION", "MemoryAnomaly", 35, 2),
    ("HIDDEN_PROCESS", "ProcessBehavior", 20, 1),
    ("PACKED_BINARY", "StaticAnalysis", 28, 2),
    ("TEMP_EXECUTION", "FilesystemAnomaly", 15, 4),
    ("HIGH_CPU_USAGE", "ProcessBehavior", 10, 5),
    ("EXTERNAL_CONNECT", "NetworkAnomaly", 18, 3),
];

static CATEGORIES: &[(&str, u32)] = &[
    ("ProcessBehavior", 45),
    ("NetworkAnomaly", 38),
    ("MemoryAnomaly", 28),
    ("StaticAnalysis", 25),
    ("FilesystemAnomaly", 12),
];

#[component]
pub fn ThreatAnalysis() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "🎯 Threat Analysis" }

                // Row 1: Level Distribution (5 stat cards)
                div { class: "dash-stats-row",
                    EdrStatCard { icon: "🟢".to_string(), title: "Clean".to_string(), value: "280".to_string(), subtitle: "processes".to_string(), color: "success".to_string() }
                    EdrStatCard { icon: "🔵".to_string(), title: "Low".to_string(), value: "42".to_string(), subtitle: "düşük risk".to_string(), color: "info".to_string() }
                    EdrStatCard { icon: "🟡".to_string(), title: "Medium".to_string(), value: "12".to_string(), subtitle: "orta risk".to_string(), color: "warning".to_string() }
                    EdrStatCard { icon: "🟠".to_string(), title: "High".to_string(), value: "5".to_string(), subtitle: "yüksek risk".to_string(), color: "warning".to_string() }
                    EdrStatCard { icon: "🔴".to_string(), title: "Critical".to_string(), value: "2".to_string(), subtitle: "acil müdahale".to_string(), color: "danger".to_string() }
                }

                // Row 2: Charts
                div { class: "dash-grid-2", style: "margin-top: 20px;",
                    // Donut placeholder
                    div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px;",
                        h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px;", "📊 Threat Distribution" }
                        div { style: "display: flex; justify-content: center;",
                            ThreatGauge { score: 87, size: "lg".to_string() }
                        }
                        div { style: "display: flex; justify-content: center; gap: 16px; margin-top: 16px; flex-wrap: wrap;",
                            for (level, color) in [("Clean", "#10b981"), ("Low", "#3b82f6"), ("Medium", "#f59e0b"), ("High", "#f97316"), ("Critical", "#ef4444")] {
                                div { style: "display: flex; align-items: center; gap: 6px; font-size: 11px; color: var(--qs-fg-muted);",
                                    div { style: "width: 8px; height: 8px; border-radius: 50%; background: {color};" }
                                    "{level}"
                                }
                            }
                        }
                    }

                    // Category Breakdown
                    div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px;",
                        h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px;", "📈 Category Breakdown" }
                        div { style: "display: flex; flex-direction: column; gap: 10px;",
                            for (cat, weight) in CATEGORIES.iter() {
                                {
                                    let pct = (*weight as f64 / 50.0 * 100.0).min(100.0);
                                    let icon = match *cat {
                                        "ProcessBehavior" => "🔄",
                                        "NetworkAnomaly" => "🌐",
                                        "MemoryAnomaly" => "🧠",
                                        "StaticAnalysis" => "🔒",
                                        _ => "📁",
                                    };
                                    rsx! {
                                        div {
                                            div { style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                                                span { style: "font-size: 12px; font-weight: 600; color: var(--qs-fg);", "{icon} {cat}" }
                                                span { style: "font-size: 12px; font-weight: 800; font-family: var(--font-mono); color: var(--qs-primary);", "{weight}" }
                                            }
                                            div { style: "height: 6px; background: var(--qs-muted); border-radius: 3px; overflow: hidden;",
                                                div { style: "height: 100%; width: {pct}%; background: var(--qs-primary); border-radius: 3px; transition: width 0.5s;" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Row 3: Decay Timeline
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px;",
                    h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px;", "📉 Decay Timeline (Son 24 Saat)" }
                    div { style: "height: 120px; background: var(--qs-bg); border: 1px solid var(--qs-border); border-radius: 8px; position: relative; overflow: hidden;",
                        // Zone bands
                        div { style: "position: absolute; bottom: 0; left: 0; right: 0; height: 30%; background: rgba(16,185,129,0.08);" }
                        div { style: "position: absolute; bottom: 30%; left: 0; right: 0; height: 30%; background: rgba(245,158,11,0.06);" }
                        div { style: "position: absolute; bottom: 60%; left: 0; right: 0; height: 40%; background: rgba(239,68,68,0.04);" }
                        div { style: "position: absolute; bottom: 8px; left: 12px; font-size: 9px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "0:00" }
                        div { style: "position: absolute; bottom: 8px; right: 12px; font-size: 9px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "24:00" }
                    }
                }

                // Row 4: Signal Catalog
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    div { style: "padding: 16px 20px; border-bottom: 1px solid var(--qs-border);",
                        h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0;", "🔔 Signal Catalog" }
                    }
                    table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                        thead {
                            tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                                th { style: "padding: 8px 14px; text-align: left;", "Signal" }
                                th { style: "padding: 8px 14px; text-align: left;", "Category" }
                                th { style: "padding: 8px 14px; text-align: right;", "Weight" }
                                th { style: "padding: 8px 14px; text-align: right;", "Active" }
                            }
                        }
                        tbody {
                            for (name, cat, weight, active) in SIGNALS.iter() {
                                tr { style: "border-bottom: 1px solid var(--qs-border-subtle);",
                                    td { style: "padding: 8px 14px;",
                                        SignalChip { name: name.to_string(), category: cat.to_string(), weight: *weight }
                                    }
                                    td { style: "padding: 8px 14px; font-size: 11px; color: var(--qs-fg-muted);", "{cat}" }
                                    td { style: "padding: 8px 14px; text-align: right; font-family: var(--font-mono); font-weight: 700; color: var(--qs-fg);", "{weight}" }
                                    td { style: "padding: 8px 14px; text-align: right; font-size: 11px; color: var(--qs-fg-muted);", "{active} process" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
