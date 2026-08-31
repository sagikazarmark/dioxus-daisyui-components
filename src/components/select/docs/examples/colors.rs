use dioxus::prelude::*;

use crate::components::select::{Select, SelectColor, SelectList, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// Every value of the trigger's colour axis, read off closed selects: what it
/// colours is the field, not the list under it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-4",
            for color in SelectColor::ALL.iter().copied() {
                Select::<String> {
                    SelectTrigger { color,
                        SelectValue { placeholder: "{color:?}" }
                    }
                    SelectList {
                        Options {}
                    }
                }
            }
        }
    }
}
