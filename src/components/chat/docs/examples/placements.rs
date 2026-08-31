use dioxus::prelude::*;

use crate::components::chat::{
    Chat, ChatBubble, ChatFooter, ChatHeader, ChatImage, ChatPlacement,
};

/// A local portrait keeps the Preview deterministic without importing Avatar.
const PORTRAIT: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2064%2064'%3E%3Crect%20width='64'%20height='64'%20fill='%23570df8'/%3E%3Ccircle%20cx='32'%20cy='25'%20r='12'%20fill='%23ffffff'/%3E%3Cpath%20d='M6%2064c0-14%2012-22%2026-22s26%208%2026%2022z'%20fill='%23ffffff'/%3E%3C/svg%3E";

/// Both placements with all four structural parts kept directly under Chat.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "placement", class: "flex flex-col gap-3",
            for placement in ChatPlacement::ALL.iter().copied() {
                Chat { placement,
                    ChatImage { class: "avatar",
                        div { class: "w-10 rounded-full",
                            img { src: PORTRAIT, alt: "Ada Lovelace" }
                        }
                    }
                    ChatHeader {
                        strong { "{placement:?} message" }
                        time { class: "opacity-50", datetime: "10:42", "10:42" }
                    }
                    ChatBubble { "The message follows the logical {placement:?} edge." }
                    ChatFooter {
                        span { class: "badge badge-ghost badge-sm", "Delivered" }
                    }
                }
            }
        }
    }
}
