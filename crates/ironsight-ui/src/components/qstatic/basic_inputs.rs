use dioxus::prelude::*;

// ───── Button ─────────────────────────────────────────
#[component]
pub fn QsButton(
    children: Element,
    #[props(default = "primary".to_string())] variant: String,
    #[props(default = false)] block: bool,
    #[props(default = false)] disabled: bool,
    #[props(default)] icon: String,
    #[props(default = false)] icon_only: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let (bg, fg, hover) = match variant.as_str() {
        "secondary" => ("var(--qs-button-secondary-bg, var(--qs-muted))", "var(--qs-button-secondary-fg, var(--qs-fg))", "var(--qs-button-secondary-hover, var(--qs-muted))"),
        "danger" => ("var(--qs-destructive)", "var(--qs-destructive-fg, #fff)", "var(--qs-destructive)"),
        "ghost" => ("transparent", "var(--qs-fg)", "var(--qs-muted)"),
        _ => ("var(--qs-primary)", "var(--qs-primary-fg, #fff)", "var(--qs-primary)"),
    };
    let width = if block { "width: 100%;" } else { "" };
    let opacity = if disabled { "opacity: 0.4; pointer-events: none;" } else { "" };
    let pad = if icon_only { "padding: 2px 5px;" } else { "padding: 1px 13px;" };

    rsx! {
        button {
            style: "display: inline-flex; align-items: center; justify-content: center; gap: 4px; border: 1px solid var(--qs-border); border-radius: 2px; font-family: var(--font-sans); font-size: 13px; line-height: 22px; cursor: pointer; transition: background 0.1s; background: {bg}; color: {fg}; {width} {opacity} {pad} white-space: nowrap; user-select: none;",
            disabled: disabled,
            onclick: move |e| onclick.call(e),
            if !icon.is_empty() {
                span { style: "display: flex; flex-shrink: 0; font-size: 14px;", "{icon}" }
            }
            {children}
        }
    }
}

// ───── Badge ──────────────────────────────────────────
#[component]
pub fn QsBadge(
    children: Element,
    #[props(default = "default".to_string())] variant: String,
    #[props(default = None)] count: Option<u32>,
    #[props(default = 99)] max_count: u32,
) -> Element {
    let (bg, fg) = match variant.as_str() {
        "success" => ("rgba(16,185,129,0.1)", "#10b981"),
        "warning" => ("rgba(245,158,11,0.1)", "#f59e0b"),
        "danger" => ("rgba(239,68,68,0.1)", "#ef4444"),
        "info" => ("rgba(59,130,246,0.1)", "#3b82f6"),
        "counter" => ("var(--qs-badge-bg, var(--qs-primary))", "var(--qs-badge-fg, #fff)"),
        _ => ("var(--qs-badge-bg, var(--qs-primary))", "var(--qs-badge-fg, #fff)"),
    };
    let border = match variant.as_str() {
        "success" | "warning" | "danger" | "info" => format!("border: 1px solid {fg}30;"),
        _ => "border: 1px solid transparent;".to_string(),
    };
    let display = if let Some(c) = count {
        if c > max_count { format!("{}+", max_count) } else { c.to_string() }
    } else {
        String::new()
    };

    rsx! {
        span { style: "display: inline-block; box-sizing: border-box; border-radius: 2px; font-size: 10px; font-weight: 700; text-transform: uppercase; padding: 2px 6px; text-align: center; white-space: nowrap; background: {bg}; color: {fg}; {border}",
            if count.is_some() {
                "{display}"
            } else {
                {children}
            }
        }
    }
}

// ───── TextField ──────────────────────────────────────
#[component]
pub fn QsTextField(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] readonly: bool,
    #[props(default = "none".to_string())] validation: String,
    #[props(default)] validation_message: String,
    #[props(default)] oninput: EventHandler<FormEvent>,
) -> Element {
    let border_color = match validation.as_str() {
        "info" => "var(--qs-info)",
        "warning" => "var(--qs-warning)",
        "error" => "var(--qs-destructive)",
        _ => "var(--qs-input-border, var(--qs-border))",
    };
    let opacity = if disabled { "opacity: 0.5; cursor: not-allowed;" } else { "" };

    rsx! {
        div { style: "display: flex; flex-direction: column;",
            input {
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: disabled,
                readonly: readonly,
                oninput: move |e| oninput.call(e),
                style: "box-sizing: border-box; width: 100%; border-radius: 2px; border: 1px solid {border_color}; padding: 3px 4px; background: var(--qs-input-bg, var(--qs-bg)); color: var(--qs-input-fg, var(--qs-fg)); font-family: var(--font-sans); font-size: 13px; line-height: 20px; outline: none; {opacity}",
            }
            if validation != "none" && !validation_message.is_empty() {
                div { style: "margin-top: 4px; display: flex; align-items: flex-start; gap: 4px; font-size: 11px; line-height: 14px; color: {border_color};",
                    span { style: "flex-shrink: 0; font-size: 12px;",
                        {match validation.as_str() { "info" => "ℹ", "warning" => "⚠", "error" => "✕", _ => "" }}
                    }
                    span { "{validation_message}" }
                }
            }
        }
    }
}

