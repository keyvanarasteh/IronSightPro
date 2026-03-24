use dioxus::prelude::*;

/// VSCode-style code block with language header and body.
#[component]
pub fn VscCodeBlock(
    code: String,
    #[props(default = "plaintext".to_string())] lang: String,
    #[props(default = false)] show_line_numbers: bool,
    #[props(default)] class: String,
) -> Element {
    let lines: Vec<&str> = code.lines().collect();

    rsx! {
        div { class: "qs-code-block {class}",
            div { class: "qs-code-header",
                span { class: "qs-lang-label", "{lang}" }
                span { style: "font-size: 10px; color: var(--qs-fg-subtle); font-family: var(--font-mono);",
                    "{lines.len()} lines"
                }
            }
            div { class: "qs-code-body",
                pre {
                    code {
                        for (i, line) in lines.iter().enumerate() {
                            if show_line_numbers {
                                span {
                                    style: "display: inline-block; width: 2rem; text-align: right; margin-right: 1.5rem; color: var(--qs-fg-subtle); opacity: 0.5; user-select: none;",
                                    "{i+1}"
                                }
                            }
                            span { "{line}" }
                            br {}
                        }
                    }
                }
            }
        }
    }
}

/// Inline code span styled like VSCode.
#[component]
pub fn VscInlineCode(children: Element) -> Element {
    rsx! {
        code { class: "qs-inline-code", {children} }
    }
}
