use dioxus::prelude::*;

use crate::components::card::{Card, CardActions, CardBody, CardTitle};

/// A caller's classes and attributes joined with every part's own.
#[component]
pub fn Example() -> Element {
    rsx! {
        Card { id: "caller-card", class: "w-72 rounded-none bg-base-100 shadow-sm",
            CardBody { id: "caller-body", class: "p-2",
                CardTitle { id: "caller-title", class: "italic", "Caller title" }
                p { "Every part remains reachable." }
                CardActions { id: "caller-actions", class: "justify-center",
                    button { class: "btn btn-primary", "Centered action" }
                }
            }
        }
    }
}
