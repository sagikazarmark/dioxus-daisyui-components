use dioxus::prelude::*;

use crate::components::radial_progress::RadialProgress;

/// A radial progress indicator with caller-owned content inside its ring.
#[component]
pub fn Example() -> Element {
    rsx! {
        RadialProgress {
            value: 70.0,
            class: "text-primary",
            aria_label: "Upload progress",
            strong { "70%" }
        }
    }
}
