use dioxus::prelude::*;

use crate::components::skeleton::{Skeleton, SkeletonText};

/// Caller classes and attributes joined with both skeleton forms.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col items-start gap-4",
            Skeleton {
                id: "caller-block",
                class: "h-12 w-32 rounded-none",
                "data-owner": "caller",
            }
            SkeletonText {
                id: "caller-text",
                class: "text-xl",
                "data-owner": "caller",
                "Caller text survives"
            }
        }
    }
}
