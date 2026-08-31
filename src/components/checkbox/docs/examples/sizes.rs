use dioxus::prelude::*;

use crate::components::checkbox::{Checkbox, CheckboxSize};

/// Every value of the size axis, smallest to largest. Left unchecked, since
/// size is the box rather than the mark.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in CheckboxSize::ALL.iter().copied() {
                Checkbox { size, aria_label: "{size:?}" }
            }
        }
    }
}
