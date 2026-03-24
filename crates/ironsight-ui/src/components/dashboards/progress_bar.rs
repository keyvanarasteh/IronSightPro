use dioxus::prelude::*;

/// A labeled progress/gauge bar with color gradient.
#[component]
pub fn DashProgress(
    label: String,
    value: f64,
    #[props(default)] value_text: String,
    #[props(default = "default".to_string())] variant: String,
    #[props(default = 100.0)] max: f64,
) -> Element {
    let pct = ((value / max) * 100.0).min(100.0).max(0.0);

    let bar_color = match variant.as_str() {
        "critical" | "danger" => "linear-gradient(90deg, var(--accent-red), var(--accent-orange))",
        "warning" => "linear-gradient(90deg, var(--accent-orange), var(--accent-yellow))",
        "success" => "linear-gradient(90deg, var(--accent-green), var(--accent-cyan))",
        "purple" => "linear-gradient(90deg, var(--accent-cyan), var(--accent-blue))",
        _ => "linear-gradient(90deg, var(--accent-blue), var(--accent-cyan))",
    };

    let display_val = if value_text.is_empty() {
        format!("{:.0}%", pct)
    } else {
        value_text.clone()
    };

    rsx! {
        div {
            style: "margin-bottom: 12px;",
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px;",
                span {
                    style: "font-size: 11px; color: var(--text-secondary); font-weight: 500;",
                    "{label}"
                }
                span {
                    style: "font-size: 11px; font-weight: 700; font-family: var(--font-mono); color: var(--text-primary);",
                    "{display_val}"
                }
            }
            div {
                style: "width: 100%; height: 6px; background: var(--bg-hover); border-radius: 3px; overflow: hidden;",
                div {
                    style: "width: {pct}%; height: 100%; background: {bar_color}; border-radius: 3px; transition: width 0.8s ease;",
                }
            }
        }
    }
}

/// A tiny inline score bar (used inside table cells).
#[component]
pub fn ScoreBar(
    score: f64,
    #[props(default = 100.0)] max: f64,
) -> Element {
    let pct = ((score / max) * 100.0).min(100.0).max(0.0);
    let color = if pct > 70.0 {
        "var(--accent-red)"
    } else if pct > 40.0 {
        "var(--accent-orange)"
    } else {
        "var(--accent-blue)"
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px;",
            div {
                style: "width: 48px; height: 4px; background: var(--bg-hover); border-radius: 2px; overflow: hidden;",
                div {
                    style: "width: {pct}%; height: 100%; background: {color}; border-radius: 2px;",
                }
            }
            span {
                style: "font-size: 10px; font-weight: 700; color: {color}; font-family: var(--font-mono);",
                "{score:.0}"
            }
        }
    }
}
