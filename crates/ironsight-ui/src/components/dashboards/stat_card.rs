use dioxus::prelude::*;

/// A generic dashboard stat card with icon, label, value, and optional trend text.
#[component]
pub fn DashStatCard(
    label: String,
    value: String,
    #[props(default)] icon: String,
    #[props(default)] trend: String,
    #[props(default = "default".to_string())] variant: String,
) -> Element {
    let border_color = match variant.as_str() {
        "critical" | "danger" => "var(--accent-red)",
        "high" | "warning" => "var(--accent-orange)",
        "medium" => "var(--accent-yellow)",
        "success" | "green" => "var(--accent-green)",
        "info" | "blue" => "var(--accent-blue)",
        "purple" => "var(--accent-cyan)",
        _ => "var(--border)",
    };

    let value_color = match variant.as_str() {
        "critical" | "danger" => "var(--accent-red)",
        "high" | "warning" => "var(--accent-orange)",
        "medium" => "var(--accent-yellow)",
        "success" | "green" => "var(--accent-green)",
        "info" | "blue" => "var(--accent-blue)",
        "purple" => "var(--accent-cyan)",
        _ => "var(--text-primary)",
    };

    rsx! {
        div {
            class: "dash-stat-card",
            style: "border-left: 3px solid {border_color};",
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;",
                span {
                    style: "font-size: 10px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.5px;",
                    "{label}"
                }
                if !icon.is_empty() {
                    span { style: "font-size: 16px;", "{icon}" }
                }
            }
            div {
                style: "display: flex; align-items: baseline; gap: 8px;",
                span {
                    style: "font-size: 28px; font-weight: 800; color: {value_color}; font-family: var(--font-mono); line-height: 1;",
                    "{value}"
                }
                if !trend.is_empty() {
                    span {
                        style: "font-size: 10px; color: var(--text-muted); white-space: nowrap;",
                        "{trend}"
                    }
                }
            }
        }
    }
}
