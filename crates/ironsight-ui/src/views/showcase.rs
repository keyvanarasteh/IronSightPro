//! Component Showcase — live demo of every UI component in the IronSight design system.

use dioxus_primitives::slider::SliderValue;
use dioxus_primitives::checkbox::CheckboxState;

use dioxus::prelude::*;
use crate::components::accordion::*;
use crate::components::alert_dialog::*;
use crate::components::avatar::*;
use crate::components::badge::*;
use crate::components::button::*;
use crate::components::card::*;
use crate::components::checkbox::*;
use crate::components::collapsible::*;
use crate::components::dialog::*;
use crate::components::input::*;
use crate::components::label::*;
use crate::components::popover::*;
use crate::components::progress::*;
use crate::components::radio_group::*;
use crate::components::select::*;
use crate::components::separator::*;
use crate::components::skeleton::*;
use crate::components::slider::*;
use crate::components::switch::*;
use crate::components::tabs::*;
use crate::components::textarea::*;
use crate::components::toggle::*;
use crate::components::toggle_group::*;
use crate::components::tooltip::*;

const SHOWCASE_CSS: &str = r#"
.showcase-page { padding: 32px; overflow-y: auto; height: 100%; }
.showcase-page h1 { font-size: 28px; font-weight: 700; color: var(--text-primary); margin-bottom: 8px; letter-spacing: -0.5px; }
.showcase-page .subtitle { color: var(--text-muted); font-size: 14px; margin-bottom: 32px; }

.showcase-layout {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 24px;
    align-items: start;
    padding-bottom: 64px;
}
.showcase-section {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 32px 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    position: relative;
    box-shadow: var(--shadow-card);
}
.showcase-section::after {
    content: "↗";
    position: absolute;
    top: 16px;
    right: 16px;
    color: var(--text-muted);
    font-size: 14px;
    cursor: pointer;
    transition: color 0.15s ease;
}
.showcase-section:hover::after { color: var(--text-primary); }

.showcase-section h2 {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 24px;
    text-align: center;
}
.showcase-section .desc { display: none; }

.showcase-row { display: flex; gap: 12px; align-items: center; justify-content: center; flex-wrap: wrap; width: 100%; }
.showcase-col { display: flex; flex-direction: column; gap: 16px; align-items: center; justify-content: center; width: 100%; }
.showcase-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px; width: 100%; }
.showcase-box { background: var(--bg-card); border: 1px solid var(--border); border-radius: 8px; padding: 16px; }
.showcase-label { font-size: 12px; color: var(--text-muted); margin-bottom: 8px; text-align: center; display: block; font-weight: 500;}
"#;

#[component]
pub fn Showcase() -> Element {
    rsx! {
        style { {SHOWCASE_CSS} }
        div { class: "showcase-page",
            h1 { "Component Showcase" }
            p { class: "subtitle", "IronSight Design System Preview" }

            div { class: "showcase-layout",
                ShowcaseAccordion {}
                ShowcaseAlertDialog {}
                // ShowcaseAspectRatio {} // Placeholder, can be added later
                ShowcaseAvatar {}
                ShowcaseBadges {}
                ShowcaseButtons {}
                ShowcaseCards {}
                ShowcaseInputs {}
                ShowcaseCheckboxSwitch {}
                ShowcaseProgress {}
                ShowcaseSlider {}
                ShowcaseSkeleton {}
                ShowcaseSeparator {}
                ShowcaseRadioGroup {}
                ShowcaseSelect {}
                ShowcaseTabs {}
                ShowcaseCollapsible {}
                ShowcaseToggle {}
                ShowcaseToggleGroup {}
                ShowcaseTooltip {}
                ShowcasePopover {}
                ShowcaseDialog {}
                ShowcaseTextarea {}
            }
        }
    }
}

// ── Buttons ──

