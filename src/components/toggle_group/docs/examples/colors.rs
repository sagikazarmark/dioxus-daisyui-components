use dioxus::prelude::*;

use crate::components::toggle_group::{ToggleGroup, ToggleItem, ToggleItemColor};

/// Every value of the colour axis, on one joined row.
///
/// The axis is on the item rather than on the group, because daisyUI's classes
/// are per-button: a joined row is a row of buttons that happen to be joined.
/// These are rendered unpressed, which is the state the colour is plainest in;
/// a pressed button is the same colour darkened.
#[component]
pub fn Example() -> Element {
    rsx! {
        ToggleGroup { "data-axis": "color", horizontal: true, allow_multiple_pressed: true,
            for (index , color) in ToggleItemColor::ALL.iter().copied().enumerate() {
                ToggleItem { index, color, aria_label: "{color:?}", "{color:?}" }
            }
        }
    }
}
