use dioxus::prelude::*;

use crate::components::separator::{Separator, SeparatorPlacement};

/// Every value of the placement axis, which is where the separator's own
/// content sits along the rule.
///
/// daisyUI places it by dropping half the line rather than by moving anything:
/// `divider-start` hides the rule before the content and `divider-end` the rule
/// after it. A separator with nothing in it renders under every value all the
/// same (there is nothing to place) but it still loses the half that was
/// dropped, so these carry text.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "placement", class: "flex flex-col",
            for placement in SeparatorPlacement::ALL.iter().copied() {
                Separator { placement, "{placement:?}" }
            }
        }
    }
}
