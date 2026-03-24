use dioxus::prelude::*;

/// A security alert card with severity, title, and description.
#[component]
pub fn AlertCard(
    title: String,
    description: String,
    #[props(default)] code: String,
    #[props(default)] timestamp: String,
    #[props(default = "info".to_string())] severity: String,
) -> Element {
    let (_, icon) = match severity.as_str() {
        "critical" => ("var(--accent-red)", "🚨"),
        "high" => ("var(--accent-orange)", "⚠️"),
        "medium" => ("var(--accent-yellow)", "⚡"),
        _ => ("var(--accent-blue)", "ℹ️"),
    };

    let title_color = match severity.as_str() {
        "critical" => "var(--accent-red)",
        "high" => "var(--accent-orange)",
        "medium" => "var(--accent-yellow)",
        _ => "var(--accent-blue)",
    };

    rsx! {
        div {
            style: "padding: 12px 16px; border-bottom: 1px solid var(--border); transition: background 0.15s; cursor: pointer;",
            div {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 6px;",
                span { style: "font-size: 14px;", "{icon}" }
                span { style: "font-size: 11px; font-weight: 700; color: {title_color}; text-transform: uppercase;", "{title}" }
                if !timestamp.is_empty() {
                    span { style: "margin-left: auto; font-size: 10px; font-family: var(--font-mono); color: var(--text-muted);", "{timestamp}" }
                }
            }
            p { style: "font-size: 11px; color: var(--text-secondary); margin: 0 0 4px 0; line-height: 1.5;", "{description}" }
            if !code.is_empty() {
                span { style: "font-size: 9px; font-family: var(--font-mono); color: var(--text-muted);", "ERR_CODE: {code}" }
            }
        }
    }
}

/// Container for alert cards in a list.
#[component]
pub fn AlertList(
    #[props(default)] title: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "alert-list-panel",
            if !title.is_empty() {
                div {
                    style: "padding: 12px 16px; border-bottom: 1px solid var(--border);",
                    span { style: "font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.2px;", "{title}" }
                }
            }
            {children}
        }
    }
}
