use dioxus::prelude::*;

// ───── TitleBar ───────────────────────────────────────
#[component]
pub fn QsTitleBar(
    #[props(default)] workspace_name: String,
    #[props(default)] on_toggle_sidebar: EventHandler<()>,
    #[props(default)] on_toggle_terminal: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        header { style: "display: flex; align-items: center; justify-content: space-between; height: 36px; padding: 0 12px; background: var(--qs-titlebar-bg, var(--qs-bg)); color: var(--qs-titlebar-fg, var(--qs-fg)); border-bottom: 1px solid var(--qs-border); z-index: 50; flex-shrink: 0;",
            // Left: menu controls
            div { style: "display: flex; align-items: center; gap: 12px;",
                button { style: "background: transparent; border: none; padding: 4px; cursor: pointer; color: inherit; border-radius: 4px; display: flex;",
                    onclick: move |_| on_toggle_sidebar.call(()),
                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        path { d: "M3 12h18M3 6h18M3 18h18" }
                    }
                }
                {children}
            }
            // Center: workspace title
            div { style: "flex: 1; text-align: center; font-size: 11px; opacity: 0.7; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin: 0 16px;",
                "{workspace_name}"
            }
            // Right: layout controls
            div { style: "display: flex; align-items: center; gap: 8px;",
                button { style: "background: transparent; border: none; padding: 2px; cursor: pointer; color: inherit; display: flex;",
                    onclick: move |_| on_toggle_terminal.call(()),
                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        path { d: "M4 20h16a2 2 0 002-2v-4H2v4a2 2 0 002 2zM2 10h20" }
                    }
                }
                // Window controls
                div { style: "display: flex; gap: 6px; margin-left: 8px;",
                    div { style: "width: 12px; height: 12px; border-radius: 50%; background: rgba(255,255,255,0.2); cursor: pointer;" }
                    div { style: "width: 12px; height: 12px; border-radius: 50%; background: rgba(255,255,255,0.2); cursor: pointer;" }
                    div { style: "width: 12px; height: 12px; border-radius: 50%; background: rgba(255,255,255,0.2); cursor: pointer;" }
                }
            }
        }
    }
}

// ───── StatusBar ──────────────────────────────────────
#[component]
pub fn QsStatusBar(children: Element) -> Element {
    rsx! {
        footer { style: "display: flex; align-items: center; justify-content: space-between; height: 24px; padding: 0 12px; background: var(--qs-statusbar-bg, var(--qs-primary)); color: var(--qs-statusbar-fg, var(--qs-primary-fg, #fff)); border-top: 1px solid var(--qs-border); font-size: 11px; flex-shrink: 0; overflow: hidden;",
            {children}
        }
    }
}

// ───── StatusItem ─────────────────────────────────────
#[component]
pub fn QsStatusItem(
    #[props(default)] icon: String,
    label: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button { style: "display: flex; align-items: center; gap: 4px; height: 100%; padding: 0 8px; background: transparent; border: none; color: inherit; font-size: 11px; cursor: default; white-space: nowrap; transition: background 0.1s;",
            onclick: move |e| onclick.call(e),
            if !icon.is_empty() { span { style: "font-size: 12px;", "{icon}" } }
            span { "{label}" }
        }
    }
}

// ───── EditorTabs ─────────────────────────────────────
#[component]
pub fn QsEditorTabs(children: Element) -> Element {
    rsx! {
        div { style: "display: flex; height: 36px; overflow-x: auto; background: var(--qs-muted); border-bottom: 1px solid var(--qs-border); flex-shrink: 0;",
            {children}
        }
    }
}

#[component]
pub fn QsEditorTab(
    label: String,
    #[props(default)] icon: String,
    #[props(default = false)] active: bool,
    #[props(default = true)] closable: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onclose: EventHandler<MouseEvent>,
) -> Element {
    let bg = if active { "var(--qs-bg)" } else { "var(--qs-muted)" };
    let fg = if active { "var(--qs-fg)" } else { "var(--qs-fg-muted)" };
    rsx! {
        button { style: "position: relative; display: flex; align-items: center; min-width: 120px; flex-shrink: 0; padding: 0 12px; gap: 6px; border: none; border-right: 1px solid var(--qs-border); background: {bg}; color: {fg}; cursor: pointer; font-size: 12px; text-align: left;",
            onclick: move |e| onclick.call(e),
            if active { div { style: "position: absolute; top: 0; left: 0; right: 0; height: 1px; background: var(--qs-primary);" } }
            if !icon.is_empty() { span { style: "font-size: 14px;", "{icon}" } }
            span { style: "flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{label}" }
            if closable {
                div { style: "display: flex; align-items: center; padding: 2px; border-radius: 4px; margin-left: 4px;",
                    onclick: move |e| { e.stop_propagation(); onclose.call(e); },
                    "✕"
                }
            }
        }
    }
}

