use dioxus::prelude::*;

use crate::components::hover_card::{
    HoverCard, HoverCardBody, HoverCardContent, HoverCardContentAppearance, HoverCardPositioning,
    HoverCardTitle, HoverCardTrigger,
};

/// A caller's own classes, on the panel and on the body.
///
/// The first one keeps this component's utilities and adds the width daisyUI's
/// own card examples always carry; this component emits none, because a width
/// is the most content-dependent thing about a panel.
///
/// The second drops to the parts, switches the panel's paint off and paints its
/// own in its place, and reaches the body directly to change what it pads by.
/// That is the case the parts exist for: the collapsed component lands caller
/// attributes on the panel, which is the element worth reaching, and the body is
/// only reachable by writing it out.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-4",
            HoverCard { open: Some(true),
                HoverCardTrigger { class: "sr-only", "Caller width" }
                HoverCardContent {
                    id: "caller-attributes",
                    positioning: HoverCardPositioning::None,
                    class: "w-56",
                    HoverCardBody {
                        HoverCardTitle { "A wider panel" }
                        p { "The width here is the caller's." }
                    }
                }
            }

            HoverCard { open: Some(true),
                HoverCardTrigger { class: "sr-only", "Caller paint" }
                HoverCardContent {
                    id: "caller-paint",
                    positioning: HoverCardPositioning::None,
                    appearance: HoverCardContentAppearance::None,
                    class: "bg-primary text-primary-content w-56 rounded-none",
                    HoverCardBody { class: "p-3",
                        HoverCardTitle { "A repainted panel" }
                        p { "The fill and the padding here are the caller's." }
                    }
                }
            }
        }
    }
}
