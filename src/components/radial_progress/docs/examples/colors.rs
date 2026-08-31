use dioxus::prelude::*;

use crate::components::radial_progress::RadialProgress;

/// Caller text colours paint both pseudo-elements through `currentColor`.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { id: "color-samples", class: "flex flex-wrap items-center gap-4",
            RadialProgress { value: 65.0, class: "text-primary", aria_label: "Primary progress", "65%" }
            RadialProgress { value: 65.0, class: "text-secondary", aria_label: "Secondary progress", "65%" }
            RadialProgress { value: 65.0, class: "text-success", aria_label: "Successful progress", "65%" }
            RadialProgress { value: 65.0, class: "text-error", aria_label: "Failed progress", "65%" }
        }
    }
}
