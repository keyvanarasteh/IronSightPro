use dioxus::prelude::*;
use crate::components::edr_widgets::*;

#[component]
pub fn Memory() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "🧠 Memory Forensics" }

                div { class: "dash-stats-row",
                    EdrStatCard { icon: "📦".to_string(), title: "Regions".to_string(), value: "1842".to_string(), subtitle: "bellek bölgesi".to_string(), color: "primary".to_string() }
                    EdrStatCard { icon: "⚠️".to_string(), title: "W^X Violations".to_string(), value: "4".to_string(), subtitle: "ihlal tespit".to_string(), color: "danger".to_string() }
                    EdrStatCard { icon: "🔓".to_string(), title: "Anon Exec".to_string(), value: "7".to_string(), subtitle: "anonim çalıştırılabilir".to_string(), color: "warning".to_string() }
                    EdrStatCard { icon: "🔍".to_string(), title: "Patterns".to_string(), value: "12".to_string(), subtitle: "kalıp bulundu".to_string(), color: "info".to_string() }
                }

                // Process Picker
                div { style: "margin-top: 20px; padding: 12px 16px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 10px; display: flex; align-items: center; gap: 12px;",
                    span { style: "font-size: 12px; font-weight: 600; color: var(--qs-fg-muted);", "Process:" }
                    span { style: "padding: 6px 14px; border-radius: 6px; font-size: 12px; font-weight: 700; background: var(--qs-primary-selection); color: var(--qs-primary); font-family: var(--font-mono); border: 1px solid rgba(0,120,212,0.2);",
                        "PID:666 evil_payload ▾"
                    }
                }

                // Memory Map
                div { style: "margin-top: 20px;",
                    h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 12px;", "🗺️ Memory Map Visualization" }
                    MemoryMapBar {
                        MemoryRegion { name: ".text".to_string(), permissions: "r-x".to_string(), size_ratio: 2.0 }
                        MemoryRegion { name: ".data".to_string(), permissions: "rw-".to_string(), size_ratio: 1.0 }
                        MemoryRegion { name: "heap".to_string(), permissions: "rw-".to_string(), size_ratio: 4.0 }
                        MemoryRegion { name: "libc".to_string(), permissions: "r--".to_string(), size_ratio: 1.5 }
                        MemoryRegion { name: "stack".to_string(), permissions: "rw-".to_string(), size_ratio: 1.5 }
                        MemoryRegion { name: "[anon]".to_string(), permissions: "rwx".to_string(), size_ratio: 2.0, violation: true }
                    }
                    div { style: "display: flex; gap: 16px; margin-top: 8px; flex-wrap: wrap;",
                        for (perm, color, label) in [("r--", "#3b82f6", "Read"), ("rw-", "#10b981", "Read/Write"), ("r-x", "#f59e0b", "Execute"), ("rwx", "#ef4444", "W^X Violation")] {
                            div { style: "display: flex; align-items: center; gap: 6px; font-size: 10px; color: var(--qs-fg-muted);",
                                div { style: "width: 12px; height: 12px; border-radius: 3px; background: {color};" }
                                span { style: "font-family: var(--font-mono); font-size: 9px;", "{perm}" }
                                span { "{label}" }
                            }
                        }
                    }
                }

                // Tabs + Region Table
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                    div { style: "display: flex; gap: 4px; padding: 12px 16px; border-bottom: 1px solid var(--qs-border); background: var(--qs-bg-elevated);",
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg);", "Regions" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "🔴 Violations" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Patterns" }
                        span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Changes" }
                    }
                    table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                        thead {
                            tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                                th { style: "padding: 8px 12px; text-align: left;", "Address Range" }
                                th { style: "padding: 8px 12px; text-align: left;", "Permissions" }
                                th { style: "padding: 8px 12px; text-align: right;", "Size" }
                                th { style: "padding: 8px 12px; text-align: left;", "Pathname" }
                                th { style: "padding: 8px 12px; text-align: left;", "Flags" }
                            }
                        }
                        tbody {
                            for (addr, perm, size, path, flag) in [
                                ("0x400000-0x420000", "r-x", "128 KB", "/usr/bin/evil", ""),
                                ("0x620000-0x625000", "rw-", "20 KB", "/usr/bin/evil", ""),
                                ("0x1000000-0x1400000", "rw-", "4 MB", "[heap]", ""),
                                ("0x7f000000-0x7f200000", "r--", "2 MB", "/lib/libc.so.6", ""),
                                ("0x7ffc0000-0x7ffe0000", "rw-", "128 KB", "[stack]", ""),
                                ("0x7f500000-0x7f700000", "rwx", "2 MB", "[anon]", "⚠️ W^X"),
                            ] {
                                {
                                    let is_viol = flag.contains("W^X");
                                    let row_bg = if is_viol { "rgba(239,68,68,0.08)" } else { "transparent" };
                                    rsx! {
                                        tr { style: "border-bottom: 1px solid var(--qs-border-subtle); background: {row_bg};",
                                            td { style: "padding: 8px 12px; font-family: var(--font-mono); font-size: 11px; color: var(--qs-fg);", "{addr}" }
                                            td {
                                                style: {
                                                    let perm_color = if is_viol { "var(--qs-destructive)" } else { "var(--qs-fg-muted)" };
                                                    format!("padding: 8px 12px; font-family: var(--font-mono); font-size: 11px; font-weight: 700; color: {perm_color};")
                                                },
                                                "{perm}"
                                            }
                                            td { style: "padding: 8px 12px; text-align: right; font-family: var(--font-mono); font-size: 11px; color: var(--qs-fg-muted);", "{size}" }
                                            td { style: "padding: 8px 12px; font-size: 11px; color: var(--qs-fg-muted);", "{path}" }
                                            td { style: "padding: 8px 12px; font-size: 11px; font-weight: 700; color: var(--qs-destructive);", "{flag}" }
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
