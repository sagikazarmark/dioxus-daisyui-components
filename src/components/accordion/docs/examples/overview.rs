use dioxus::prelude::*;

use crate::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionItemMarker, AccordionTrigger,
};

/// Three questions, the first of them answered.
///
/// One item is open at a time, which is the primitive's default: opening the
/// second closes the first. The panel that is closed is not merely hidden: the
/// primitive mounts a panel when its item opens and unmounts it once the
/// closing animation has run.
#[component]
pub fn Example() -> Element {
    let entries = [
        (
            "What does the registry install?",
            "One directory per component, copied into your own source tree, with the Cargo dependency added for you.",
        ),
        (
            "Does it ship any CSS?",
            "None. Components emit daisyUI class names and nothing else, so your theme restyles all of them at once.",
        ),
        (
            "Can I edit what I install?",
            "It is your code once it lands. That is the point of a registry rather than a crate.",
        ),
    ];

    rsx! {
        Accordion {
            for (index , (question , answer)) in entries.into_iter().enumerate() {
                AccordionItem {
                    index,
                    marker: AccordionItemMarker::Arrow,
                    default_open: index == 0,
                    AccordionTrigger { "{question}" }
                    AccordionContent { "{answer}" }
                }
            }
        }
    }
}
