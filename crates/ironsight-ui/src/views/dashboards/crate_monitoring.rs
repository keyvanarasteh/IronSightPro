use dioxus::prelude::*;
use crate::components::dashboards::*;

struct MemRegion { address: &'static str, size: &'static str, perms: &'static str, usage: f64 }
struct ScanHit { offset: &'static str, pattern: &'static str, context: &'static str, severity: &'static str }

#[component]
pub fn CrateMonitoring() -> Element {
    let regions = vec![
        MemRegion { address: "0x00400000", size: "64 KB", perms: "r-x", usage: 85.0 },
        MemRegion { address: "0x00600000", size: "4 KB", perms: "rw-", usage: 32.0 },
        MemRegion { address: "0x7f000000", size: "1.2 MB", perms: "r--", usage: 60.0 },
        MemRegion { address: "0x7fff0000", size: "132 KB", perms: "rwx", usage: 95.0 },
        MemRegion { address: "0x55500000", size: "256 KB", perms: "r-x", usage: 45.0 },
        MemRegion { address: "0xdeadbeef", size: "8 KB", perms: "rw-", usage: 78.0 },
    ];
    let scan_hits = vec![
        ScanHit { offset: "0x0042F100", pattern: "MZ\\x90\\x00", context: "PE Header — possible injected module", severity: "critical" },
        ScanHit { offset: "0x0043A280", pattern: "cmd.exe /c", context: "Shell command in heap buffer", severity: "high" },
        ScanHit { offset: "0x004500C0", pattern: "VirtualAlloc", context: "API import — runtime code loading", severity: "medium" },
        ScanHit { offset: "0x00462000", pattern: "CONNECT %s", context: "C2 beacon pattern detected", severity: "critical" },
        ScanHit { offset: "0x00470FF0", pattern: "\\x55\\x8B\\xEC", context: "Function prologue — packed code", severity: "high" },
    ];

    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                div { style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "🧬 MEMORY FORENSICS" }
                    StatusBadge { label: "LIVE ANALYSIS".to_string(), severity: "success".to_string() }
                }
                div { class: "header-actions",
                    span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "Target: PID 9901 — unknown_agent" }
                }
            }
            div { class: "main-content",
                div { class: "dash-stats-row",
                    DashStatCard { label: "Mapped Regions".to_string(), value: "47".to_string(), icon: "🗺️".to_string(), variant: "info".to_string() }
                    DashStatCard { label: "RWX Violations".to_string(), value: "3".to_string(), icon: "⚠️".to_string(), variant: "critical".to_string() }
                    DashStatCard { label: "Pattern Hits".to_string(), value: "12".to_string(), icon: "🔍".to_string(), variant: "high".to_string() }
                    DashStatCard { label: "Entropy (avg)".to_string(), value: "6.82".to_string(), icon: "📊".to_string(), variant: "medium".to_string() }
                    DashStatCard { label: "Heap Size".to_string(), value: "124 MB".to_string(), icon: "💾".to_string(), variant: "blue".to_string() }
                }
                div { class: "dash-grid-2",
                    MemoryMapPanel {
                        for r in regions.iter() {
                            MemoryRegionBar { address: r.address.to_string(), size: r.size.to_string(), permissions: r.perms.to_string(), usage_pct: r.usage }
                        }
                    }
                    GlassPanel { style: "padding: 16px;".to_string(),
                        PanelHeader { title: "Memory Watcher".to_string(), StatusBadge { label: "ACTIVE".to_string(), severity: "success".to_string() } }
                        ChangeFeedItem { timestamp: "10:04:22".to_string(), event_type: "PERM CHANGE".to_string(), description: "Region 0x7fff0000 changed from rw- to rwx (+execute)".to_string(), severity: "critical".to_string() }
                        ChangeFeedItem { timestamp: "10:04:18".to_string(), event_type: "ALLOCATION".to_string(), description: "New region allocated at 0xdeadbeef (8 KB, rw-)".to_string(), severity: "warning".to_string() }
                        ChangeFeedItem { timestamp: "10:03:55".to_string(), event_type: "WRITE".to_string(), description: "Heap buffer modified at 0x0043A280 (256 bytes)".to_string(), severity: "info".to_string() }
                        ChangeFeedItem { timestamp: "10:03:41".to_string(), event_type: "MAP".to_string(), description: "Shared library mapped: libc.so.6 at 0x7f000000".to_string(), severity: "success".to_string() }
                    }
                }
                ScannerResultsPanel { title: "Pattern Scanner Results".to_string(), count: "5 hits".to_string(),
                    for hit in scan_hits.iter() {
                        ScannerResultRow { offset: hit.offset.to_string(), pattern: hit.pattern.to_string(), context: hit.context.to_string(), severity: hit.severity.to_string() }
                    }
                }
            }
        }
    }
}