// ───── Terminal ───────────────────────────────────────
#[component]
pub fn QsTerminal(
    #[props(default = false)] is_open: bool,
    #[props(default)] onclose: EventHandler<()>,
    children: Element,
) -> Element {
    let height = if is_open { "height: 25%; min-height: 160px;" } else { "height: 0;" };
    rsx! {
        section { style: "display: flex; flex-direction: column; overflow: hidden; border-top: 1px solid var(--qs-border); background: var(--qs-bg); transition: height 0.3s; {height}",
            div { style: "display: flex; align-items: center; justify-content: space-between; height: 36px; padding: 0 16px; flex-shrink: 0;",
                span { style: "font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em; color: var(--qs-fg-muted);", "Terminal" }
                button { style: "background: transparent; border: none; padding: 2px; cursor: pointer; color: var(--qs-fg); opacity: 0.5;",
                    onclick: move |_| onclose.call(()),
                    "✕"
                }
            }
            div { style: "flex: 1; overflow: auto; padding: 12px; font-family: var(--font-mono); font-size: 12px; color: var(--qs-fg); background: rgba(0,0,0,0.2);",
                {children}
            }
        }
    }
}

// ───── TerminalLine ──────────────────────────────────
#[component]
pub fn QsTerminalLine(
    text: String,
    #[props(default = "output".to_string())] line_type: String,
) -> Element {
    match line_type.as_str() {
        "prompt" => rsx! {
            div { style: "display: flex; gap: 8px;",
                span { style: "color: #10b981;", "➜" }
                span { style: "color: #3b82f6;", "ironsight" }
                span { style: "color: #f59e0b;", "git:(main)" }
                span { "{text}" }
            }
        },
        "success" => rsx! { div { style: "margin-top: 4px; color: #10b981;", "{text}" } },
        "error" => rsx! { div { style: "margin-top: 4px; color: var(--qs-destructive);", "{text}" } },
        _ => rsx! { div { style: "margin-top: 4px; color: var(--qs-fg-muted);", "{text}" } },
    }
}

// ───── PanelPart ─────────────────────────────────────
#[component]
pub fn QsPanelPart(
    #[props(default = false)] open: bool,
    children: Element,
) -> Element {
    let height = if open { "height: 25%; min-height: 160px;" } else { "height: 0;" };
    rsx! {
        div { style: "display: flex; flex-direction: column; overflow: hidden; border-top: 1px solid var(--qs-border); transition: height 0.3s; {height}",
            {children}
        }
    }
}

// ───── PanelTab ──────────────────────────────────────
#[component]
pub fn QsPanelTab(
    label: String,
    #[props(default = None)] count: Option<u32>,
    #[props(default = false)] active: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let border_bottom = if active { "border-bottom: 1px solid var(--qs-primary);" } else { "" };
    let fg = if active { "var(--qs-fg)" } else { "var(--qs-fg-muted)" };
    rsx! {
        button { style: "display: flex; align-items: center; gap: 6px; padding: 0 12px; height: 100%; background: transparent; border: none; font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em; color: {fg}; cursor: pointer; {border_bottom}",
            onclick: move |e| onclick.call(e),
            "{label}"
            if let Some(c) = count {
                span { style: "font-size: 10px; font-weight: 600; padding: 1px 6px; border-radius: 10px; background: var(--qs-muted); color: var(--qs-fg-muted);", "{c}" }
            }
        }
    }
}

// ───── SidebarPart ───────────────────────────────────
#[component]
pub fn QsSidebarPart(
    #[props(default = true)] open: bool,
    #[props(default = "left".to_string())] position: String,
    children: Element,
) -> Element {
    let width = if open { "width: 260px;" } else { "width: 0;" };
    let border = if position == "right" { "border-left: 1px solid var(--qs-border);" } else { "border-right: 1px solid var(--qs-border);" };
    rsx! {
        aside { style: "display: flex; flex-direction: column; flex-shrink: 0; overflow: hidden; background: var(--qs-bg); transition: width 0.2s; {width} {border}",
            {children}
        }
    }
}

