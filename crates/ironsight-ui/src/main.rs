//! IronSight Desktop Dashboard — Dioxus 0.6
//!
//! Dark hacker-themed EDR visualization.

mod bridge;
mod views;
mod components;
mod i18n;

use dioxus::prelude::*;
use views::*;
use components::*;
use components::card::*;
use components::button::*;
use i18n::*;

use pulldown_cmark::{html, Parser, Options};

use bridge::{run_scan, UiSnapshot};

const VARIABLES_CSS: &str = include_str!("../assets/css/variables.css");
const LAYOUT_CSS: &str = include_str!("../assets/css/layout.css");
const COMPONENTS_CSS: &str = include_str!("../assets/css/components.css");
const MARKDOWN_CSS: &str = include_str!("../assets/css/markdown.css");
const DASHBOARDS_CSS: &str = include_str!("../assets/css/dashboards.css");
const QSTATIC_CSS: &str = include_str!("../assets/css/qstatic.css");

const THEME_TOGGLE_CSS: &str = r#"
.sidebar-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.theme-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    border-radius: 6px;
    background: var(--bg-hover);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    cursor: pointer;
    font-family: var(--font-mono);
    font-size: 12px;
    transition: all 0.2s;
    width: 100%;
    justify-content: center;
}
.theme-toggle:hover {
    background: var(--bg-card);
    color: var(--text-primary);
    border-color: var(--accent-cyan);
}
.theme-toggle .icon {
    font-size: 16px;
    transition: transform 0.3s ease;
}
.theme-toggle:hover .icon {
    transform: rotate(20deg);
}
"#;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Dashboard {},
    #[route("/processes")]
    Processes {},
    #[route("/security")]
    Security {},
    #[route("/network")]
    Network {},
    #[route("/memory")]
    Memory {},
    #[route("/alerts")]
    Alerts {},
    #[route("/settings")]
    Settings {},
    #[route("/threats")]
    ThreatAnalysis {},
    #[route("/kernel")]
    KernelMonitor {},
    #[route("/response")]
    ResponseCenter {},
    #[route("/reports")]
    Reports {},
    #[route("/config")]
    Configuration {},
    #[route("/showcase")]
    Showcase {},
    #[route("/dash/memory-forensics")]
    CrateMonitoring {},
    #[route("/dash/heuristic-monitor")]
    HeuristicMonitor {},
    #[route("/dash/incident-reporting")]
    IncidentReporting {},
    #[route("/dash/network-monitoring")]
    NetworkMonitoring {},
    #[route("/dash/pro-dash")]
    ProDash {},
    #[route("/dash/process-explorer")]
    ProcessExplorer {},
    #[route("/dash/process-monitor")]
    ProcessMonitor {},
    #[route("/dash/response")]
    ResponseDashboard {},
    #[route("/docs/idea")]
    IdeaDoc {},
    #[route("/docs/plan")]
    PlanDoc {},
    #[route("/docs/roadmap")]
    RoadmapDoc {},
    #[route("/report/00-educational")]
    Page00Educational {},
    #[route("/report/00-rust-workspace-olusturma")]
    Page00RustWorkspaceOlusturma {},
    #[route("/report/01-core-unit-testleri")]
    Page01CoreUnitTestleri {},
    #[route("/report/01-educational")]
    Page01Educational {},
    #[route("/report/02-educational")]
    Page02Educational {},
    #[route("/report/02-security-layer")]
    Page02SecurityLayer {},
    #[route("/report/03-educational")]
    Page03Educational {},
    #[route("/report/03-network-layer")]
    Page03NetworkLayer {},
    #[route("/report/04-educational")]
    Page04Educational {},
    #[route("/report/04-memory-scanner")]
    Page04MemoryScanner {},
    #[route("/report/05-educational")]
    Page05Educational {},
    #[route("/report/05-heuristic-response")]
    Page05HeuristicResponse {},
    #[route("/report/06-educational")]
    Page06Educational {},
    #[route("/report/06-report-service")]
    Page06ReportService {},
    #[route("/report/07-educational")]
    Page07Educational {},
    #[route("/report/07-service-hardening")]
    Page07ServiceHardening {},
    #[route("/report/08-educational")]
    Page08Educational {},
    #[route("/report/08-time-decay-scoring")]
    Page08TimeDecayScoring {},
    #[route("/report/09-dioxus-dashboard")]
    Page09DioxusDashboard {},
    #[route("/report/09-educational")]
    Page09Educational {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Global theme state: true = dark mode (default)
    let is_dark = use_signal(|| true);
    use_context_provider(|| is_dark);

    // Global locale state
    let locale = use_signal(|| Locale::En);
    use_context_provider(|| locale);

    rsx! {
        style { {VARIABLES_CSS} }
        style { {LAYOUT_CSS} }
        style { {COMPONENTS_CSS} }
        style { {MARKDOWN_CSS} }
        style { {THEME_TOGGLE_CSS} }
        style { {DASHBOARDS_CSS} }
        style { {QSTATIC_CSS} }
        Router::<Route> {}
    }
}

