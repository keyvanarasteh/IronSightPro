use dioxus::prelude::*;

/// A forensic timeline step with numbered circle and status.
#[component]
pub fn TimelineStep(
    step: u32,
    label: String,
    status: String,
    #[props(default = "success".to_string())] variant: String,
) -> Element {
    let (bg, fg) = match variant.as_str() {
        "success" => ("var(--accent-green)", "var(--bg-primary)"),
        "danger" | "critical" => ("var(--accent-red)", "var(--bg-primary)"),
        "warning" => ("var(--accent-orange)", "var(--bg-primary)"),
        "pending" => ("var(--text-muted)", "var(--bg-primary)"),
        _ => ("var(--accent-blue)", "var(--bg-primary)"),
    };

    let status_color = match variant.as_str() {
        "success" => "var(--accent-green)",
        "danger" | "critical" => "var(--accent-red)",
        "warning" => "var(--accent-orange)",
        _ => "var(--text-muted)",
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; position: relative; z-index: 1;",
            div {
                style: "width: 28px; height: 28px; background: {bg}; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: 700; color: {fg}; border: 3px solid var(--bg-primary); flex-shrink: 0;",
                "{step}"
            }
            div {
                class: "timeline-step-body",
                style: "flex: 1; padding: 8px 12px;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span {
                        style: "font-size: 12px; font-weight: 700; color: var(--text-secondary);",
                        "{label}"
                    }
                    span {
                        style: "font-size: 10px; font-family: var(--font-mono); color: {status_color}; font-weight: 600;",
                        "{status}"
                    }
                }
            }
        }
    }
}

/// Container for timeline steps with connecting vertical line.
#[component]
pub fn TimelineContainer(children: Element) -> Element {
    rsx! {
        div {
            style: "position: relative; display: flex; flex-direction: column; gap: 8px;",
            div {
                class: "timeline-connector",
                style: "position: absolute; left: 14px; top: 0; bottom: 0; width: 2px; z-index: 0;",
            }
            {children}
        }
    }
}