#[component]
fn ShowcaseButtons() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Button" }
            p { class: "desc", "5 variants: Primary, Secondary, Destructive, Outline, Ghost" }
            div { class: "showcase-col",
                Button { variant: ButtonVariant::Primary, "Primary" }
                Button { variant: ButtonVariant::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Destructive, "Destructive" }
                Button { variant: ButtonVariant::Outline, "Outline" }
                Button { variant: ButtonVariant::Ghost, "Ghost" }
                Button { variant: ButtonVariant::Primary, disabled: true, "Disabled" }
            }
        }
    }
}

// ── Badges ──

#[component]
fn ShowcaseBadges() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Badge" }
            p { class: "desc", "4 variants: Primary, Secondary, Destructive, Outline" }
            div { class: "showcase-row",
                Badge { variant: BadgeVariant::Primary, "Primary" }
                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                Badge { variant: BadgeVariant::Destructive, "Destructive" }
                Badge { variant: BadgeVariant::Outline, "Outline" }
            }
        }
    }
}

// ── Cards ──

#[component]
fn ShowcaseCards() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Card" }
            p { class: "desc", "Composable card with Header, Title, Description, Content, Action, and Footer slots" }
            div { class: "showcase-grid",
                Card {
                    CardHeader {
                        CardTitle { "System Integrity" }
                        CardDescription { "Real-time system health overview" }
                    }
                    CardContent {
                        p { style: "color: var(--accent-green); font-size: 28px; font-weight: 700;", "98.7%" }
                    }
                    CardFooter {
                        span { style: "color: var(--text-muted); font-size: 11px;", "Last checked 2m ago" }
                    }
                }
                Card {
                    CardHeader {
                        CardTitle { "Alerts" }
                        CardDescription { "Active security notifications" }
                        CardAction {
                            Button { variant: ButtonVariant::Outline, "View All" }
                        }
                    }
                    CardContent {
                        p { style: "color: var(--accent-orange); font-size: 28px; font-weight: 700;", "3" }
                    }
                }
            }
        }
    }
}

// ── Inputs ──

#[component]
fn ShowcaseInputs() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Input" }
            p { class: "desc", "Text input with label composition" }
            div { class: "showcase-col",
                div { style: "display: flex; flex-direction: column; gap: 6px; width: 100%;",
                    Label { html_for: "email-input", "Email" }
                    Input { id: "email-input", r#type: "email", placeholder: "agent@ironsight.io" }
                }
                div { style: "display: flex; flex-direction: column; gap: 6px; width: 100%;",
                    Label { html_for: "search-input", "Search" }
                    Input { id: "search-input", r#type: "text", placeholder: "Search processes..." }
                }
                div { style: "display: flex; flex-direction: column; gap: 6px; width: 100%;",
                    Label { html_for: "disabled-input", "Disabled" }
                    Input { id: "disabled-input", disabled: true, placeholder: "Locked" }
                }
            }
        }
    }
}

// ── Checkbox & Switch ──

#[component]
fn ShowcaseCheckboxSwitch() -> Element {
    let mut switch_checked = use_signal(|| false);

    rsx! {
        div { class: "showcase-section",
            h2 { "Checkbox & Switch" }
            p { class: "desc", "Boolean toggle controls" }
            div { class: "showcase-col", style: "align-items: flex-start; width: 100%;",
                div { style: "display: flex; align-items: center; gap: 8px;",
                    Checkbox { name: "demo-check" }
                    Label { html_for: "demo-check", "Enable auto-scan" }
                }
                div { style: "display: flex; align-items: center; gap: 8px;",
                    Checkbox { default_checked: CheckboxState::Checked, name: "demo-check2" }
                    Label { html_for: "demo-check2", "Real-time monitoring" }
                }
                div { style: "display: flex; align-items: center; gap: 8px;",
                    Switch {
                        default_checked: *switch_checked.read(),
                        on_checked_change: move |v: bool| switch_checked.set(v),
                        SwitchThumb {}
                    }
                    span { style: "font-size: 13px; color: var(--text-primary); margin-left: 4px;",
                        if *switch_checked.read() { "ON" } else { "OFF" }
                    }
                }
            }
        }
    }
}

