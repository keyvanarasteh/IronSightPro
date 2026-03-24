use dioxus::prelude::*;

/// A single change event in the memory watcher feed.
#[component]
pub fn ChangeFeedItem(
    timestamp: String,
    event_type: String,
    description: String,
    #[props(default = "info".to_string())] severity: String,
) -> Element {
    let (border_color, icon) = match severity.as_str() {
        "critical" => ("var(--accent-red)", "🔴"),
        "high" | "warning" => ("var(--accent-orange)", "🟠"),
        "medium" => ("var(--accent-yellow)", "🟡"),
        "success" => ("var(--accent-green)", "🟢"),
        _ => ("var(--accent-blue)", "🔵"),
    };

    rsx! {
        div {
            class: "change-feed-item",
            style: "display: flex; gap: 10px; padding: 10px 12px; border-left: 3px solid {border_color};",
            span { style: "font-size: 12px; flex-shrink: 0; margin-top: 1px;", "{icon}" }
            div {
                style: "flex: 1; min-width: 0;",
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 4px;",
                    span { style: "font-size: 11px; font-weight: 700; color: var(--text-primary); text-transform: uppercase;", "{event_type}" }
                    span { style: "font-size: 10px; font-family: var(--font-mono); color: var(--text-muted);", "{timestamp}" }
                }
                p { style: "font-size: 11px; color: var(--text-secondary); margin: 0; line-height: 1.4;", "{description}" }
            }
        }
    }
}
