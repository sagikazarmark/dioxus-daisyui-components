use dioxus::prelude::*;

use crate::components::select::{Select, SelectAlign, SelectList, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// Every value of the alignment axis, which moves a popup along the side the
/// placement axis opened it on.
///
/// Every one of these is placed the default way, so the row only needs room
/// under it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "align",
            class: "flex flex-wrap items-center justify-center gap-40 pb-48",
            for align in SelectAlign::ALL.iter().copied() {
                Select::<String> { align, open: Some(true),
                    SelectTrigger {
                        SelectValue { placeholder: "{align:?}" }
                    }
                    SelectList {
                        Options {}
                    }
                }
            }
        }
    }
}
