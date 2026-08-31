use dioxus::prelude::*;

use crate::components::file_input::{FileInput, FileInputColor};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in FileInputColor::ALL.iter().copied() {
                FileInput {
                    color,
                    aria_label: format!("{color:?}"),
                }
            }
        }
    }
}
