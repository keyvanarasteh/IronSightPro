use dioxus::prelude::*;

/// A styled data table wrapper for dashboard views.
#[component]
pub fn DashTable(
    #[props(default)] class: String,
    #[props(default)] style: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "dash-table-wrap {class}",
            style: "{style}",
            table {
                {children}
            }
        }
    }
}

/// Table header
#[component]
pub fn DashTableHead(children: Element) -> Element {
    rsx! {
        thead {
            {children}
        }
    }
}

/// Table header cell
#[component]
pub fn DashTh(
    #[props(default)] style: String,
    children: Element,
) -> Element {
    rsx! {
        th {
            style: "{style}",
            {children}
        }
    }
}

/// Table body
#[component]
pub fn DashTableBody(children: Element) -> Element {
    rsx! {
        tbody {
            {children}
        }
    }
}

/// Table row with hover effects
#[component]
pub fn DashTr(
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        tr {
            class: "{class}",
            style: "{style}",
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}

/// Table data cell
#[component]
pub fn DashTd(
    #[props(default)] style: String,
    children: Element,
) -> Element {
    rsx! {
        td {
            style: "{style}",
            {children}
        }
    }
}
