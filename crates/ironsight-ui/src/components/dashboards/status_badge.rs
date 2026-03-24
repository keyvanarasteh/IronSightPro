use dioxus::prelude::*;

/// Severity/status badge with color coding.
#[component]
pub fn StatusBadge(
    label: String,
    #[props(default = "info".to_string())] severity: String,
) -> Element {
    let (bg, fg) = match severity.as_str() {
        "critical" => ("rgba(239, 68, 68, 0.15)", "var(--accent-red)"),
        "high" | "danger" => ("rgba(249, 115, 22, 0.15)", "var(--accent-orange)"),
        "medium" | "warning" => ("rgba(250, 204, 21, 0.15)", "var(--accent-yellow)"),
        "low" | "success" => ("rgba(34, 197, 94, 0.15)", "var(--accent-green)"),
        "clean" => ("rgba(34, 197, 94, 0.1)", "var(--accent-green)"),
        "purple" => ("rgba(167, 139, 250, 0.15)", "var(--accent-cyan)"),
        _ => ("rgba(99, 102, 241, 0.1)", "var(--accent-blue)"),
    };

    rsx! {
        span {
            style: "display: inline-flex; align-items: center; gap: 4px; padding: 2px 8px; border-radius: 4px; font-size: 10px; font-weight: 700; background: {bg}; color: {fg}; border: 1px solid {fg}30; text-transform: uppercase; letter-spacing: 0.5px; white-space: nowrap;",
            "{label}"
        }
    }
}

/// Pulsing status dot indicator.
#[component]
pub fn StatusDot(
    #[props(default = "green".to_string())] color: String,
) -> Element {
    let dot_color = match color.as_str() {
        "red" => "var(--accent-red)",
        "orange" => "var(--accent-orange)",
        "yellow" => "var(--accent-yellow)",
        "blue" => "var(--accent-blue)",
        _ => "var(--accent-green)",
    };

    rsx! {
        span {
            style: "display: inline-block; width: 7px; height: 7px; border-radius: 50%; background: {dot_color}; box-shadow: 0 0 6px {dot_color}; animation: dash-pulse 2s ease-in-out infinite;",
        }
    }
}
