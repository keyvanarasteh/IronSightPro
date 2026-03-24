use dioxus::prelude::*;

/// Frosted-glass container panel with subtle border and hover glow.
#[component]
pub fn GlassPanel(
    #[props(default)] class: String,
    #[props(default)] style: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "glass-panel {class}",
            style: "{style}",
            {children}
        }
    }
}

/// Section header bar inside a glass panel.
#[component]
pub fn PanelHeader(
    title: String,
    #[props(default)] subtitle: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
            div {
                h3 {
                    style: "font-size: 13px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 1.5px; margin: 0;",
                    "{title}"
                }
                if !subtitle.is_empty() {
                    p {
                        style: "font-size: 11px; color: var(--text-muted); margin: 4px 0 0 0;",
                        "{subtitle}"
                    }
                }
            }
            {children}
        }
    }
}
