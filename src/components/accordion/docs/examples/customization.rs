use dioxus::prelude::*;

use crate::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionItemAppearance, AccordionItemMarker,
    AccordionTrigger,
};

/// A caller's own classes and attributes.
///
/// The first item squares off the corner radius daisyUI's `collapse` sets,
/// which is the case worth demonstrating: the two are both single class
/// selectors, and the caller's wins on cascade layers rather than on
/// specificity.
///
/// The second switches this component's surface utilities off and paints its
/// own in their place, which is what a caller does when they want to win
/// against a utility rather than against daisyUI (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        Accordion {
            AccordionItem {
                index: 0usize,
                id: "caller-attributes",
                marker: AccordionItemMarker::Arrow,
                class: "rounded-none",
                default_open: true,
                AccordionTrigger { "A squared-off item" }
                AccordionContent { "The corner radius here is the caller's." }
            }

            AccordionItem {
                index: 1usize,
                id: "caller-surface",
                appearance: AccordionItemAppearance::None,
                marker: AccordionItemMarker::Arrow,
                class: "bg-primary text-primary-content rounded-box",
                AccordionTrigger { "A repainted item" }
                AccordionContent { "The fill here is the caller's, with this component's switched off." }
            }
        }
    }
}
