use dioxus::prelude::*;
use crate::components::edr_widgets::*;

#[component]
pub fn Configuration() -> Element {
    rsx! {
        div { class: "dash-page",
            div { style: "padding: 20px 24px;",
                h1 { style: "font-size: 20px; font-weight: 800; color: var(--qs-fg); margin: 0 0 20px;", "⚙️ Configuration" }

                // Tabs
                div { style: "display: flex; gap: 4px; margin-bottom: 20px;",
                    span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg);", "Thresholds" }
                    span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Scan" }
                    span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "Watchdog" }
                    span { style: "padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; background: var(--qs-muted); color: var(--qs-fg-muted);", "System" }
                }

                div { style: "background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; padding: 24px;",
                    h3 { style: "font-size: 14px; font-weight: 700; color: var(--qs-fg); margin: 0 0 20px;", "Threat Thresholds" }
                    ConfigSlider { label: "Low Score Threshold".to_string(), value: 10, min: 0, max: 100, description: "Minimum score for Low threat level".to_string() }
                    ConfigSlider { label: "Medium Score Threshold".to_string(), value: 30, min: 0, max: 100, description: "Minimum score for Medium threat level".to_string() }
                    ConfigSlider { label: "High Score Threshold".to_string(), value: 50, min: 0, max: 100, description: "Minimum score for High threat level".to_string() }
                    ConfigSlider { label: "Critical Score Threshold".to_string(), value: 70, min: 0, max: 100, description: "Minimum score for Critical threat level".to_string() }
                    ConfigSlider { label: "Export Min Score".to_string(), value: 10, min: 0, max: 100, description: "Minimum score for report generation".to_string() }

                    hr { style: "border: none; border-top: 1px solid var(--qs-border); margin: 16px 0;" }
                    h3 { style: "font-size: 14px; font-weight: 700; color: var(--qs-fg); margin: 0 0 16px;", "Response Settings" }
                    ConfigToggle { label: "Auto Response".to_string(), enabled: false, description: "Automatically suspend high-threat processes".to_string() }
                    ConfigSlider { label: "Auto Response Min Score".to_string(), value: 70, min: 0, max: 100, description: "Minimum score for automatic response".to_string() }

                    // Action buttons
                    div { style: "display: flex; gap: 8px; margin-top: 24px; padding-top: 16px; border-top: 1px solid var(--qs-border);",
                        button { style: "padding: 8px 20px; border-radius: 8px; border: none; background: var(--qs-primary); color: var(--qs-primary-fg); font-size: 12px; font-weight: 700; cursor: pointer;", "💾 Save Changes" }
                        button { style: "padding: 8px 20px; border-radius: 8px; border: 1px solid var(--qs-border); background: var(--qs-card); color: var(--qs-fg-muted); font-size: 12px; font-weight: 600; cursor: pointer;", "↺ Reset Defaults" }
                        button { style: "padding: 8px 20px; border-radius: 8px; border: 1px solid var(--qs-border); background: var(--qs-card); color: var(--qs-fg-muted); font-size: 12px; font-weight: 600; cursor: pointer;", "📤 Export Config" }
                    }
                }
            }
        }
    }
}