// ── Progress ──

#[component]
fn ShowcaseProgress() -> Element {
    let progress_val = use_signal(|| Some(65.0f64));

    rsx! {
        div { class: "showcase-section",
            h2 { "Progress" }
            p { class: "desc", "Deterministic progress indicator with animated fill" }
            div { style: "max-width: 400px;",
                Progress {
                    value: progress_val,
                    ProgressIndicator {}
                }
                p { style: "color: var(--text-muted); font-size: 11px; margin-top: 6px;",
                    "Scan progress: 65%"
                }
            }
        }
    }
}

// ── Slider ──

#[component]
fn ShowcaseSlider() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Slider" }
            p { class: "desc", "Draggable range input with track, range, and thumb" }
            div { style: "max-width: 400px;",
                Slider {
                    default_value: SliderValue::Single(42.0),
                    label: "Sensitivity",
                    SliderTrack {
                        SliderRange {}
                    }
                    SliderThumb { index: 0usize }
                }
            }
        }
    }
}

// ── Avatar ──

#[component]
fn ShowcaseAvatar() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Avatar" }
            p { class: "desc", "3 sizes (Small, Medium, Large) × 2 shapes (Circle, Rounded) with fallback" }
            div { class: "showcase-row", style: "align-items: flex-end;",
                div { class: "showcase-col", style: "gap: 8px;",
                    span { class: "showcase-label", "Basic Usage" }
                    Avatar { size: AvatarImageSize::Small, shape: AvatarShape::Circle,
                        AvatarFallback { "IS" }
                    }
                }
                div { class: "showcase-col", style: "gap: 8px;",
                    span { class: "showcase-label", "Rounded" }
                    Avatar { size: AvatarImageSize::Medium, shape: AvatarShape::Rounded,
                        AvatarFallback { "KA" }
                    }
                }
                div { class: "showcase-col", style: "gap: 8px;",
                    span { class: "showcase-label", "Error State" }
                    Avatar { size: AvatarImageSize::Medium, shape: AvatarShape::Circle,
                        AvatarFallback { "JK" }
                    }
                }
                div { class: "showcase-col", style: "gap: 8px;",
                    span { class: "showcase-label", "Large Size" }
                    Avatar { size: AvatarImageSize::Large, shape: AvatarShape::Circle,
                        AvatarFallback { "QX" }
                    }
                }
            }
        }
    }
}

// ── Skeleton ──

#[component]
fn ShowcaseSkeleton() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Skeleton" }
            p { class: "desc", "Loading placeholder with pulse animation" }
            div { style: "display: flex; gap: 12px; align-items: center;",
                Skeleton { style: "width: 48px; height: 48px; border-radius: 50%;" }
                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    Skeleton { style: "width: 200px; height: 14px; border-radius: 4px;" }
                    Skeleton { style: "width: 140px; height: 10px; border-radius: 4px;" }
                }
            }
        }
    }
}

// ── Separator ──

#[component]
fn ShowcaseSeparator() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Separator" }
            p { class: "desc", "Visual divider — horizontal or decorative" }
            div { style: "max-width: 400px;",
                span { style: "color: var(--text-secondary); font-size: 12px;", "Section A" }
                Separator {}
                span { style: "color: var(--text-secondary); font-size: 12px;", "Section B" }
            }
        }
    }
}

// ── RadioGroup ──

#[component]
fn ShowcaseRadioGroup() -> Element {
    let mut radio_val = use_signal(|| "medium".to_string());

    rsx! {
        div { class: "showcase-section",
            h2 { "Radio Group" }
            p { class: "desc", "Exclusive selection from a group of options" }
            RadioGroup {
                default_value: "medium",
                on_value_change: move |v: String| radio_val.set(v),
                RadioItem { value: "low", index: 0usize, "🟢 Low" }
                RadioItem { value: "medium", index: 1usize, "🟡 Medium" }
                RadioItem { value: "high", index: 2usize, "🔴 High" }
            }
            p { style: "color: var(--text-muted); font-size: 11px; margin-top: 6px;",
                "Selected: {radio_val}"
            }
        }
    }
}

