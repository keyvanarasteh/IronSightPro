use dioxus::prelude::*;

/// A pattern scanner result row.
#[component]
pub fn ScannerResultRow(
    offset: String,
    pattern: String,
    context: String,
    #[props(default = "info".to_string())] severity: String,
) -> Element {
    let sev_color = match severity.as_str() {
        "critical" => "var(--accent-red)",
        "high" => "var(--accent-orange)",
        "medium" => "var(--accent-yellow)",
        _ => "var(--accent-blue)",
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 8px 12px; border-bottom: 1px solid var(--border); transition: background 0.15s;",
            span { style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-muted); min-width: 90px;", "{offset}" }
            span { style: "font-family: var(--font-mono); font-size: 11px; color: {sev_color}; font-weight: 600; min-width: 120px;", "{pattern}" }
            span { style: "font-family: var(--font-mono); font-size: 10px; color: var(--text-secondary); flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{context}" }
        }
    }
}

/// Container for scanner results with header.
#[component]
pub fn ScannerResultsPanel(
    title: String,
    #[props(default)] count: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "scanner-results-panel",
            div {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-bottom: 1px solid var(--border);",
                span { style: "font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.2px;", "{title}" }
                if !count.is_empty() {
                    span {
                        style: "font-size: 10px; font-family: var(--font-mono); background: var(--badge-info-bg); color: var(--accent-blue); padding: 2px 8px; border-radius: 4px; font-weight: 600;",
                        "{count}"
                    }
                }
            }
            div { style: "max-height: 240px; overflow-y: auto;", {children} }
        }
    }
}
