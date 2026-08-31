use dioxus::prelude::*;

use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

/// A caller's own classes and attributes.
///
/// The alignment is the case cascade layers decide: daisyUI centres a bubble's
/// text and the caller starts it, both from a single class selector, and the
/// caller's wins on layers rather than on specificity.
///
/// The width is the case they do not. daisyUI caps the bubble at `20rem` with
/// `max-width`, and a `width` utility loses to that cap however the cascade
/// resolves (different property, not a tie) so a wider bubble takes
/// `max-w-none` as well. Worth knowing before reading a layer fight into it.
///
/// Both land on the bubble because that is the element they were written on;
/// the outer element takes classes of its own, and the two are separate parts
/// for exactly that reason.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-6 pt-16",
            Tooltip { id: "caller-tooltip", default_open: true,
                TooltipTrigger { class: "text-sm underline decoration-dotted", "A wider bubble" }
                TooltipContent { id: "caller-attributes", class: "w-96 max-w-none text-start",
                    "daisyUI caps a tooltip at twenty rem and centres its text. The align is a class the caller beats on cascade layers; the cap is a max-width, which takes lifting rather than out-ranking."
                }
            }
        }
    }
}
