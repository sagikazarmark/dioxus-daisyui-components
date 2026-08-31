use dioxus::prelude::*;

use crate::components::select::{Select, SelectList, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// A disabled select, a select that starts on a value, and a caller's classes
/// on the box.
///
/// The disabled one needs no class of its own: the primitive puts the attribute
/// on the trigger and daisyUI's `.select` matches it. The chosen row does need
/// one: `.menu` matches no ARIA attribute at all, so `menu-active` is the only
/// thing telling it from the rest, and it is emitted from the value this
/// component lifted (ADR-0006).
///
/// The caller's class is a width, which is the case worth demonstrating: it is
/// the class that says whether the split reached the options: a width on the
/// box that the list inside it did not take would leave the options
/// shrink-wrapped in a wider box.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8 pb-72",
            Select::<String> { id: "disabled", disabled: true,
                SelectTrigger {
                    SelectValue { placeholder: "Disabled" }
                }
                SelectList {
                    Options {}
                }
            }

            Select::<String> {
                id: "selected",
                default_value: "Lemon".to_string(),
                open: Some(true),
                SelectTrigger {
                    SelectValue {}
                }
                SelectList {
                    Options {}
                }
            }

            Select::<String> { open: Some(true),
                SelectTrigger {
                    SelectValue { placeholder: "Caller" }
                }
                SelectList { id: "caller-attributes", class: "w-52",
                    Options {}
                }
            }
        }
    }
}
