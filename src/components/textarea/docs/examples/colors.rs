use dioxus::prelude::*;

use crate::components::textarea::{Textarea, TextareaColor};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in TextareaColor::ALL.iter().copied() {
                Textarea {
                    color,
                    aria_label: format!("{color:?}"),
                    placeholder: format!("{color:?}"),
                }
            }
        }
    }
}
