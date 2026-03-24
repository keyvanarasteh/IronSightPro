use dioxus::prelude::*;

#[component]
pub fn Alerts() -> Element {
    rsx! {
        div { class: "app-container",
            div { class: "header",
                div {}
                div { class: "header-actions", "Incident Alerts" }
            }
            div { class: "main-content",
                div { class: "loading", "Alerts Module Loading..." }
            }
        }
    }
}