/// Navigation Bar layout component wrapping the sidebar and content area
#[component]
fn NavBar() -> Element {
    let mut is_dark: Signal<bool> = use_context();
    let t = use_i18n();

    let theme_attr = if *is_dark.read() { "dark" } else { "light" };
    let theme_icon = if *is_dark.read() { "🌙" } else { "☀️" };
    let theme_label = if *is_dark.read() { t("theme.dark") } else { t("theme.light") };

    // Pre-compute all translations
    let version = t("common.version");
    let grp_monitoring = t("nav.group.monitoring");
    let nav_dashboard = t("nav.dashboard");
    let nav_process_monitor = t("nav.process_monitor");
    let nav_threat_analysis = t("nav.threat_analysis");
    let grp_forensics = t("nav.group.forensics");
    let nav_network_intel = t("nav.network_intel");
    let nav_memory_forensics = t("nav.memory_forensics");
    let nav_security_audit = t("nav.security_audit");
    let nav_kernel_monitor = t("nav.kernel_monitor");
    let grp_response = t("nav.group.response");
    let nav_response_center = t("nav.response_center");
    let nav_reports = t("nav.reports");
    let nav_configuration = t("nav.configuration");
    let grp_dashboards = t("nav.group.dashboards");
    let nav_mem_dash = t("nav.memory_forensics_dash");
    let nav_heuristic = t("nav.heuristic_monitor");
    let nav_incident = t("nav.incident_reporting");
    let nav_net_mon = t("nav.network_monitoring");
    let nav_proc_insp = t("nav.process_inspector");
    let nav_proc_exp = t("nav.process_explorer");
    let nav_proc_mon = t("nav.process_monitor_dash");
    let nav_forensic_resp = t("nav.forensic_response");
    let grp_design = t("nav.group.design_system");
    let nav_showcase = t("nav.component_showcase");
    let grp_project = t("nav.group.project_tasks");
    let nav_idea = t("nav.idea_doc");
    let nav_plan = t("nav.plan_doc");
    let nav_roadmap = t("nav.roadmap_doc");
    let grp_reports = t("nav.group.reports_logs");

    rsx! {
        div {
            class: "layout-container",
            "data-theme": "{theme_attr}",
            // Sidebar
            div { class: "sidebar",
                div { class: "sidebar-header",
                    span { class: "sidebar-title", "🔬 IRONSIGHT" }
                    div { style: "color: var(--text-muted); font-size: 10px; margin-top: 4px;", "{version}" }
                }

                div { class: "nav-menu",
                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_monitoring}" }
                        Link { to: Route::Dashboard {}, class: "nav-item", "{nav_dashboard}" }
                        Link { to: Route::Processes {}, class: "nav-item", "{nav_process_monitor}" }
                        Link { to: Route::ThreatAnalysis {}, class: "nav-item", "{nav_threat_analysis}" }
                    }

                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_forensics}" }
                        Link { to: Route::Network {}, class: "nav-item", "{nav_network_intel}" }
                        Link { to: Route::Memory {}, class: "nav-item", "{nav_memory_forensics}" }
                        Link { to: Route::Security {}, class: "nav-item", "{nav_security_audit}" }
                        Link { to: Route::KernelMonitor {}, class: "nav-item", "{nav_kernel_monitor}" }
                    }

                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_response}" }
                        Link { to: Route::ResponseCenter {}, class: "nav-item", "{nav_response_center}" }
                        Link { to: Route::Reports {}, class: "nav-item", "{nav_reports}" }
                        Link { to: Route::Configuration {}, class: "nav-item", "{nav_configuration}" }
                    }

                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_dashboards}" }
                        Link { to: Route::CrateMonitoring {}, class: "nav-item", "{nav_mem_dash}" }
                        Link { to: Route::HeuristicMonitor {}, class: "nav-item", "{nav_heuristic}" }
                        Link { to: Route::IncidentReporting {}, class: "nav-item", "{nav_incident}" }
                        Link { to: Route::NetworkMonitoring {}, class: "nav-item", "{nav_net_mon}" }
                        Link { to: Route::ProDash {}, class: "nav-item", "{nav_proc_insp}" }
                        Link { to: Route::ProcessExplorer {}, class: "nav-item", "{nav_proc_exp}" }
                        Link { to: Route::ProcessMonitor {}, class: "nav-item", "{nav_proc_mon}" }
                        Link { to: Route::ResponseDashboard {}, class: "nav-item", "{nav_forensic_resp}" }
                    }

                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_design}" }
                        Link { to: Route::Showcase {}, class: "nav-item", "{nav_showcase}" }
                    }

                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_project}" }
                        Link { to: Route::IdeaDoc {}, class: "nav-item", "{nav_idea}" }
                        Link { to: Route::PlanDoc {}, class: "nav-item", "{nav_plan}" }
                        Link { to: Route::RoadmapDoc {}, class: "nav-item", "{nav_roadmap}" }
                    }

                    div { class: "nav-group",
                        div { class: "nav-group-title", "{grp_reports}" }
                        Link { to: Route::Page00Educational {}, class: "nav-item", "00-educational.md" }
                        Link { to: Route::Page00RustWorkspaceOlusturma {}, class: "nav-item", "00-rust-workspace-olusturma.md" }
                        Link { to: Route::Page01CoreUnitTestleri {}, class: "nav-item", "01-core-unit-testleri.md" }
                        Link { to: Route::Page01Educational {}, class: "nav-item", "01-educational.md" }
                        Link { to: Route::Page02Educational {}, class: "nav-item", "02-educational.md" }
                        Link { to: Route::Page02SecurityLayer {}, class: "nav-item", "02-security-layer.md" }
                        Link { to: Route::Page03Educational {}, class: "nav-item", "03-educational.md" }
                        Link { to: Route::Page03NetworkLayer {}, class: "nav-item", "03-network-layer.md" }
                        Link { to: Route::Page04Educational {}, class: "nav-item", "04-educational.md" }
                        Link { to: Route::Page04MemoryScanner {}, class: "nav-item", "04-memory-scanner.md" }
                        Link { to: Route::Page05Educational {}, class: "nav-item", "05-educational.md" }
                        Link { to: Route::Page05HeuristicResponse {}, class: "nav-item", "05-heuristic-response.md" }
                        Link { to: Route::Page06Educational {}, class: "nav-item", "06-educational.md" }
                        Link { to: Route::Page06ReportService {}, class: "nav-item", "06-report-service.md" }
                        Link { to: Route::Page07Educational {}, class: "nav-item", "07-educational.md" }
                        Link { to: Route::Page07ServiceHardening {}, class: "nav-item", "07-service-hardening.md" }
                        Link { to: Route::Page08Educational {}, class: "nav-item", "08-educational.md" }
                        Link { to: Route::Page08TimeDecayScoring {}, class: "nav-item", "08-time-decay-scoring.md" }
                        Link { to: Route::Page09DioxusDashboard {}, class: "nav-item", "09-dioxus-dashboard.md" }
                        Link { to: Route::Page09Educational {}, class: "nav-item", "09-educational.md" }
                    }
                }

                // Language Selector + Theme Toggle Footer
                div { class: "sidebar-footer", style: "flex-direction: column; gap: 8px;",
                    LanguageSelector {}
                    button {
                        class: "theme-toggle",
                        onclick: move |_| { let current = *is_dark.read(); is_dark.set(!current); },
                        span { class: "icon", "{theme_icon}" }
                        span { "{theme_label}" }
                    }
                }
            }

            // Routed Content
            div { class: "content-area",
                Outlet::<Route> {}
            }
        }
    }
}


