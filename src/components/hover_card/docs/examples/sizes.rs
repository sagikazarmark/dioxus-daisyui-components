use dioxus::prelude::*;

use crate::components::hover_card::{
    HoverCard, HoverCardBorder, HoverCardPanel, HoverCardPositioning, HoverCardSize, HoverCardTitle,
    HoverCardTrigger,
};

/// Every value of the size axis and every value of the border axis, which are
/// daisyUI's own two axes for a card.
///
/// The size sets the body's padding and the title's size rather than anything on
/// the card itself, which is why each panel here carries a title as well as a
/// line of text.
///
/// The panels are held open and, unlike the placement rows, left **in the flow**:
/// the positioning axis is switched off, so a row of them reads as a row of cards
/// rather than as four panels stacked on one trigger.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "size", class: "flex flex-wrap items-start gap-4",
                for size in HoverCardSize::ALL.iter().copied() {
                    HoverCard { open: Some(true),
                        HoverCardTrigger { class: "sr-only", "{size:?}" }
                        HoverCardPanel {
                            size,
                            positioning: HoverCardPositioning::None,
                            class: "w-40",
                            HoverCardTitle { "{size:?}" }
                            p { "A panel at this size." }
                        }
                    }
                }
            }

            div { "data-axis": "border", class: "flex flex-wrap items-start gap-4",
                for border in HoverCardBorder::ALL.iter().copied() {
                    HoverCard { open: Some(true),
                        HoverCardTrigger { class: "sr-only", "{border:?}" }
                        HoverCardPanel {
                            border,
                            positioning: HoverCardPositioning::None,
                            class: "w-40",
                            HoverCardTitle { "{border:?}" }
                            p { "The edge is this axis." }
                        }
                    }
                }
            }
        }
    }
}
