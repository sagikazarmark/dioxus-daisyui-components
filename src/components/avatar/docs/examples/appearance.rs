use dioxus::prelude::*;

use crate::components::avatar::{
    AvatarFallback, AvatarFallbackAppearance, AvatarFrame, AvatarFrameAppearance, AvatarImage,
    AvatarRoot,
};

/// A portrait, as a data URI so that the preview renders the same picture with
/// or without a network.
const PORTRAIT: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2064%2064'%3E%3Crect%20width='64'%20height='64'%20fill='%23570df8'/%3E%3Ccircle%20cx='32'%20cy='25'%20r='12'%20fill='%23ffffff'/%3E%3Cpath%20d='M6%2064c0-14%2012-22%2026-22s26%208%2026%2022z'%20fill='%23ffffff'/%3E%3C/svg%3E";

/// Both appearance axes, which are the utilities this component emits where
/// daisyUI has no class of its own.
///
/// The frame's axis is the size and the shape: daisyUI writes both as Tailwind
/// utilities in its own examples, and `.avatar > div` itself only squares the
/// box and clips it. The fallback's axis is the fill and the text colour daisyUI
/// puts on the frame in its placeholder examples, emitted on the fallback here
/// so that a loaded image is not sitting on top of a painted circle.
///
/// Each `None` emits nothing at all, which is how a caller wins a tie against a
/// utility rather than trying to out-rank it (ADR-0004), and it is the smaller
/// avatar that needs it, since a larger one would have won on stylesheet order
/// by luck.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "frame", class: "flex flex-wrap items-center gap-4",
                for appearance in AvatarFrameAppearance::ALL.iter().copied() {
                    AvatarRoot { aria_label: "Frame: {appearance:?}",
                        AvatarFrame {
                            appearance,
                            class: if appearance == AvatarFrameAppearance::None { "w-10 rounded-box" },
                            AvatarImage { src: PORTRAIT, alt: "" }
                            AvatarFallback { "AL" }
                        }
                    }
                }
            }

            div { "data-axis": "fallback", class: "flex flex-wrap items-center gap-4",
                for appearance in AvatarFallbackAppearance::ALL.iter().copied() {
                    AvatarRoot { aria_label: "Fallback: {appearance:?}",
                        AvatarFrame {
                            AvatarFallback { appearance, "AL" }
                        }
                    }
                }
            }
        }
    }
}
