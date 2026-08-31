use dioxus::prelude::*;

use crate::components::collapsible::{
    Collapsible, CollapsibleAppearance, CollapsibleContent, CollapsibleTrigger,
    CollapsibleTriggerAppearance,
};

/// Both appearance axes, which are the utilities this component emits where
/// daisyUI has no class of its own.
///
/// The root's axis paints the surface, and the trigger's puts back what a
/// `button` loses against the `div` daisyUI wrote `.collapse-title` for: a
/// start-aligned label and a pointer. Each `None` emits nothing at all, which is
/// how a caller wins a tie against a utility rather than trying to out-rank it
/// (ADR-0004).
///
/// The panels are held open so that the surface is worth looking at.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full flex-col gap-6",
            div { "data-axis": "root", class: "grid items-start gap-3 sm:grid-cols-2",
                for appearance in CollapsibleAppearance::ALL.iter().copied() {
                    Collapsible { appearance, default_open: true,
                        CollapsibleTrigger { "Root: {appearance:?}" }
                        CollapsibleContent { "The fill and the border are this axis." }
                    }
                }
            }

            div { "data-axis": "trigger", class: "grid items-start gap-3 sm:grid-cols-2",
                for appearance in CollapsibleTriggerAppearance::ALL.iter().copied() {
                    Collapsible {
                        CollapsibleTrigger { appearance, "Trigger: {appearance:?}" }
                        CollapsibleContent { "The alignment of the title above is this axis." }
                    }
                }
            }
        }
    }
}
