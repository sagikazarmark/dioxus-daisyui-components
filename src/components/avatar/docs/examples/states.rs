use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarState};

/// A portrait, as a data URI so that the preview renders the same picture with
/// or without a network.
const PORTRAIT: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2064%2064'%3E%3Crect%20width='64'%20height='64'%20fill='%23570df8'/%3E%3Ccircle%20cx='32'%20cy='25'%20r='12'%20fill='%23ffffff'/%3E%3Cpath%20d='M6%2064c0-14%2012-22%2026-22s26%208%2026%2022z'%20fill='%23ffffff'/%3E%3C/svg%3E";

/// The three states an avatar arrives in, and the one class that follows them.
///
/// An avatar that loads its image shows it. One with no image at all, and one
/// whose image fails, both show the fallback, and both carry daisyUI's
/// `avatar-placeholder`, which is the class this component mirrors the
/// primitive's state to emit.
///
/// The last one reports every transition it goes through. The state travels out
/// through the same callback this component reads, so what is counted here is
/// what the class is emitted from.
#[component]
pub fn Example() -> Element {
    let mut state = use_signal(|| AvatarState::Empty);

    rsx! {
        div { class: "flex flex-wrap items-center gap-6",
            div { class: "flex flex-col items-center gap-2",
                Avatar { id: "avatar-loaded", label: "Loaded",
                    AvatarImage { src: PORTRAIT, alt: "" }
                    AvatarFallback { "AL" }
                }
                span { class: "text-xs opacity-70", "Loaded" }
            }

            div { class: "flex flex-col items-center gap-2",
                Avatar { id: "avatar-empty", label: "No image",
                    AvatarFallback { "GH" }
                }
                span { class: "text-xs opacity-70", "No image" }
            }

            div { class: "flex flex-col items-center gap-2",
                Avatar { id: "avatar-broken", label: "Failed",
                    AvatarImage { src: "/no-such-portrait.png", alt: "" }
                    AvatarFallback { "KJ" }
                }
                span { class: "text-xs opacity-70", "Failed to load" }
            }

            div { class: "flex flex-col items-center gap-2",
                Avatar {
                    id: "avatar-watched",
                    label: "Watched",
                    on_state_change: move |next| state.set(next),
                    AvatarImage { src: PORTRAIT, alt: "" }
                    AvatarFallback { "WA" }
                }
                span { class: "text-xs opacity-70",
                    "State: "
                    span { "data-testid": "state", "{state():?}" }
                }
            }
        }
    }
}
