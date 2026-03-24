use dioxus::prelude::*;
use crate::components::edr_widgets::*;

#[component]
pub fn KernelMonitor() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "⚡ Kernel Monitor" }

                div { class: "dash-stats-row",
                    EdrStatCard { icon: "📊".to_string(), title: "Total Events".to_string(), value: "14.2K".to_string(), subtitle: "toplam olay".to_string(), color: "primary".to_string() }
                    EdrStatCard { icon: "⚡".to_string(), title: "Events/sec".to_string(), value: "847".to_string(), subtitle: "şu an".to_string(), color: "info".to_string() }
                    EdrStatCard { icon: "🚨".to_string(), title: "Alerts".to_string(), value: "3".to_string(), subtitle: "şüpheli olay".to_string(), color: "danger".to_string() }
                    EdrStatCard { icon: "🔗".to_string(), title: "Tracepoints".to_string(), value: "6".to_string(), subtitle: "aktif izleme".to_string(), color: "success".to_string() }
                }

                // Event Stream
                div { style: "margin-top: 20px;",
                    EventStream {
                        EventEntry { timestamp: "10:05:23.142".to_string(), syscall: "mprotect".to_string(), pid: "666".to_string(), args: "rwx→r-x".to_string(), alert: true }
                        EventEntry { timestamp: "10:05:23.145".to_string(), syscall: "connect".to_string(), pid: "888".to_string(), args: "→93.184.216.34:4444".to_string(), alert: true }
                        EventEntry { timestamp: "10:05:23.201".to_string(), syscall: "execve".to_string(), pid: "1201".to_string(), args: "/tmp/payload".to_string(), alert: true }
                        EventEntry { timestamp: "10:05:23.342".to_string(), syscall: "read".to_string(), pid: "142".to_string(), args: "fd=5, buf=0x7fff".to_string() }
                        EventEntry { timestamp: "10:05:23.415".to_string(), syscall: "write".to_string(), pid: "789".to_string(), args: "fd=3, size=4096".to_string() }
                        EventEntry { timestamp: "10:05:23.520".to_string(), syscall: "connect".to_string(), pid: "789".to_string(), args: "→142.250.74.14:443".to_string() }
                        EventEntry { timestamp: "10:05:23.678".to_string(), syscall: "mmap".to_string(), pid: "1024".to_string(), args: "len=65536, prot=PROT_READ".to_string() }
                        EventEntry { timestamp: "10:05:23.801".to_string(), syscall: "ptrace".to_string(), pid: "666".to_string(), args: "PTRACE_ATTACH pid=450".to_string(), alert: true }
                    }
                }

                // Row 3: Syscall Distribution + Hot Processes
                div { class: "dash-grid-2", style: "margin-top: 20px;",
                    div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px;",
                        h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px;", "📊 Syscall Distribution" }
                        for (name, count, color) in [("connect", 67, "var(--qs-info)"), ("execve", 45, "#f97316"), ("mprotect", 23, "var(--qs-destructive)"), ("mmap", 18, "var(--qs-success)"), ("ptrace", 2, "var(--qs-destructive)")] {
                            div { style: "margin-bottom: 8px;",
                                div { style: "display: flex; justify-content: space-between; margin-bottom: 3px;",
                                    span { style: "font-size: 12px; font-weight: 600; color: var(--qs-fg); font-family: var(--font-mono);", "{name}" }
                                    span { style: "font-size: 12px; font-weight: 800; font-family: var(--font-mono); color: {color};", "{count}" }
                                }
                                div { style: "height: 5px; background: var(--qs-muted); border-radius: 3px; overflow: hidden;",
                                    div { style: "height: 100%; width: {(count as f64 / 70.0 * 100.0).min(100.0)}%; background: {color}; border-radius: 3px;" }
                                }
                            }
                        }
                    }
                    div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
                        div { style: "padding: 16px 20px; border-bottom: 1px solid var(--qs-border);",
                            h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0;", "🔥 Hot Processes" }
                        }
                        table { style: "width: 100%; border-collapse: collapse; font-size: 12px;",
                            thead { tr { style: "background: var(--qs-muted); font-size: 10px; text-transform: uppercase; color: var(--qs-muted-fg);",
                                th { style: "padding: 8px 12px; text-align: left;", "PID" }
                                th { style: "padding: 8px 12px; text-align: left;", "Name" }
                                th { style: "padding: 8px 12px; text-align: right;", "Events/min" }
                                th { style: "padding: 8px 12px; text-align: left;", "Top Syscall" }
                                th { style: "padding: 8px 12px; text-align: center;", "Alert" }
                            }}
                            tbody {
                                for (pid, name, epm, top, alert) in [("666", "evil_payload", "342", "mprotect", true), ("888", "suspicious", "128", "connect", true), ("789", "chrome", "89", "write", false)] {
                                    tr { style: "border-bottom: 1px solid var(--qs-border-subtle);",
                                        td { style: "padding: 8px 12px; font-family: var(--font-mono); color: var(--qs-primary); font-weight: 600;", "{pid}" }
                                        td { style: "padding: 8px 12px; font-weight: 600; color: var(--qs-fg);", "{name}" }
                                        td { style: "padding: 8px 12px; text-align: right; font-family: var(--font-mono); font-weight: 700;", "{epm}" }
                                        td { style: "padding: 8px 12px; font-family: var(--font-mono); font-size: 11px; color: var(--qs-fg-muted);", "{top}" }
                                        td { style: "padding: 8px 12px; text-align: center;", if alert { "🚨" } else { "—" } }
                                    }
                                }
                            }
                        }
                    }
                }

                // Tracepoint Config
                div { style: "margin-top: 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 20px;",
                    h3 { style: "font-size: 12px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.08em; margin: 0 0 16px;", "⚙️ Tracepoint Configuration" }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 8px;",
                        ConfigToggle { label: "sys_enter_mprotect".to_string(), enabled: true, description: "Memory protection changes".to_string() }
                        ConfigToggle { label: "sys_enter_execve".to_string(), enabled: true, description: "Process execution".to_string() }
                        ConfigToggle { label: "sys_enter_connect".to_string(), enabled: true, description: "Network connections".to_string() }
                        ConfigToggle { label: "sys_enter_ptrace".to_string(), enabled: true, description: "Process debugging".to_string() }
                        ConfigToggle { label: "sys_enter_write".to_string(), enabled: false, description: "⚠️ Event flood riski".to_string() }
                        ConfigToggle { label: "sys_enter_read".to_string(), enabled: false, description: "⚠️ Event flood riski".to_string() }
                    }
                }
            }
        }
    }
}