// ── Select ──

#[component]
fn ShowcaseSelect() -> Element {
    let mut select_val = use_signal(|| None::<String>);

    rsx! {
        div { class: "showcase-section",
            h2 { "Select" }
            p { class: "desc", "Dropdown select with groups, options, and indicators" }
            div { style: "width: 100%;",
                Select::<String> {
                    on_value_change: move |v: Option<String>| select_val.set(v),
                    placeholder: "Choose scan mode",
                    SelectTrigger { SelectValue {} }
                    SelectList {
                        SelectGroup {
                            SelectGroupLabel { "Scan Modes" }
                            SelectOption::<String> { value: "quick".to_string(), text_value: "Quick Scan", index: 0usize,
                                SelectItemIndicator {} "⚡ Quick Scan"
                            }
                            SelectOption::<String> { value: "deep".to_string(), text_value: "Deep Scan", index: 1usize,
                                SelectItemIndicator {} "🔍 Deep Scan"
                            }
                            SelectOption::<String> { value: "full".to_string(), text_value: "Full System", index: 2usize,
                                SelectItemIndicator {} "🛡️ Full System"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Tabs ──

#[component]
fn ShowcaseTabs() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Tabs" }
            p { class: "desc", "Tab navigation with Default and Ghost variants" }
            Tabs {
                default_value: "overview",
                TabList {
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "threats", index: 1usize, "Threats" }
                    TabTrigger { value: "config", index: 2usize, "Config" }
                }
                TabContent { value: "overview", index: 0usize,
                    p { style: "color: var(--text-secondary); padding: 12px;", "System overview panel content" }
                }
                TabContent { value: "threats", index: 1usize,
                    p { style: "color: var(--text-secondary); padding: 12px;", "Active threat analysis panel content" }
                }
                TabContent { value: "config", index: 2usize,
                    p { style: "color: var(--text-secondary); padding: 12px;", "Configuration panel content" }
                }
            }
        }
    }
}

// ── Accordion ──

#[component]
fn ShowcaseAccordion() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Accordion" }
            p { class: "desc", "Expandable content panels" }
            Accordion {
                AccordionItem { index: 0usize,
                    AccordionTrigger { "What is IronSight?" }
                    AccordionContent { "IronSight is a heuristic-based endpoint detection and response (EDR) system built entirely in Rust." }
                }
                AccordionItem { index: 1usize,
                    AccordionTrigger { "How does scanning work?" }
                    AccordionContent { "The scanner evaluates running processes against a multi-signal heuristic engine to assign threat scores." }
                }
                AccordionItem { index: 2usize,
                    AccordionTrigger { "Is it production-ready?" }
                    AccordionContent { "IronSight is an educational and research project demonstrating Rust-based security tooling." }
                }
            }
        }
    }
}

// ── Collapsible ──

#[component]
fn ShowcaseCollapsible() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Collapsible" }
            p { class: "desc", "Toggle visibility of content sections" }
            Collapsible {
                CollapsibleTrigger { "Show scan details" }
                CollapsibleContent {
                    CollapsibleList {
                        CollapsibleItem { "Process scanning: 342 evaluated" }
                        CollapsibleItem { "Network ports: 18 open" }
                        CollapsibleItem { "Memory regions: 2048 checked" }
                    }
                }
            }
        }
    }
}

// ── Toggle ──

#[component]
fn ShowcaseToggle() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Toggle" }
            p { class: "desc", "Pressed/unpressed toggle button" }
            div { class: "showcase-row",
                Toggle { "Bold" }
                Toggle { "Italic" }
                Toggle { default_pressed: true, "Underline" }
                Toggle { disabled: true, "Disabled" }
            }
        }
    }
}

// ── ToggleGroup ──