/// Helper to load & render Markdown from an absolute/relative path
#[component]
fn MarkdownView(path: ReadSignal<String>) -> Element {
    let content = use_resource(move || {
        let p = path.read().clone();
        async move {
            match tokio::fs::read_to_string(&p).await {
                Ok(file_content) => {
                    let mut options = Options::empty();
                    options.insert(Options::ENABLE_STRIKETHROUGH);
                    options.insert(Options::ENABLE_TABLES);
                    
                    let parser = Parser::new_ext(&file_content, options);
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);
                    
                    html_output
                }
                Err(e) => {
                    format!("<h2 style='color: var(--accent-red)'>Error loading file</h2><p>Could not read: <code>{}</code></p><p>Error: {}</p>", p, e)
                }
            }
        }
    });

    rsx! {
        div {
            class: "markdown-body",
            dangerous_inner_html: "{content.read().as_deref().unwrap_or(\"... loading ...\")}"
        }
    }
}





/// Background scanner for the real-time Dashboard
fn do_scan(mut snapshot: Signal<Option<UiSnapshot>>, mut scanning: Signal<bool>) {
    scanning.set(true);
    spawn(async move {
        let result = tokio::task::spawn_blocking(run_scan).await.expect("scan thread panicked");
        snapshot.set(Some(result));
        scanning.set(false);
    });
}

