use dioxus::prelude::*;

use crate::components::label::{Label, LabelAppearance};

/// A caller's own classes and attributes.
///
/// The first repaints the muting daisyUI's `label` applies, which is the case
/// worth demonstrating: both are single class selectors, and the caller's wins
/// on cascade layers rather than on specificity.
///
/// The second switches the class off altogether and writes its own caption,
/// which is what `None` is for: a label styled by the caller rather than
/// against daisyUI.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            div { class: "flex flex-col gap-1",
                Label {
                    id: "caller-attributes",
                    html_for: "caller-required",
                    class: "text-error font-semibold",
                    "Crate name (required)"
                }
                input { id: "caller-required", class: "input", placeholder: "my-app" }
            }

            div { class: "flex flex-col gap-1",
                Label {
                    id: "caller-none",
                    appearance: LabelAppearance::None,
                    html_for: "caller-notes",
                    class: "text-xs tracking-wide uppercase",
                    "Notes"
                }
                input { id: "caller-notes", class: "input", placeholder: "Anything else" }
            }
        }
    }
}
