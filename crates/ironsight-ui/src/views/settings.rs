use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let mut is_dark: Signal<bool> = use_context();

    rsx! {
        div { class: "app-container",
            div { class: "header",
                span { style: "font-size: 20px; font-weight: 600; letter-spacing: 1px;", "⚙ SETTINGS" }
                div { class: "header-actions", "System Configuration" }
            }
            div { style: "padding: 24px; overflow-y: auto; height: 100%;",

                // ── Appearance Section ──
                div {
                    style: "background: var(--bg-card); border: 1px solid var(--border); border-radius: 8px; padding: 24px; max-width: 600px; margin-bottom: 24px; box-shadow: var(--shadow-card);",
                    h3 { style: "color: var(--text-primary); font-size: 16px; font-weight: 700; margin-bottom: 4px; letter-spacing: 0.5px;", "Appearance" }
                    p { style: "color: var(--text-muted); font-size: 12px; margin-bottom: 20px;", "Customize the visual theme of the IronSight dashboard." }

                    // Theme selector
                    div {
                        style: "display: flex; gap: 12px;",

                        // Dark mode option
                        button {
                            style: format!(
                                "flex: 1; padding: 16px; border-radius: 8px; cursor: pointer; border: 2px solid {}; background: {}; transition: all 0.2s; text-align: center;",
                                if *is_dark.read() { "var(--accent-cyan)" } else { "var(--border)" },
                                if *is_dark.read() { "rgba(6, 182, 212, 0.08)" } else { "transparent" }
                            ),
                            onclick: move |_| { is_dark.set(true); },
                            div { style: "font-size: 28px; margin-bottom: 8px;", "🌙" }
                            div { style: "font-weight: 600; color: var(--text-primary); font-size: 13px;", "Dark" }
                            div { style: "color: var(--text-muted); font-size: 11px; margin-top: 4px;", "Hacker terminal aesthetic" }
                        }

                        // Light mode option
                        button {
                            style: format!(
                                "flex: 1; padding: 16px; border-radius: 8px; cursor: pointer; border: 2px solid {}; background: {}; transition: all 0.2s; text-align: center;",
                                if !*is_dark.read() { "var(--accent-cyan)" } else { "var(--border)" },
                                if !*is_dark.read() { "rgba(6, 182, 212, 0.08)" } else { "transparent" }
                            ),
                            onclick: move |_| { is_dark.set(false); },
                            div { style: "font-size: 28px; margin-bottom: 8px;", "☀️" }
                            div { style: "font-weight: 600; color: var(--text-primary); font-size: 13px;", "Light" }
                            div { style: "color: var(--text-muted); font-size: 11px; margin-top: 4px;", "Clean and professional" }
                        }
                    }
                }

                // ── More Settings placeholder ──
                div {
                    style: "background: var(--bg-card); border: 1px solid var(--border); border-radius: 8px; padding: 24px; max-width: 600px; box-shadow: var(--shadow-card);",
                    h3 { style: "color: var(--text-primary); font-size: 16px; font-weight: 700; margin-bottom: 4px; letter-spacing: 0.5px;", "Scan Configuration" }
                    p { style: "color: var(--text-muted); font-size: 12px; margin-bottom: 20px;", "Configure system scan parameters and thresholds." }
                    div { style: "color: var(--text-muted); font-size: 12px; font-style: italic;", "Additional settings coming soon..." }
                }
            }
        }
    }
}
