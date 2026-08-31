use dioxus::prelude::*;
use std::collections::HashSet;

use crate::components::toggle_group::{ToggleGroup, ToggleItem, ToggleItemColor};

/// Every state an item renders in, and one group this page drives.
///
/// Pressed and unpressed differ by a class this component emits: daisyUI marks
/// a pressed button with `btn-active` and matches `aria-pressed` nowhere, so the
/// state has to be lifted out of the primitive to be styled at all (ADR-0006).
/// Disabled is the other way around: the primitive sets the attribute and
/// daisyUI's rule is `.btn:disabled`, so nothing is emitted for it.
///
/// The last group is controlled by this example, so the set travels out through
/// the change callback and back in through the pressed prop. It allows one at a
/// time, which is what makes a second press visible: the first item is released
/// as the second goes down.
#[component]
pub fn Example() -> Element {
    let mut pressed = use_signal(HashSet::<usize>::new);

    rsx! {
        div { class: "flex flex-col gap-4",
            ToggleGroup {
                id: "group-states",
                horizontal: true,
                allow_multiple_pressed: true,
                default_pressed: [1usize].into_iter().collect(),
                ToggleItem { index: 0usize, "Off" }
                ToggleItem { index: 1usize, "On" }
                ToggleItem { index: 2usize, disabled: true, "Disabled" }
            }

            div { class: "flex flex-wrap items-center gap-3",
                ToggleGroup {
                    id: "group-controlled",
                    horizontal: true,
                    pressed: Some(pressed()),
                    on_pressed_change: move |next| pressed.set(next),
                    ToggleItem { index: 0usize, color: ToggleItemColor::Primary, "Grid" }
                    ToggleItem { index: 1usize, color: ToggleItemColor::Primary, "List" }
                }

                p { class: "text-sm opacity-70",
                    "Pressed: "
                    span { "data-testid": "pressed",
                        if pressed().is_empty() {
                            "nothing"
                        } else {
                            {
                                let mut indices: Vec<_> = pressed().into_iter().collect();
                                indices.sort();
                                indices.iter().map(usize::to_string).collect::<Vec<_>>().join(", ")
                            }
                        }
                    }
                }
            }
        }
    }
}
