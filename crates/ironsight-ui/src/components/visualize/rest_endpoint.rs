use dioxus::prelude::*;

/// HTTP method badge with color coding.
#[component]
pub fn MethodBadge(method: String) -> Element {
    let cls = match method.to_uppercase().as_str() {
        "GET" => "qs-method qs-method-get",
        "POST" => "qs-method qs-method-post",
        "PUT" => "qs-method qs-method-put",
        "PATCH" => "qs-method qs-method-patch",
        "DELETE" => "qs-method qs-method-delete",
        _ => "qs-method",
    };
    rsx! { span { class: "{cls}", "{method}" } }
}

/// HTTP status code badge.
#[component]
pub fn StatusCodeBadge(
    code: u16,
    #[props(default = false)] active: bool,
) -> Element {
    let cls = if code >= 200 && code < 300 { "qs-status qs-status-2xx" }
        else if code >= 300 && code < 400 { "qs-status qs-status-3xx" }
        else if code >= 400 && code < 500 { "qs-status qs-status-4xx" }
        else { "qs-status qs-status-5xx" };
    let active_cls = if active { "active" } else { "" };
    rsx! { span { class: "{cls} {active_cls}", "{code}" } }
}

/// Security badge (auth scheme, roles).
#[component]
pub fn SecurityBadge(
    requires_auth: bool,
    #[props(default)] scheme: String,
    #[props(default)] roles: Vec<String>,
    #[props(default)] scopes: Vec<String>,
) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: flex-end; gap: 6px;",
            div { style: "display: flex; align-items: center; gap: 8px; padding: 6px 12px; background: var(--qs-secondary); border: 1px solid var(--qs-border); border-radius: 20px;",
                if requires_auth {
                    span { style: "font-size: 14px;", "🔒" }
                    span { style: "font-size: 12px; font-weight: 600; color: var(--qs-fg);", "Authenticated" }
                    if !scheme.is_empty() {
                        span { class: "qs-badge qs-badge-primary", "{scheme}" }
                    }
                } else {
                    span { style: "font-size: 14px;", "🔓" }
                    span { style: "font-size: 12px; font-weight: 600; color: var(--qs-success);", "Public Access" }
                }
            }
            if !roles.is_empty() {
                div { style: "display: flex; gap: 4px; flex-wrap: wrap;",
                    for role in roles.iter() {
                        span { style: "display: inline-flex; align-items: center; gap: 4px; font-size: 9px; font-weight: 700; color: #8b5cf6; background: rgba(139,92,246,0.1); padding: 2px 6px; border-radius: 4px; text-transform: uppercase;",
                            "🔑 {role}"
                        }
                    }
                }
            }
            if !scopes.is_empty() {
                div { style: "display: flex; gap: 4px; flex-wrap: wrap;",
                    for scope in scopes.iter() {
                        span { style: "display: inline-flex; align-items: center; gap: 4px; font-size: 9px; font-weight: 700; color: var(--qs-info); background: var(--qs-info-bg); padding: 2px 6px; border-radius: 4px; text-transform: uppercase;",
                            "🛡️ {scope}"
                        }
                    }
                }
            }
        }
    }
}

/// Rate limit indicator badge.
#[component]
pub fn RateLimitBadge(
    requests: u32,
    window_seconds: u32,
) -> Element {
    rsx! {
        div { style: "display: inline-flex; align-items: center; gap: 6px; padding: 4px 10px; border-radius: 20px; border: 1px solid rgba(245,158,11,0.2); background: rgba(245,158,11,0.05); color: var(--qs-warning); font-size: 10px; font-weight: 700;",
            "⚠️ {requests}/{window_seconds}s"
        }
    }
}

