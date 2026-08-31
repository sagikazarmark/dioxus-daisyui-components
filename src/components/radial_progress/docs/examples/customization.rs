use dioxus::prelude::*;

use crate::components::radial_progress::RadialProgress;

/// Caller styles compose around the Registry-owned value declaration.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-5",
            RadialProgress {
                id: "default-geometry",
                value: 25.0,
                aria_label: "Default geometry",
                "default"
            }
            RadialProgress {
                id: "custom-size",
                value: 45.0,
                style: "--size: 7rem;",
                aria_label: "Custom size",
                "size"
            }
            RadialProgress {
                id: "custom-thickness",
                value: 60.0,
                style: "--thickness: 1rem;",
                aria_label: "Custom thickness",
                "thick"
            }
            RadialProgress {
                id: "caller-attributes",
                value: 70.0,
                class: "text-secondary",
                "STYLE": "--size: 6rem; --thickness: 0.5rem; --value: 1 !important; outline-offset: 3px;",
                "data-owner": "caller",
                "DATA-STATE": "caller",
                "Data-Value": "1",
                "data-MAX": "1",
                "ROLE": "meter",
                "ARIA-VALUEMIN": "1",
                "aria-VALUEMAX": "1",
                "Aria-ValueNow": "1",
                aria_label: "Caller styles and attributes",
                span { "70% caller" }
            }
        }
    }
}
