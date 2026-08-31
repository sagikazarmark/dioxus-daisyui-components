use dioxus::prelude::*;

use crate::components::hover_card::{
    HoverCard, HoverCardPanel, HoverCardSide, HoverCardTitle, HoverCardTrigger,
};

/// Two cards that open the way a reader opens them: by pointing at the trigger,
/// or by tabbing to it.
///
/// The panel closes as soon as the pointer leaves the trigger, including when
/// where it goes is the panel itself, which the primitive unmounts before its
/// own `mouseenter` can arrive. So a hover card here is something to read
/// rather than something to reach into; the component's documentation records
/// it in full.
///
/// The trigger emits nothing: `link` here is the caller's, and a card hangs off
/// whatever they wrote.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-10 py-6",
            HoverCard { id: "overview",
                HoverCardTrigger { id: "overview-trigger", class: "link", "Ada Lovelace" }
                HoverCardPanel { id: "overview-panel", class: "w-64",
                    HoverCardTitle { "Ada Lovelace" }
                    p {
                        "Wrote the first algorithm intended to be carried out by a machine, in notes on the analytical engine."
                    }
                }
            }

            HoverCard {
                HoverCardTrigger { class: "link", "Grace Hopper" }
                HoverCardPanel { class: "w-64", side: HoverCardSide::Top,
                    HoverCardTitle { "Grace Hopper" }
                    p { "Built the first compiler, and found the first literal bug." }
                }
            }
        }
    }
}
