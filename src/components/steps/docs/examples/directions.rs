use dioxus::prelude::*;

use crate::components::steps::{Step, StepColor, Steps, StepsDirection};

/// Every value of the direction Axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "direction", class: "grid gap-8 sm:grid-cols-2",
            for direction in StepsDirection::ALL.iter().copied() {
                Steps { direction,
                    Step { color: StepColor::Primary, "Start" }
                    Step { color: StepColor::Primary, "{direction:?}" }
                    Step { "Finish" }
                }
            }
        }
    }
}
