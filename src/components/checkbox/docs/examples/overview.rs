use dioxus::prelude::*;

use crate::components::checkbox::{Checkbox, CheckboxColor, CheckboxState};

/// A checkbox beside the text that names it.
///
/// The primitive renders a `button` with `role="checkbox"` rather than an
/// `input`, so a wrapping `label` would not reach it; the text is tied to the
/// checkbox with `aria-labelledby` instead, which is what a screen reader
/// announces it by.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            div { class: "flex items-center gap-2",
                Checkbox { aria_labelledby: "overview-releases" }
                span { id: "overview-releases", class: "text-sm", "Email me about releases" }
            }

            div { class: "flex items-center gap-2",
                Checkbox {
                    color: CheckboxColor::Primary,
                    default_value: CheckboxState::Checked,
                    aria_labelledby: "overview-security",
                }
                span { id: "overview-security", class: "text-sm", "Email me about security advisories" }
            }
        }
    }
}
