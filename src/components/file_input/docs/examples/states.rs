use dioxus::prelude::*;

use crate::components::file_input::{FileInput, FileInputColor};

/// Enabled and disabled file inputs with identical Component axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            FileInput {
                id: "enabled-fileinput",
                color: FileInputColor::Primary,
                aria_label: "Enabled file input",
            }
            FileInput {
                id: "disabled-fileinput",
                color: FileInputColor::Primary,
                aria_label: "Disabled file input",
                disabled: true,
            }
        }
    }
}
