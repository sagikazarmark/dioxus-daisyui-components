use dioxus::prelude::*;

use crate::components::badge::{Badge, BadgeAppearance, BadgeColor};

/// A caller's own class and attributes joined with the badge's.
#[component]
pub fn Example() -> Element {
    rsx! {
        Badge {
            id: "caller-attributes",
            class: "rounded-none",
            color: BadgeColor::Primary,
            appearance: BadgeAppearance::Outline,
            "Caller classes and attributes"
        }
    }
}
