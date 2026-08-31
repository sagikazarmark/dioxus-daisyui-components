use dioxus::prelude::*;

use crate::components::toolbar::{Toolbar, ToolbarButton, ToolbarButtonSize};

/// A disabled control in the middle of a toolbar, and a toolbar that is
/// disabled outright.
///
/// Neither emits a class. The primitive sets the native `disabled` attribute on
/// every button it applies to, daisyUI's rule is `.btn:disabled`, and the
/// attribute is also what makes the arrow keys skip the control rather than
/// landing on something inert.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            Toolbar { id: "states", aria_label: "One control disabled",
                ToolbarButton { index: 0usize, size: ToolbarButtonSize::Sm, "Align left" }
                ToolbarButton {
                    index: 1usize,
                    size: ToolbarButtonSize::Sm,
                    disabled: true,
                    "Justify"
                }
                ToolbarButton { index: 2usize, size: ToolbarButtonSize::Sm, "Align right" }
            }

            Toolbar { id: "disabled", disabled: true, aria_label: "Disabled toolbar",
                ToolbarButton { index: 0usize, size: ToolbarButtonSize::Sm, "Cut" }
                ToolbarButton { index: 1usize, size: ToolbarButtonSize::Sm, "Copy" }
            }
        }
    }
}
