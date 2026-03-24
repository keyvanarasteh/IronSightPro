//! Multi-language (i18n) support for IronSight UI.
//!
//! Usage:
//!   1. Call `use_context_provider(|| Signal::new(Locale::En))` in your root App.
//!   2. In any component: `let t = use_i18n();` then `t("nav.dashboard")`.
//!   3. Toggle language with `set_locale(Locale::Tr)`.

use dioxus::prelude::*;
use std::collections::HashMap;

// ───── Supported Locales ─────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Tr,
}

impl Locale {
    pub fn label(&self) -> &'static str {
        match self {
            Locale::En => "English",
            Locale::Tr => "Türkçe",
        }
    }
    pub fn flag(&self) -> &'static str {
        match self {
            Locale::En => "🇬🇧",
            Locale::Tr => "🇹🇷",
        }
    }
    pub fn all() -> &'static [Locale] {
        &[Locale::En, Locale::Tr]
    }
}

// ───── Translation Table ─────────────────────────────
pub type TranslationMap = HashMap<&'static str, &'static str>;

fn english() -> TranslationMap {
    let mut m = HashMap::new();

    // ── Sidebar Groups ──
    m.insert("nav.group.monitoring", "MONITORING");
    m.insert("nav.group.forensics", "FORENSICS");
    m.insert("nav.group.response", "RESPONSE");
    m.insert("nav.group.dashboards", "DASHBOARDS");
    m.insert("nav.group.design_system", "DESIGN SYSTEM");
    m.insert("nav.group.project_tasks", "PROJECT TASKS");
    m.insert("nav.group.reports_logs", "REPORTS & LOGS");

    // ── Sidebar Items ──
    m.insert("nav.dashboard", "🏠 Dashboard");
    m.insert("nav.process_monitor", "👁️ Process Monitor");
    m.insert("nav.threat_analysis", "🎯 Threat Analysis");
    m.insert("nav.network_intel", "🌐 Network Intel");
    m.insert("nav.memory_forensics", "🧠 Memory Forensics");
    m.insert("nav.security_audit", "🔒 Security Audit");
    m.insert("nav.kernel_monitor", "⚡ Kernel Monitor");
    m.insert("nav.response_center", "🛡️ Response Center");
    m.insert("nav.reports", "📊 Reports");
    m.insert("nav.configuration", "⚙️ Configuration");
    m.insert("nav.memory_forensics_dash", "🧬 Memory Forensics");
    m.insert("nav.heuristic_monitor", "⚡ Heuristic Monitor");
    m.insert("nav.incident_reporting", "📋 Incident Reporting");
    m.insert("nav.network_monitoring", "🌐 Network Intelligence");
    m.insert("nav.process_inspector", "🖥️ Process Inspector");
    m.insert("nav.process_explorer", "🔬 Process Explorer");
    m.insert("nav.process_monitor_dash", "📊 Process Monitor");
    m.insert("nav.forensic_response", "🎯 Forensic Response");
    m.insert("nav.component_showcase", "🧩 Component Showcase");
    m.insert("nav.idea_doc", "Idea Document");
    m.insert("nav.plan_doc", "Implementation Plan");
    m.insert("nav.roadmap_doc", "Roadmap");

    // ── Dashboard ──
    m.insert("dash.system_status", "SYSTEM STATUS");
    m.insert("dash.scanning", "⏳ Scanning System...");
    m.insert("dash.refresh", "▶ Refresh Scan");
    m.insert("dash.loading", "🔄 Running initial system scan...");
    m.insert("dash.threat_index", "AGGREGATED THREAT INDEX");
    m.insert("dash.risk_index", "RISK INDEX");
    m.insert("dash.critical", "CRITICAL");
    m.insert("dash.high", "HIGH");
    m.insert("dash.medium", "MEDIUM");
    m.insert("dash.processes", "PROCESSES");
    m.insert("dash.scan_time", "SCAN TIME");
    m.insert("dash.active_assessments", "Active Process Assessments");
    m.insert("dash.realtime_telemetry", "Real-time process telemetry and heuristic analysis");

    // ── Theme ──
    m.insert("theme.dark", "Dark Mode");
    m.insert("theme.light", "Light Mode");

    // ── Common ──
    m.insert("common.version", "SecOps Forensics v0.1.0");
    m.insert("common.language", "Language");
    m.insert("common.close", "Close");
    m.insert("common.save", "Save");
    m.insert("common.cancel", "Cancel");
    m.insert("common.search", "Search...");
    m.insert("common.no_data", "No data available");

    // ── Page Titles ──
    m.insert("page.processes", "👁️ Process Monitor");
    m.insert("page.threats", "🎯 Threat Analysis");
    m.insert("page.network", "🌐 Network Intelligence");
    m.insert("page.memory", "🧠 Memory Forensics");
    m.insert("page.security", "🔒 Security Audit");
    m.insert("page.kernel", "⚡ Kernel Monitor");
    m.insert("page.response", "🛡️ Response Center");
    m.insert("page.reports", "📊 Reports");
    m.insert("page.config", "⚙️ Configuration");
    m.insert("page.alerts", "🚨 Alerts");

    // ── Stats / EDR ──
    m.insert("edr.total_sockets", "Total Sockets");
    m.insert("edr.listeners", "Listeners");
    m.insert("edr.external", "External");
    m.insert("edr.suspicious", "Suspicious");
    m.insert("edr.regions", "Regions");
    m.insert("edr.wx_violations", "W^X Violations");
    m.insert("edr.anon_exec", "Anon Exec");
    m.insert("edr.patterns", "Patterns");
    m.insert("edr.all_connections", "All Connections");
    m.insert("edr.dns", "DNS");
    m.insert("edr.event_stream", "Event Stream");
    m.insert("edr.live", "LIVE");
    m.insert("edr.paused", "PAUSED");

    m
}

