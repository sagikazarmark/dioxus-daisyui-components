use dioxus::prelude::*;

use crate::components::button::{Button, ButtonSize};

/// Every value of the size axis, smallest to largest.
///
/// `ButtonSize::Default` emits no class at all and sits in the middle of the
/// list, which is where daisyUI's own unset size renders.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in ButtonSize::ALL.iter().copied() {
                Button { size, "{size:?}" }
            }
        }
    }
}
