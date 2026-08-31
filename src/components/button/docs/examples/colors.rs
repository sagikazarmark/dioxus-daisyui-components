use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};

/// Every value of the colour axis, iterating the axis' own variant list rather
/// than naming values by hand.
///
/// The row it renders is what the browser tests read computed styles off, which
/// is what `data-axis` marks: one element per value of the axis and nothing
/// else.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in ButtonColor::ALL.iter().copied() {
                Button { color, "{color:?}" }
            }
        }
    }
}