/// Main dashboard widget
#[component]
fn Dashboard() -> Element {
    let t = use_i18n();
    let snapshot = use_signal::<Option<UiSnapshot>>(|| None);
    let scanning = use_signal(|| false);

    // Pre-compute translations
    let lbl_status = t("dash.system_status");
    let lbl_scanning = t("dash.scanning");
    let lbl_refresh = t("dash.refresh");
    let lbl_loading = t("dash.loading");
    let lbl_threat = t("dash.threat_index");
    let lbl_risk = t("dash.risk_index");
    let lbl_critical = t("dash.critical");
    let lbl_high = t("dash.high");
    let lbl_medium = t("dash.medium");
    let lbl_processes = t("dash.processes");
    let lbl_scan_time = t("dash.scan_time");
    let lbl_assessments = t("dash.active_assessments");
    let lbl_telemetry = t("dash.realtime_telemetry");

    use_effect(move || {
        do_scan(snapshot, scanning);
    });

    let scan_btn_text = if *scanning.read() { lbl_scanning.clone() } else { lbl_refresh.clone() };

    rsx! {
        div { class: "app-container",
            div { class: "header",
                div {
                    style: "display: flex; gap: 12px; align-items: center;",
                    span { style: "font-size: 20px; font-weight: bold-600; letter-spacing: 1px;", "{lbl_status}" }
                    if let Some(snap) = snapshot.read().as_ref() {
                        SecurityBadge { level: ironsight_heuristic::ThreatLevel::from_score(snap.risk_index) }
                    }
                }
                div { class: "header-actions",
                    Button {
                        class: "btn btn-scan",
                        disabled: *scanning.read(),
                        onclick: move |_| { do_scan(snapshot, scanning); },
                        "{scan_btn_text}"
                    }
                }
            }

            match snapshot.read().as_ref() {
                None => rsx! {
                    div { class: "loading pulse", "{lbl_loading}" }
                },
                Some(snap) => rsx! {
                    div { class: "main-content",
                        
                        // Threat Level Overview Card
                        Card {
                            style: "margin-bottom: 24px; padding: 20px; background: rgba(0,0,0,0.2); border: 1px solid var(--border-color);",
                            div {
                                style: "display: flex; justify-content: space-between; margin-bottom: 12px;",
                                span { style: "font-size: 14px; color: var(--text-muted);", "{lbl_threat}" }
                                span { style: "font-size: 14px; font-weight: 600;", "{snap.risk_index:.1} / 100" }
                            }
                            ThreatGauge { score: snap.risk_index as f32 }
                        }

                        div { style: "display: flex; gap: 16px; margin-bottom: 24px; flex-wrap: wrap;",
                            StatCard { label: lbl_risk.clone(), value: format!("{:.1}", snap.risk_index), variant: "risk".to_string() }
                            StatCard { label: lbl_critical.clone(), value: snap.critical_count.to_string(), variant: "critical".to_string() }
                            StatCard { label: lbl_high.clone(), value: snap.high_count.to_string(), variant: "high".to_string() }
                            StatCard { label: lbl_medium.clone(), value: snap.medium_count.to_string(), variant: "medium".to_string() }
                            StatCard { label: lbl_processes.clone(), value: snap.total_processes.to_string(), variant: "total".to_string() }
                            StatCard { label: lbl_scan_time.clone(), value: format!("{}ms", snap.scan_time_ms), variant: "time".to_string() }
                        }

                        // We can wrap ProcessTable in a nice Card
                        Card {
                            CardHeader {
                                CardTitle { "{lbl_assessments}" }
                                CardDescription { "{lbl_telemetry}" }
                            }
                            CardContent {
                                ProcessTable { assessments: snap.assessments.clone() }
                            }
                        }
                    }
                },
            }
        }
    }
}

