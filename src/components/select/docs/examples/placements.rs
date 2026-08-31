use dioxus::prelude::*;

use crate::components::select::{Select, SelectList, SelectSide, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// Every value of the popup's placement axis, which is the dropdown's axis
/// borrowed along with its structure.
///
/// One select per value, all of them held open by the caller (the primitive
/// closes a popup that nothing in it is focused) with room above and below for
/// the ones that open upwards.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "side",
            class: "flex flex-wrap items-center justify-center gap-40 py-40",
            for side in SelectSide::ALL.iter().copied() {
                Select::<String> { side, open: Some(true),
                    SelectTrigger {
                        SelectValue { placeholder: "{side:?}" }
                    }
                    SelectList {
                        Options {}
                    }
                }
            }
        }
    }
}
