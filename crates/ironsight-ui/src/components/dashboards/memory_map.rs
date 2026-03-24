use dioxus::prelude::*;

/// A single memory region bar in the memory map visualizer.
#[component]
pub fn MemoryRegionBar(
    address: String,
    size: String,
    permissions: String,
    #[props(default = 0.0)] usage_pct: f64,
) -> Element {
    let bar_color = if permissions.contains("rwx") {
        "var(--accent-red)"
    } else if permissions.contains("rw") {
        "var(--accent-orange)"
    } else if permissions.contains("r-x") {
        "var(--accent-blue)"
    } else {
        "var(--accent-green)"
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; padding: 6px 0; border-bottom: 1px solid var(--border);",
            span {
                style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-muted); min-width: 110px;",
                "{address}"
            }
            div {
                style: "flex: 1; height: 8px; background: var(--bg-hover); border-radius: 4px; overflow: hidden;",
                div {
                    style: "width: {usage_pct}%; height: 100%; background: {bar_color}; border-radius: 4px; transition: width 0.5s ease;",
                }
            }
            span {
                style: "font-family: var(--font-mono); font-size: 10px; color: {bar_color}; font-weight: 700; min-width: 40px; text-align: center;",
                "{permissions}"
            }
            span {
                style: "font-family: var(--font-mono); font-size: 10px; color: var(--text-muted); min-width: 60px; text-align: right;",
                "{size}"
            }
        }
    }
}

/// Container for the memory map visualizer.
#[component]
pub fn MemoryMapPanel(children: Element) -> Element {
    rsx! {
        div {
            class: "memory-map-panel",
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                span {
                    style: "font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.2px;",
                    "🗺️ Memory Region Map"
                }
                span {
                    style: "font-size: 10px; color: var(--text-muted); font-family: var(--font-mono);",
                    "Virtual Address Space"
                }
            }
            {children}
        }
    }
}
