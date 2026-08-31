use dioxus::prelude::*;

use crate::components::popover::{
    Popover, PopoverContent, PopoverContentAppearance, PopoverTrigger,
};

/// A caller's own classes, and the box utilities switched off.
///
/// The first panel keeps this component's box and adds a width to it, and the
/// trigger takes a colour the way any daisyUI button does, through `class`,
/// which concatenates.
///
/// The second switches the box utilities off and paints its own in their place,
/// which is what a caller does when they want to win against a utility rather
/// than against daisyUI (ADR-0004). The positioning is untouched, because that
/// is `dropdown-content`'s and not this component's.
#[component]
pub fn Example() -> Element {
    rsx! {
        // Both panels are held open, so the row is spaced by more than the
        // widest of them: a panel is positioned out of the flow, and the space
        // it needs is the row's rather than its own.
        div { class: "flex flex-wrap items-start gap-x-64 gap-y-40 pb-32",
            Popover { id: "caller-attributes", open: Some(true), is_modal: false,
                PopoverTrigger { class: "btn-primary", "Painted trigger" }
                PopoverContent { id: "caller-panel", class: "w-56",
                    "The width here is the caller's."
                }
            }

            Popover { id: "caller-none", open: Some(true), is_modal: false,
                PopoverTrigger { class: "btn-ghost", "Repainted panel" }
                PopoverContent {
                    id: "caller-repainted",
                    appearance: PopoverContentAppearance::None,
                    class: "bg-primary text-primary-content rounded-none p-6",
                    "The fill here is the caller's, with this component's switched off."
                }
            }
        }
    }
}