#[component]
fn ShowcaseToggleGroup() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Toggle Group" }
            p { class: "desc", "Grouped toggle buttons with single or multi-select" }
            ToggleGroup {
                ToggleItem { index: 0usize, "Left" }
                ToggleItem { index: 1usize, "Center" }
                ToggleItem { index: 2usize, "Right" }
            }
        }
    }
}

// ── Tooltip ──

#[component]
fn ShowcaseTooltip() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Tooltip" }
            p { class: "desc", "Hover to reveal contextual information" }
            div { class: "showcase-row",
                Tooltip {
                    TooltipTrigger {
                        Button { variant: ButtonVariant::Outline, "Hover me" }
                    }
                    TooltipContent { "Initiates a full system scan" }
                }
            }
        }
    }
}

// ── Popover ──

#[component]
fn ShowcasePopover() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Popover" }
            p { class: "desc", "Click-triggered floating content panel" }
            PopoverRoot {
                PopoverTrigger {
                    Button { variant: ButtonVariant::Outline, "Open Popover" }
                }
                PopoverContent {
                    div { style: "padding: 8px;",
                        p { style: "font-weight: 600; margin-bottom: 8px; color: var(--text-primary);", "Quick Actions" }
                        p { style: "color: var(--text-muted); font-size: 12px;", "Configure scan parameters and notification preferences." }
                    }
                }
            }
        }
    }
}

// ── Dialog ──

#[component]
fn ShowcaseDialog() -> Element {
    let mut dialog_open = use_signal(|| false);

    rsx! {
        div { class: "showcase-section",
            h2 { "Dialog" }
            p { class: "desc", "Modal dialog with title, description, and content" }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| dialog_open.set(true),
                "Open Dialog"
            }
            DialogRoot {
                open: *dialog_open.read(),
                on_open_change: move |v: bool| dialog_open.set(v),
                DialogContent {
                    DialogTitle { "Confirm Action" }
                    DialogDescription { "Are you sure you want to quarantine this process?" }
                    div { style: "display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px;",
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| dialog_open.set(false),
                            "Cancel"
                        }
                        Button {
                            variant: ButtonVariant::Destructive,
                            onclick: move |_| dialog_open.set(false),
                            "Quarantine"
                        }
                    }
                }
            }
        }
    }
}

// ── AlertDialog ──

#[component]
fn ShowcaseAlertDialog() -> Element {
    let mut alert_open = use_signal(|| false);

    rsx! {
        div { class: "showcase-section",
            h2 { "Alert Dialog" }
            p { class: "desc", "Non-dismissible alert with required action" }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| alert_open.set(true),
                "Trigger Alert"
            }
            AlertDialogRoot {
                open: *alert_open.read(),
                on_open_change: move |v: bool| alert_open.set(v),
                AlertDialogContent {
                    AlertDialogTitle { "⚠️ Critical Threat Detected" }
                    AlertDialogDescription { "Process 'suspicious.exe' (PID 4821) scored 94.2 — immediate action required." }
                    AlertDialogActions {
                        AlertDialogCancel { on_click: move |_| alert_open.set(false), "Ignore" }
                        AlertDialogAction { on_click: move |_| alert_open.set(false), "Kill Process" }
                    }
                }
            }
        }
    }
}

// ── Textarea ──

#[component]
fn ShowcaseTextarea() -> Element {
    rsx! {
        div { class: "showcase-section",
            h2 { "Textarea" }
            p { class: "desc", "4 variants: Default, Fade, Outline, Ghost" }
            div { class: "showcase-col", style: "width: 100%;",
                div { style: "width: 100%;",
                    span { class: "showcase-label", "Default" }
                    Textarea { variant: TextareaVariant::Default, placeholder: "Enter notes...", rows: "3" }
                }
                div { style: "width: 100%;",
                    span { class: "showcase-label", "Outline" }
                    Textarea { variant: TextareaVariant::Outline, placeholder: "Enter notes...", rows: "3" }
                }
            }
        }
    }
}
