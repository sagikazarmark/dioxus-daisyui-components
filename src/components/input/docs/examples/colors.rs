use dioxus::prelude::*;

use crate::components::input::{Input, InputColor};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in InputColor::ALL.iter().copied() {
                Input {
                    color,
                    aria_label: format!("{color:?}"),
                    placeholder: format!("{color:?}"),
                }
            }
        }
    }
}
