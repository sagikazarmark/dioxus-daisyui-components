use dioxus::prelude::*;

use crate::components::chat::{Chat, ChatBubble, ChatPlacement};

/// Both logical placements under left-to-right and right-to-left direction.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "grid gap-6 md:grid-cols-2",
            div { dir: "ltr", class: "rounded-box border border-base-300 p-4",
                p { class: "mb-2 text-sm font-semibold", "LTR" }
                Chat { id: "ltr-start", placement: ChatPlacement::Start,
                    ChatBubble { "Start" }
                }
                Chat { id: "ltr-end", placement: ChatPlacement::End,
                    ChatBubble { "End" }
                }
            }
            div { dir: "rtl", class: "rounded-box border border-base-300 p-4",
                p { class: "mb-2 text-sm font-semibold", "RTL" }
                Chat { id: "rtl-start", placement: ChatPlacement::Start,
                    ChatBubble { "Start" }
                }
                Chat { id: "rtl-end", placement: ChatPlacement::End,
                    ChatBubble { "End" }
                }
            }
        }
    }
}
