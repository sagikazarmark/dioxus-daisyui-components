use dioxus::prelude::*;

use crate::components::separator::Separator;

/// A rule between two sections, and a rule with a word in it.
///
/// The second one is the reason a separator takes children at all: daisyUI
/// draws the line as two halves with a gap between them, and whatever is
/// written here is what sits in that gap.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col",
            p { class: "text-sm", "Everything above the line." }
            Separator {}
            p { class: "text-sm", "Everything below it." }

            Separator { "OR" }

            p { class: "text-sm", "And everything after that." }
        }
    }
}