// ───── Textarea ───────────────────────────────────────
#[component]
pub fn QsTextarea(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default = false)] disabled: bool,
    #[props(default = 4)] rows: u32,
    #[props(default)] oninput: EventHandler<FormEvent>,
) -> Element {
    let opacity = if disabled { "opacity: 0.5; cursor: not-allowed;" } else { "" };
    rsx! {
        textarea {
            value: "{value}",
            placeholder: "{placeholder}",
            disabled: disabled,
            rows: "{rows}",
            oninput: move |e| oninput.call(e),
            style: "box-sizing: border-box; width: 100%; border-radius: 2px; border: 1px solid var(--qs-input-border, var(--qs-border)); padding: 3px 6px; background: var(--qs-input-bg, var(--qs-bg)); color: var(--qs-input-fg, var(--qs-fg)); font-family: var(--font-sans); font-size: 13px; line-height: 20px; outline: none; resize: vertical; {opacity}",
        }
    }
}

// ───── Checkbox ───────────────────────────────────────
#[component]
pub fn QsCheckbox(
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] toggle: bool,
    #[props(default)] label: String,
    #[props(default)] onchange: EventHandler<MouseEvent>,
) -> Element {
    if toggle {
        let (bg, dot_pos) = if checked {
            ("var(--qs-primary)", "translateX(16px)")
        } else {
            ("var(--qs-muted)", "translateX(0)")
        };
        let dis_style = if disabled { "opacity: 0.4; pointer-events: none;" } else { "" };
        rsx! {
            div { style: "display: inline-flex; align-items: center; gap: 8px; cursor: pointer; {dis_style}",
                onclick: move |e| onchange.call(e),
                div { style: "position: relative; width: 36px; height: 20px; border-radius: 10px; background: {bg}; transition: background 0.2s;",
                    div { style: "position: absolute; top: 2px; left: 2px; width: 16px; height: 16px; border-radius: 50%; background: white; transform: {dot_pos}; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.3);" }
                }
                if !label.is_empty() { span { style: "font-size: 13px; user-select: none;", "{label}" } }
            }
        }
    } else {
        let check_bg = if checked { "var(--qs-primary)" } else { "var(--qs-bg)" };
        let check_border = if checked { "var(--qs-primary)" } else { "var(--qs-border)" };
        let dis_style = if disabled { "opacity: 0.4; pointer-events: none;" } else { "" };
        rsx! {
            div { style: "display: inline-flex; align-items: center; gap: 8px; cursor: pointer; {dis_style}",
                onclick: move |e| onchange.call(e),
                div { style: "width: 16px; height: 16px; border-radius: 3px; border: 1px solid {check_border}; background: {check_bg}; display: flex; align-items: center; justify-content: center; transition: all 0.1s;",
                    if checked {
                        svg { width: "14", height: "14", view_box: "0 0 16 16", fill: "currentColor",
                            path { d: "M14.431 3.323l-8.47 10-.79-.036-3.35-4.77.818-.574 2.978 4.24 8.051-9.506.764.646z" }
                        }
                    }
                }
                if !label.is_empty() { span { style: "font-size: 13px; user-select: none;", "{label}" } }
            }
        }
    }
}

// ───── Radio ──────────────────────────────────────────
#[component]
pub fn QsRadio(
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default)] label: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let dot_scale = if checked { "scale(1)" } else { "scale(0)" };
    let dis_style = if disabled { "opacity: 0.4; pointer-events: none;" } else { "" };
    rsx! {
        div { style: "display: inline-flex; align-items: center; gap: 8px; cursor: pointer; {dis_style}",
            onclick: move |e| onclick.call(e),
            div { style: "width: 16px; height: 16px; border-radius: 50%; border: 1px solid var(--qs-border); background: var(--qs-bg); display: flex; align-items: center; justify-content: center;",
                span { style: "width: 8px; height: 8px; border-radius: 50%; background: var(--qs-fg); transform: {dot_scale}; transition: transform 0.1s;" }
            }
            if !label.is_empty() { span { style: "font-size: 13px; user-select: none;", "{label}" } }
        }
    }
}

