use dioxus::prelude::*;

use crate::components::chat::{
    Chat, ChatBubble, ChatBubbleColor, ChatFooter, ChatHeader, ChatImage, ChatPlacement,
};

/// Caller classes and attributes joined with all five parts' own.
#[component]
pub fn Example() -> Element {
    rsx! {
        Chat {
            id: "caller-chat",
            placement: ChatPlacement::End,
            class: "rounded-box bg-base-200 p-4",
            "data-owner": "caller-chat",
            ChatImage {
                id: "caller-image",
                class: "avatar opacity-75",
                "data-owner": "caller-image",
                div { class: "grid w-10 place-items-center rounded-full bg-primary text-primary-content",
                    span { "AL" }
                }
            }
            ChatHeader {
                id: "caller-header",
                class: "font-semibold",
                "data-owner": "caller-header",
                "Ada Lovelace"
            }
            ChatBubble {
                id: "caller-bubble",
                color: ChatBubbleColor::Accent,
                class: "rounded-none italic",
                "data-owner": "caller-bubble",
                aria_label: "Caller-owned bubble",
                "Every attribute survives."
            }
            ChatFooter {
                id: "caller-footer",
                class: "opacity-60",
                "data-owner": "caller-footer",
                "Delivered"
            }
        }
    }
}
