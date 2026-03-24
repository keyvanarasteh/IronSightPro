use dioxus::prelude::*;
use crate::components::dashboards::*;

/// Response Dashboard — Forensic Orchestrator
#[component]
pub fn ResponseDashboard() -> Element {
    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                div { style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "🎯 FORENSIC ORCHESTRATOR" }
                    StatusBadge { label: "SYSTEM STABLE".to_string(), severity: "success".to_string() }
                }
            }
            div { class: "main-content",
                div { class: "dash-grid-3",
                    // Left column: Metrics
                    div { class: "dash-col",
                        div { class: "dash-stats-row",
                            DashStatCard { label: "Active Interventions".to_string(), value: "14".to_string(), variant: "blue".to_string(), trend: "↑12%".to_string() }
                            DashStatCard { label: "Safety Exclusions".to_string(), value: "152".to_string(), variant: "default".to_string(), trend: "Static".to_string() }
                        }
                        GlassPanel { style: "padding: 20px;".to_string(),
                            PanelHeader { title: "Integrity Monitor".to_string(), span {} }
                            DashProgress { label: "Memory Dump Success".to_string(), value: 98.4, value_text: "98.4%".to_string(), variant: "success".to_string() }
                            DashProgress { label: "Suspend-to-Dump Latency".to_string(), value: 65.0, value_text: "0.84ms".to_string(), variant: "warning".to_string() }
                            DashProgress { label: "SIGKILL Acknowledgement".to_string(), value: 100.0, value_text: "100%".to_string(), variant: "purple".to_string() }
                        }
                    }

                    // Center: Forensic Sequence Feed
                    div { class: "dash-col",
                        span { style: "font-size: 14px; font-weight: 700; color: var(--text-primary); margin-bottom: 4px;", "⚡ Live Forensic Sequence" }

                        // Kill Sequence Entry
                        GlassPanel { style: "padding: 20px; border-left: 4px solid var(--accent-red);".to_string(),
                            div { style: "display: flex; justify-content: space-between; margin-bottom: 12px; flex-wrap: wrap; gap: 8px;",
                                div {
                                    div { style: "display: flex; gap: 6px; margin-bottom: 4px; flex-wrap: wrap;",
                                        StatusBadge { label: "ACTION::FORENSIC_KILL".to_string(), severity: "critical".to_string() }
                                        span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "PID 8912" }
                                    }
                                    h3 { style: "font-size: 16px; font-weight: 700; margin: 0; color: var(--text-primary);", "mal_backdoor_v4" }
                                    p { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono); margin: 2px 0 0;", "/tmp/.hidden/payload_64" }
                                }
                                div { style: "text-align: right;",
                                    span { style: "font-size: 24px; font-weight: 900; color: var(--accent-red); font-family: var(--font-mono);", "98.2" }
                                    p { style: "font-size: 9px; color: var(--text-muted); font-weight: 700; text-transform: uppercase; letter-spacing: 1px;", "Threat Score" }
                                }
                            }
                            TimelineContainer {
                                TimelineStep { step: 1, label: "Suspend Process".to_string(), status: "SIGSTOP SUCCESS".to_string(), variant: "success".to_string() }
                                TimelineStep { step: 2, label: "Forensic Memory Dump".to_string(), status: "DUMPED 124.2 MB".to_string(), variant: "success".to_string() }
                                TimelineStep { step: 3, label: "Kill Process".to_string(), status: "SIGKILL ACK".to_string(), variant: "danger".to_string() }
                            }
                        }

                        // Exclusion Entry
                        GlassPanel { style: "padding: 20px; border-left: 4px solid var(--accent-blue); opacity: 0.6;".to_string(),
                            div { style: "display: flex; justify-content: space-between; flex-wrap: wrap; gap: 8px;",
                                div {
                                    div { style: "display: flex; gap: 6px; margin-bottom: 4px; flex-wrap: wrap;",
                                        StatusBadge { label: "SAFETY EXCLUSION".to_string(), severity: "info".to_string() }
                                        span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "PID 1" }
                                    }
                                    h3 { style: "font-size: 16px; font-weight: 700; color: var(--text-secondary); font-style: italic; margin: 0;", "systemd" }
                                    p { style: "font-size: 11px; color: var(--text-muted); margin: 6px 0 0;", "Skipped: critical system process matched ExclusionList::pids." }
                                }
                                span { style: "font-size: 16px; font-weight: 700; color: var(--text-muted); font-family: var(--font-mono);", "--" }
                            }
                        }

                        // Suspend Entry
                        GlassPanel { style: "padding: 20px; border-left: 4px solid var(--accent-orange);".to_string(),
                            div { style: "display: flex; justify-content: space-between; margin-bottom: 12px; flex-wrap: wrap; gap: 8px;",
                                div {
                                    div { style: "display: flex; gap: 6px; margin-bottom: 4px; flex-wrap: wrap;",
                                        StatusBadge { label: "ACTION::SUSPEND".to_string(), severity: "warning".to_string() }
                                        span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "PID 4421" }
                                    }
                                    h3 { style: "font-size: 16px; font-weight: 700; margin: 0; color: var(--text-primary);", "suspicious_script.sh" }
                                }
                                span { style: "font-size: 24px; font-weight: 900; color: var(--accent-orange); font-family: var(--font-mono);", "64.5" }
                            }
                            p {
                                class: "dash-code-block",
                                style: "font-style: italic; color: var(--text-secondary);",
                                "\"Process frozen for manual triage by SOC-1.\""
                            }
                        }
                    }

                    // Right column: Policies
                    GlassPanel { style: "padding: 20px;".to_string(),
                        PanelHeader { title: "Configured Policies".to_string(), span {} }
                        div { style: "margin-bottom: 20px;",
                            span { style: "font-size: 10px; font-weight: 800; color: var(--text-muted); text-transform: uppercase; display: block; margin-bottom: 8px;", "Protected Namespaces" }
                            div { style: "display: flex; flex-wrap: wrap; gap: 6px;",
                                span { class: "policy-chip", "/usr/sbin/*" }
                                span { class: "policy-chip", "/lib/systemd/*" }
                                span { class: "policy-chip", "/bin/bash" }
                            }
                        }
                        div { style: "margin-bottom: 20px;",
                            span { style: "font-size: 10px; font-weight: 800; color: var(--text-muted); text-transform: uppercase; display: block; margin-bottom: 8px;", "Response Thresholds" }
                            div { style: "display: flex; flex-direction: column; gap: 8px;",
                                div { class: "threshold-row",
                                    span { style: "font-size: 11px; color: var(--text-secondary);", "Suspend @" }
                                    span { style: "font-size: 11px; font-weight: 700; color: var(--accent-orange); font-family: var(--font-mono);", "> 60.0" }
                                }
                                div { class: "threshold-row",
                                    span { style: "font-size: 11px; color: var(--text-secondary);", "Forensic Kill @" }
                                    span { style: "font-size: 11px; font-weight: 700; color: var(--accent-red); font-family: var(--font-mono);", "> 85.0" }
                                }
                            }
                        }
                        div { class: "dash-info-block",
                            h4 { style: "font-size: 11px; font-weight: 700; color: var(--accent-blue); margin: 0 0 4px 0;", "Forensic Storage" }
                            p { style: "font-size: 10px; color: var(--text-muted); margin: 0 0 8px 0;", "Memory dumps stored at:" }
                            div { class: "dash-code-block", style: "color: var(--accent-blue);", "/var/lib/ironsight/dumps/" }
                        }
                    }
                }
            }
        }
    }
}
