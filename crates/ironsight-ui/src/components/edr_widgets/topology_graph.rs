use dioxus::prelude::*;

/// Static network topology graph with node circles and connection lines.
#[component]
pub fn TopologyGraph(
    #[props(default = "400px".to_string())] height: String,
    children: Element,
) -> Element {
    rsx! {
        div { style: "position: relative; width: 100%; height: {height}; background: var(--qs-bg-panel); border: 1px solid var(--qs-border); border-radius: 12px; overflow: hidden;",
            {children}
        }
    }
}

/// Network node in the topology graph.
#[component]
pub fn TopoNode(
    label: String,
    #[props(default = "process".to_string())] node_type: String,
    x: f64,
    y: f64,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let (color, icon) = match node_type.as_str() {
        "process" => ("#3b82f6", "🔵"),
        "local" | "listener" => ("#10b981", "🟢"),
        "remote" | "external" => ("#f97316", "🟠"),
        "suspicious" => ("#ef4444", "🔴"),
        _ => ("var(--qs-fg-muted)", "⚪"),
    };

    rsx! {
        div {
            style: "position: absolute; left: {x}%; top: {y}%; transform: translate(-50%, -50%); display: flex; flex-direction: column; align-items: center; gap: 4px; cursor: pointer; z-index: 2;",
            onclick: move |e| onclick.call(e),
            div { style: "width: 36px; height: 36px; border-radius: 50%; background: {color}20; border: 2px solid {color}; display: flex; align-items: center; justify-content: center; font-size: 14px; transition: transform 0.15s;",
                "{icon}"
            }
            span { style: "font-size: 9px; font-weight: 600; color: var(--qs-fg); font-family: var(--font-mono); background: var(--qs-card); padding: 1px 6px; border-radius: 4px; border: 1px solid var(--qs-border); white-space: nowrap; max-width: 100px; overflow: hidden; text-overflow: ellipsis;",
                "{label}"
            }
        }
    }
}

/// Connection line between two nodes (SVG overlay).
#[component]
pub fn TopoEdge(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    #[props(default = false)] suspicious: bool,
    #[props(default = 1.0)] thickness: f64,
) -> Element {
    let color = if suspicious { "#ef4444" } else { "var(--qs-flow-edge)" };
    rsx! {
        svg { style: "position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1;",
            line {
                x1: "{x1}%",
                y1: "{y1}%",
                x2: "{x2}%",
                y2: "{y2}%",
                stroke: "{color}",
                stroke_width: "{thickness}",
                stroke_opacity: "0.6",
            }
        }
    }
}
