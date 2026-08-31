use dioxus::prelude::*;

use crate::components::textarea::{Textarea, TextareaAppearance};

/// Every value of the appearance axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-2",
            for appearance in TextareaAppearance::ALL.iter().copied() {
                Textarea {
                    appearance,
                    aria_label: format!("{appearance:?}"),
                    placeholder: format!("{appearance:?}"),
                }
            }
        }
    }
}
