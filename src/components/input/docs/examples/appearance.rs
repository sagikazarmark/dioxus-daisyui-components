use dioxus::prelude::*;

use crate::components::input::{Input, InputAppearance};

/// Every value of the appearance axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-2",
            for appearance in InputAppearance::ALL.iter().copied() {
                Input {
                    appearance,
                    aria_label: format!("{appearance:?}"),
                    placeholder: format!("{appearance:?}"),
                }
            }
        }
    }
}
