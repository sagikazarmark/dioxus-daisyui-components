use dioxus::prelude::*;

use crate::components::status::{Status, StatusColor};

/// Default, labelled and visible-text uses of a status indicator.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-6",
            Status { id: "default-status", aria_hidden: "true" }
            span { class: "inline-flex items-center gap-2",
                Status { color: StatusColor::Success, aria_hidden: "true" }
                "Online"
            }
            Status {
                color: StatusColor::Error,
                role: "img",
                aria_label: "Service unavailable",
            }
        }
    }
}
