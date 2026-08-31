use dioxus::prelude::*;

use crate::components::skeleton::SkeletonText;

/// Text content clipped out of the skeleton gradient.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex max-w-md flex-col gap-2",
            SkeletonText {
                id: "text-placeholder",
                class: "text-2xl font-bold",
                aria_hidden: "true",
                "Preparing your account details"
            }
            SkeletonText { aria_hidden: "true",
                "Your preferences and recent activity will appear here when loading finishes."
            }
        }
    }
}
