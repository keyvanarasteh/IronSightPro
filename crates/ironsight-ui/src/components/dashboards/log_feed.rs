use dioxus::prelude::*;

/// A single log entry in the signal/event feed.
#[component]
pub fn LogEntry(
    timestamp: String,
    level: String,
    message: String,
) -> Element {
    let level_color = match level.as_str() {
        "CRITICAL" | "ERROR" => "var(--accent-red)",
        "WARNING" | "WARN" => "var(--accent-orange)",
        "KERNEL" | "STRINGS" => "var(--accent-yellow)",
        "INFO" => "var(--accent-blue)",
        "AUTH" => "var(--accent-cyan)",
        _ => "var(--text-secondary)",
    };

    rsx! {
        div {
            style: "display: flex; gap: 12px; padding: 4px 8px; font-family: var(--font-mono); font-size: 12px; border-radius: 4px;",
            span { style: "color: var(--text-muted); opacity: 0.5; white-space: nowrap;", "[{timestamp}]" }
            span { style: "color: {level_color}; font-weight: 700; min-width: 60px;", "[{level}]" }
            span { style: "color: var(--text-secondary);", "{message}" }
        }
    }
}

/// Container for log feed with optional blinking cursor.
#[component]
pub fn LogFeedPanel(
    title: String,
    #[props(default = true)] show_cursor: bool,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "log-feed-panel",
            div {
                style: "padding: 12px 16px; border-bottom: 1px solid var(--border);",
                span { style: "font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.2px;", "{title}" }
            }
            div {
                class: "log-feed-body",
                style: "padding: 12px; max-height: 300px; overflow-y: auto;",
                {children}
                if show_cursor {
                    div {
                        style: "color: var(--accent-blue); font-family: var(--font-mono); font-size: 12px; opacity: 0.7; margin-top: 4px;",
                        "_ system_monitoring_active..."
                    }
                }
            }
        }
    }
}
