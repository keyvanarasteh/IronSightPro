use dioxus::prelude::*;

// ───── Dialog ─────────────────────────────────────────
#[component]
pub fn QsDialog(
    #[props(default = false)] open: bool,
    #[props(default = "none".to_string())] dialog_type: String,
    #[props(default)] title: String,
    #[props(default)] message: String,
    #[props(default)] detail: String,
    #[props(default = false)] loading: bool,
    #[props(default)] onclose: EventHandler<()>,
    children: Element,
) -> Element {
    if !open { return rsx! {} }
    let type_icon = match dialog_type.as_str() {
        "info" => "ℹ️", "warning" => "⚠️", "error" => "❌", _ => "",
    };
    let type_color = match dialog_type.as_str() {
        "info" => "var(--qs-info)", "warning" => "var(--qs-warning)", "error" => "var(--qs-destructive)", _ => "var(--qs-fg)",
    };
    rsx! {
        div { style: "position: fixed; inset: 0; z-index: 9999; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.4);",
            onclick: move |_| onclose.call(()),
            div { style: "position: relative; display: flex; flex-direction: column; width: 100%; max-width: 450px; min-width: 280px; border-radius: 6px; box-shadow: 0 4px 16px var(--qs-shadow, rgba(0,0,0,0.3)); background: var(--qs-card); border: 1px solid var(--qs-border); color: var(--qs-fg); font-size: 13px;",
                onclick: move |e| e.stop_propagation(),
                if !title.is_empty() || dialog_type != "none" {
                    div { style: "display: flex; align-items: center; gap: 8px; padding: 16px 16px 8px;",
                        if !type_icon.is_empty() {
                            span { style: "flex-shrink: 0; font-size: 20px; color: {type_color};", "{type_icon}" }
                        }
                        if !title.is_empty() {
                            h2 { style: "margin: 0; font-size: 14px; font-weight: 600;", "{title}" }
                        }
                    }
                }
                div { style: "flex: 1; padding: 8px 16px;",
                    if !message.is_empty() { p { style: "margin: 0 0 8px; line-height: 1.5;", "{message}" } }
                    if !detail.is_empty() { p { style: "margin: 0 0 8px; font-size: 12px; color: var(--qs-fg-muted);", "{detail}" } }
                    if loading {
                        div { style: "height: 4px; width: 100%; background: var(--qs-muted); border-radius: 2px; overflow: hidden; margin-top: 16px;",
                            div { style: "height: 100%; width: 100%; background: var(--qs-primary); animation: pulse 1.5s ease infinite;" }
                        }
                    }
                    {children}
                }
            }
        }
    }
}

// ───── Modal ──────────────────────────────────────────
#[component]
pub fn QsModal(
    #[props(default = false)] open: bool,
    #[props(default)] title: String,
    #[props(default)] description: String,
    #[props(default = "md".to_string())] size: String,
    #[props(default = true)] closable: bool,
    #[props(default)] onclose: EventHandler<()>,
    children: Element,
) -> Element {
    if !open { return rsx! {} }
    let max_w = match size.as_str() {
        "sm" => "max-width: 384px;", "lg" => "max-width: 672px;",
        "xl" => "max-width: 896px;", "full" => "position: fixed; inset: 16px; max-width: none;",
        _ => "max-width: 512px;",
    };
    rsx! {
        div { style: "position: fixed; inset: 0; z-index: 200; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);",
            onclick: move |_| if closable { onclose.call(()) },
            div { style: "display: flex; flex-direction: column; overflow: hidden; border-radius: 16px; border: 1px solid var(--qs-border); background: var(--qs-card); color: var(--qs-fg); box-shadow: 0 25px 50px rgba(0,0,0,0.25); width: 100%; max-height: 90vh; margin: 0 16px; {max_w}",
                onclick: move |e| e.stop_propagation(),
                if !title.is_empty() || closable {
                    div { style: "display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid var(--qs-border-subtle); padding: 16px 24px; flex-shrink: 0;",
                        div {
                            if !title.is_empty() { h2 { style: "margin: 0; font-size: 16px; font-weight: 700;", "{title}" } }
                            if !description.is_empty() { p { style: "margin: 4px 0 0; font-size: 12px; color: var(--qs-fg-muted);", "{description}" } }
                        }
                        if closable {
                            button { style: "background: transparent; border: none; padding: 8px; border-radius: 8px; cursor: pointer; color: var(--qs-fg-muted); font-size: 16px;",
                                onclick: move |_| onclose.call(()),
                                "✕"
                            }
                        }
                    }
                }
                div { style: "flex: 1; overflow-y: auto; padding: 16px 24px;", {children} }
            }
        }
    }
}

