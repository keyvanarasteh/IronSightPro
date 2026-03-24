use dioxus::prelude::*;
use ironsight_heuristic::ThreatLevel;

#[component]
pub fn ThreatGauge(score: f32) -> Element {
    let percentage = score.min(100.0).max(0.0);
    
    let color = if percentage < 30.0 {
        "var(--accent-green)"
    } else if percentage < 70.0 {
        "var(--accent-orange)"
    } else {
        "var(--accent-red)"
    };

    rsx! {
        div {
            class: "threat-gauge-wrapper",
            style: "width: 100%; height: 8px; background: var(--surface-bg); border-radius: 4px; overflow: hidden;",
            div {
                class: "threat-gauge-fill",
                style: "width: {percentage}%; height: 100%; background: {color}; transition: width 0.3s ease-in-out;"
            }
        }
    }
}

#[component]
pub fn SecurityBadge(level: ThreatLevel) -> Element {
    let (bg, txt, icon, text) = match level {
        ThreatLevel::Clean => ("rgba(0, 255, 0, 0.1)", "var(--accent-green)", "✅", "Clean"),
        ThreatLevel::Low => ("rgba(255, 255, 0, 0.1)", "var(--accent-yellow)", "⚠️", "Low"),
        ThreatLevel::Medium => ("rgba(255, 165, 0, 0.1)", "var(--accent-orange)", "☣️", "Medium"),
        ThreatLevel::High => ("rgba(255, 0, 0, 0.1)", "var(--accent-red)", "🚨", "High"),
        ThreatLevel::Critical => ("#ff0000", "#ffffff", "☠️", "Critical"),
    };

    rsx! {
        span {
            style: "display: inline-flex; align-items: center; gap: 4px; padding: 2px 8px; border-radius: 12px; font-size: 11px; font-weight: 600; background: {bg}; color: {txt}; text-transform: uppercase; letter-spacing: 0.5px;",
            "{icon} {text}"
        }
    }
}

#[component]
pub fn ActionConfirmModal(
    title: String, 
    description: String, 
    confirm_text: String, 
    confirm_action: EventHandler<MouseEvent>, 
    cancel_action: EventHandler<MouseEvent>
) -> Element {
    rsx! {
        div {
            class: "modal-backdrop",
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); backdrop-filter: blur(4px); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            div {
                class: "modal-panel",
                style: "background: var(--panel-bg); border: 1px solid var(--border-color); border-top: 2px solid var(--accent-red); border-radius: 8px; padding: 24px; max-width: 400px; width: 100%; box-shadow: 0 10px 30px rgba(0,0,0,0.5);",
                h3 {
                    style: "margin-top: 0; color: var(--text-main); font-size: 18px;",
                    "{title}"
                }
                p {
                    style: "color: var(--text-muted); font-size: 14px; margin-bottom: 24px; line-height: 1.5;",
                    "{description}"
                }
                div {
                    style: "display: flex; gap: 12px; justify-content: flex-end;",
                    button {
                        class: "btn btn-secondary",
                        style: "padding: 8px 16px; border-radius: 4px; background: transparent; border: 1px solid var(--border-color); color: var(--text-main); cursor: pointer;",
                        onclick: move |e| cancel_action.call(e),
                        "Cancel"
                    }
                    button {
                        class: "btn btn-danger",
                        style: "padding: 8px 16px; border-radius: 4px; background: rgba(255,0,0,0.2); border: 1px solid var(--accent-red); color: var(--accent-red); font-weight: 600; cursor: pointer;",
                        onclick: move |e| confirm_action.call(e),
                        "{confirm_text}"
                    }
                }
            }
        }
    }
}
