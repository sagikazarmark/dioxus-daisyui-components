use dioxus::prelude::*;

use crate::components::steps::{Step, StepColor, Steps};

/// Every value of the colour Axis on both a node and its connector.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
            for color in StepColor::ALL.iter().copied() {
                Steps { class: "w-full",
                    Step { color, "{color:?}" }
                    Step { color, "Connector" }
                }
            }
        }
    }
}
