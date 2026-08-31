use dioxus::prelude::*;

use crate::components::steps::{Step, StepColor, StepIcon, Steps};

/// Custom generated content and a direct custom icon.
#[component]
pub fn Example() -> Element {
    rsx! {
        Steps { id: "custom-markers", class: "w-full",
            Step {
                id: "custom-content-step",
                color: StepColor::Secondary,
                "data-content": "!",
                "Review"
            }
            Step { id: "custom-icon-step", color: StepColor::Secondary,
                StepIcon { id: "custom-icon", aria_hidden: "true", "\u{2713}" }
                "Approved"
            }
            Step { "Publish" }
        }
    }
}
