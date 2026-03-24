use crate::components::accordion::*;
use crate::components::badge::*;
use crate::components::card::*;
use crate::components::separator::*;
use crate::components::tabs::*;
use crate::views::course_layout::CourseLayout;
use dioxus::prelude::*;

#[component]
pub fn CodeBlock(code: String, language: String) -> Element {
    rsx! {
        pre { class: "p-4 bg-[var(--bg-secondary)] rounded-md overflow-x-auto text-sm font-mono text-[var(--text-primary)] border border-[var(--border-primary)]",
            code { "{code}" }
        }
    }
}

#[component]
pub fn QuoteBlock(quote: String) -> Element {
    rsx! {
        blockquote { class: "border-l-4 border-[var(--accent-primary)] pl-4 py-1 my-2 bg-[var(--bg-hover)] italic text-[var(--text-secondary)]",
            "{quote}"
        }
    }
}

#[component]
pub fn Table(headers: Vec<String>, rows: Vec<Vec<Element>>) -> Element {
    rsx! {
        div { class: "overflow-x-auto w-full border border-[var(--border-primary)] rounded-md",
            table { class: "w-full text-sm text-left text-[var(--text-primary)]",
                thead { class: "text-xs text-[var(--text-secondary)] uppercase bg-[var(--bg-secondary)] border-b border-[var(--border-primary)]",
                    tr {
                        for h in headers {
                            th { class: "px-6 py-3 whitespace-nowrap", "{h}" }
                        }
                    }
                }
                tbody { class: "divide-y divide-[var(--border-primary)]",
                    for row in rows {
                        tr { class: "bg-transparent hover:bg-[var(--bg-hover)]",
                            for cell in row {
                                td { class: "px-6 py-4", {cell} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Page00Educational() -> Element {
    rsx! {
        CourseLayout { title: "00-educational".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/00-educational.md".to_string(),
            div { class: "flex flex-col gap-12 text-sm text-[var(--text-secondary)]",

                div { class: "border-l-4 border-[var(--accent-blue)] pl-6 py-2 bg-[var(--bg-hover)] rounded-r-lg",
                    p { class: "italic text-[var(--text-primary)] font-serif text-lg mb-2", "\"Kodu yazmak kolay — ama neden öyle yazdığını açıklamak zor. Burada her kararın arkasındaki sebebi anlatıyoruz.\"" }
                    p { class: "text-xs font-semibold uppercase tracking-wider text-[var(--accent-cyan)]", "Keyvan Arasteh · IronSight · 2026" }
                }

                div { class: "space-y-6",
                    h2 { class: "text-2xl font-bold text-[var(--text-primary)]", "1. Neden Workspace? Neden 9 Ayrı Crate?" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        Card {
                            CardHeader {
                                CardTitle { "Hastane Analojisi 🏥" }
                                CardDescription { "Her crate bağımsız çalışır ama birlikte bir sistemi oluşturur." }
                            }
                            CardContent {
                                div { class: "flex flex-col gap-4",
                                    AnalogyItem { name: "ironsight-core", role: "Hasta kayıt sistemi", desc: "Tüm bölümler bunu kullanıyor" }
                                    AnalogyItem { name: "ironsight-security", role: "Röntgen / MR", desc: "Core'dan bağımsız analiz yapabilir" }
                                    AnalogyItem { name: "ironsight-memory", role: "Ameliyathane", desc: "Kendi araçlarıyla çalışır" }
                                    AnalogyItem { name: "ironsight-heuristic", role: "Teşhis bölümü", desc: "Raporları birleştirip karar verir" }
                                    AnalogyItem { name: "ironsight-response", role: "Acil müdahale", desc: "Karar gelince aksiyon alır" }
                                }
                            }
                        }
                        div { class: "flex flex-col justify-center gap-6",
                            h3 { class: "text-lg font-bold text-[var(--text-primary)]", "Teknik Faydaları" }
                            Accordion {
                                AccordionItem { index: 0usize,
                                    AccordionTrigger { "Derleme Hızı (Incremental)" }
                                    AccordionContent { "Sadece değişen crate derlenir. 50.000 satır monolit yerine ufak modüller derlenir, 5-10x hızlanma sağlar." }
                                }
                                AccordionItem { index: 1usize,
                                    AccordionTrigger { "Bağımlılık İzolasyonu" }
                                    AccordionContent { "Her crate sadece kendi bağımlılıklarını kullanır (örn. memory: regex, kernel: eBPF), diğerlerini şişirmez." }
                                }
                                AccordionItem { index: 2usize,
                                    AccordionTrigger { "Sorumluluk Ayrımı" }
                                    AccordionContent { "Hash hesaplayan modül kill kararı vermez. Modüllerin sınırları nettir." }
                                }
                            }
                        }
                    }
                }

                Separator {}

                div { class: "space-y-6",
                    h2 { class: "text-2xl font-bold text-[var(--text-primary)]", "2. Neden thiserror? Neden anyhow değil?" }
                    Tabs { default_value: "bad",
                        TabList {
                            TabTrigger { value: "bad", index: 0usize, "❌ Elle Hata Tanımlama" }
                            TabTrigger { value: "good", index: 1usize, "✅ thiserror ile" }
                        }
                        TabContent { value: "bad", index: 0usize,
                            div { class: "p-4 bg-[var(--bg-card)] rounded-xl border border-[var(--border)] font-mono text-xs text-[var(--text-muted)] leading-loose whitespace-pre",
                                "impl std::fmt::Display for SuiteError {{\n"
                                "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{\n"
                                "        match self {{\n"
                                "            SuiteError::Io(e) => write!(f, \"IO error: {{}}\", e),\n"
                                "            // ... 10 tane daha\n"
                                "        }}\n"
                                "    }}\n"
                                "}}"
                            }
                        }
                        TabContent { value: "good", index: 1usize,
                            div { class: "p-4 bg-[var(--bg-card)] rounded-xl border border-[var(--border)] font-mono text-xs text-[var(--accent-green)] leading-loose whitespace-pre",
                                "#[derive(Debug, Error)]\n"
                                "pub enum SuiteError {{\n"
                                "    #[error(\"IO error: {{0}}\")]\n"
                                "    Io(#[from] std::io::Error),\n"
                                "}}"
                            }
                        }
                    }
                    div { class: "grid grid-cols-2 gap-4 mt-4",
                        Card {
                            CardContent { class: "p-4",
                                h4 { class: "font-bold text-[var(--text-primary)] mb-2", "Kütüphane (thiserror)" }
                                p { "Türü belli, pattern matching yapılabilir. Caller ne hatası olduğunu bilir." }
                            }
                        }
                        Card {
                            CardContent { class: "p-4",
                                h4 { class: "font-bold text-[var(--text-primary)] mb-2", "Uygulama (anyhow)" }
                                p { "Sadece mesaj yeterlidir. Hata en üstte yakalanıp loglanır." }
                            }
                        }
                    }
                }

                Separator {}

                div { class: "space-y-6",
                    h2 { class: "text-2xl font-bold text-[var(--text-primary)]", "3. Neden Builder Pattern (ProcessFilter)?" }
                    div { class: "flex gap-6",
                        div { class: "flex-1 flex flex-col gap-2",
                            Badge { variant: BadgeVariant::Destructive, "Kötü Yaklaşım" }
                            div { class: "p-4 bg-[var(--bg-card)] border border-[var(--border)] rounded-lg font-mono text-xs text-[var(--text-muted)]",
                                "filter(&snap, Some(\"chrome\"), None, Some(100.0), None, None);"
                            }
                            p { class: "text-xs italic mt-2", "6 parametreli fonksiyon, 4'ü None. Kimse ne anlama geldiğini anlayamaz." }
                        }
                        div { class: "flex-1 flex flex-col gap-2",
                            Badge { variant: BadgeVariant::Primary, "İyi Yaklaşım" }
                            div { class: "p-4 bg-[var(--bg-card)] border border-[var(--accent-green)] rounded-lg font-mono text-xs text-[var(--text-primary)] leading-loose whitespace-pre",
                                "ProcessFilter::new(&snapshot)\n"
                                "    .name_contains(\"chrome\")\n"
                                "    .memory_above_mib(100.0)\n"
                                "    .collect();"
                            }
                            p { class: "text-xs italic mt-2", "Okunabilir, genişletilebilir ve type-safe." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AnalogyItem(name: String, role: String, desc: String) -> Element {
    rsx! {
        div { class: "flex items-start justify-between p-3 border border-[var(--border)] rounded-md hover:bg-[var(--bg-hover)] transition-colors",
            div { class: "flex flex-col gap-1",
                span { class: "font-bold text-[var(--text-primary)]", "{role}" }
                span { class: "text-xs text-[var(--text-muted)]", "{desc}" }
            }
            Badge { variant: BadgeVariant::Outline, "{name}" }
        }
    }
}

#[component]
pub fn Page00RustWorkspaceOlusturma() -> Element {
    rsx! {
        CourseLayout { title: "00-rust-workspace-olusturma".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/00-rust-workspace-olusturma.md".to_string(),
            div { class: "flex flex-col gap-10 text-sm",

                div { class: "flex items-center gap-4",
                    Badge { variant: BadgeVariant::Outline, "Tarih: 2026-03-24" }
                    Badge { variant: BadgeVariant::Primary, "Durum: Tamamlandı" }
                    Badge { variant: BadgeVariant::Secondary, "Süre: İlk sprint'in başlangıcı" }
                }

                div { class: "border-l-4 border-[var(--accent-green)] pl-6 py-2 bg-[var(--bg-hover)] rounded-r-lg",
                    p { class: "italic text-[var(--text-primary)] font-serif text-lg mb-2", "\"İnşaata başlamadan önce temeli asman lazım. Workspace temeline oturan her crate, kendi sorumluluğunu bilecek.\"" }
                }

                div { class: "space-y-4",
                    h2 { class: "text-2xl font-bold text-[var(--text-primary)]", "Oluşturulan Yapı" }
                    div { class: "p-6 bg-[var(--bg-card)] rounded-xl border border-[var(--border)] font-mono text-xs text-[var(--accent-blue)] leading-relaxed whitespace-pre",
"ironsight/
├── Cargo.toml                     ← Workspace root (9 member)
├── crates/
│   ├── ironsight-core/            ← ✅ TAM IMPLEMENTASYON
│   │   ├── src/
│   │   │   ├── error.rs           ← SuiteError enum
│   │   │   ├── filter.rs          ← ProcessFilter builder
│   │   │   └── spy.rs             ← ProcessSpy facade
│   ├── ironsight-security/        ← 🔲 Stub (Sprint 2)
│   ├── ironsight-memory/          ← 🔲 Stub (Sprint 4)
│   └── ironsight-service/         ← ✅ Binary (demo)"
                    }
                }

                Separator {}

                div { class: "space-y-4",
                    h2 { class: "text-2xl font-bold text-[var(--text-primary)]", "Bağımlılık Haritası" }
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        DepCard { name: "sysinfo", version: "0.34", reason: "Cross-platform process enumeration" }
                        DepCard { name: "crossbeam-channel", version: "0.5", reason: "Lock-free event channel" }
                        DepCard { name: "thiserror", version: "2.0", reason: "Ergonomic error derivation" }
                        DepCard { name: "tracing", version: "0.1", reason: "Structured logging" }
                        DepCard { name: "tokio", version: "1.0", reason: "Async runtime (service)" }
                        DepCard { name: "nix", version: "0.29", reason: "Unix signal/process API" }
                    }
                }

                div { class: "space-y-4 mt-4",
                    h2 { class: "text-xl font-bold text-[var(--text-primary)]", "Doğrulama Sonuçları" }
                    div { class: "flex items-center justify-between p-4 border border-[var(--border)] bg-[var(--bg-card)] rounded-lg",
                        div { class: "flex items-center gap-3",
                            div { class: "text-2xl", "✅" }
                            div {
                                div { class: "font-bold text-[var(--text-primary)]", "cargo check — Geçti" }
                                div { class: "text-xs text-[var(--text-muted)]", "9/9 crate hatasız derlendi." }
                            }
                        }
                        Badge { variant: BadgeVariant::Primary, "0.56s" }
                    }
                }

            }
        }
    }
}

#[component]
fn DepCard(name: String, version: String, reason: String) -> Element {
    rsx! {
        div { class: "p-4 border border-[var(--border)] rounded-lg bg-[var(--bg-card)]",
            div { class: "flex items-center justify-between mb-2",
                span { class: "font-bold text-[var(--accent-cyan)]", "{name}" }
                Badge { variant: BadgeVariant::Outline, "v{version}" }
            }
            p { class: "text-xs text-[var(--text-secondary)]", "{reason}" }
        }
    }
}
#[component]
pub fn Page01CoreUnitTestleri() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Adım 01 — Core Unit Test'leri"
                }
                div { class: "flex items-center gap-4 text-sm flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "2026-03-24" }
                    Badge { variant: BadgeVariant::Success, "Tamamlandı" }
                    Badge { variant: BadgeVariant::Secondary, "34/34 Test Geçti" }
                }
                QuoteBlock {
                    quote: "Kodu yazdın — ama doğru çalıştığını nasıl biliyorsun? Test yazana kadar güvenmiyoruz."
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                // Left Column: Test Kapsam Dağılımı
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-xl font-semibold text-[var(--text-primary)]", "Test Kapsam Dağılımı" }
                        Table {
                            headers: vec!["Modül".to_string(), "Adet".to_string(), "Kapsam".to_string()],
                            rows: vec![
                                vec![rsx!{span { class:"font-mono text-xs", "snapshot_tests"}}, rsx!{"10"}, rsx!{"Snapshot alma, sıralama, algoritmalar"}],
                                vec![rsx!{span { class:"font-mono text-xs", "diff_tests"}}, rsx!{"7"}, rsx!{"Yeniler, çıkanlar, threshold değişimi"}],
                                vec![rsx!{span { class:"font-mono text-xs", "filter_tests"}}, rsx!{"8"}, rsx!{"İsim, CPU, RAM, zincirleme"}],
                                vec![rsx!{span { class:"font-mono text-xs", "process_info_tests"}}, rsx!{"3"}, rsx!{"Dönüşümler, hesaplamalar"}],
                                vec![rsx!{span { class:"font-mono text-xs", "spy_tests"}}, rsx!{"6"}, rsx!{"Sistem bilgisi, yetkiler, hata akışları"}],
                            ]
                        }
                    }
                }

                // Right Column: Output Log & Next Step
                div { class: "flex flex-col gap-6",
                    Card {
                        div { class: "flex flex-col gap-4 p-6",
                            h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                                i { class: "fa-solid fa-terminal text-[var(--text-secondary)]" }
                                "Test Runner Yürütmesi"
                            }
                            CodeBlock {
                                code: "running 34 tests\ntest tests::diff_tests::detects_cpu_change_above_threshold ... ok\ntest tests::diff_tests::detects_exited_processes ... ok\ntest tests::diff_tests::detects_memory_change_above_threshold ... ok\n...\ntest result: ok. 34 passed; 0 failed; 0 ignored",
                                language: "bash"
                            }
                        }
                    }

                    Card {
                        div { class: "flex flex-col gap-3 p-6 bg-[var(--bg-secondary)] border-l-4 border-l-[var(--accent-primary)] rounded-r-lg",
                            h3 { class: "text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Bir Sonraki Adım" }
                            h2 { class: "text-lg font-bold text-[var(--text-primary)]", "02: Security Layer" }
                            p { class: "text-sm text-[var(--text-secondary)]", "SHA-256 hash, Shannon entropy, suspicious path detection, SecurityAudit orchestrator." }
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page01Educational() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Eğitim Notu 01 — Test Mühendisliği"
                }
                div { class: "flex items-center gap-4 text-sm text-[var(--text-secondary)] flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "Keyvan Arasteh" }
                    Badge { variant: BadgeVariant::Secondary, "IronSight" }
                    Badge { variant: BadgeVariant::Secondary, "2026" }
                }
                QuoteBlock {
                    quote: "Test yazmak zaman kaybı mı? Hayır — test yazmamak zaman kaybı. Çünkü hata production'da çıkarsa, düzeltme maliyeti 10x artar."
                }
            }

            // 1. Neden Test Yazıyoruz?
            Card {
                div { class: "flex flex-col gap-4 p-6",
                    h2 { class: "text-xl font-semibold text-[var(--text-primary)]", "1. Neden Test Yazıyoruz?" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div { class: "flex flex-col gap-2",
                            p { class: "text-[var(--text-secondary)] leading-relaxed",
                                "Ameliyattan önce tüm aletler sterilize edildi mi, doktor kontrol eder. Operasyon ortasında 'sterilize miydik ya?' diye sormak istemezsin."
                                br {}
                                br {}
                                "Test'ler aynı şeyi yapıyor — kodu "
                                span { class: "font-semibold text-[var(--text-primary)]", "deploy etmeden önce" }
                                " doğru çalıştığını garanti ediyor."
                            }
                        }
                        div { class: "flex flex-col gap-2 p-4 bg-[var(--bg-secondary)] rounded-lg border border-[var(--border-primary)]",
                            h3 { class: "text-sm font-semibold text-[var(--text-primary)] mb-2", "Hata Maliyeti Dağılımı" }
                            Table {
                                headers: vec!["Nerede Yakalandı?".to_string(), "Maliyet".to_string()],
                                rows: vec![
                                    vec![rsx!{"Geliştirme (test)"}, rsx!{Badge { variant: BadgeVariant::Success, "1x" }}],
                                    vec![rsx!{"Code Review"}, rsx!{Badge { variant: BadgeVariant::Warning, "3x" }}],
                                    vec![rsx!{"Staging"}, rsx!{Badge { variant: BadgeVariant::Warning, "5x" }}],
                                    vec![rsx!{"Production"}, rsx!{Badge { variant: BadgeVariant::Destructive, "10-100x" }}]
                                ]
                            }
                        }
                    }
                }
            }

            // 2. Live Test vs Mock Test
            Tabs {
                default_value: "live",
                TabList {
                    TabTrigger { value: "live", index: 0usize, "Live Test" }
                    TabTrigger { value: "mock", index: 1usize, "Mock Test" }
                }
                TabContent { value: "live", index: 0usize,
                    div { class: "flex flex-col gap-4 p-4",
                        p { class: "text-[var(--text-secondary)]",
                            "Gerçek dünya sistemlerini okur. Örneğin:"
                        }
                        CodeBlock {
                            code: "#[test]\nfn snapshot_has_processes() {{\n    let snap = take_snapshot();\n    assert!(snap.processes.len() > 1);\n}}",
                            language: "rust"
                        }
                        div { class: "flex gap-4 mt-2",
                            Badge { variant: BadgeVariant::Success, "Avantaj: Gerçekliği kanıtlar" }
                            Badge { variant: BadgeVariant::Destructive, "Dezavantaj: Ortam bağımlı" }
                        }
                    }
                }
                TabContent { value: "mock", index: 1usize,
                    div { class: "flex flex-col gap-4 p-4",
                        p { class: "text-[var(--text-secondary)]",
                            "Sentetik veya simüle edilmiş verileri test eder. Örneğin:"
                        }
                        CodeBlock {
                            code: "let old = make_snapshot(vec![make_process(1, \"init\", 0.0, 1000)]);\nlet new = make_snapshot(vec![\n    make_process(1, \"init\", 0.0, 1000),\n    make_process(42, \"malware\", 50.0, 99999),\n]);\nlet diff = ProcessDiff::compute(&old, &new);\nassert_eq!(diff.spawned.len(), 1);",
                            language: "rust"
                        }
                        div { class: "flex gap-4 mt-2",
                            Badge { variant: BadgeVariant::Success, "Avantaj: Deterministik" }
                            Badge { variant: BadgeVariant::Destructive, "Dezavantaj: Gerçeği %100 yansıtmayabilir" }
                        }
                    }
                }
            }

            // 3 & 4: Helpers & Thresholds
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-wand-magic-sparkles text-[var(--accent-primary)]" }
                            "Helper Fonksiyonlar"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "15 satırlık Object Boilerplate'ini 1 satıra indirger. Testin hikayesini okumak daha kolay hale gelir."
                        }
                        CodeBlock {
                            code: "let p = make_process(1, \"test\", 25.0, 1024);",
                            language: "rust"
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-ruler text-[var(--accent-primary)]" }
                            "Sınır Değer (Threshold) Analizi"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "Sınırların doğruluğunu garanti etmek için eşik değerinin hem altını hem de üstünü test edin."
                        }
                        CodeBlock {
                            code: "assert_eq!(diff.changed.len(), 1); // 45.0 > 1.0 ✅\nassert!(diff.changed.is_empty());  // 0.5 < 1.0 ❌",
                            language: "rust"
                        }
                    }
                }
            }

            // 5. Error Path Testing
            Card {
                div { class: "flex flex-col gap-4 p-6",
                    h2 { class: "text-xl font-semibold text-[var(--text-primary)]", "Error Path Testing — 'Olmayan Şeyleri Sor'" }
                    p { class: "text-[var(--text-secondary)] mb-2",
                        "Happy path testleri kolaydır. Gerçek kaliteyi unhappy path testleri belirler."
                    }
                    Table {
                        headers: vec!["Happy Path".to_string(), "Unhappy Path".to_string()],
                        rows: vec![
                            vec![rsx!{"PID 1234'ü kill et → OK"}, rsx!{"PID MAX'ı kill et → Err"}],
                            vec![rsx!{"'chrome' ara → bulundu"}, rsx!{"'nonexistent' ara → boş liste"}],
                            vec![rsx!{"Env vars al → liste"}, rsx!{"Geçersiz PID env vars → Err"}],
                        ]
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page02Educational() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Eğitim Notu 02 — Statik Analiz Teknikleri"
                }
                div { class: "flex items-center gap-4 text-sm text-[var(--text-secondary)] flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "Keyvan Arasteh" }
                    Badge { variant: BadgeVariant::Secondary, "IronSight" }
                    Badge { variant: BadgeVariant::Secondary, "2026" }
                }
                QuoteBlock {
                    quote: "Programı çalıştırmadan, sadece dosyaya bakarak ne kadar bilgi edinebiliriz? Cevap: Şaşırtıcı derecede çok."
                }
            }

            // 1. Hash & 2. Entropy
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-fingerprint text-[var(--accent-primary)]" }
                            "Neden SHA-256?"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "Parmak izi benzersizdir. SHA-256 de böyledir, dosyanın dijital parmak izidir."
                        }
                        Table {
                            headers: vec!["Algoritma".to_string(), "Durum".to_string()],
                            rows: vec![
                                vec![rsx!{"MD5"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Kırıldı (2004)" }}],
                                vec![rsx!{"SHA-1"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Kırıldı (2017)" }}],
                                vec![rsx!{"SHA-256"}, rsx!{Badge { variant: BadgeVariant::Success, "Güvenli (Bizim seçimimiz)" }}],
                            ]
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-wave-square text-[var(--accent-primary)]" }
                            "Shannon Entropi"
                        }
                        p { class: "text-sm text-[var(--text-secondary)] text-justify",
                            "Her byte değerinin (0-255) dosyada ne kadar sıklıkla göründüğünü hesaplıyoruz. Normal bir programın entropisi 5-6 civarında olur. 7.5'ten yüksekse packed veya encrypted olma ihtimali yüksektir."
                        }
                        CodeBlock {
                            code: "pub fn shannon_entropy(data: &[u8]) -> f64 {{\n    let mut freq = [0u64; 256];\n    for &byte in data {{ freq[byte as usize] += 1; }}\n    // ... calculation\n}}",
                            language: "rust"
                        }
                    }
                }
            }

            // 3. Path Analizi
            Card {
                div { class: "flex flex-col gap-4 p-6",
                    h2 { class: "text-xl font-semibold text-[var(--text-primary)]", "Neden Path Analizi?" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div { class: "flex flex-col gap-4",
                            p { class: "text-[var(--text-secondary)] leading-relaxed",
                                "Saldırgan neden /tmp/ kullanıyor? Çünkü herkes yazabilir, genellikle mount noexec değil, ve çoğu güvenlik aracı taramaz."
                            }
                            CodeBlock {
                                code: "let path_analysis = analyze_path(None);\n// is_suspicious = true\n// reason = \"No executable path — potential fileless process\"",
                                language: "rust"
                            }
                        }
                        div { class: "flex flex-col gap-2 p-4 bg-[var(--bg-secondary)] rounded-lg border border-[var(--border-primary)]",
                            h3 { class: "text-sm font-semibold text-[var(--text-primary)] mb-2", "Risk Haritası" }
                            Table {
                                headers: vec!["Dizin".to_string(), "Risk".to_string()],
                                rows: vec![
                                    vec![rsx!{span { class:"font-mono text-xs", "/tmp/"}}, rsx!{Badge { variant: BadgeVariant::Destructive, "Kritik" }}],
                                    vec![rsx!{span { class:"font-mono text-xs", "/dev/shm/"}}, rsx!{Badge { variant: BadgeVariant::Destructive, "Kritik" }}],
                                    vec![rsx!{span { class:"font-mono text-xs", "/var/tmp/"}}, rsx!{Badge { variant: BadgeVariant::Warning, "Yüksek" }}],
                                    vec![rsx!{span { class:"font-mono text-xs", "None (Fileless)"}}, rsx!{Badge { variant: BadgeVariant::Destructive, "Kritik" }}],
                                ]
                            }
                        }
                    }
                }
            }

            // 4. Orchestrator & Option
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-sitemap text-[var(--accent-primary)]" }
                            "Orchestrator Pattern"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "SecurityAudit tüm bağımsız kontrolleri tek bir API üzerinden orkestra eder. Yeni bir kontrol eklemek sadece 5 satır sürer."
                        }
                        CodeBlock {
                            code: "let result = SecurityAudit::audit(pid, name, exe_path);\n// result.flag_count → kaç tane sorun var?\n// result.flags → neler var?",
                            language: "rust"
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-cube text-[var(--accent-primary)]" }
                            "Neden Option<> Kullanıyoruz?"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "Rust'ın null güvenliğidir. Erişim hatası olduğunda uygulamanın crash olup olmamasını kontrol ederiz. Ne kadar veri varsa o dönülür."
                        }
                        CodeBlock {
                            code: "pub struct AuditResult {{\n    pub hash: Option<HashResult>,\n    pub entropy: Option<EntropyResult>,\n}}",
                            language: "rust"
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page02SecurityLayer() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Adım 02 — Security Layer"
                }
                div { class: "flex items-center gap-4 text-sm flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "2026-03-24" }
                    Badge { variant: BadgeVariant::Success, "Tamamlandı" }
                    Badge { variant: BadgeVariant::Secondary, "16/16 Test Geçti" }
                }
                QuoteBlock {
                    quote: "Röntgen çekiyoruz — programı çalıştırmadan içerisine bakıyoruz. ironsight-security crate'ini 5 modülle implement ettik."
                }
            }

            // Tabs for Modules
            Tabs {
                default_value: "hash",
                TabList {
                    TabTrigger { value: "hash", index: 0usize, "hash.rs" }
                    TabTrigger { value: "entropy", index: 1usize, "entropy.rs" }
                    TabTrigger { value: "path", index: 2usize, "path_analysis.rs" }
                    TabTrigger { value: "audit", index: 3usize, "audit.rs" }
                }
                TabContent { value: "hash", index: 0usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "SHA-256 Parmak İzi" }
                        p { class: "text-[var(--text-secondary)]", "Binary'nin disk üzerindeki SHA-256 hash'ini hesaplıyor." }
                        CodeBlock {
                            code: "let result = compute_sha256(Path::new(\"/usr/bin/ls\"))?;\n// result.sha256 = \"a1b2c3d4...\" (VirusTotal'da aranabilir)\n// result.file_size = 142144",
                            language: "rust"
                        }
                    }
                }
                TabContent { value: "entropy", index: 1usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "Shannon Entropi (Yoğunluk Ölçümü)" }
                        div { class: "overflow-x-auto",
                            Table {
                                headers: vec!["Entropi".to_string(), "Risk".to_string(), "Anlam".to_string()],
                                rows: vec![
                                    vec![rsx!{"0-5.0"}, rsx!{Badge { variant: BadgeVariant::Success, "Low" }}, rsx!{"Normal uygulama"}],
                                    vec![rsx!{"5.0-7.0"}, rsx!{Badge { variant: BadgeVariant::Warning, "Medium" }}, rsx!{"Sıkıştırılmış/obfuscated"}],
                                    vec![rsx!{"7.0-7.5"}, rsx!{Badge { variant: BadgeVariant::Destructive, "High" }}, rsx!{"Muhtemelen packed"}],
                                    vec![rsx!{"7.5+"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Critical" }}, rsx!{"Muhtemelen encrypted malware"}],
                                ]
                            }
                        }
                    }
                }
                TabContent { value: "path", index: 2usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "Suspicious Directory Detection" }
                        p { class: "text-[var(--text-secondary)]", "Bilinen staging alanlarını kontrol ediyor." }
                        div { class: "overflow-x-auto",
                            Table {
                                headers: vec!["Platform".to_string(), "Dizinler".to_string()],
                                rows: vec![
                                    vec![rsx!{"Linux"}, rsx!{span { class: "font-mono text-xs", "/tmp/, /var/tmp/, /dev/shm/" }}],
                                    vec![rsx!{"Windows"}, rsx!{span { class: "font-mono text-xs", "AppData\\Local\\Temp, Windows\\Temp" }}],
                                    vec![rsx!{"Genel"}, rsx!{span { class: "font-mono text-xs", "Downloads/, Desktop/" }}],
                                    vec![rsx!{"Fileless"}, rsx!{span { class: "font-mono text-xs", "Exe path = None" }}],
                                ]
                            }
                        }
                    }
                }
                TabContent { value: "audit", index: 3usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "SecurityAudit Orchestrator" }
                        p { class: "text-[var(--text-secondary)]", "Tüm kontrolleri birleştirip tek AuditResult döndürüyor." }
                        CodeBlock {
                            code: "let result = SecurityAudit::audit(1234, \"suspicious_proc\", Some(&path));\n// result.flag_count = 3\n// result.flags = [\n//   \"Binary running from staging area: /tmp/\",\n//   \"Critical entropy (7.82) — likely encrypted/packed\",\n//   \"Unsigned binary\"\n// ]",
                            language: "rust"
                        }
                    }
                }
            }

            // Output Log & Next Step
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-terminal text-[var(--text-secondary)]" }
                            "Test Runner Yürütmesi"
                        }
                        CodeBlock {
                            code: "running 16 tests\ntest entropy::tests::entropy_of_empty ... ok\ntest hash::tests::hash_known_content ... ok\ntest path_analysis::tests::detects_dev_shm ... ok\n...\ntest result: ok. 16 passed; 0 failed; 0 ignored",
                            language: "bash"
                        }
                    }
                }

                Card {
                    div { class: "flex flex-col gap-3 p-6 h-full bg-[var(--bg-secondary)] border-l-4 border-l-[var(--accent-primary)] rounded-r-lg",
                        h3 { class: "text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Bir Sonraki Adım" }
                        h2 { class: "text-lg font-bold text-[var(--text-primary)]", "03: Network Layer" }
                        p { class: "text-sm text-[var(--text-secondary)]", "Socket→PID mapping, listener detection, DNS enrichment." }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page03Educational() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Eğitim Notu 03 — Network Forensics Temelleri"
                }
                div { class: "flex items-center gap-4 text-sm text-[var(--text-secondary)] flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "Keyvan Arasteh" }
                    Badge { variant: BadgeVariant::Secondary, "IronSight" }
                    Badge { variant: BadgeVariant::Secondary, "2026" }
                }
                QuoteBlock {
                    quote: "Bir bilgisayarın ne yaptığını anlamak için iki şeye bakarsın: çalıştırdığı programlara ve konuştuğu kişilere."
                }
            }

            // 1. Socket -> PID & 2. TCP State Machine
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-xl font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-plug text-[var(--accent-primary)]" }
                            "Socket → PID Eşleştirmesi"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "Telefon numarası (IP+Port) ve telefon sahibi (PID). Her socket bir PID'ye aittir."
                        }
                        CodeBlock {
                            code: "# /proc/net/tcp format:\nsl  local_address rem_address   st tx_queue rx_queue uid inode\n 0: 0100007F:0050 00000000:0000 0A 00000000:00000000 1000 12345",
                            language: "bash"
                        }
                        p { class: "text-sm text-[var(--text-secondary)] mt-2",
                            "Her process'in açık dosya tanımlayıcıları /proc/<pid>/fd/ altında bulunur. Inode numaralarını eşleştirerek socket sahibi PID bulunur."
                        }
                    }
                }

                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-xl font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-arrows-spin text-[var(--accent-primary)]" }
                            "TCP State Machine"
                        }
                        div { class: "overflow-x-auto",
                            Table {
                                headers: vec!["State".to_string(), "Normal mi?".to_string(), "Açıklama".to_string()],
                                rows: vec![
                                    vec![rsx!{"ESTABLISHED"}, rsx!{Badge { variant: BadgeVariant::Success, "Evet" }}, rsx!{"İnternete bağlanmak normal"}],
                                    vec![rsx!{"LISTEN"}, rsx!{Badge { variant: BadgeVariant::Success, "Server" }}, rsx!{"Beklenmedik listener ise şüpheli"}],
                                    vec![rsx!{"SYN_SENT"}, rsx!{Badge { variant: BadgeVariant::Warning, "Dikkat" }}, rsx!{"Çok fazlaysa port scanning olabilir"}],
                                    vec![rsx!{"CLOSE_WAIT"}, rsx!{Badge { variant: BadgeVariant::Warning, "Dikkat" }}, rsx!{"Birikiyorsa resource leak"}],
                                    vec![rsx!{"TIME_WAIT"}, rsx!{Badge { variant: BadgeVariant::Success, "Evet" }}, rsx!{"Kapatma sonrası bekleme"}],
                                ]
                            }
                        }
                    }
                }
            }

            // 3. Port İstihbaratı
            Card {
                div { class: "flex flex-col gap-4 p-6",
                    h2 { class: "text-xl font-semibold text-[var(--text-primary)] flex items-center gap-2",
                        i { class: "fa-solid fa-radar text-[var(--accent-primary)]" }
                        "Port İstihbaratı — Hangi Portlar Şüphelidir?"
                    }
                    p { class: "text-[var(--text-secondary)] mb-2",
                        "Saldırganlar tembeldir, varsayılan portları nadiren değiştirirler. 31337 gibi portların bir geçmişi vardır (ELEET)."
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                        div { class: "p-4 border border-[var(--border-primary)] rounded bg-[var(--bg-secondary)]",
                            h3 { class: "font-semibold text-[var(--text-primary)] mb-1", "Reverse Shell" }
                            p { class: "text-xs text-[var(--text-secondary)]", "4444, 1337" }
                        }
                        div { class: "p-4 border border-[var(--border-primary)] rounded bg-[var(--bg-secondary)]",
                            h3 { class: "font-semibold text-[var(--text-primary)] mb-1", "C2 (Command & Control)" }
                            p { class: "text-xs text-[var(--text-secondary)]", "6667, 8443" }
                        }
                        div { class: "p-4 border border-[var(--border-primary)] rounded bg-[var(--bg-secondary)]",
                            h3 { class: "font-semibold text-[var(--text-primary)] mb-1", "Anonimleştirme" }
                            p { class: "text-xs text-[var(--text-secondary)]", "9001 (Tor), 9050" }
                        }
                    }
                }
            }

            // 4. IP Türleri & 5. Orchestrator
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-network-wired text-[var(--accent-primary)]" }
                            "Private vs Public IP"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "İç ağ trafiği düşük risk taşır, dış ağ trafiği detaylı incelenmelidir. Özellikle şüpheli port + dış IP kombinasyonları tehlikelidir."
                        }
                        CodeBlock {
                            code: "192.168.1.5:3306 → ✅ Private\n8.8.8.8:53 → ⚠️ Public (DNS)\n185.234.218.x:4444 → 🔴 Public + Suspicious",
                            language: "bash"
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-sitemap text-[var(--accent-primary)]" }
                            "NetworkAudit Orchestrator & Multi-Platform"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "NetworkAudit, tüm PID ağ eşleştirmelerini koordine eder. Conditional compilation ile macOS, Windows ve Linux desteklenir."
                        }
                        CodeBlock {
                            code: "#[cfg(target_os = \"linux\")]\npub fn scan() -> Vec<SocketInfo> {{\n    // /proc/net/tcp parse\n}}",
                            language: "rust"
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page03NetworkLayer() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Adım 03 — Network Layer"
                }
                div { class: "flex items-center gap-4 text-sm flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "2026-03-24" }
                    Badge { variant: BadgeVariant::Success, "Tamamlandı" }
                    Badge { variant: BadgeVariant::Secondary, "10/10 Test Geçti" }
                }
                QuoteBlock {
                    quote: "Bir process'in network bağlantılarını görmek, onun kiminle konuştuğunu öğrenmektir — tıpkı telefon kayıtları gibi."
                }
            }

            // Tabs for Modules
            Tabs {
                default_value: "mapper",
                TabList {
                    TabTrigger { value: "mapper", index: 0usize, "socket_mapper.rs" }
                    TabTrigger { value: "dns", index: 1usize, "dns.rs" }
                    TabTrigger { value: "audit", index: 2usize, "audit.rs" }
                }
                TabContent { value: "mapper", index: 0usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "Socket → PID Eşleştirme" }
                        p { class: "text-[var(--text-secondary)]", "Linux'ta /proc/net/tcp dosyalarını parse edip inode eşleşmesini yapıyor. 11 TCP state'ini tanıyor." }
                        CodeBlock {
                            code: "let sockets = SocketMapper::scan();\n// 127.0.0.1:8080 [LISTEN] → PID 1234 (nginx)\n// 10.0.0.5:54321 → 93.184.216.34:443 [ESTABLISHED] → PID 5678",
                            language: "rust"
                        }
                    }
                }
                TabContent { value: "dns", index: 1usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "DNS Zenginleştirme + Port İstihbaratı" }
                        div { class: "overflow-x-auto",
                            Table {
                                headers: vec!["Port".to_string(), "Servis".to_string(), "Risk".to_string()],
                                rows: vec![
                                    vec![rsx!{"4444"}, rsx!{"Metasploit"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Reverse shell" }}],
                                    vec![rsx!{"5555"}, rsx!{"Android ADB"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Remote debug" }}],
                                    vec![rsx!{"31337"}, rsx!{"Back Orifice"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Klasik backdoor" }}],
                                    vec![rsx!{"9001"}, rsx!{"Tor"}, rsx!{Badge { variant: BadgeVariant::Warning, "Anonimleştirme" }}],
                                ]
                            }
                        }
                    }
                }
                TabContent { value: "audit", index: 2usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "NetworkAudit Orchestrator" }
                        CodeBlock {
                            code: "let audit = NetworkAudit::scan();\n// audit.total_sockets = 47\n// audit.listeners = [{{port:80, pid:1234, name:\"nginx\"}}, ...]\n// audit.suspicious_connections = [{{port:4444, \"Metasploit default\"}}]\n// audit.external_connections = [{{remote:93.184.216.34, dns:\"example.com\"}}]",
                            language: "rust"
                        }
                    }
                }
            }

            // Output Log & Next Step
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-terminal text-[var(--text-secondary)]" }
                            "Test Runner Yürütmesi"
                        }
                        CodeBlock {
                            code: "running 10 tests\ntest dns::tests::known_suspicious_ports ... ok\ntest socket_mapper::tests::filter_listeners ... ok\ntest socket_mapper::tests::tcp_state_parsing ... ok\n...\ntest result: ok. 10 passed; 0 failed",
                            language: "bash"
                        }
                    }
                }

                Card {
                    div { class: "flex flex-col gap-3 p-6 h-full bg-[var(--bg-secondary)] border-l-4 border-l-[var(--accent-primary)] rounded-r-lg",
                        h3 { class: "text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Bir Sonraki Adım" }
                        h2 { class: "text-lg font-bold text-[var(--text-primary)]", "04: Memory Scanner" }
                        p { class: "text-sm text-[var(--text-secondary)]", "Pattern matcher ve memory watcher ile RAM üzerinden tehdit tespiti." }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page04Educational() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Eğitim Notu 04 — Memory Forensics & Shellcode Tespiti"
                }
                div { class: "flex items-center gap-4 text-sm text-[var(--text-secondary)] flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "Keyvan Arasteh" }
                    Badge { variant: BadgeVariant::Secondary, "IronSight" }
                    Badge { variant: BadgeVariant::Secondary, "2026" }
                }
                QuoteBlock {
                    quote: "Disk'te silinen delil gider. Ama RAM'de hala duruyor — program kapatılana kadar."
                }
            }

            // 1. Virtual Memory & 2. W^X
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-map text-[var(--accent-primary)]" }
                            "Virtual Memory (/proc/maps)"
                        }
                        p { class: "text-sm text-[var(--text-secondary)] text-justify",
                            "Bir binanın kat planı gibidir — her kata (region) farklı izinler verilmiş. Her bölgenin 4 izin biti vardır: r, w, x, p."
                        }
                        CodeBlock {
                            code: "0x01000000 ─── Veri bölümü (rw-) ── okunup yazılabilir\n    ...\n0x7FFE0000 ─── Stack (rw-) ── yerel değişkenler",
                            language: "bash".to_string()
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-shield-halved text-[var(--accent-primary)]" }
                            "W^X (Write XOR Execute) İlkesi"
                        }
                        p { class: "text-sm text-[var(--text-secondary)] text-justify",
                            "Bir sayfa ya yazılabilir (rw) ya da çalıştırılabilir (rx) olmalı. İkisi aynı anda (rwx) savunmasızdır."
                        }
                        Table {
                            headers: vec!["Durum".to_string(), "Normal mi?".to_string()],
                            rows: vec![
                                vec![rsx!{"JIT derleyici"}, rsx!{Badge { variant: BadgeVariant::Outline, "Geçici olarak Evet" }}],
                                vec![rsx!{"Bilinmeyen process"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Muhtemelen exploit" }}],
                            ]
                        }
                    }
                }
            }

            // 3. Anonymous & 4. Pattern Scanning
            Card {
                div { class: "flex flex-col gap-4 p-6",
                    h2 { class: "text-xl font-semibold text-[var(--text-primary)]", "Anonymous Executable & Pattern Scanning" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div { class: "flex flex-col gap-4",
                            p { class: "text-[var(--text-secondary)] leading-relaxed",
                                "Dosyası olmayan çalıştırılabilir bellek, hayalet gibidir — anti-virüs diski tarar, ama ramdeki bu içerik diske dokunmaz bile."
                            }
                            CodeBlock {
                                code: "7f1234000000-7f1234002000 rwxp 00000000 00:00 0\n                                              ↑ inode=0\nuse regex::bytes::Regex; // ← String Regex DEĞİL!",
                                language: "rust".to_string()
                            }
                        }
                        div { class: "flex flex-col gap-2 p-4 bg-[var(--bg-secondary)] rounded-lg border border-[var(--border-primary)]",
                            h3 { class: "text-sm font-semibold text-[var(--text-primary)] mb-2", "Tarama Kategorileri" }
                            div { class: "flex flex-col gap-2 overflow-x-auto",
                                Table {
                                    headers: vec!["Desen".to_string(), "Kategori".to_string()],
                                    rows: vec![
                                        vec![rsx!{span { class:"font-mono text-xs", "/bin/sh"}}, rsx!{Badge { variant: BadgeVariant::Secondary, "Shell" }}],
                                        vec![rsx!{span { class:"font-mono text-xs", "wget http"}}, rsx!{Badge { variant: BadgeVariant::Warning, "Download" }}],
                                        vec![rsx!{span { class:"font-mono text-xs", "nc -e"}}, rsx!{Badge { variant: BadgeVariant::Destructive, "Reverse Shell" }}],
                                        vec![rsx!{span { class:"font-mono text-xs", "password="}}, rsx!{Badge { variant: BadgeVariant::Destructive, "Credential" }}],
                                    ]
                                }
                            }
                        }
                    }
                }
            }

            // 5. Context, 6. Watcher, 7. maps/mem
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-eye text-[var(--accent-primary)]" }
                            "Context Display"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "Olay yeri inceleme. 16 byte context penceresi pattern'in bağlamını ve doğruluğunu analiz eder."
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-clock-rotate-left text-[var(--accent-primary)]" }
                            "Memory Watcher"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "Snapshot karşılaştırması; Zamanla rwx alanlarına nelerin eklendiğini tesptir eder."
                        }
                    }
                }
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-layer-group text-[var(--accent-primary)]" }
                            "maps vs mem"
                        }
                        p { class: "text-sm text-[var(--text-secondary)]",
                            "maps haritadır nerede ne var. mem ise gerçek arazidir oraya gidip okuma işlemi yapar."
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Page04MemoryScanner() -> Element {
    rsx! {
        div { class: "w-full max-w-5xl mx-auto flex flex-col gap-6 pb-20 animate-fade-in",
            // Header
            div { class: "flex flex-col gap-4 border-b border-[var(--border-primary)] pb-6 mb-4",
                h1 { class: "text-3xl font-bold text-[var(--text-primary)]",
                    "Adım 04 — Memory Scanner"
                }
                div { class: "flex items-center gap-4 text-sm flex-wrap",
                    Badge { variant: BadgeVariant::Outline, "2026-03-24" }
                    Badge { variant: BadgeVariant::Success, "Tamamlandı" }
                    Badge { variant: BadgeVariant::Secondary, "19/19 Test Geçti" }
                }
                QuoteBlock {
                    quote: "Process'in RAM'ine bakıyoruz — orada ne saklıyor? Şifre, shellcode, C2 adresi? ironsight-memory crate'ini 3 modülle implement ettik."
                }
            }

            // Tabs for Modules
            Tabs {
                default_value: "maps",
                TabList {
                    TabTrigger { value: "maps", index: 0usize, "maps.rs" }
                    TabTrigger { value: "scanner", index: 1usize, "scanner.rs" }
                    TabTrigger { value: "watcher", index: 2usize, "watcher.rs" }
                }
                TabContent { value: "maps", index: 0usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "Memory Region Haritası" }
                        p { class: "text-[var(--text-secondary)]", "/proc/<pid>/maps parse ederek virtual memory layout'u çıkarıyor." }
                        CodeBlock {
                            code: "55a2e3400000-55a2e3402000 r--p /usr/bin/ls\n55a2e4c00000-55a2e4c21000 rw-p [heap]\n7f1234000000-7f1234002000 rwxp (⚠️ W^X violation!)",
                            language: "text"
                        }
                        ul { class: "list-disc pl-5 text-[var(--text-secondary)] space-y-1",
                            li { "W^X violation detection" }
                            li { "Anonymous executable region detection" }
                            li { "Heap/stack boyut analizi" }
                        }
                    }
                }
                TabContent { value: "scanner", index: 1usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "Pattern Scanner" }
                        p { class: "text-[var(--text-secondary)]", "14 adet suspicious pattern ile memory tarama." }
                        div { class: "overflow-x-auto",
                            Table {
                                headers: vec!["Pattern".to_string(), "Tehdit".to_string()],
                                rows: vec![
                                    vec![rsx!{"/bin/sh, /bin/bash"}, rsx!{"Komut çalıştırma"}],
                                    vec![rsx!{"nc -e, nc -l"}, rsx!{"Reverse shell"}],
                                    vec![rsx!{"PRIVMSG #"}, rsx!{"IRC C2 iletişimi"}],
                                    vec![rsx!{"BEGIN RSA PRIVATE KEY"}, rsx!{"Bellek içi key sızıntısı"}],
                                ]
                            }
                        }
                    }
                }
                TabContent { value: "watcher", index: 2usize,
                    div { class: "flex flex-col gap-4 p-4",
                        h3 { class: "text-lg font-semibold text-[var(--text-primary)]", "Memory Region Diff" }
                        p { class: "text-[var(--text-secondary)]", "İki snapshot arasındaki değişimleri tespit ediyor." }
                        div { class: "overflow-x-auto",
                            Table {
                                headers: vec!["Değişim Tipi".to_string(), "Risk".to_string()],
                                rows: vec![
                                    vec![rsx!{"RegionAdded (rwx, anonymous)"}, rsx!{Badge { variant: BadgeVariant::Destructive, "Shellcode injection" }}],
                                    vec![rsx!{"PermissionChanged (rw→rwx)"}, rsx!{Badge { variant: BadgeVariant::Destructive, "JIT-spray / ROP" }}],
                                    vec![rsx!{"RegionRemoved"}, rsx!{Badge { variant: BadgeVariant::Warning, "Anti-forensics?" }}],
                                    vec![rsx!{"SizeChanged (heap büyüme)"}, rsx!{Badge { variant: BadgeVariant::Warning, "Memory leak" }}],
                                ]
                            }
                        }
                    }
                }
            }

            // Output Log & Next Step
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    div { class: "flex flex-col gap-4 p-6 h-full",
                        h2 { class: "text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2",
                            i { class: "fa-solid fa-terminal text-[var(--text-secondary)]" }
                            "Test Runner Yürütmesi"
                        }
                        CodeBlock {
                            code: "running 19 tests — all passed\nmaps: 10 tests\nscanner: 5 tests\nwatcher: 5 tests",
                            language: "bash".to_string()
                        }
                    }
                }

                Card {
                    div { class: "flex flex-col gap-3 p-6 h-full bg-[var(--bg-secondary)] border-l-4 border-l-[var(--accent-primary)] rounded-r-lg",
                        h3 { class: "text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider", "Bir Sonraki Adım" }
                        h2 { class: "text-lg font-bold text-[var(--text-primary)]", "05: Heuristic Engine" }
                        p { class: "text-sm text-[var(--text-secondary)]", "Threat scoring + response handler ve detection birleştirmeleri." }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Page05Educational() -> Element {
    rsx! { CourseLayout { title: "05-educational".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/05-educational.md".to_string(), div {} } }
}
#[component]
pub fn Page05HeuristicResponse() -> Element {
    rsx! { CourseLayout { title: "05-heuristic-response".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/05-heuristic-response.md".to_string(), div {} } }
}
#[component]
pub fn Page06Educational() -> Element {
    rsx! { CourseLayout { title: "06-educational".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/06-educational.md".to_string(), div {} } }
}
#[component]
pub fn Page06ReportService() -> Element {
    rsx! { CourseLayout { title: "06-report-service".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/06-report-service.md".to_string(), div {} } }
}
#[component]
pub fn Page07Educational() -> Element {
    rsx! { CourseLayout { title: "07-educational".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/07-educational.md".to_string(), div {} } }
}
#[component]
pub fn Page07ServiceHardening() -> Element {
    rsx! { CourseLayout { title: "07-service-hardening".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/07-service-hardening.md".to_string(), div {} } }
}
#[component]
pub fn Page08Educational() -> Element {
    rsx! { CourseLayout { title: "08-educational".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/08-educational.md".to_string(), div {} } }
}
#[component]
pub fn Page08TimeDecayScoring() -> Element {
    rsx! { CourseLayout { title: "08-time-decay-scoring".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/08-time-decay-scoring.md".to_string(), div {} } }
}
#[component]
pub fn Page09DioxusDashboard() -> Element {
    rsx! { CourseLayout { title: "09-dioxus-dashboard".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/09-dioxus-dashboard.md".to_string(), div {} } }
}
#[component]
pub fn Page09Educational() -> Element {
    rsx! { CourseLayout { title: "09-educational".to_string(), file_path: "/home/drvoid/Qix/IronSight/report/09-educational.md".to_string(), div {} } }
}
