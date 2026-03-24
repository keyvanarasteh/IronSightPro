use dioxus::prelude::*;
use crate::components::dashboards::*;

struct Incident { id: &'static str, title: &'static str, severity: &'static str, status: &'static str, timestamp: &'static str, assignee: &'static str, pid: u32, description: &'static str }

#[component]
pub fn IncidentReporting() -> Element {
    let incidents = vec![
        Incident { id: "INC-001", title: "Malicious Process Injection", severity: "critical", status: "Active", timestamp: "2024-01-15 10:04:22", assignee: "SOC-1", pid: 9901, description: "Memory injection via VirtualAllocEx detected in PID 9901." },
        Incident { id: "INC-002", title: "Unsigned Binary Execution", severity: "high", status: "Active", timestamp: "2024-01-15 10:05:01", assignee: "SOC-2", pid: 8812, description: "Unsigned binary svchost.exe launched from non-standard path." },
        Incident { id: "INC-003", title: "Suspicious Network Callback", severity: "high", status: "Investigating", timestamp: "2024-01-15 09:58:30", assignee: "SOC-1", pid: 9901, description: "Outbound connection to suspicious external IP 45.12.33.1 on port 80." },
        Incident { id: "INC-004", title: "Excessive CPU Utilization", severity: "medium", status: "Resolved", timestamp: "2024-01-15 09:45:00", assignee: "SOC-3", pid: 3310, description: "Process temp_installer consuming 95% CPU. Possible cryptominer." },
        Incident { id: "INC-005", title: "Kernel Audit Log Anomaly", severity: "low", status: "Closed", timestamp: "2024-01-15 08:30:15", assignee: "SOC-2", pid: 1, description: "Unusual mprotect syscalls from PID 1. Likely benign." },
    ];

    rsx! {
        div { class: "app-container dash-page",
            div { class: "header",
                div { style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "📋 INCIDENT REPORTING" }
                    StatusBadge { label: "SOC ACTIVE".to_string(), severity: "success".to_string() }
                }
                div { class: "header-actions",
                    span { style: "font-size: 11px; color: var(--text-muted); font-family: var(--font-mono);", "2 Active • 1 Investigating" }
                }
            }
            div { class: "main-content",
                div { class: "dash-stats-row",
                    DashStatCard { label: "Total Incidents".to_string(), value: "5".to_string(), icon: "📊".to_string(), variant: "blue".to_string() }
                    DashStatCard { label: "Critical".to_string(), value: "1".to_string(), icon: "🚨".to_string(), variant: "critical".to_string() }
                    DashStatCard { label: "Active".to_string(), value: "2".to_string(), icon: "🔴".to_string(), variant: "high".to_string() }
                    DashStatCard { label: "Avg Response".to_string(), value: "4.2m".to_string(), icon: "⏱️".to_string(), variant: "green".to_string(), trend: "-12% vs avg".to_string() }
                }
                div { class: "dash-card-list",
                    for inc in incidents.iter() {
                        GlassPanel { style: "padding: 20px;".to_string(),
                            div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 12px; flex-wrap: wrap; gap: 8px;",
                                div {
                                    div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 6px; flex-wrap: wrap;",
                                        StatusBadge { label: inc.severity.to_uppercase(), severity: inc.severity.to_string() }
                                        span { style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);", "{inc.id}" }
                                        span { style: "font-family: var(--font-mono); font-size: 10px; color: var(--text-muted);", "PID {inc.pid}" }
                                    }
                                    h3 { style: "font-size: 16px; font-weight: 700; color: var(--text-primary); margin: 0;", "{inc.title}" }
                                }
                                StatusBadge {
                                    label: inc.status.to_string(),
                                    severity: match inc.status { "Active" => "critical", "Investigating" => "warning", "Resolved" => "success", _ => "info" }.to_string(),
                                }
                            }
                            p { style: "font-size: 12px; color: var(--text-secondary); line-height: 1.6; margin: 0 0 12px 0;", "{inc.description}" }
                            div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                                span { style: "font-size: 10px; color: var(--text-muted); font-family: var(--font-mono);", "🕐 {inc.timestamp}" }
                                span { style: "font-size: 10px; color: var(--text-muted);", "👤 {inc.assignee}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
