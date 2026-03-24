use dioxus::prelude::*;
use crate::components::card::*;

#[component]
pub fn StatCard(label: String, value: String, variant: String) -> Element {
    let color = match variant.as_str() {
        "risk" => "var(--accent-red)",
        "critical" => "var(--accent-red)",
        "high" => "var(--accent-orange)",
        "medium" => "var(--accent-yellow)",
        "time" => "var(--accent-blue)",
        _ => "var(--text-main)",
    };

    rsx! {
        Card {
            style: "flex: 1; min-width: 150px; background: var(--surface-bg); border: 1px solid var(--border-color);",
            CardHeader {
                style: "padding-bottom: 8px;",
                CardTitle { 
                    style: "font-size: 12px; color: var(--text-muted); font-weight: 500; letter-spacing: 0.5px;", 
                    "{label}" 
                }
            }
            CardContent {
                style: "padding-top: 0;",
                div {
                    style: "font-size: 28px; font-weight: 700; color: {color}; text-shadow: 0 0 10px {color}40;",
                    "{value}"
                }
            }
        }
    }
}
