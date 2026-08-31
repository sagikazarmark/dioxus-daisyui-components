use dioxus::prelude::*;

use crate::components::status::{Status, StatusColor, StatusSize};

/// Every colour, at a size large enough to compare its paint.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-6",
            for color in StatusColor::ALL.iter().copied() {
                Status {
                    color,
                    size: StatusSize::Lg,
                    role: "img",
                    aria_label: "{color:?} status",
                }
            }
        }
    }
}
