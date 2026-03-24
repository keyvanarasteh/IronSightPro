use dioxus::prelude::*;
use crate::components::card::*;
use crate::components::accordion::*;
use crate::components::tabs::*;

#[component]
pub fn IdeaDoc() -> Element {
    rsx! {
        div { class: "app-container",
            div { class: "header",
                div {}
                div { class: "header-actions", "Fikir Dokümanı — Process Spy" }
            }
            div { class: "main-content", style: "padding: 24px; max-width: 1000px; margin: 0 auto; display: flex; flex-direction: column; gap: 32px;",
                
                Card {
                    style: "padding: 24px; border-left: 4px solid var(--accent-blue);",
                    h2 { style: "margin-top: 0; color: var(--accent-blue);", "Neden Önemli?" }
                    p { "Her gün duyuyoruz — şu şirket hacklendi, şu altyapı çöktü, veriler sızdı. Ama saldırının %90'ı bir process ile başlıyor." }
                    p { style: "font-style: italic; color: var(--text-muted);", "\"Sence bir saldırgan sisteme girdiğinde ilk ne yapar? Bir process başlatır.\"" }
                }

                div {
                    h2 { "Gerçek Hayatta Ne Olur?" }
                    Accordion {
                        AccordionItem { index: 0,
                            AccordionTrigger { "Senaryo 1: Fileless Malware" }
                            AccordionContent {
                                p { "Saldırgan sisteme giriyor. Diske hiçbir şey yazmıyor. RAM'de buffer açıyor, shellcode yazıyor, protection'ı RW → RX yapıyor." }
                                p { style: "color: var(--accent-red);", "Röntgen çekmek yetmez — ameliyata girmemiz lazım." }
                            }
                        }
                        AccordionItem { index: 1,
                            AccordionTrigger { "Senaryo 2: DLL Sideloading" }
                            AccordionContent {
                                p { "Meşru bir process (svchost.exe). Herkes güveniyor. Ama içine zararlı bir DLL yüklenmiş. Modülleri listelediğinde /tmp/ altından yüklenmiş bir .dll var." }
                            }
                        }
                        AccordionItem { index: 2,
                            AccordionTrigger { "Senaryo 3: C2 Callback" }
                            AccordionContent {
                                p { "CPU %0.1, RAM 5 MB. Dikkat çekmiyor. Ama network'e bakıyorsun — her 30 saniyede bir Rusya'daki bir IP'ye bağlanıyor." }
                            }
                        }
                    }
                }

                div {
                    h2 { "Rust Tabanlı Modüler Mimari" }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 16px;",
                        Card { CardHeader { CardTitle { "ProcessSpy" } CardDescription { "Kim çalışıyor?" } } }
                        Card { CardHeader { CardTitle { "SecurityAudit" } CardDescription { "Güvenilir mi?" } } }
                        Card { CardHeader { CardTitle { "NetworkMapper" } CardDescription { "Nereye bağlanıyor?" } } }
                        Card { CardHeader { CardTitle { "MemoryScanner" } CardDescription { "RAM'de ne saklıyor?" } } }
                        Card { CardHeader { CardTitle { "HeuristicEngine" } CardDescription { "Tehdit puanı ne?" } } }
                        Card { CardHeader { CardTitle { "ResponseHandler" } CardDescription { "Dondur / Dump / Öldür" } } }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PlanDoc() -> Element {
    let mut selected_sprint = use_signal(|| Some("Sprint 1".to_string()));
    
    let sprints = vec![
        ("Sprint 1", "Foundation (Core Process & Snapshotting)"),
        ("Sprint 2", "Security Layer (Hash, Entropy, Signatures)"),
        ("Sprint 3", "Network Layer (Socket ↔ PID Mapping)"),
        ("Sprint 4", "Memory Analysis (Pattern & State Watching)"),
        ("Sprint 5", "Kernel Integration (eBPF & ETW)"),
        ("Sprint 6", "Intelligence (Heuristic Scoring)"),
        ("Sprint 7", "Response (Suspend, Dump, Kill)"),
        ("Sprint 8", "Reporting (SIEM JSON)"),
        ("Sprint 9", "Service Orchestration & Docker"),
        ("Sprint 10", "Desktop UI Dashboard"),
    ];

    rsx! {
        div { class: "app-container",
            div { class: "header",
                div {}
                div { class: "header-actions", "Uygulama Planı ve Görevler" }
            }
            div { class: "main-content", style: "padding: 24px; max-width: 1000px; margin: 0 auto; display: flex; flex-direction: column; gap: 24px;",
                
                Card {
                    style: "background: var(--surface-bg); border-left: 4px solid var(--accent-green); padding: 16px;",
                    h2 { style: "margin-top: 0;", "Öncelik Sırası" }
                    code { style: "display: block; padding: 12px; background: rgba(0,0,0,0.5); border-radius: 4px;",
                        "[KRİTİK] Sprint 1-2: Core + Security\n[YÜKSEK] Sprint 3-4: Network + Memory\n[YÜKSEK] Sprint 5-6: Kernel + Heuristic\n[ORTA]   Sprint 7-8: Response + Report\n[ORTA]   Sprint 9: Service + Docker\n[DÜŞÜK]  Sprint 10: UI Dashboard"
                    }
                }

                h2 { "Sprint Planlaması" }
                Tabs {
                    value: selected_sprint,
                    on_value_change: move |v| selected_sprint.set(Some(v)),
                    TabList {
                        for (i, (sprint, _)) in sprints.iter().enumerate() {
                            TabTrigger {
                                value: sprint.to_string(),
                                index: i,
                                "{sprint}"
                            }
                        }
                    }

                    for (i, (sprint, title)) in sprints.iter().enumerate() {
                        TabContent {
                            value: sprint.to_string(),
                            index: i,
                            Card {
                                style: "padding: 24px; border: 1px solid var(--border-color);",
                                h3 { style: "margin-top: 0;", "{sprint} - {title}" }
                                ul { style: "color: var(--text-main); line-height: 1.6;",
                                    if *sprint == "Sprint 1" {
                                        li { "Workspace Cargo.toml oluştur" }
                                        li { "ProcessSpy facade & builder pattern" }
                                        li { "Snapshot, Diff, Filter mekanizmaları" }
                                    } else if *sprint == "Sprint 2" {
                                        li { "SecurityAudit struct" }
                                        li { "SHA-256 ve Shannon entropi" }
                                        li { "Authenticode doğrulama ve path analizleri" }
                                    } else if *sprint == "Sprint 10" {
                                        li { "Dioxus Desktop Dashboard" }
                                        li { "Gerçek zamanlı Process Table" }
                                        li { "Threat Gauge & Security Badge UI" }
                                    } else {
                                        li { "Görev detayları için orijinal markdown dokümanına başvurun." }
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

#[component]
pub fn RoadmapDoc() -> Element {
    rsx! {
        div { class: "app-container",
            div { class: "header",
                div {}
                div { class: "header-actions", "Yol Haritası (Katmanlar)" }
            }
            div { class: "main-content", style: "padding: 24px; max-width: 1000px; margin: 0 auto;",
                
                h2 { style: "margin-bottom: 24px;", "Güvenlik Katmanları" }

                Accordion {
                    AccordionItem { index: 0,
                        AccordionTrigger { "Faz 1: Temel — Kim Çalışıyor?" }
                        AccordionContent {
                            p { "Process Enumeration Core, Sıralama, Filtreleme, Process Tree ve Snapshot Diff." }
                        }
                    }
                    AccordionItem { index: 1,
                        AccordionTrigger { "Faz 2: Güvenlik — Güvenilir mi?" }
                        AccordionContent {
                            p { "Binary Integrity (Hash/Entropi), Imza Doğrulaması, ve module enumeration." }
                        }
                    }
                    AccordionItem { index: 2,
                        AccordionTrigger { "Faz 3: Ağ — Nereye Bağlanıyor?" }
                        AccordionContent {
                            p { "Socket to PID Mapping ve DNS Enrichment." }
                        }
                    }
                    AccordionItem { index: 3,
                        AccordionTrigger { "Faz 4: Bellek — RAM'de Ne Saklıyor?" }
                        AccordionContent {
                            p { "Memory Scanner (Pattern tarama) ve Memory Watcher (Değişim takibi)." }
                        }
                    }
                    AccordionItem { index: 4,
                        AccordionTrigger { "Faz 5: Kernel — Gerçek Zamanlı Algılama" }
                        AccordionContent {
                            p { "eBPF (Linux) ve ETW (Windows) izleme, Memory Protection geçişlerini yakalama." }
                        }
                    }
                    AccordionItem { index: 5,
                        AccordionTrigger { "Faz 6 & 7: Beyin ve Kas" }
                        AccordionContent {
                            p { "Heuristic Scoring Engine (0-100 Puanlama) ve Response Handler (Dondur > Dump Al > Öldür)." }
                        }
                    }
                }
                
                Card {
                    style: "margin-top: 32px; padding: 24px; background: rgba(0,0,0,0.3);",
                    h3 { style: "margin-top: 0;", "Özet Analitik Formül" }
                    div { style: "font-family: monospace; font-size: 16px; color: var(--accent-orange); text-align: center; padding: 16px;",
                        "R = Σ (Score × Impact) / n"
                    }
                    p { style: "text-align: center; color: var(--text-muted);", "Sistem Risk Endeksi bu formüle göre hesaplanıp Dashboard üzerinde gösterilir." }
                }
            }
        }
    }
}
