use dioxus::prelude::*;

use crate::components::accordion::{
    Accordion, AccordionAppearance, AccordionContent, AccordionItem, AccordionItemAppearance,
    AccordionTrigger, AccordionTriggerAppearance,
};

/// All three appearance axes, which are the utilities this component emits
/// where daisyUI has no class of its own.
///
/// The set's axis stacks the items, the item's paints its surface, and the
/// trigger's puts back what a `button` loses against the `div` daisyUI wrote
/// `.collapse-title` for: a start-aligned label and a pointer. Each `None`
/// emits nothing at all, which is how a caller wins a tie against a utility
/// rather than trying to out-rank it (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "set", class: "grid grid-cols-2 items-start gap-4",
                for appearance in AccordionAppearance::ALL.iter().copied() {
                    Accordion { appearance,
                        AccordionItem { index: 0usize,
                            AccordionTrigger { "Set: {appearance:?}" }
                            AccordionContent { "The gap between items is this axis." }
                        }
                        AccordionItem { index: 1usize,
                            AccordionTrigger { "And a second item" }
                            AccordionContent { "Which is what there is a gap between." }
                        }
                    }
                }
            }

            Accordion { "data-axis": "item", class: "grid grid-cols-2 items-start",
                for (index , appearance) in AccordionItemAppearance::ALL.iter().copied().enumerate() {
                    AccordionItem { index, appearance, default_open: true,
                        AccordionTrigger { "Item: {appearance:?}" }
                        AccordionContent { "The fill and the border are this axis." }
                    }
                }
            }

            Accordion { "data-axis": "trigger", class: "grid grid-cols-2 items-start",
                for (index , appearance) in AccordionTriggerAppearance::ALL.iter().copied().enumerate() {
                    AccordionItem { index,
                        AccordionTrigger { appearance, "Trigger: {appearance:?}" }
                        AccordionContent { "The alignment of the title above is this axis." }
                    }
                }
            }
        }
    }
}
