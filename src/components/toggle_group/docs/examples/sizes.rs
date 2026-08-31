use dioxus::prelude::*;

use crate::components::toggle_group::{ToggleGroup, ToggleItem, ToggleItemSize};

/// Every value of the size axis, smallest first.
///
/// A row of joined buttons at different sizes is not what a toolbar looks like
/// (the sizes are meant to be picked once for the whole row) but it is what
/// makes each one visible next to the others, and daisyUI's join copes with it:
/// the borders still meet, because the negative margin it pulls them together
/// with is a border width rather than a size.
#[component]
pub fn Example() -> Element {
    rsx! {
        ToggleGroup { "data-axis": "size", horizontal: true, allow_multiple_pressed: true,
            for (index , size) in ToggleItemSize::ALL.iter().copied().enumerate() {
                ToggleItem { index, size, aria_label: "{size:?}", "{size:?}" }
            }
        }
    }
}
