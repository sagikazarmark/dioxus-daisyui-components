use dioxus::prelude::*;

use crate::components::badge::{Badge, BadgeColor};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in BadgeColor::ALL.iter().copied() {
                Badge { color, "{color:?}" }
            }
        }
    }
}