/// Parameter/property row for request parameters.
#[component]
pub fn ParamRow(
    name: String,
    location: String,
    param_type: String,
    #[props(default)] description: String,
    #[props(default = false)] required: bool,
    #[props(default = false)] deprecated: bool,
    #[props(default)] enum_values: Vec<String>,
    #[props(default)] default_value: String,
    #[props(default)] example: String,
) -> Element {
    rsx! {
        tr { style: if deprecated { "opacity: 0.6;" } else { "" },
            td { style: "padding: 8px 14px; font-family: var(--font-mono); font-size: 12px; color: var(--qs-primary); vertical-align: top;",
                "{name}"
                if required {
                    span { style: "margin-left: 4px; font-size: 10px; color: var(--qs-destructive);", "*" }
                }
                if deprecated {
                    span { class: "qs-badge qs-badge-warning", style: "margin-left: 4px;", "DEPRECATED" }
                }
            }
            td { style: "padding: 8px 14px; vertical-align: top;",
                span { style: "font-size: 10px; font-weight: 500; color: var(--qs-muted-fg); background: var(--qs-muted); padding: 2px 6px; border-radius: 4px;",
                    "{location}"
                }
            }
            td { style: "padding: 8px 14px; font-family: var(--font-mono); font-size: 10px; color: var(--qs-fg-muted); vertical-align: top;",
                "{param_type}"
            }
            td { style: "padding: 8px 14px; vertical-align: top;",
                if !description.is_empty() {
                    p { style: "font-size: 12px; color: var(--qs-fg); margin: 0;", "{description}" }
                }
                if !enum_values.is_empty() {
                    p { style: "font-size: 10px; font-family: var(--font-mono); color: var(--qs-fg-muted); margin: 4px 0 0;",
                        "Enum: {enum_values.join(\" | \")}"
                    }
                }
                if !default_value.is_empty() {
                    p { style: "font-size: 10px; font-family: var(--font-mono); color: var(--qs-fg-muted); margin: 2px 0 0;",
                        "Default: {default_value}"
                    }
                }
                if !example.is_empty() {
                    p { style: "font-size: 10px; font-family: var(--font-mono); color: var(--qs-fg-subtle); margin: 2px 0 0;",
                        "Example: {example}"
                    }
                }
            }
        }
    }
}

/// Schema property row for request/response bodies.
#[component]
pub fn PropertyRow(
    name: String,
    prop_type: String,
    #[props(default)] description: String,
    #[props(default = false)] required: bool,
    #[props(default = false)] nullable: bool,
    #[props(default = false)] deprecated: bool,
    #[props(default)] enum_values: Vec<String>,
    #[props(default)] default_value: String,
) -> Element {
    rsx! {
        div { style: "display: flex; align-items: flex-start; gap: 8px; padding: 6px; border-radius: 4px;",
            div { style: "flex: 1;",
                span {
                    style: format!("font-family: var(--font-mono); font-size: 12px; font-weight: 600; color: var(--qs-primary);{}", if deprecated { " text-decoration: line-through; opacity: 0.6;" } else { "" }),
                    "{name}"
                }
                if required {
                    span { style: "margin-left: 4px; font-size: 10px; color: var(--qs-destructive);", "*" }
                }
                if nullable {
                    span { style: "margin-left: 4px; font-size: 8px; font-weight: 500; color: var(--qs-muted-fg); background: var(--qs-muted); padding: 1px 4px; border-radius: 3px;", "nullable" }
                }
                span { style: "margin-left: 8px; font-family: var(--font-mono); font-size: 10px; color: var(--qs-fg-muted);", "{prop_type}" }
                if !description.is_empty() {
                    p { style: "font-size: 11px; color: var(--qs-fg-muted); margin: 2px 0 0; line-height: 1.4;", "{description}" }
                }
                if !enum_values.is_empty() {
                    p { style: "font-size: 10px; font-family: var(--font-mono); color: var(--qs-fg-subtle); margin: 2px 0 0;",
                        "Enum: {enum_values.join(\" | \")}"
                    }
                }
                if !default_value.is_empty() {
                    p { style: "font-size: 10px; font-family: var(--font-mono); color: var(--qs-fg-subtle); margin: 2px 0 0;",
                        "Default: {default_value}"
                    }
                }
            }
        }
    }
}

