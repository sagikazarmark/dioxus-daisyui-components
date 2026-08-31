use dioxus::prelude::*;

use crate::components::alert::{Alert, AlertDirection};

/// Every value of the direction axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "direction", class: "flex flex-col gap-2",
            for direction in AlertDirection::ALL.iter().copied() {
                Alert { direction,
                    span { "!" }
                    span { "{direction:?} alert" }
                }
            }
        }
    }
}
