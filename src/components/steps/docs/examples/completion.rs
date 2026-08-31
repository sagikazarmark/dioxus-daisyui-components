use dioxus::prelude::*;

use crate::components::steps::{Step, StepColor, Steps};

/// Completion and current-step meaning supplied by the caller's text and ARIA.
#[component]
pub fn Example() -> Element {
    rsx! {
        Steps { class: "w-full", aria_label: "Release checklist",
            Step { id: "completed-step", color: StepColor::Success,
                span { class: "sr-only", "Completed: " }
                "Build"
            }
            Step { id: "current-step", color: StepColor::Primary, aria_current: "step",
                span { class: "sr-only", "Current step: " }
                "Review"
            }
            Step {
                span { class: "sr-only", "Pending: " }
                "Publish"
            }
        }
    }
}
