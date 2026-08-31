use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarStatus};

/// Every value of the status axis.
///
/// daisyUI draws the dot as a `::before` on the avatar itself, sized as a
/// fraction of it and outlined in the page's own surface colour so that it
/// reads against the picture behind it. `Default` draws none, which is an
/// avatar that says nothing about whether the person behind it is around,
/// distinct from offline, which says they are not.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "status", class: "flex flex-wrap items-center gap-4",
            for status in AvatarStatus::ALL.iter().copied() {
                Avatar { label: "{status:?}", status,
                    AvatarFallback { class: "text-xs", "{status:?}" }
                }
            }
        }
    }
}
