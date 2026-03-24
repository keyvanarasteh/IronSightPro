use dioxus::prelude::*;

/// Chart panel wrapper with title, description, icon, and optional legend.
#[component]
pub fn ChartPanel(
    title: String,
    #[props(default)] description: String,
    #[props(default)] icon: String,
    #[props(default = "400px".to_string())] height: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "qs-chart-panel",
            div { style: "display: flex; align-items: center; gap: 12px; margin-bottom: 12px;",
                if !icon.is_empty() {
                    div { class: "qs-doc-icon", "{icon}" }
                }
                div {
                    h4 { style: "font-size: 14px; font-weight: 500; color: var(--qs-fg); margin: 0;", "{title}" }
                    if !description.is_empty() {
                        p { style: "font-size: 12px; color: var(--qs-fg-muted); margin: 2px 0 0;", "{description}" }
                    }
                }
            }
            div {
                style: "width: 100%; height: {height}; border: 1px solid var(--qs-border); border-radius: 8px; background: var(--qs-bg); overflow: hidden; position: relative;",
                {children}
            }
        }
    }
}

/// Chart legend item with colored dot.
#[component]
pub fn ChartLegend(children: Element) -> Element {
    rsx! {
        div { class: "qs-chart-legend", {children} }
    }
}

/// Single legend item.
#[component]
pub fn ChartLegendItem(
    label: String,
    color: String,
) -> Element {
    rsx! {
        div { class: "qs-chart-legend-item",
            div { class: "qs-chart-legend-dot", style: "background: {color};" }
            span { "{label}" }
        }
    }
}

/// Inline CSS-only mini bar chart (sparkline).
#[component]
pub fn ChartMiniBar(
    values: Vec<f64>,
    #[props(default = "var(--qs-primary)".to_string())] color: String,
    #[props(default = 32.0)] height: f64,
) -> Element {
    let max_val = values.iter().cloned().fold(1.0f64, f64::max);

    rsx! {
        div { class: "qs-mini-bars", style: "height: {height}px;",
            for (i, val) in values.iter().enumerate() {
                {
                    let pct = (val / max_val * 100.0).max(2.0);
                    rsx! {
                        div {
                            key: "{i}",
                            class: "qs-mini-bar",
                            style: "height: {pct}%; background: {color};",
                        }
                    }
                }
            }
        }
    }
}
