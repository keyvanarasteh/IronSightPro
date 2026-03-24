use dioxus::prelude::*;

/// Service/Worker status card with name, state, uptime, and metrics.
#[component]
pub fn ServiceStatusCard(
    name: String,
    #[props(default)] description: String,
    #[props(default = "running".to_string())] status: String,
    #[props(default)] uptime: String,
    #[props(default)] icon: String,
    children: Element,
) -> Element {
    let (status_color, status_icon) = match status.to_lowercase().as_str() {
        "running" | "active" | "healthy" => ("var(--qs-success)", "🟢"),
        "warning" | "degraded" => ("var(--qs-warning)", "🟡"),
        "error" | "critical" | "down" => ("var(--qs-destructive)", "🔴"),
        "stopped" | "inactive" => ("var(--qs-fg-muted)", "⚪"),
        _ => ("var(--qs-fg-muted)", "⚪"),
    };

    rsx! {
        div { class: "qs-service-card",
            div { style: "display: flex; align-items: center; gap: 12px;",
                if !icon.is_empty() {
                    div { class: "qs-doc-icon", "{icon}" }
                }
                div { style: "flex: 1; min-width: 0;",
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        h4 { style: "font-size: 14px; font-weight: 600; color: var(--qs-fg); margin: 0;", "{name}" }
                        span { style: "font-size: 10px;", "{status_icon}" }
                        span { style: "font-size: 10px; font-weight: 700; color: {status_color}; text-transform: uppercase;", "{status}" }
                    }
                    if !description.is_empty() {
                        p { style: "font-size: 11px; color: var(--qs-fg-muted); margin: 2px 0 0;", "{description}" }
                    }
                }
                if !uptime.is_empty() {
                    div { style: "text-align: right; flex-shrink: 0;",
                        div { style: "font-size: 10px; color: var(--qs-fg-subtle); text-transform: uppercase; letter-spacing: 0.05em;", "Uptime" }
                        div { style: "font-size: 13px; font-weight: 700; font-family: var(--font-mono); color: var(--qs-fg);", "{uptime}" }
                    }
                }
            }
            // Metrics (children)
            {children}
        }
    }
}

/// A single metric row inside a service card.
#[component]
pub fn ServiceMetric(
    label: String,
    value: String,
    #[props(default)] unit: String,
) -> Element {
    rsx! {
        div { class: "qs-service-metric",
            span { style: "font-size: 12px; color: var(--qs-fg-muted);", "{label}" }
            div { style: "display: flex; align-items: baseline; gap: 4px;",
                span { style: "font-size: 13px; font-weight: 700; font-family: var(--font-mono); color: var(--qs-fg);", "{value}" }
                if !unit.is_empty() {
                    span { style: "font-size: 10px; color: var(--qs-fg-subtle);", "{unit}" }
                }
            }
        }
    }
}

/// Grid layout for multiple service cards.
#[component]
pub fn ServiceGrid(children: Element) -> Element {
    rsx! {
        div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 12px;",
            {children}
        }
    }
}
