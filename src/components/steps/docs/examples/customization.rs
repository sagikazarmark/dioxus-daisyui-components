use dioxus::prelude::*;

use crate::components::steps::{Step, StepColor, StepIcon, Steps, StepsDirection};

/// Caller classes and attributes joined with every Compound part's own.
#[component]
pub fn Example() -> Element {
    rsx! {
        Steps {
            id: "caller-steps",
            class: "gap-4",
            direction: StepsDirection::Vertical,
            "data-owner": "caller",
            aria_label: "Caller-owned process",
            Step {
                id: "caller-step",
                class: "font-bold",
                color: StepColor::Warning,
                "data-content": "!",
                "data-owner": "caller",
                aria_current: "step",
                title: "Waiting for approval",
                "Approve"
            }
            Step { color: StepColor::Error,
                StepIcon {
                    id: "caller-icon",
                    class: "size-10",
                    "data-owner": "caller",
                    aria_hidden: "true",
                    "X"
                }
                "Rejected"
            }
        }
    }
}