fn turkish() -> TranslationMap {
    let mut m = HashMap::new();

    // ── Sidebar Groups ──
    m.insert("nav.group.monitoring", "İZLEME");
    m.insert("nav.group.forensics", "ADLİ BİLİŞİM");
    m.insert("nav.group.response", "MÜDAHALE");
    m.insert("nav.group.dashboards", "PANOLAR");
    m.insert("nav.group.design_system", "TASARIM SİSTEMİ");
    m.insert("nav.group.project_tasks", "PROJE GÖREVLERİ");
    m.insert("nav.group.reports_logs", "RAPORLAR & GÜNLÜKLER");

    // ── Sidebar Items ──
    m.insert("nav.dashboard", "🏠 Kontrol Paneli");
    m.insert("nav.process_monitor", "👁️ Süreç İzleyici");
    m.insert("nav.threat_analysis", "🎯 Tehdit Analizi");
    m.insert("nav.network_intel", "🌐 Ağ İstihbaratı");
    m.insert("nav.memory_forensics", "🧠 Bellek Adli Bilişim");
    m.insert("nav.security_audit", "🔒 Güvenlik Denetimi");
    m.insert("nav.kernel_monitor", "⚡ Çekirdek İzleyici");
    m.insert("nav.response_center", "🛡️ Müdahale Merkezi");
    m.insert("nav.reports", "📊 Raporlar");
    m.insert("nav.configuration", "⚙️ Yapılandırma");
    m.insert("nav.memory_forensics_dash", "🧬 Bellek Adli Bilişim");
    m.insert("nav.heuristic_monitor", "⚡ Sezgisel İzleyici");
    m.insert("nav.incident_reporting", "📋 Olay Raporlama");
    m.insert("nav.network_monitoring", "🌐 Ağ İstihbaratı");
    m.insert("nav.process_inspector", "🖥️ Süreç Denetçisi");
    m.insert("nav.process_explorer", "🔬 Süreç Keşfi");
    m.insert("nav.process_monitor_dash", "📊 Süreç İzleyici");
    m.insert("nav.forensic_response", "🎯 Adli Müdahale");
    m.insert("nav.component_showcase", "🧩 Bileşen Vitrini");
    m.insert("nav.idea_doc", "Fikir Dokümanı");
    m.insert("nav.plan_doc", "Uygulama Planı");
    m.insert("nav.roadmap_doc", "Yol Haritası");

    // ── Dashboard ──
    m.insert("dash.system_status", "SİSTEM DURUMU");
    m.insert("dash.scanning", "⏳ Sistem Taranıyor...");
    m.insert("dash.refresh", "▶ Taramayı Yenile");
    m.insert("dash.loading", "🔄 İlk sistem taraması çalışıyor...");
    m.insert("dash.threat_index", "BÜTÜNLEŞİK TEHDİT ENDEKSİ");
    m.insert("dash.risk_index", "RİSK ENDEKSİ");
    m.insert("dash.critical", "KRİTİK");
    m.insert("dash.high", "YÜKSEK");
    m.insert("dash.medium", "ORTA");
    m.insert("dash.processes", "SÜREÇLER");
    m.insert("dash.scan_time", "TARAMA SÜRESİ");
    m.insert("dash.active_assessments", "Aktif Süreç Değerlendirmeleri");
    m.insert("dash.realtime_telemetry", "Gerçek zamanlı süreç telemetrisi ve sezgisel analiz");

    // ── Theme ──
    m.insert("theme.dark", "Karanlık Mod");
    m.insert("theme.light", "Aydınlık Mod");

    // ── Common ──
    m.insert("common.version", "GüvOps Adli Bilişim v0.1.0");
    m.insert("common.language", "Dil");
    m.insert("common.close", "Kapat");
    m.insert("common.save", "Kaydet");
    m.insert("common.cancel", "İptal");
    m.insert("common.search", "Ara...");
    m.insert("common.no_data", "Veri bulunamadı");

    // ── Page Titles ──
    m.insert("page.processes", "👁️ Süreç İzleyici");
    m.insert("page.threats", "🎯 Tehdit Analizi");
    m.insert("page.network", "🌐 Ağ İstihbaratı");
    m.insert("page.memory", "🧠 Bellek Adli Bilişim");
    m.insert("page.security", "🔒 Güvenlik Denetimi");
    m.insert("page.kernel", "⚡ Çekirdek İzleyici");
    m.insert("page.response", "🛡️ Müdahale Merkezi");
    m.insert("page.reports", "📊 Raporlar");
    m.insert("page.config", "⚙️ Yapılandırma");
    m.insert("page.alerts", "🚨 Uyarılar");

    // ── Stats / EDR ──
    m.insert("edr.total_sockets", "Toplam Soketler");
    m.insert("edr.listeners", "Dinleyiciler");
    m.insert("edr.external", "Harici");
    m.insert("edr.suspicious", "Şüpheli");
    m.insert("edr.regions", "Bölgeler");
    m.insert("edr.wx_violations", "W^X İhlalleri");
    m.insert("edr.anon_exec", "Anonim Çalıştırılabilir");
    m.insert("edr.patterns", "Kalıplar");
    m.insert("edr.all_connections", "Tüm Bağlantılar");
    m.insert("edr.dns", "DNS");
    m.insert("edr.event_stream", "Olay Akışı");
    m.insert("edr.live", "CANLI");
    m.insert("edr.paused", "DURDURULDU");

    m
}

