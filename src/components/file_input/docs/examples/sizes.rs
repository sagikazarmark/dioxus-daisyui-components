use dioxus::prelude::*;

use crate::components::file_input::{FileInput, FileInputSize};

/// Every value of the size axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in FileInputSize::ALL.iter().copied() {
                FileInput {
                    size,
                    aria_label: format!("{size:?}"),
                }
            }
        }
    }
}
