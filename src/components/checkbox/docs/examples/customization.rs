use dioxus::prelude::*;

use crate::components::checkbox::{Checkbox, CheckboxColor, CheckboxState};

/// A caller's own classes and attributes.
///
/// The class squares off a corner radius daisyUI itself sets, which is the case
/// worth demonstrating: the two are both single class selectors, and the
/// caller's wins on cascade layers rather than on specificity.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Checkbox {
                color: CheckboxColor::Primary,
                default_value: CheckboxState::Checked,
                class: "rounded-none",
                id: "caller-attributes",
                aria_label: "Caller classes and attributes",
            }
        }
    }
}
