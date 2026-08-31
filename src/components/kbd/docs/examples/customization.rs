use dioxus::prelude::*;

use crate::components::kbd::{Kbd, KbdSize};

/// Caller classes and attributes joined with a selected size.
#[component]
pub fn Example() -> Element {
    rsx! {
        Kbd {
            id: "caller-attributes",
            class: "rounded-none text-primary",
            size: KbdSize::Lg,
            "data-owner": "caller",
            aria_label: "Command key",
            "Command"
        }
    }
}