// ───── Tooltip ────────────────────────────────────────
#[component]
pub fn QsTooltip(
    text: String,
    #[props(default = "top".to_string())] position: String,
    children: Element,
) -> Element {
    // CSS-only tooltip via ::after pseudo
    let pos_style = match position.as_str() {
        "bottom" => "top: 100%; left: 50%; transform: translateX(-50%); margin-top: 6px;",
        "left" => "right: 100%; top: 50%; transform: translateY(-50%); margin-right: 6px;",
        "right" => "left: 100%; top: 50%; transform: translateY(-50%); margin-left: 6px;",
        _ => "bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 6px;",
    };
    rsx! {
        div { class: "qs-tooltip-wrap", style: "position: relative; display: inline-flex;",
            {children}
            div { class: "qs-tooltip-tip", style: "position: absolute; {pos_style} pointer-events: none; opacity: 0; transition: opacity 0.15s; z-index: 10000; padding: 2px 6px; border-radius: 3px; font-size: 12px; line-height: 18px; white-space: nowrap; background: var(--qs-card); color: var(--qs-fg); border: 1px solid var(--qs-border); box-shadow: 0 2px 8px rgba(0,0,0,0.2);",
                "{text}"
            }
        }
    }
}

// ───── QuickAccess (Command Palette) ──────────────────
#[component]
pub fn QsQuickAccess(
    #[props(default = false)] visible: bool,
    #[props(default)] placeholder: String,
    #[props(default)] prefix: String,
    #[props(default)] onclose: EventHandler<()>,
    children: Element,
) -> Element {
    if !visible { return rsx! {} }
    rsx! {
        div { style: "position: fixed; inset: 0; z-index: 200;",
            onclick: move |_| onclose.call(()),
            div { style: "position: fixed; top: 0; left: 50%; z-index: 201; margin-top: 4px; width: 600px; max-width: 90vw; transform: translateX(-50%); overflow: hidden; border-radius: 8px; border: 1px solid var(--qs-border); background: var(--qs-card); box-shadow: 0 25px 50px rgba(0,0,0,0.25);",
                onclick: move |e| e.stop_propagation(),
                div { style: "display: flex; height: 34px; align-items: center; gap: 8px; border-bottom: 1px solid var(--qs-border); padding: 0 12px;",
                    span { style: "font-size: 14px; opacity: 0.5;", "🔍" }
                    if !prefix.is_empty() {
                        span { style: "font-size: 12px; opacity: 0.6;", "{prefix}" }
                    }
                    input { r#type: "text", placeholder: "{placeholder}", style: "flex: 1; min-width: 0; border: none; background: transparent; color: var(--qs-fg); font-size: 13px; outline: none; padding: 0;" }
                }
                div { style: "max-height: 300px; overflow-y: auto;", {children} }
            }
        }
    }
}

// ───── QuickAccessItem ────────────────────────────────
#[component]
pub fn QsQuickAccessItem(
    label: String,
    #[props(default)] description: String,
    #[props(default)] keybinding: String,
    #[props(default = false)] highlighted: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = if highlighted { "var(--qs-primary)" } else { "transparent" };
    let fg = if highlighted { "var(--qs-primary-fg, #fff)" } else { "var(--qs-fg)" };
    rsx! {
        button { style: "display: flex; width: 100%; align-items: center; gap: 8px; border: none; background: {bg}; color: {fg}; padding: 6px 12px; text-align: left; font-size: 12px; cursor: pointer;",
            onclick: move |e| onclick.call(e),
            div { style: "flex: 1; min-width: 0;",
                span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{label}" }
                if !description.is_empty() { span { style: "margin-left: 8px; opacity: 0.5;", "{description}" } }
            }
            if !keybinding.is_empty() {
                kbd { style: "flex-shrink: 0; font-family: var(--font-mono); font-size: 10px; opacity: 0.4;", "{keybinding}" }
            }
        }
    }
}

// ───── NotificationViewer ─────────────────────────────
#[component]
pub fn QsNotification(
    #[props(default = "info".to_string())] severity: String,
    #[props(default)] message: String,
    #[props(default)] detail: String,
    #[props(default)] source: String,
    #[props(default = None)] progress: Option<f64>,
    #[props(default)] ondismiss: EventHandler<()>,
) -> Element {
    let (icon, color) = match severity.as_str() {
        "warning" => ("⚠️", "#cca700"),
        "error" => ("❌", "#f14c4c"),
        _ => ("ℹ️", "var(--qs-info)"),
    };
    rsx! {
        div { style: "display: flex; flex-direction: column; border-bottom: 1px solid var(--qs-border); background: var(--qs-card); font-size: 12px;",
            if let Some(p) = progress {
                div { style: "height: 2px; background: var(--qs-muted);",
                    div { style: "height: 100%; width: {p.clamp(0.0,100.0)}%; background: var(--qs-primary); transition: width 0.3s;" }
                }
            }
            div { style: "display: flex; align-items: flex-start; gap: 8px; padding: 8px;",
                span { style: "flex-shrink: 0; margin-top: 2px; font-size: 14px; color: {color};", "{icon}" }
                div { style: "flex: 1; min-width: 0; line-height: 20px;",
                    if !source.is_empty() { span { style: "font-weight: 600;", "{source}: " } }
                    span { "{message}" }
                    if !detail.is_empty() { p { style: "margin: 4px 0 0; white-space: pre-wrap; opacity: 0.7;", "{detail}" } }
                }
                button { style: "flex-shrink: 0; padding: 2px; background: transparent; border: none; cursor: pointer; color: var(--qs-fg); opacity: 0.6; font-size: 14px;",
                    onclick: move |_| ondismiss.call(()),
                    "✕"
                }
            }
        }
    }
}

// ───── NotificationBadge ──────────────────────────────
#[component]
pub fn QsNotificationBadge(count: u32) -> Element {
    if count == 0 { return rsx! {} }
    rsx! {
        span { style: "display: inline-flex; align-items: center; justify-content: center; min-width: 18px; height: 18px; padding: 0 6px; border-radius: 9px; font-size: 10px; font-weight: 700; background: var(--qs-primary); color: var(--qs-primary-fg, #fff);",
            "{count}"
        }
    }
}

// ───── ContextView ────────────────────────────────────
#[component]
pub fn QsContextView(
    #[props(default = false)] open: bool,
    x: f64,
    y: f64,
    #[props(default)] onclose: EventHandler<()>,
    children: Element,
) -> Element {
    if !open { return rsx! {} }
    rsx! {
        div { style: "position: fixed; inset: 0; z-index: 9998;",
            onclick: move |_| onclose.call(()),
            div { style: "position: fixed; left: {x}px; top: {y}px; z-index: 9999; min-width: 200px; border-radius: 8px; border: 1px solid var(--qs-border-subtle); padding: 4px; background: var(--qs-card); box-shadow: 0 25px 50px rgba(0,0,0,0.25); backdrop-filter: blur(20px);",
                onclick: move |e| e.stop_propagation(),
                {children}
            }
        }
    }
}

// ───── ContextViewItem ────────────────────────────────
#[component]
pub fn QsContextItem(
    label: String,
    #[props(default)] icon: String,
    #[props(default)] shortcut: String,
    #[props(default = false)] danger: bool,
    #[props(default = false)] divider: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    if divider {
        return rsx! { div { style: "border-top: 1px solid var(--qs-border-subtle); margin: 3px 4px;" } }
    }
    let color = if danger { "var(--qs-destructive)" } else { "var(--qs-fg)" };
    rsx! {
        button { style: "display: flex; width: 100%; align-items: center; gap: 10px; border: none; background: transparent; padding: 5px 10px; border-radius: 6px; text-align: left; font-size: 13px; cursor: pointer; color: {color}; transition: background 0.05s;",
            onclick: move |e| onclick.call(e),
            if !icon.is_empty() { span { style: "font-size: 14px; opacity: 0.5; width: 14px;", "{icon}" } }
            span { style: "flex: 1;", "{label}" }
            if !shortcut.is_empty() { kbd { style: "font-family: var(--font-mono); font-size: 10px; opacity: 0.4;", "{shortcut}" } }
        }
    }
}
