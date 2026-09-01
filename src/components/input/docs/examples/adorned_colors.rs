use dioxus::prelude::*;

use crate::components::input::{Input, InputColor};

/// Every value of the colour axis, relocated to the adorned wrapper.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "adorned-color", class: "flex flex-wrap items-center gap-2",
            for color in InputColor::ALL.iter().copied() {
                Input {
                    color,
                    suffix: rsx! { "EUR" },
                    aria_label: format!("{color:?} amount"),
                    placeholder: format!("{color:?}"),
                }
            }
        }
    }
}
