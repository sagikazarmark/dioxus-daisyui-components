use dioxus::prelude::*;

use crate::components::separator::Separator;

/// A rule across the content and a rule down it, plus one that is only a rule.
///
/// `horizontal` is the primitive's word for the line the separator draws, and
/// it is what lands in the accessibility tree. daisyUI's word for the same line
/// is the other one (a rule down a row is `divider-horizontal` there) so the
/// class the component emits reads as the opposite of the prop that asked for
/// it.
///
/// The last one is decorative: it draws the same line and announces nothing, so
/// a screen reader is not told about a rule that divides nothing.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex flex-col",
                p { class: "text-sm", "Above" }
                Separator { id: "rule-across", "across" }
                p { class: "text-sm", "Below" }
            }

            div { class: "flex h-24 items-stretch",
                p { class: "flex-1 text-sm", "Before" }
                Separator { id: "rule-down", horizontal: false, "down" }
                p { class: "flex-1 text-sm", "After" }
            }

            div { class: "flex flex-col",
                p { class: "text-sm", "Above" }
                Separator { id: "rule-decorative", decorative: true }
                p { class: "text-sm", "Below" }
            }
        }
    }
}