// ───── Select ─────────────────────────────────────────
#[component]
pub fn QsSelect(
    children: Element,
    #[props(default)] value: String,
    #[props(default = false)] disabled: bool,
    #[props(default)] placeholder: String,
) -> Element {
    let display = if value.is_empty() { &placeholder } else { &value };
    let opacity = if disabled { "opacity: 0.5; cursor: not-allowed;" } else { "" };
    rsx! {
        div { style: "position: relative; display: inline-block; width: 100%; max-width: 320px; font-size: 13px;",
            button { style: "box-sizing: border-box; display: flex; width: 100%; align-items: center; justify-content: space-between; border-radius: 4px; border: 1px solid var(--qs-border); padding: 3px 4px; background: var(--qs-bg-elevated, var(--qs-card)); color: var(--qs-fg); text-align: left; cursor: pointer; {opacity}",
                disabled: disabled,
                span { style: "flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{display}" }
                svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none", style: "flex-shrink: 0;",
                    path { d: "M4.2 6.1L8 10L11.8 6.1L11 5.4L8 8.6L5 5.4L4.2 6.1Z", fill: "currentColor" }
                }
            }
            // Dropdown slots via children
            {children}
        }
    }
}

// ───── SelectOption ───────────────────────────────────
#[component]
pub fn QsSelectOption(
    label: String,
    value: String,
    #[props(default = false)] selected: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg = if selected { "var(--qs-primary)" } else { "transparent" };
    let fg = if selected { "var(--qs-primary-fg, #fff)" } else { "var(--qs-fg)" };
    rsx! {
        div {
            style: "display: flex; align-items: center; padding: 4px 8px; cursor: pointer; font-size: 13px; background: {bg}; color: {fg}; transition: background 0.1s;",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

// ───── ProgressBar ────────────────────────────────────
#[component]
pub fn QsProgressBar(
    #[props(default = None)] value: Option<f64>,
    #[props(default = 100.0)] max: f64,
    #[props(default = false)] indeterminate: bool,
) -> Element {
    let pct = if !indeterminate {
        if let Some(v) = value { (v / max * 100.0).clamp(0.0, 100.0) } else { 0.0 }
    } else { 0.0 };

    rsx! {
        div { style: "position: relative; display: block; height: 2px; width: 100%; overflow: hidden;",
            div { style: "position: absolute; inset: 0; background: transparent;" }
            if indeterminate {
                div { style: "position: absolute; top: 0; bottom: 0; left: 0; width: 2%; background: var(--qs-primary); animation: qs-progress 4s linear infinite;" }
            } else {
                div { style: "position: absolute; top: 0; bottom: 0; left: 0; width: {pct}%; background: var(--qs-primary); transition: width 0.1s ease;" }
            }
        }
    }
}

// ───── ProgressRing ───────────────────────────────────
#[component]
pub fn QsProgressRing(
    #[props(default = None)] value: Option<f64>,
    #[props(default = 32)] size: u32,
) -> Element {
    let r = (size as f64 / 2.0) - 3.0;
    let circumference = 2.0 * std::f64::consts::PI * r;
    let offset = if let Some(v) = value {
        circumference * (1.0 - (v / 100.0).clamp(0.0, 1.0))
    } else { circumference * 0.75 };
    let center = size as f64 / 2.0;
    let anim = if value.is_none() { "animation: spin 1s linear infinite;" } else { "" };

    rsx! {
        svg { width: "{size}", height: "{size}", view_box: "0 0 {size} {size}", style: "{anim}",
            circle { cx: "{center}", cy: "{center}", r: "{r}", fill: "none", stroke: "var(--qs-muted)", stroke_width: "2" }
            circle { cx: "{center}", cy: "{center}", r: "{r}", fill: "none", stroke: "var(--qs-primary)", stroke_width: "2",
                stroke_dasharray: "{circumference}", stroke_dashoffset: "{offset}", stroke_linecap: "round",
                transform: "rotate(-90 {center} {center})",
            }
        }
    }
}

// ───── Icon ───────────────────────────────────────────
#[component]
pub fn QsIcon(
    name: String,
    #[props(default = 16)] size: u32,
    #[props(default = false)] spin: bool,
) -> Element {
    let anim = if spin { "animation: spin 1s linear infinite;" } else { "" };
    rsx! {
        span { style: "display: inline-flex; align-items: center; justify-content: center; width: {size}px; height: {size}px; font-size: {size}px; {anim}", "{name}" }
    }
}

// ───── ButtonGroup ────────────────────────────────────
#[component]
pub fn QsButtonGroup(children: Element) -> Element {
    rsx! {
        div { style: "display: inline-flex; align-items: stretch;", {children} }
    }
}
