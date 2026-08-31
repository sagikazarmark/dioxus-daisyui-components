use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarStatus};

/// A portrait, as a data URI so that the preview renders the same picture with
/// or without a network. A real app points `src` at a real file.
const PORTRAIT: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2064%2064'%3E%3Crect%20width='64'%20height='64'%20fill='%23570df8'/%3E%3Ccircle%20cx='32'%20cy='25'%20r='12'%20fill='%23ffffff'/%3E%3Cpath%20d='M6%2064c0-14%2012-22%2026-22s26%208%2026%2022z'%20fill='%23ffffff'/%3E%3C/svg%3E";

/// A portrait, the initials shown when there is none, and a status dot.
///
/// Every one of these is named through `label`. The primitive gives the avatar
/// `role="img"`, and an image is named by its author rather than by what is
/// inside it, so an avatar with no label is announced as "image" and nothing
/// else, however good the initials look.
///
/// The fallback is written even where the image is certain to load. The
/// primitive renders a `??` of its own for an avatar that has an image and no
/// fallback, and that stand-in lands outside the frame, where daisyUI's rules
/// do not reach it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Avatar { label: "Ada Lovelace",
                AvatarImage { src: PORTRAIT, alt: "" }
                AvatarFallback { "AL" }
            }

            Avatar { label: "Grace Hopper",
                AvatarFallback { "GH" }
            }

            Avatar { label: "Katherine Johnson", status: AvatarStatus::Online,
                AvatarImage { src: PORTRAIT, alt: "" }
                AvatarFallback { "KJ" }
            }
        }
    }
}
