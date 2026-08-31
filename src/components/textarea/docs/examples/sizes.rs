use dioxus::prelude::*;

use crate::components::textarea::{Textarea, TextareaSize};

/// Every value of the size axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in TextareaSize::ALL.iter().copied() {
                Textarea {
                    size,
                    aria_label: format!("{size:?}"),
                    placeholder: format!("{size:?}"),
                }
            }
        }
    }
}
