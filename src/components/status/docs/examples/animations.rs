use dioxus::prelude::*;

use crate::components::status::{Status, StatusColor};

/// Caller-composed ping and bounce utilities with visible status meaning.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-8",
            span { class: "inline-flex items-center gap-2",
                span { class: "inline-grid *:[grid-area:1/1]",
                    Status {
                        id: "caller-ping",
                        class: "animate-ping",
                        color: StatusColor::Error,
                        aria_hidden: "true",
                    }
                    Status { color: StatusColor::Error, aria_hidden: "true" }
                }
                "Server is down"
            }
            span { class: "inline-flex items-center gap-2",
                Status {
                    id: "caller-bounce",
                    class: "animate-bounce",
                    color: StatusColor::Info,
                    aria_hidden: "true",
                }
                "Unread messages"
            }
        }
    }
}
