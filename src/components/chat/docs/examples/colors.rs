use dioxus::prelude::*;

use crate::components::chat::{Chat, ChatBubble, ChatBubbleColor, ChatPlacement};

/// Every bubble colour, alternating placements to keep the conversation legible.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-col gap-1",
            for (index , color) in ChatBubbleColor::ALL.iter().copied().enumerate() {
                Chat {
                    placement: if index % 2 == 0 {
                        ChatPlacement::Start
                    } else {
                        ChatPlacement::End
                    },
                    ChatBubble { color, "{color:?}" }
                }
            }
        }
    }
}
