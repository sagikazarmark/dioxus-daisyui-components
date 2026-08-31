use dioxus::prelude::*;

use crate::components::file_input::{FileInput, FileInputAppearance, FileInputColor, FileInputSize};

/// Caller classes and file-specific configuration on the rendered input.
#[component]
pub fn Example() -> Element {
    rsx! {
        FileInput {
            id: "configured-fileinput",
            class: "rounded-none",
            color: FileInputColor::Primary,
            size: FileInputSize::Lg,
            appearance: FileInputAppearance::Ghost,
            accept: "image/png,image/jpeg",
            multiple: true,
            aria_label: "Choose images",
        }
    }
}
