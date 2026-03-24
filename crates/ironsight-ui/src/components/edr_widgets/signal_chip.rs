use dioxus::prelude::*;

/// Heuristic signal chip with category icon and weight.
#[component]
pub fn SignalChip(
    name: String,
    category: String,
    weight: u32,
) -> Element {
    let (icon, color) = match category.to_lowercase().as_str() {
        "processbehavior" | "process" => ("🔄", "#3b82f6"),
        "networkanomaly" | "network" => ("🌐", "#8b5cf6"),
        "memoryanomaly" | "memory" => ("🧠", "#f59e0b"),
        "staticanalysis" | "static" => ("🔒", "#ef4444"),
        "filesystemanomaly" | "filesystem" => ("📁", "#6366f1"),
        _ => ("📌", "var(--qs-fg-muted)"),
    };

    rsx! {
        span { style: "display: inline-flex; align-items: center; gap: 4px; padding: 3px 8px; border-radius: 6px; font-size: 10px; font-weight: 700; font-family: var(--font-mono); color: {color}; background: {color}15; border: 1px solid {color}30; white-space: nowrap;",
            span { style: "font-size: 11px;", "{icon}" }
            "{name}"
            span { style: "opacity: 0.7;", ": {weight}" }
        }
    }
}

/// Signal chip list wrapper.
#[component]
pub fn SignalChipList(children: Element) -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 4px;", {children} }
    }
}
