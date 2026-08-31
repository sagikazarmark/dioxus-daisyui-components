use dioxus::prelude::*;

use crate::components::hover_card::{
    HoverCard, HoverCardAppearance, HoverCardContentAppearance, HoverCardPanel, HoverCardPositioning,
    HoverCardTrigger,
};

/// The two appearance axes and the positioning axis, which between them are every
/// utility this component emits where daisyUI has no class of its own.
///
/// The root's appearance makes it the box the panel is positioned against, which
/// is what daisyUI's `dropdown` would have done. The panel's appearance paints
/// it: daisyUI's `card` rounds a box and lays it out, and the fill and the
/// shadow in its own examples are utilities beside it.
/// The placement axis takes the panel out of the flow and puts it on the side
/// the axes name; switched off, the panel lands where the document would have
/// put it, which is what a caller positioning it themselves needs.
///
/// Each `None` emits nothing at all, which is how a caller wins a tie against a
/// utility rather than trying to out-rank it (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "appearance", class: "flex flex-wrap items-start gap-4",
                for appearance in HoverCardContentAppearance::ALL.iter().copied() {
                    HoverCard { open: Some(true),
                        HoverCardTrigger { class: "sr-only", "{appearance:?}" }
                        HoverCardPanel {
                            appearance,
                            positioning: HoverCardPositioning::None,
                            class: "w-40",
                            "Panel: {appearance:?}"
                        }
                    }
                }
            }

            div { "data-axis": "root", class: "flex flex-wrap items-start gap-4",
                for appearance in HoverCardAppearance::ALL.iter().copied() {
                    HoverCard {
                        appearance,
                        class: "border-base-300 rounded-box border p-2",
                        HoverCardTrigger { class: "text-sm", "Root: {appearance:?}" }
                    }
                }
            }

            div { "data-axis": "positioning", class: "flex flex-wrap items-start gap-4",
                for positioning in HoverCardPositioning::ALL.iter().copied() {
                    HoverCard { open: Some(true), class: "border-base-300 rounded-box border p-2",
                        HoverCardTrigger { class: "text-sm", "Placement: {positioning:?}" }
                        HoverCardPanel { positioning, class: "w-40", "This panel." }
                    }
                }
            }
        }
    }
}
