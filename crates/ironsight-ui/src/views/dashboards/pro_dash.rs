use dioxus::prelude::*;
use crate::components::dashboards::*;

static PROCESSES: &[(u32, &str, f64, &str, f64, &str, &str, &str, f64, bool)] = &[
    (1204, "systemd", 0.1, "12MB", 5.0, "root", "/usr/lib/systemd/systemd", "e3b0c442...", 4.2, true),
    (4492, "chrome.exe", 4.2, "450MB", 12.0, "user", "/usr/bin/chrome", "8f3a1c...", 5.1, true),
    (8812, "svchost.exe", 0.5, "24MB", 45.0, "SYSTEM", "/usr/sbin/svchost", "a1b2c3...", 6.8, false),
    (9901, "unknown_agent", 12.5, "120MB", 88.0, "user", "/tmp/ldr_stage2", "f2e3d4...", 7.9, false),
];

static SOCKETS: &[(u32, &str, &str, &str, &str)] = &[
    (9901, "192.168.1.5:49211", "45.12.33.1:80", "ESTABLISHED", "TCP"),
    (4492, "192.168.1.5:55201", "142.250.190.46:443", "ESTABLISHED", "TCP"),
    (8812, "0.0.0.0:135", "*:*", "LISTENING", "TCP"),
];

#[component]
pub fn ProDash() -> Element {
    let mut selected_pid = use_signal::<Option<u32>>(|| None);
    let mut active_view = use_signal(|| "processes".to_string());

    let detail_panel = {
        let sel = *selected_pid.read();
        if let Some(pid) = sel {
            if let Some(p) = PROCESSES.iter().find(|p| p.0 == pid) {
                let sig_val = if p.9 { "Valid (Microsoft)" } else { "Unsigned / Unknown" };
                let ent_str = format!("{:.1} ({})", p.8, if p.8 > 7.0 { "High" } else { "Normal" });
                rsx! {
                    DetailHeader { name: p.1.to_string(), pid: p.0.to_string() }
                    MetadataSection { title: "Identification".to_string(),
                        MetaRow { label: "Path".to_string(), value: p.6.to_string() }
                        MetaRow { label: "SHA-256".to_string(), value: p.7.to_string() }
                        MetaRow { label: "Signature".to_string(), value: sig_val.to_string(), highlight: !p.9 }
                    }
                    div { style: "margin-top: 12px;",
                        MetadataSection { title: "Live Analysis".to_string(),
                            MetaRow { label: "Entropy".to_string(), value: ent_str }
                            MetaRow { label: "Memory".to_string(), value: p.3.to_string() }
                            MetaRow { label: "CPU".to_string(), value: format!("{}%", p.2) }
                        }
                    }
                }
            } else { rsx! { div { style: "opacity:0.3; padding:40px; text-align:center;", "Process not found" } } }
        } else { rsx! { div { style: "opacity:0.3; padding:40px; text-align:center; font-style:italic;", "Select a process to inspect" } } }
    };

    let cur = active_view.read().clone();

    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "🖥️ PROCESS INSPECTOR" }
                StatusBadge { label: "LIVE TELEMETRY".to_string(), severity: "success".to_string() }
            }
            div { class: "main-content",
                div { style: "display: flex; gap: 0; border-bottom: 1px solid var(--border); margin-bottom: 16px;",
                    for (key, label) in [("processes", "⚡ Process Tree"), ("kernel", "📝 Kernel Logs"), ("sockets", "🌐 Network Sockets")] {
                        button {
                            class: if cur == key { "dash-tab-btn active" } else { "dash-tab-btn" },
                            onclick: move |_| active_view.set(key.to_string()),
                            "{label}"
                        }
                    }
                }
                div { class: "dash-grid-sidebar",
                    div {
                        if cur == "processes" {
                            DashTable {
                                DashTableHead { tr { DashTh {"PID"} DashTh {"Name"} DashTh {"CPU"} DashTh {"Risk"} DashTh {"User"} } }
                                DashTableBody {
                                    for p in PROCESSES.iter() {
                                        DashTr {
                                            style: if Some(p.0) == *selected_pid.read() { "background: var(--badge-info-bg);".to_string() } else { String::new() },
                                            onclick: move |_| selected_pid.set(Some(p.0)),
                                            DashTd { style: "color: var(--accent-blue);".to_string(), "{p.0}" }
                                            DashTd { style: "font-weight: 600;".to_string(), "{p.1}" }
                                            DashTd { "{p.2}%" }
                                            DashTd { ScoreBar { score: p.4 } }
                                            DashTd { style: "opacity: 0.6;".to_string(), "{p.5}" }
                                        }
                                    }
                                }
                            }
                        }
                        if cur == "kernel" {
                            LogFeedPanel { title: "Kernel Syscall Audit Log".to_string(),
                                for i in 0..8u32 {
                                    LogEntry { timestamp: format!("0x0042F{}", i), level: "mmap".to_string(), message: format!("addr=0x7fff.. len=4096 prot=RX — PID: {}", 9000+i) }
                                }
                            }
                        }
                        if cur == "sockets" {
                            DashTable {
                                DashTableHead { tr { DashTh {"PID"} DashTh {"Local"} DashTh {"Remote"} DashTh {"State"} DashTh {"Proto"} } }
                                DashTableBody {
                                    for s in SOCKETS.iter() {
                                        DashTr {
                                            DashTd { style: "color: var(--accent-blue); font-weight: 700;".to_string(), "{s.0}" }
                                            DashTd { "{s.1}" }
                                            DashTd { style: "color: var(--accent-green);".to_string(), "{s.2}" }
                                            DashTd { StatusBadge { label: s.3.to_string(), severity: if s.3 == "ESTABLISHED" { "success".to_string() } else { "warning".to_string() } } }
                                            DashTd { "{s.4}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    GlassPanel { style: "padding: 16px; align-self: start;".to_string(),
                        PanelHeader { title: "Properties".to_string(), span {} }
                        {detail_panel}
                    }
                }
            }
        }
    }
}
