use dioxus::prelude::*;

/// Flow graph container with toolbar and canvas area.
#[component]
pub fn FlowGraph(
    name: String,
    description: String,
    #[props(default)] icon: String,
    #[props(default = "var(--qs-primary)".to_string())] color: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "qs-flow-container",
            div { class: "qs-flow-toolbar",
                div { style: "display: flex; align-items: center; gap: 10px;",
                    if !icon.is_empty() {
                        span { style: "font-size: 18px;", "{icon}" }
                    }
                    div {
                        h3 { style: "font-size: 14px; font-weight: 700; color: var(--qs-fg); margin: 0;", "{name}" }
                        p { style: "font-size: 11px; color: var(--qs-fg-muted); margin: 2px 0 0;", "{description}" }
                    }
                }
            }
            div { class: "qs-flow-canvas",
                {children}
            }
        }
    }
}

/// Single flow node card.
#[component]
pub fn FlowNodeCard(
    label: String,
    sub_label: String,
    #[props(default)] icon: String,
    #[props(default = false)] active: bool,
    #[props(default = false)] completed: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let state_cls = if active { "active" } else if completed { "completed" } else { "" };
    let status_indicator = if active { "🔵" } else if completed { "✅" } else { "⚪" };

    rsx! {
        div {
            class: "qs-flow-node {state_cls}",
            onclick: move |e| onclick.call(e),
            if !icon.is_empty() {
                div { style: "width: 36px; height: 36px; display: flex; align-items: center; justify-content: center; border-radius: 8px; background: var(--qs-primary-selection); font-size: 16px; flex-shrink: 0;",
                    "{icon}"
                }
            }
            div { style: "flex: 1; min-width: 0;",
                div { style: "font-size: 13px; font-weight: 600; color: var(--qs-fg);", "{label}" }
                div { style: "font-size: 11px; color: var(--qs-fg-muted); margin-top: 2px;", "{sub_label}" }
            }
            span { style: "font-size: 12px; flex-shrink: 0;", "{status_indicator}" }
        }
    }
}

/// Flow edge connector line between nodes.
#[component]
pub fn FlowEdgeLine(
    #[props(default = false)] active: bool,
    #[props(default = false)] completed: bool,
) -> Element {
    let state_cls = if active { "active" } else if completed { "completed" } else { "" };
    rsx! {
        div { class: "qs-flow-edge-line {state_cls}" }
    }
}

/// Flow toolbar with simulation controls.
#[component]
pub fn FlowSimControls(
    #[props(default = "IDLE".to_string())] state: String,
    #[props(default)] on_play: EventHandler<MouseEvent>,
    #[props(default)] on_reset: EventHandler<MouseEvent>,
) -> Element {
    let (badge_cls, label) = match state.as_str() {
        "RUNNING" => ("qs-badge qs-badge-info", "⏵ Running"),
        "COMPLETED" => ("qs-badge qs-badge-success", "✓ Completed"),
        _ => ("qs-badge qs-badge-muted", "⏸ Idle"),
    };
    rsx! {
        div { style: "display: flex; align-items: center; gap: 8px;",
            span { class: "{badge_cls}", "{label}" }
            button {
                style: "padding: 4px 12px; font-size: 11px; font-weight: 600; border-radius: 6px; border: 1px solid var(--qs-border); background: var(--qs-btn-bg); color: var(--qs-btn-fg); cursor: pointer;",
                onclick: move |e| on_play.call(e),
                "▶ Play"
            }
            button {
                style: "padding: 4px 12px; font-size: 11px; font-weight: 600; border-radius: 6px; border: 1px solid var(--qs-border); background: var(--qs-btn-secondary-bg); color: var(--qs-btn-secondary-fg); cursor: pointer;",
                onclick: move |e| on_reset.call(e),
                "↺ Reset"
            }
        }
    }
}

/// Flow execution log panel.
#[component]
pub fn FlowLogPanel(
    #[props(default = true)] visible: bool,
    children: Element,
) -> Element {
    if !visible { return rsx! {} }
    rsx! {
        div { class: "qs-flow-log-panel",
            {children}
        }
    }
}

/// Single log entry.
#[component]
pub fn FlowLogEntry(
    timestamp: String,
    message: String,
) -> Element {
    rsx! {
        div { class: "qs-flow-log-entry",
            span { class: "timestamp", "[{timestamp}]" }
            span { "{message}" }
        }
    }
}

/// Status overlay card (top-left corner of flow canvas).
#[component]
pub fn FlowStatusCard(
    state: String,
    #[props(default)] active_node: String,
) -> Element {
    let (color, icon) = match state.as_str() {
        "RUNNING" => ("var(--qs-info)", "⏵"),
        "COMPLETED" => ("var(--qs-success)", "✓"),
        _ => ("var(--qs-fg-muted)", "⏸"),
    };
    rsx! {
        div { style: "position: absolute; top: 12px; left: 12px; z-index: 10; padding: 8px 14px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 8px; box-shadow: 0 2px 8px var(--qs-shadow);",
            div { style: "display: flex; align-items: center; gap: 6px;",
                span { style: "font-size: 12px; color: {color};", "{icon}" }
                span { style: "font-size: 11px; font-weight: 700; color: var(--qs-fg); text-transform: uppercase;", "System: {state}" }
            }
            if !active_node.is_empty() {
                div { style: "font-size: 10px; color: var(--qs-fg-muted); margin-top: 4px; font-family: var(--font-mono);",
                    "Active: {active_node}"
                }
            }
        }
    }
}
