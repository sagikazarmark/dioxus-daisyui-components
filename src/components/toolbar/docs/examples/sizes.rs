use dioxus::prelude::*;

use crate::components::toolbar::{Toolbar, ToolbarButton, ToolbarButtonSize};

/// Every value of the button size axis, smallest first.
///
/// `Default` emits no class and renders at the same size as daisyUI's explicit
/// `btn-md`, which is why it sits in the middle of the row rather than at one
/// end of it.
#[component]
pub fn Example() -> Element {
    rsx! {
        Toolbar { "data-axis": "size", aria_label: "Sizes",
            for (index , size) in ToolbarButtonSize::ALL.iter().copied().enumerate() {
                ToolbarButton { index, size, "{size:?}" }
            }
        }
    }
}
