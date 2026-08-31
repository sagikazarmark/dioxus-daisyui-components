use dioxus::prelude::*;

use crate::components::separator::{Separator, SeparatorColor};

/// A caller's own classes and attributes, including the one case the component
/// creates for itself.
///
/// The first drops the margin daisyUI's own class sets, which the caller wins
/// on cascade layers rather than on specificity.
///
/// The second is daisyUI's answer to a layout that changes at a breakpoint: a
/// row that stacks on a small screen wants a rule across it there and a rule
/// down it once it is a row. This component always emits an orientation class
/// (ADR-0008), so the caller's `md:divider-horizontal` meets a
/// `divider-vertical` that is already on the element, and wins, because
/// Tailwind generates responsive variants after the utilities they vary. That
/// is a source-order argument rather than a specificity one, so it is rendered
/// here and asserted in the browser rather than only described.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex flex-col",
                p { class: "text-sm", "Above" }
                Separator {
                    color: SeparatorColor::Primary,
                    class: "m-0",
                    id: "caller-attributes",
                    "no margin"
                }
                p { class: "text-sm", "Below" }
            }

            div { class: "flex h-24 flex-col md:flex-row",
                p { class: "flex-1 text-sm", "Stacked below the breakpoint" }
                Separator { class: "md:divider-horizontal", id: "caller-responsive" }
                p { class: "flex-1 text-sm", "Side by side above it" }
            }
        }
    }
}
