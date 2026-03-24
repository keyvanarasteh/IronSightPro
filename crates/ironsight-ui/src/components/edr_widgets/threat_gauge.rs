use dioxus::prelude::*;

/// Circular SVG threat gauge (0–100) with color zones.
#[component]
pub fn ThreatGauge(
    score: u32,
    #[props(default = "md".to_string())] size: String,
) -> Element {
    let clamped = score.min(100);
    let (color, level) = match clamped {
        0..=30 => ("#10b981", "CLEAN"),
        31..=50 => ("#3b82f6", "LOW"),
        51..=70 => ("#f59e0b", "MEDIUM"),
        71..=85 => ("#f97316", "HIGH"),
        _ => ("#ef4444", "CRITICAL"),
    };

    let dim = match size.as_str() {
        "sm" => 120,
        "lg" => 240,
        _ => 180,
    };
    let r = (dim as f64 / 2.0) - 16.0;
    let circumference = 2.0 * std::f64::consts::PI * r;
    let progress = circumference * (1.0 - clamped as f64 / 100.0);
    let center = dim as f64 / 2.0;
    let font_score = dim / 4;
    let font_label = dim / 12;

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
            svg {
                width: "{dim}",
                height: "{dim}",
                view_box: "0 0 {dim} {dim}",
                // Background circle
                circle {
                    cx: "{center}",
                    cy: "{center}",
                    r: "{r}",
                    fill: "none",
                    stroke: "var(--qs-border)",
                    stroke_width: "10",
                }
                // Progress arc
                circle {
                    cx: "{center}",
                    cy: "{center}",
                    r: "{r}",
                    fill: "none",
                    stroke: "{color}",
                    stroke_width: "10",
                    stroke_dasharray: "{circumference}",
                    stroke_dashoffset: "{progress}",
                    stroke_linecap: "round",
                    transform: "rotate(-90 {center} {center})",
                    style: "transition: stroke-dashoffset 0.8s ease;",
                }
                // Score text
                text {
                    x: "{center}",
                    y: "{center}",
                    text_anchor: "middle",
                    dominant_baseline: "central",
                    fill: "{color}",
                    font_size: "{font_score}",
                    font_weight: "700",
                    font_family: "var(--font-mono)",
                    "{clamped}"
                }
                // Level label
                text {
                    x: "{center}",
                    y: "{center + font_score as f64 * 0.7}",
                    text_anchor: "middle",
                    dominant_baseline: "central",
                    fill: "var(--qs-fg-muted)",
                    font_size: "{font_label}",
                    font_weight: "600",
                    letter_spacing: "0.1em",
                    "{level}"
                }
            }
        }
    }
}

/// Threat level badge (colored pill).
#[component]
pub fn ThreatBadge(level: String) -> Element {
    let (color, bg) = match level.to_lowercase().as_str() {
        "clean" => ("var(--qs-success)", "var(--qs-success-bg)"),
        "low" => ("var(--qs-info)", "var(--qs-info-bg)"),
        "medium" => ("var(--qs-warning)", "var(--qs-warning-bg)"),
        "high" => ("#f97316", "rgba(249,115,22,0.15)"),
        "critical" => ("var(--qs-destructive)", "var(--qs-destructive-bg)"),
        _ => ("var(--qs-fg-muted)", "var(--qs-muted)"),
    };
    rsx! {
        span { style: "display: inline-flex; align-items: center; gap: 4px; padding: 2px 10px; border-radius: 20px; font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: {color}; background: {bg};",
            "{level}"
        }
    }
}

/// EDR StatCard with icon, value, subtitle, optional trend.
#[component]
pub fn EdrStatCard(
    icon: String,
    title: String,
    value: String,
    #[props(default)] subtitle: String,
    #[props(default)] trend: String,
    #[props(default = "primary".to_string())] color: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let accent = match color.as_str() {
        "success" => "var(--qs-success)",
        "warning" => "var(--qs-warning)",
        "danger" => "var(--qs-destructive)",
        "info" => "var(--qs-info)",
        _ => "var(--qs-primary)",
    };

    rsx! {
        div {
            style: "padding: 16px 20px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px; cursor: pointer; transition: all 0.15s; min-width: 140px; flex: 1;",
            onclick: move |e| onclick.call(e),
            div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                span { style: "font-size: 16px;", "{icon}" }
                span { style: "font-size: 11px; font-weight: 600; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.05em;", "{title}" }
            }
            div { style: "font-size: 28px; font-weight: 800; font-family: var(--font-mono); color: {accent}; line-height: 1;", "{value}" }
            if !subtitle.is_empty() || !trend.is_empty() {
                div { style: "display: flex; align-items: center; gap: 8px; margin-top: 6px;",
                    if !subtitle.is_empty() {
                        span { style: "font-size: 11px; color: var(--qs-fg-muted);", "{subtitle}" }
                    }
                    if !trend.is_empty() {
                        span { style: "font-size: 10px; font-weight: 700; font-family: var(--font-mono); color: {accent};", "{trend}" }
                    }
                }
            }
        }
    }
}
