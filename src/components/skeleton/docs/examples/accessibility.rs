use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

/// Decorative placeholders inside the region whose content is updating.
#[component]
pub fn Example() -> Element {
    rsx! {
        section {
            class: "flex w-64 flex-col gap-3",
            aria_busy: "true",
            aria_live: "polite",
            aria_labelledby: "loading-account",
            h3 { id: "loading-account", class: "sr-only", "Loading account" }
            div { class: "flex items-center gap-3", aria_hidden: "true",
                Skeleton { class: "size-12 shrink-0 rounded-full" }
                div { class: "flex grow flex-col gap-2",
                    Skeleton { class: "h-4 w-full" }
                    Skeleton { class: "h-4 w-3/4" }
                }
            }
        }
    }
}
