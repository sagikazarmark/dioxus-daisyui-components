use dioxus::prelude::*;

use crate::components::input::{Input, InputSize};

/// Every value of the size axis on the adorned wrapper, smallest to largest.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "adorned-size", class: "flex flex-wrap items-center gap-2",
            for size in InputSize::ALL.iter().copied() {
                Input {
                    size,
                    suffix: rsx! { "EUR" },
                    aria_label: format!("{size:?} amount"),
                    placeholder: format!("{size:?}"),
                }
            }
        }
    }
}
