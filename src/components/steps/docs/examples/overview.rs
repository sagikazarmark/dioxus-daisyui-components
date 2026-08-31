use dioxus::prelude::*;

use crate::components::steps::{Step, StepColor, Steps};

/// A process whose generated counters remain visible.
#[component]
pub fn Example() -> Element {
    rsx! {
        Steps { id: "default-steps", class: "w-full", aria_label: "Account setup",
            Step { color: StepColor::Success, "Create account" }
            Step { color: StepColor::Primary, aria_current: "step", "Choose plan" }
            Step { "Confirm purchase" }
        }
    }
}
