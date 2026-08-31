use dioxus::prelude::*;

use crate::components::hover_card::{
    HoverCard, HoverCardAlign, HoverCardPanel, HoverCardSide, HoverCardTrigger,
};

/// Every value of the side axis, and every value of the alignment axis, with
/// all of the cards held open.
///
/// A card that is open because its caller says so is the only kind that can
/// stand still to be looked at: a real one is open while the pointer is on it.
/// They are spaced far apart because a panel is positioned out of the flow:
/// the room it needs is padding on the row rather than height of its own.
///
/// Neither axis is a daisyUI class. daisyUI's only floating box is the dropdown,
/// and ADR-0015 records why a hover card cannot be one; these are the utilities
/// that place the panel instead, and the same values are what the primitive is
/// told, so `data-side` and `data-align` agree with where the panel is.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-16",
            div {
                "data-axis": "side",
                class: "flex flex-wrap items-center justify-center gap-32 py-24",
                for side in HoverCardSide::ALL.iter().copied() {
                    HoverCard { open: Some(true),
                        HoverCardTrigger { class: "link", "{side:?}" }
                        HoverCardPanel { side, class: "w-40", "On the {side:?} side." }
                    }
                }
            }

            div {
                "data-axis": "align",
                class: "flex flex-wrap items-center justify-center gap-48 py-24",
                for align in HoverCardAlign::ALL.iter().copied() {
                    HoverCard { open: Some(true),
                        HoverCardTrigger { class: "link", "{align:?}" }
                        HoverCardPanel { align, class: "w-40", "Aligned {align:?}." }
                    }
                }
            }
        }
    }
}
