use dioxus::prelude::*;

use crate::components::toggle_group::{ToggleGroup, ToggleItem};

/// A row and a column, which is one prop deciding two things.
///
/// `horizontal` is what the primitive listens to (left and right move through
/// a row, up and down through a column) and it is also what this component
/// emits daisyUI's orientation class from. An unclassed `.join` is a row
/// whatever the keyboard is doing, which is why the class is emitted for both
/// values rather than only for the one that departs from daisyUI's default
/// (ADR-0008).
///
/// The column is the primitive's own default, and the row is what a toolbar
/// asks for.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8",
            ToggleGroup {
                id: "group-horizontal",
                horizontal: true,
                allow_multiple_pressed: true,
                ToggleItem { index: 0usize, "Left" }
                ToggleItem { index: 1usize, "Middle" }
                ToggleItem { index: 2usize, "Right" }
            }

            ToggleGroup { id: "group-vertical", allow_multiple_pressed: true,
                ToggleItem { index: 0usize, "Top" }
                ToggleItem { index: 1usize, "Centre" }
                ToggleItem { index: 2usize, "Bottom" }
            }
        }
    }
}
