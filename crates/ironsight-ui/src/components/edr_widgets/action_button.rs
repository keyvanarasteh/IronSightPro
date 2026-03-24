use dioxus::prelude::*;

/// Context menu action button with dropdown items.
#[component]
pub fn ActionButton(
    children: Element,
) -> Element {
    rsx! {
        div { style: "position: relative; display: inline-flex;",
            div { style: "display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; border: 1px solid var(--qs-border); background: var(--qs-card); color: var(--qs-fg); font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.15s;",
                "⚡ Actions ▾"
            }
            // Dropdown content via children
            {children}
        }
    }
}

/// Single action item in the dropdown.
#[component]
pub fn ActionItem(
    icon: String,
    label: String,
    #[props(default = false)] danger: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let color = if danger { "var(--qs-destructive)" } else { "var(--qs-fg)" };
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 6px 12px; cursor: pointer; transition: background 0.1s; border-radius: 4px; color: {color};",
            onclick: move |e| onclick.call(e),
            span { style: "font-size: 13px;", "{icon}" }
            span { style: "font-size: 12px; font-weight: 500;", "{label}" }
        }
    }
}

/// Quick action row for batch operations.
#[component]
pub fn BatchActionBar(
    selected_count: u32,
    children: Element,
) -> Element {
    if selected_count == 0 { return rsx! {} }
    rsx! {
        div { style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: var(--qs-primary-selection); border: 1px solid rgba(0,120,212,0.3); border-radius: 8px; margin-bottom: 12px;",
            span { style: "font-size: 12px; font-weight: 700; color: var(--qs-primary);",
                "{selected_count} selected"
            }
            div { style: "display: flex; gap: 6px;", {children} }
        }
    }
}
