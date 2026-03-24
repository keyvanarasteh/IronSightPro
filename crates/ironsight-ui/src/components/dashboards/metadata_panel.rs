use dioxus::prelude::*;

/// A key-value metadata row.
#[component]
pub fn MetaRow(
    label: String,
    value: String,
    #[props(default = false)] highlight: bool,
) -> Element {
    let value_color = if highlight { "var(--accent-red)" } else { "var(--text-primary)" };
    let font_weight = if highlight { "700" } else { "400" };

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: flex-start; padding: 5px 0; border-bottom: 1px solid var(--border);",
            span { style: "font-size: 11px; color: var(--text-muted);", "{label}" }
            span {
                style: "font-family: var(--font-mono); font-size: 11px; color: {value_color}; text-align: right; max-width: 160px; overflow: hidden; text-overflow: ellipsis; word-break: break-all; font-weight: {font_weight};",
                "{value}"
            }
        }
    }
}

/// A titled section of key-value metadata.
#[component]
pub fn MetadataSection(title: String, children: Element) -> Element {
    rsx! {
        div {
            class: "meta-section",
            div {
                style: "font-size: 10px; font-weight: 600; text-transform: uppercase; letter-spacing: 1.5px; color: var(--text-muted); margin-bottom: 10px;",
                "{title}"
            }
            {children}
        }
    }
}

/// Process detail header with name and PID.
#[component]
pub fn DetailHeader(
    name: String,
    pid: String,
    #[props(default)] icon: String,
) -> Element {
    let display_icon = if icon.is_empty() {
        name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()
    } else {
        icon.clone()
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; margin-bottom: 14px;",
            div {
                style: "width: 36px; height: 36px; border-radius: 8px; background: var(--badge-info-bg); display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 700; color: var(--accent-blue); font-family: var(--font-mono); border: 1px solid var(--border); flex-shrink: 0;",
                "{display_icon}"
            }
            div {
                div { style: "font-weight: 700; font-size: 15px; color: var(--text-primary);", "{name}" }
                div { style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);", "PID {pid}" }
            }
        }
    }
}
