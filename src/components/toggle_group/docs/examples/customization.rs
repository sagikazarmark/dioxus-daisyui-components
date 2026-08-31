use dioxus::prelude::*;

use crate::components::toggle_group::{ToggleGroup, ToggleItem, ToggleItemColor};

/// A caller's own classes and attributes, on the group and on an item.
///
/// The group's class widens the row and the item's squares a corner daisyUI's
/// join rounds; both of them daisyUI component classes on the other side, so
/// the caller wins on cascade layers rather than on specificity (ADR-0004).
///
/// The looks the button component does not expose as axes are reached the same
/// way: `btn-outline` here is a class rather than a prop, and it concatenates
/// with the ones the item already carries.
#[component]
pub fn Example() -> Element {
    rsx! {
        ToggleGroup {
            class: "w-64",
            id: "caller-group",
            horizontal: true,
            allow_multiple_pressed: true,
            default_pressed: [0usize].into_iter().collect(),
            ToggleItem {
                index: 0usize,
                color: ToggleItemColor::Primary,
                class: "flex-1 rounded-s-none",
                id: "caller-attributes",
                "Pressed"
            }
            ToggleItem {
                index: 1usize,
                color: ToggleItemColor::Primary,
                class: "flex-1 btn-outline",
                id: "caller-outline",
                "Outlined"
            }
        }
    }
}
