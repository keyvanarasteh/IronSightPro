use dioxus::prelude::*;

/// Styled prose container with VSCode markdown token colors.
/// Uses `.qs-prose` class for headings, links, inline code, etc.
#[component]
pub fn MarkdownView(
    #[props(default)] class: String,
    #[props(default = "20px 28px".to_string())] padding: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "qs-prose {class}",
            style: "padding: {padding};",
            {children}
        }
    }
}

/// Markdown-style heading (h1–h4).
#[component]
pub fn MdHeading(
    #[props(default = 2)] level: u8,
    children: Element,
) -> Element {
    match level {
        1 => rsx! { h1 { style: "color: var(--qs-heading); font-weight: 600; font-size: 1.7em; line-height: 1.3; margin: 1.4em 0 0.5em;", {children} } },
        2 => rsx! { h2 { style: "color: var(--qs-heading); font-weight: 600; font-size: 1.35em; line-height: 1.3; margin: 1.4em 0 0.5em; border-bottom: 1px solid var(--qs-hr); padding-bottom: 6px;", {children} } },
        3 => rsx! { h3 { style: "color: var(--qs-heading); font-weight: 600; font-size: 1.12em; line-height: 1.3; margin: 1.4em 0 0.5em;", {children} } },
        _ => rsx! { h4 { style: "color: var(--qs-heading); font-weight: 600; font-size: 1em; line-height: 1.3; margin: 1.4em 0 0.5em;", {children} } },
    }
}

/// Markdown blockquote.
#[component]
pub fn MdBlockquote(children: Element) -> Element {
    rsx! {
        blockquote {
            style: "margin: 12px 0; padding: 8px 16px; border-left: 3px solid var(--qs-heading); background: var(--qs-blockquote-bg); color: var(--qs-fg-muted); border-radius: 0 4px 4px 0;",
            {children}
        }
    }
}

/// Markdown table wrapper.
#[component]
pub fn MdTable(children: Element) -> Element {
    rsx! {
        div { style: "overflow-x: auto; margin: 16px 0;",
            table { class: "qs-param-table",
                {children}
            }
        }
    }
}
