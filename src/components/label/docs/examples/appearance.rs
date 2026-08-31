use dioxus::prelude::*;

use crate::components::label::{Label, LabelAppearance};

/// Every value of the appearance axis, each naming a field of its own.
///
/// daisyUI has three label classes for one element, so they are values of one
/// axis: the current caption beside a control, the legacy `fieldset-label`, and
/// the one that wraps its control and lifts its own text as the field fills.
/// `None` emits nothing, which is a caption the caller styles.
///
/// The floating one is written the way daisyUI wants it (a `span` and then the
/// control, both inside the label) because that is what its rules reach for.
/// The others are written beside their control, which is what theirs reach for.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-end gap-6",
            for appearance in LabelAppearance::ALL.iter().copied() {
                Label {
                    appearance,
                    html_for: "appearance-{appearance:?}",
                    if appearance == LabelAppearance::Floating {
                        span { "{appearance:?}" }
                        input {
                            id: "appearance-{appearance:?}",
                            class: "input input-sm",
                            placeholder: "{appearance:?}",
                        }
                    } else {
                        "{appearance:?}"
                    }
                }
            }
        }
    }
}
