use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

/// Rectangular and circular placeholders grouped into a loading card.
#[component]
pub fn Example() -> Element {
    rsx! {
        article { class: "flex w-64 flex-col gap-4",
            Skeleton {
                id: "rectangular-placeholder",
                class: "h-32 w-64",
                aria_hidden: "true",
            }
            div { class: "flex items-center gap-4",
                Skeleton { class: "size-16 shrink-0 rounded-full", aria_hidden: "true" }
                div { class: "flex grow flex-col gap-2",
                    Skeleton { class: "h-4 w-full", aria_hidden: "true" }
                    Skeleton { class: "h-4 w-2/3", aria_hidden: "true" }
                }
            }
        }
    }
}