// ───── I18n Context ──────────────────────────────────

/// Get all translations for a locale.
pub fn translations_for(locale: Locale) -> TranslationMap {
    match locale {
        Locale::En => english(),
        Locale::Tr => turkish(),
    }
}

/// Hook: get the current locale signal from context.
pub fn use_locale() -> Signal<Locale> {
    use_context::<Signal<Locale>>()
}

/// Hook: returns a closure that translates a key using the current locale.
/// Usage: `let t = use_i18n(); rsx! { span { "{t(\"nav.dashboard\")}" } }`
pub fn use_i18n() -> impl Fn(&str) -> String {
    let locale = use_locale();
    let map = translations_for(*locale.read());
    move |key: &str| -> String {
        map.get(key).map(|s| s.to_string()).unwrap_or_else(|| format!("[{}]", key))
    }
}

/// Component: Language selector dropdown
#[component]
pub fn LanguageSelector() -> Element {
    let mut locale = use_locale();
    let current = *locale.read();

    rsx! {
        div { style: "display: flex; align-items: center; gap: 4px;",
            for lang in Locale::all().iter() {
                {
                    let l = *lang;
                    let is_active = l == current;
                    let bg = if is_active { "var(--qs-primary)" } else { "transparent" };
                    let fg = if is_active { "var(--qs-primary-fg, #fff)" } else { "var(--qs-fg-muted)" };
                    rsx! {
                        button {
                            key: "{l:?}",
                            style: "display: flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; border: 1px solid var(--qs-border); background: {bg}; color: {fg}; cursor: pointer; font-size: 11px; font-weight: 600; transition: all 0.15s;",
                            onclick: move |_| locale.set(l),
                            span { "{l.flag()}" }
                            span { "{l.label()}" }
                        }
                    }
                }
            }
        }
    }
}
