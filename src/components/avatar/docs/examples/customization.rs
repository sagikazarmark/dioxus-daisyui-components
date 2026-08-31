use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarFrameAppearance, AvatarImage};

/// A portrait, as a data URI so that the preview renders the same picture with
/// or without a network.
const PORTRAIT: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2064%2064'%3E%3Crect%20width='64'%20height='64'%20fill='%23570df8'/%3E%3Ccircle%20cx='32'%20cy='25'%20r='12'%20fill='%23ffffff'/%3E%3Cpath%20d='M6%2064c0-14%2012-22%2026-22s26%208%2026%2022z'%20fill='%23ffffff'/%3E%3C/svg%3E";

/// A caller's own classes and attributes, which land on the frame.
///
/// The frame is the element daisyUI's avatar is drawn on, so it is the one
/// worth reaching: the first avatar keeps the size and the shape this component
/// emits and adds a ring to them.
///
/// The second is smaller than the default, which is the case the appearance
/// axis exists for. Both widths are utilities in the same layer, so they tie,
/// and a tie is settled by stylesheet order, where `w-10` loses to the `w-16`
/// already on the element. Switching ours off is what wins it (ADR-0004), and
/// the shape goes with it since the two are one axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Avatar {
                label: "Caller classes and attributes",
                class: "ring-2 ring-primary ring-offset-2 ring-offset-base-100",
                id: "caller-attributes",
                AvatarImage { src: PORTRAIT, alt: "" }
                AvatarFallback { "AL" }
            }

            Avatar {
                label: "Caller size",
                appearance: AvatarFrameAppearance::None,
                class: "w-10 rounded-box",
                id: "caller-size",
                AvatarImage { src: PORTRAIT, alt: "" }
                AvatarFallback { "AL" }
            }
        }
    }
}
