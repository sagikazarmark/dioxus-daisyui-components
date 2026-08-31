use dioxus::prelude::*;

use crate::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionItemMarker, AccordionTrigger,
};

/// Every value of the marker axis, one item each.
///
/// daisyUI draws the marker in a pseudo-element of the title rather than on the
/// item that carries the class, and turns it as the item opens, so each of
/// these is worth clicking as well as looking at.
#[component]
pub fn Example() -> Element {
    rsx! {
        Accordion { "data-axis": "marker",
            for (index , marker) in AccordionItemMarker::ALL.iter().copied().enumerate() {
                AccordionItem { index, marker,
                    AccordionTrigger { "{marker:?}" }
                    AccordionContent { "The marker is drawn on the title, from the class on the item." }
                }
            }
        }
    }
}
