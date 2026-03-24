use dioxus::prelude::*;

/// Config slider with color zone bands for threshold values.
#[component]
pub fn ConfigSlider(
    label: String,
    value: u32,
    min: u32,
    max: u32,
    #[props(default)] description: String,
) -> Element {
    let range = max - min;
    let pct = if range > 0 { ((value - min) as f64 / range as f64 * 100.0).min(100.0) } else { 0.0 };

    // Color zone preview
    let color = if pct < 30.0 { "var(--qs-success)" }
        else if pct < 60.0 { "var(--qs-warning)" }
        else { "var(--qs-destructive)" };

    rsx! {
        div { style: "padding: 12px 0;",
            div { style: "display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 6px;",
                span { style: "font-size: 13px; font-weight: 600; color: var(--qs-fg);", "{label}" }
                span { style: "font-size: 14px; font-weight: 800; font-family: var(--font-mono); color: {color};", "{value}" }
            }
            if !description.is_empty() {
                p { style: "font-size: 11px; color: var(--qs-fg-muted); margin: 0 0 8px;", "{description}" }
            }
            // Track
            div { style: "position: relative; height: 8px; background: var(--qs-muted); border-radius: 4px; overflow: hidden;",
                // Zone bands
                div { style: "position: absolute; inset: 0; display: flex;",
                    div { style: "flex: 3; background: rgba(16,185,129,0.15);" }
                    div { style: "flex: 3; background: rgba(245,158,11,0.15);" }
                    div { style: "flex: 4; background: rgba(239,68,68,0.15);" }
                }
                // Fill
                div { style: "position: absolute; top: 0; left: 0; height: 100%; width: {pct}%; background: {color}; border-radius: 4px; transition: width 0.3s;" }
            }
            // Labels
            div { style: "display: flex; justify-content: space-between; margin-top: 4px;",
                span { style: "font-size: 9px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "{min}" }
                span { style: "font-size: 9px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "{max}" }
            }
        }
    }
}

/// Config toggle row.
#[component]
pub fn ConfigToggle(
    label: String,
    enabled: bool,
    #[props(default)] description: String,
) -> Element {
    let (bg, dot_pos) = if enabled {
        ("var(--qs-success)", "translateX(16px)")
    } else {
        ("var(--qs-muted)", "translateX(0)")
    };
    rsx! {
        div { style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid var(--qs-border-subtle);",
            div {
                div { style: "font-size: 13px; font-weight: 600; color: var(--qs-fg);", "{label}" }
                if !description.is_empty() {
                    div { style: "font-size: 11px; color: var(--qs-fg-muted); margin-top: 2px;", "{description}" }
                }
            }
            div { style: "width: 36px; height: 20px; border-radius: 10px; background: {bg}; position: relative; cursor: pointer; transition: background 0.2s; flex-shrink: 0;",
                div { style: "width: 16px; height: 16px; border-radius: 50%; background: white; position: absolute; top: 2px; left: 2px; transform: {dot_pos}; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.3);" }
            }
        }
    }
}
