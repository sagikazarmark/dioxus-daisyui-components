use dioxus::prelude::*;

use crate::components::loading::{Loading, LoadingAnimation, LoadingSize};

/// Decorative and labeled uses of a loading indicator.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Loading { id: "default-loading", aria_hidden: "true" }
            button { class: "btn btn-primary",
                Loading { size: LoadingSize::Sm, aria_hidden: "true" }
                "Save"
            }
            Loading {
                animation: LoadingAnimation::Dots,
                role: "status",
                aria_label: "Loading account",
            }
        }
    }
}