// ───── SidebarSection ────────────────────────────────
#[component]
pub fn QsSidebarSection(
    title: String,
    #[props(default = true)] expanded: bool,
    children: Element,
) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column;",
            div { style: "display: flex; align-items: center; height: 22px; padding: 0 8px; font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em; color: var(--qs-fg-muted); cursor: pointer; user-select: none;",
                span { style: "margin-right: 4px; font-size: 10px;", if expanded { "▾" } else { "▸" } }
                "{title}"
            }
            if expanded {
                div { {children} }
            }
        }
    }
}

// ───── ActivityBar ───────────────────────────────────
#[component]
pub fn QsActivityBar(children: Element) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; width: 48px; background: var(--qs-bg-elevated, var(--qs-bg)); border-right: 1px solid var(--qs-border); padding: 4px 0; flex-shrink: 0;",
            {children}
        }
    }
}

// ───── ActivityIcon ──────────────────────────────────
#[component]
pub fn QsActivityIcon(
    icon: String,
    #[props(default = false)] active: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = if active { "var(--qs-primary-selection)" } else { "transparent" };
    let border = if active { "border-left: 2px solid var(--qs-primary);" } else { "border-left: 2px solid transparent;" };
    rsx! {
        button { style: "display: flex; align-items: center; justify-content: center; width: 48px; height: 48px; background: {bg}; border: none; padding: 0; cursor: pointer; color: var(--qs-fg); transition: background 0.1s; {border}",
            onclick: move |e| onclick.call(e),
            span { style: "font-size: 22px; opacity: 0.8;", "{icon}" }
        }
    }
}

// ───── Breadcrumbs ───────────────────────────────────
#[component]
pub fn QsBreadcrumbs(children: Element) -> Element {
    rsx! {
        nav { style: "display: flex; align-items: center; gap: 4px; padding: 4px 12px; font-size: 12px; color: var(--qs-fg-muted); overflow: hidden; flex-shrink: 0;",
            {children}
        }
    }
}

#[component]
pub fn QsBreadcrumbItem(
    label: String,
    #[props(default = false)] active: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let fg = if active { "var(--qs-fg)" } else { "var(--qs-fg-muted)" };
    rsx! {
        span { style: "display: flex; align-items: center; gap: 4px;",
            button { style: "background: transparent; border: none; padding: 2px 4px; border-radius: 3px; cursor: pointer; font-size: 12px; color: {fg};",
                onclick: move |e| onclick.call(e),
                "{label}"
            }
            span { style: "font-size: 10px; opacity: 0.4;", "›" }
        }
    }
}

// ───── BannerPart ────────────────────────────────────
#[component]
pub fn QsBanner(
    message: String,
    #[props(default = "info".to_string())] severity: String,
    #[props(default)] ondismiss: EventHandler<()>,
) -> Element {
    let (bg, icon) = match severity.as_str() {
        "warning" => ("var(--qs-warning)", "⚠️"),
        "error" => ("var(--qs-destructive)", "❌"),
        _ => ("var(--qs-info)", "ℹ️"),
    };
    rsx! {
        div { style: "display: flex; align-items: center; gap: 8px; padding: 6px 16px; background: {bg}; color: #fff; font-size: 12px; flex-shrink: 0;",
            span { "{icon}" }
            span { style: "flex: 1;", "{message}" }
            button { style: "background: transparent; border: none; padding: 2px; cursor: pointer; color: #fff; font-size: 14px;",
                onclick: move |_| ondismiss.call(()),
                "✕"
            }
        }
    }
}

// ───── EditorPlaceholder ─────────────────────────────
#[component]
pub fn QsEditorPlaceholder(
    #[props(default = "No file open".to_string())] message: String,
    #[props(default)] subtitle: String,
) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; flex: 1; gap: 12px; color: var(--qs-fg-muted); opacity: 0.5;",
            span { style: "font-size: 48px;", "📄" }
            span { style: "font-size: 14px;", "{message}" }
            if !subtitle.is_empty() { span { style: "font-size: 12px; opacity: 0.6;", "{subtitle}" } }
        }
    }
}

// ───── CommandCenter ─────────────────────────────────
#[component]
pub fn QsCommandCenter(
    #[props(default)] placeholder: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button { style: "display: flex; align-items: center; justify-content: center; gap: 8px; height: 24px; padding: 0 12px; border-radius: 4px; border: 1px solid var(--qs-border); background: rgba(255,255,255,0.05); color: inherit; font-size: 11px; cursor: pointer; opacity: 0.6; transition: opacity 0.1s; width: 100%; max-width: 320px;",
            onclick: move |e| onclick.call(e),
            "🔍"
            span { "{placeholder}" }
        }
    }
}
