use dioxus::prelude::*;

const LAYOUT_CSS: &str = r#"
.host-dashboard {
    padding: 20px 24px;
    max-width: 1400px;
    margin: 0 auto;
    font-family: var(--font-mono);
    color: var(--text-primary);
    overflow-y: auto;
    height: 100vh;
}
.host-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}
.host-title {
    font-size: 14px;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 8px;
}
.host-hero {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 16px 20px;
    margin-bottom: 20px;
}
.host-hero-top {
    margin-bottom: 16px;
}
.host-hero-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 12px;
}
.host-mini-card {
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-primary);
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
}
.host-mini-card:hover {
    background: var(--bg-hover);
    border-color: var(--accent-blue);
}
.host-main-grid {
    display: grid;
    grid-template-columns: minmax(260px, 300px) 1fr;
    gap: 20px;
}
.host-widget {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 16px 20px;
    margin-bottom: 20px;
}
.host-button-row {
    display: flex;
    gap: 12px;
    margin-bottom: 20px;
}
.host-btn {
    flex: 1;
    text-align: center;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 500;
    transition: all 0.15s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    color: var(--text-primary);
}
.host-btn:hover {
    background: var(--bg-hover);
    border-color: var(--accent-blue);
}
.host-clear-cache {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.host-badge-green {
    font-size: 24px;
    color: var(--accent-green);
    text-align: center;
    margin-bottom: 8px;
    line-height: 1;
}
"#;

#[component]
pub fn CourseLayout(title: String, file_path: String, children: Element) -> Element {
    rsx! {
        style { "{LAYOUT_CSS}" }
        div { class: "host-dashboard",
            div { class: "host-header",
                div { class: "host-title",
                    "Dashboard"
                    span { style: "color: var(--text-muted);", " / " }
                    span { style: "color: var(--text-muted);", "Reports" }
                    span { style: "color: var(--text-muted);", " / " }
                    span { style: "color: var(--text-primary); font-weight: 600;", "{title}" }
                }
                button { 
                    style: "padding: 6px 12px; color: var(--text-primary); border: 1px solid var(--border); background: var(--bg-card); border-radius: 4px; font-weight: 500; font-size: 12px; cursor: pointer;",
                    "Upgrade plan" 
                }
            }

            div { class: "host-hero",
                div { class: "host-hero-top",
                    div { style: "font-weight: 600; font-size: 14px; display: flex; align-items: center; gap: 8px;",
                        span { "📄" }
                        "{title}"
                    }
                    div { style: "color: var(--text-muted); font-size: 12px; margin-top: 4px;", "Created: 2024-08-06" }
                }

                div { class: "host-hero-grid",
                    div { class: "host-mini-card",
                        div { 
                            div { style: "font-weight: 500; font-size: 13px; margin-bottom: 2px;", "Domain" }
                            div { style: "color: var(--text-muted); font-size: 12px;", "Active" }
                        }
                        span { style: "color: var(--text-muted);", "›" }
                    }
                    div { class: "host-mini-card",
                        div { 
                            div { style: "font-weight: 500; font-size: 13px; margin-bottom: 2px;", "Hosting" }
                            div { style: "color: var(--text-muted); font-size: 12px;", "Active" }
                        }
                        span { style: "color: var(--text-muted);", "›" }
                    }
                    div { class: "host-mini-card",
                        div { 
                            div { style: "font-weight: 500; font-size: 13px; margin-bottom: 2px;", "Free Email" }
                            div { style: "color: var(--text-muted); font-size: 12px;", "Pending setup" }
                        }
                        span { style: "color: var(--text-muted);", "›" }
                    }
                    div { class: "host-mini-card",
                        div { 
                            div { style: "font-weight: 500; font-size: 13px; margin-bottom: 2px;", "Backups" }
                            div { style: "color: var(--text-muted); font-size: 12px;", "Weekly" }
                        }
                        span { style: "color: var(--text-muted);", "›" }
                    }
                }
            }

            div { class: "host-main-grid",
                div { class: "host-col-left",
                    div { class: "host-widget", style: "text-align: center;",
                        div { style: "font-weight: 600; text-align: left; margin-bottom: 16px; font-size: 13px;", "PageSpeed Insights" }
                        div { style: "width: 72px; height: 72px; border-radius: 50%; border: 4px solid var(--accent-green); display: flex; align-items: center; justify-content: center; font-size: 20px; font-weight: 600; margin: 0 auto 16px auto; color: var(--accent-green);", "98" }
                        div { style: "font-size: 13px; font-weight: 500;", "Desktop device" }
                        div { style: "color: var(--text-muted); font-size: 12px; margin-top: 4px; margin-bottom: 16px;", "Last scan on 2025-05-31" }
                        span { style: "color: var(--accent-blue); font-size: 12px; font-weight: 500; cursor: pointer; display: block; text-align: left;", "Run speed test" }
                    }
                    div { class: "host-widget",
                        div { style: "font-weight: 500; font-size: 13px; display: flex; align-items: center; gap: 8px;",
                            span { style: "color: var(--accent-green);", "✓" }
                            "Website is safe" 
                        }
                        div { style: "color: var(--text-muted); font-size: 12px; margin: 6px 0;", "No malware found" }
                        span { style: "color: var(--accent-blue); font-size: 12px; font-weight: 500; cursor: pointer; display: block; margin-top: 12px;", "See details" }
                    }
                    div { class: "host-widget",
                        div { style: "font-weight: 500; font-size: 13px; margin-bottom: 8px;", "Hosting resources usage" }
                        div { style: "color: var(--text-muted); font-size: 12px;", "28% of total resources used" }
                    }
                }

                div { class: "host-col-right",
                    div { class: "host-button-row",
                        div { class: "host-btn", "📁 File manager" }
                        div { class: "host-btn", "🗄 Databases" }
                        div { class: "host-btn", "⚡ Auto Installer" }
                    }

                    div { class: "host-widget host-clear-cache",
                        div {
                            div { style: "font-weight: 600; font-size: 13px;", "Clear cache" }
                            div { style: "color: var(--text-muted); font-size: 12px; margin-top: 4px;", "Visit your website without cache or clear it with a single click." }
                        }
                        div { style: "display: flex; gap: 8px;",
                            button { style: "padding: 6px 12px; background: transparent; border: 1px solid var(--border); color: var(--text-primary); font-size: 12px; font-weight: 500; border-radius: 4px; cursor: pointer;", "No cache preview" }
                            button { style: "padding: 6px 12px; background: var(--bg-hover); color: var(--text-primary); border: 1px solid var(--border); font-size: 12px; font-weight: 500; border-radius: 4px; cursor: pointer;", "Clear cache" }
                        }
                    }

                    div { class: "host-widget", style: "text-align: center; padding: 32px 20px;",
                        div { class: "host-badge-green", "✓" }
                        div { style: "font-size: 14px; font-weight: 600; margin-bottom: 6px;", "Your website is running smoothly" }
                        div { style: "color: var(--text-muted); font-size: 12px; margin-bottom: 24px;", "No issues were found" }

                        div { style: "text-align: left; padding: 16px; border: 1px solid var(--border); border-radius: 6px; background: var(--bg-primary);",
                            h3 { style: "margin-bottom: 12px; color: var(--text-primary); font-size: 13px; font-weight: 600;", "Developer Workspace" }
                            p { style: "margin-bottom: 12px; color: var(--text-secondary); font-size: 12px;", "Code specific components and UI for this tutorial module inside this placeholder." }
                            div { style: "margin-bottom: 16px;",
                                span { style: "color: var(--text-muted); font-size: 12px; margin-right: 8px;", "Target:" }
                                code { style: "background: var(--bg-hover); padding: 4px 8px; border-radius: 4px; font-size: 12px; border: 1px solid var(--border); color: var(--accent-cyan);", "{file_path}" }
                            }
                            
                            div { style: "margin-top: 16px; border-top: 1px dashed var(--border); padding-top: 16px;",
                                {children}
                            }
                        }
                    }
                }
            }
        }
    }
}
