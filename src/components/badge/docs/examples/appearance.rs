use dioxus::prelude::*;

use crate::components::badge::{Badge, BadgeAppearance, BadgeColor};

/// Every value of the appearance axis on one fixed colour.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-2",
            for appearance in BadgeAppearance::ALL.iter().copied() {
                Badge { color: BadgeColor::Primary, appearance, "{appearance:?}" }
            }
        }
    }
}
