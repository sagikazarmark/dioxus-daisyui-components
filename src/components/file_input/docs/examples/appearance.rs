use dioxus::prelude::*;

use crate::components::file_input::{FileInput, FileInputAppearance};

/// Every value of the appearance axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-2",
            for appearance in FileInputAppearance::ALL.iter().copied() {
                FileInput {
                    appearance,
                    aria_label: format!("{appearance:?}"),
                }
            }
        }
    }
}
