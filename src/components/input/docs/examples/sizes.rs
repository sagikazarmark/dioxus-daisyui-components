use dioxus::prelude::*;

use crate::components::input::{Input, InputSize};

/// Every value of the size axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in InputSize::ALL.iter().copied() {
                Input {
                    size,
                    aria_label: format!("{size:?}"),
                    placeholder: format!("{size:?}"),
                }
            }
        }
    }
}
