use dioxus::prelude::*;

use crate::components::collapsible::{
    Collapsible, CollapsibleAppearance, CollapsibleContent, CollapsibleMarker, CollapsibleTrigger,
};

/// A caller's own classes and attributes.
///
/// The first one squares off the corner radius daisyUI's `collapse` sets, which
/// is the case worth demonstrating: the two are both single class selectors, and
/// the caller's wins on cascade layers rather than on specificity.
///
/// The second switches this component's surface utilities off and paints its own
/// in their place, which is what a caller does when they want to win against a
/// utility rather than against daisyUI (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full max-w-md flex-col gap-3",
            Collapsible {
                id: "caller-attributes",
                marker: CollapsibleMarker::Arrow,
                class: "rounded-none",
                default_open: true,
                CollapsibleTrigger { "A squared-off disclosure" }
                CollapsibleContent { "The corner radius here is the caller's." }
            }

            Collapsible {
                id: "caller-surface",
                appearance: CollapsibleAppearance::None,
                marker: CollapsibleMarker::Arrow,
                class: "bg-primary text-primary-content rounded-box",
                CollapsibleTrigger { "A repainted disclosure" }
                CollapsibleContent {
                    "The fill here is the caller's, with this component's switched off."
                }
            }
        }
    }
}