/// Full REST endpoint documentation card (two-column layout).
#[component]
pub fn RestEndpointCard(
    method: String,
    path: String,
    summary: String,
    #[props(default)] description: String,
    #[props(default = false)] deprecated: bool,
    #[props(default)] tags: Vec<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "qs-card", style: "display: flex; flex-direction: column; overflow: hidden; box-shadow: 0 4px 24px var(--qs-shadow);",
            // Header
            header { style: "display: flex; flex-direction: column; gap: 8px; padding: 20px; border-bottom: 1px solid var(--qs-border); background: var(--qs-card);",
                div { style: "display: flex; align-items: center; gap: 12px; flex-wrap: wrap;",
                    if deprecated {
                        span { class: "qs-badge qs-badge-warning", "Deprecated" }
                    }
                    MethodBadge { method: method.clone() }
                    h2 { style: "font-family: var(--font-mono); font-size: 18px; font-weight: 600; color: var(--qs-fg); margin: 0; letter-spacing: -0.01em;", "{path}" }
                }
                p { style: "font-size: 13px; font-weight: 500; color: var(--qs-fg-muted); margin: 0;", "{summary}" }
                if !tags.is_empty() {
                    div { style: "display: flex; flex-wrap: wrap; gap: 4px; margin-top: 4px;",
                        for tag in tags.iter() {
                            span { class: "qs-badge qs-badge-primary", "{tag}" }
                        }
                    }
                }
            }
            if !description.is_empty() {
                div { style: "padding: 16px 20px; background: var(--qs-card); border-bottom: 1px solid var(--qs-border);",
                    p { style: "font-size: 13px; color: var(--qs-fg-muted); line-height: 1.6; margin: 0;", "{description}" }
                }
            }
            // Content area (children = param tables, response panels, etc.)
            div { style: "display: flex; flex: 1; flex-direction: column; overflow: hidden;",
                {children}
            }
        }
    }
}

/// Two-column layout for REST endpoint (left = request, right = response).
#[component]
pub fn RestTwoCol(
    left: Element,
    right: Element,
) -> Element {
    rsx! {
        div { class: "qs-rest-two-col", style: "display: flex; flex: 1; overflow: hidden;",
            div { style: "flex: 1; overflow-y: auto; padding: 20px; border-right: 1px solid var(--qs-border);",
                {left}
            }
            div { class: "qs-rest-right", style: "width: 42%; overflow-y: auto; background: var(--qs-bg-panel);",
                {right}
            }
        }
    }
}

/// Section header for param/body/response sections.
#[component]
pub fn RestSectionHeader(
    icon: String,
    title: String,
    #[props(default)] badge: String,
) -> Element {
    rsx! {
        h3 { style: "display: flex; align-items: center; gap: 8px; font-size: 11px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.1em; margin: 0 0 12px;",
            span { "{icon}" }
            "{title}"
            if !badge.is_empty() {
                span { style: "margin-left: auto; font-family: var(--font-mono); font-size: 10px; color: var(--qs-fg-muted); background: var(--qs-muted); padding: 2px 8px; border-radius: 4px;", "{badge}" }
            }
        }
    }
}

/// Response panel with status tabs and content.
#[component]
pub fn ResponsePanel(children: Element) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; height: 100%;",
            {children}
        }
    }
}

/// Response tab bar.
#[component]
pub fn ResponseTabs(children: Element) -> Element {
    rsx! {
        div { style: "padding: 12px 16px; border-bottom: 1px solid var(--qs-border); background: var(--qs-card);",
            h3 { style: "font-size: 11px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.1em; margin: 0 0 8px;",
                "📦 Responses"
            }
            div { style: "display: flex; flex-wrap: wrap; gap: 8px;", {children} }
        }
    }
}

/// Response body content area.
#[component]
pub fn ResponseBody(children: Element) -> Element {
    rsx! {
        div { style: "flex: 1; overflow-y: auto; padding: 20px;", {children} }
    }
}

/// SQL query card for endpoint queries section.
#[component]
pub fn QueryCard(
    name: String,
    query_type: String,
    #[props(default)] description: String,
    #[props(default)] sql: String,
    #[props(default)] table: String,
) -> Element {
    rsx! {
        div { style: "padding: 12px 16px; background: var(--qs-card); border: 1px solid var(--qs-border); border-radius: 12px;",
            div { style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 10px; font-weight: 700; font-family: var(--font-mono); color: #60a5fa; background: rgba(96,165,250,0.1); padding: 2px 8px; border-radius: 4px;", "{query_type}" }
                span { style: "font-family: var(--font-mono); font-size: 12px; font-weight: 600; color: var(--qs-fg);", "{name}" }
                if !table.is_empty() {
                    span { style: "margin-left: auto; font-family: var(--font-mono); font-size: 10px; color: var(--qs-fg-muted);", "→ {table}" }
                }
            }
            if !description.is_empty() {
                p { style: "font-size: 11px; color: var(--qs-fg-muted); margin: 6px 0 0;", "{description}" }
            }
            if !sql.is_empty() {
                div { style: "margin-top: 8px; background: #0d1117; border-radius: 8px; padding: 12px;",
                    pre { style: "font-family: var(--font-mono); font-size: 10px; color: #7dd3fc; overflow-x: auto; margin: 0;", "{sql}" }
                }
            }
        }
    }
}
