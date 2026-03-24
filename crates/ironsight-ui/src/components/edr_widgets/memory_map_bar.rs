use dioxus::prelude::*;

/// Horizontal memory region bar visualization.
#[component]
pub fn MemoryMapBar(
    children: Element,
) -> Element {
    rsx! {
        div { style: "display: flex; align-items: stretch; width: 100%; height: 48px; border-radius: 8px; overflow: hidden; border: 1px solid var(--qs-border); background: var(--qs-bg);",
            {children}
        }
    }
}

/// Single memory region block within the bar.
#[component]
pub fn MemoryRegion(
    name: String,
    permissions: String,
    #[props(default = 1.0)] size_ratio: f64,
    #[props(default = false)] violation: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let (bg, fg) = if violation {
        ("#ef4444", "#fff")
    } else {
        match permissions.as_str() {
            "r--" | "r-p" => ("#3b82f6", "#fff"),
            "rw-" | "rw-p" => ("#10b981", "#fff"),
            "r-x" | "r-xp" => ("#f59e0b", "#000"),
            "rwx" | "rwxp" => ("#ef4444", "#fff"),
            _ => ("var(--qs-muted)", "var(--qs-fg)"),
        }
    };

    rsx! {
        div {
            style: "flex: {size_ratio}; display: flex; flex-direction: column; align-items: center; justify-content: center; background: {bg}; color: {fg}; font-size: 9px; font-family: var(--font-mono); cursor: pointer; transition: opacity 0.15s; min-width: 24px; position: relative; gap: 1px;",
            onclick: move |e| onclick.call(e),
            span { style: "font-weight: 700; font-size: 10px; line-height: 1;", "{name}" }
            span { style: "opacity: 0.8; font-size: 8px;", "{permissions}" }
            if violation {
                span { style: "position: absolute; top: 2px; right: 4px; font-size: 10px;", "⚠️" }
            }
        }
    }
}
