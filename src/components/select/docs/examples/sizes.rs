use dioxus::prelude::*;

use crate::components::select::{Select, SelectList, SelectSize, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// Every value of the trigger's size axis, smallest to largest. It sizes the
/// field; the list has a size axis of its own.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-4",
            for size in SelectSize::ALL.iter().copied() {
                Select::<String> {
                    SelectTrigger { size,
                        SelectValue { placeholder: "{size:?}" }
                    }
                    SelectList {
                        Options {}
                    }
                }
            }
        }
    }
}
