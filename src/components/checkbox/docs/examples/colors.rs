use dioxus::prelude::*;

use crate::components::checkbox::{Checkbox, CheckboxColor, CheckboxState};

/// Every value of the colour axis, rendered checked.
///
/// Checked is the state daisyUI expresses colour in: an unchecked checkbox
/// carries its colour on a hairline border, a checked one fills with it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in CheckboxColor::ALL.iter().copied() {
                Checkbox {
                    color,
                    default_value: CheckboxState::Checked,
                    aria_label: "{color:?}",
                }
            }
        }
    }
}
