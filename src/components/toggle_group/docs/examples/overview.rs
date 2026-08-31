use dioxus::prelude::*;

use crate::components::toggle_group::{ToggleGroup, ToggleItem, ToggleItemColor};

/// A formatting toolbar, and a group where only one item can be on.
///
/// The first allows several at once, which is what a formatting row is: bold
/// and italic are not alternatives. The second does not, so pressing one
/// releases the last: a set of radios that happen to look like buttons.
///
/// Each item is named by `aria_label`, because a single letter is not a name a
/// screen reader can read out.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            ToggleGroup {
                horizontal: true,
                allow_multiple_pressed: true,
                default_pressed: [0usize].into_iter().collect(),
                ToggleItem {
                    index: 0usize,
                    color: ToggleItemColor::Primary,
                    aria_label: "Bold",
                    b { "B" }
                }
                ToggleItem {
                    index: 1usize,
                    color: ToggleItemColor::Primary,
                    aria_label: "Italic",
                    i { "I" }
                }
                ToggleItem {
                    index: 2usize,
                    color: ToggleItemColor::Primary,
                    aria_label: "Underline",
                    u { "U" }
                }
            }

            ToggleGroup { horizontal: true, default_pressed: [1usize].into_iter().collect(),
                ToggleItem { index: 0usize, "Day" }
                ToggleItem { index: 1usize, "Week" }
                ToggleItem { index: 2usize, "Month" }
            }
        }
    }
}
