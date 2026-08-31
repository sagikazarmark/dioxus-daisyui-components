use dioxus::prelude::*;

use crate::components::textarea::{Textarea, TextareaAppearance, TextareaColor, TextareaSize};

/// Caller classes and native attributes on the rendered textarea.
#[component]
pub fn Example() -> Element {
    rsx! {
        Textarea {
            id: "caller-attributes",
            class: "resize-none",
            color: TextareaColor::Primary,
            size: TextareaSize::Lg,
            appearance: TextareaAppearance::Ghost,
            name: "notes",
            placeholder: "Implementation notes",
            rows: 6,
            required: true,
        }
    }
}
