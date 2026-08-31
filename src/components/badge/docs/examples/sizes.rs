use dioxus::prelude::*;

use crate::components::badge::{Badge, BadgeSize};

/// Every value of the size axis, smallest to largest.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in BadgeSize::ALL.iter().copied() {
                Badge { size, "{size:?}" }
            }
        }
    }
}
