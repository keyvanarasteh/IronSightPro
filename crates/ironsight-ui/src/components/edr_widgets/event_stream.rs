use dioxus::prelude::*;

/// Terminal-style event stream container.
#[component]
pub fn EventStream(
    #[props(default = false)] paused: bool,
    #[props(default = "400px".to_string())] height: String,
    children: Element,
) -> Element {
    rsx! {
        div { style: "background: #0d1117; border: 1px solid var(--qs-border); border-radius: 10px; overflow: hidden;",
            div { style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 14px; background: var(--qs-bg-elevated); border-bottom: 1px solid var(--qs-border);",
                span { style: "font-size: 11px; font-weight: 700; color: var(--qs-fg-muted); text-transform: uppercase; letter-spacing: 0.1em;",
                    "⚡ Event Stream"
                }
                span {
                    style: {
                        let c = if paused { "var(--qs-warning)" } else { "var(--qs-success)" };
                        format!("font-size: 10px; font-weight: 600; color: {c};")
                    },
                    if paused { "⏸ PAUSED" } else { "🔴 LIVE" }
                }
            }
            div { style: "max-height: {height}; overflow-y: auto; padding: 8px 0; font-family: 'Cascadia Code', 'Fira Code', var(--font-mono); font-size: 11px; line-height: 1.6;",
                {children}
            }
        }
    }
}

/// Single event entry in the stream.
#[component]
pub fn EventEntry(
    timestamp: String,
    syscall: String,
    pid: String,
    #[props(default)] args: String,
    #[props(default = false)] alert: bool,
) -> Element {
    let syscall_color = match syscall.to_lowercase().as_str() {
        "mprotect" => "#ef4444",
        "execve" => "#f97316",
        "connect" => "#3b82f6",
        "ptrace" => "#ef4444",
        "kill" => "#f59e0b",
        _ => "#8b949e",
    };

    rsx! {
        div {
            style: {
                let bg = if alert { "background: rgba(239,68,68,0.08);" } else { "" };
                format!("display: flex; gap: 8px; padding: 2px 14px; align-items: baseline; {bg}")
            },
            span { style: "color: #484f58; font-size: 10px; flex-shrink: 0;", "{timestamp}" }
            span { style: "color: {syscall_color}; font-weight: 600; min-width: 80px;", "{syscall}" }
            span { style: "color: #58a6ff; min-width: 72px;", "PID:{pid}" }
            if !args.is_empty() {
                span { style: "color: #8b949e; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{args}" }
            }
            if alert {
                span { style: "font-size: 9px; font-weight: 700; color: #ef4444; background: rgba(239,68,68,0.2); padding: 1px 6px; border-radius: 4px; flex-shrink: 0;", "[ALERT]" }
            }
        }
    }
}
