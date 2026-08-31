use dioxus::prelude::*;

use crate::components::badge::{Badge, BadgeAppearance, BadgeColor, BadgeSize};

/// A default badge and three independent axes set together.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Badge { id: "default-badge", "Default" }
            Badge {
                color: BadgeColor::Primary,
                size: BadgeSize::Lg,
                appearance: BadgeAppearance::Outline,
                "Primary, large, outline"
            }
        }
    }
}
