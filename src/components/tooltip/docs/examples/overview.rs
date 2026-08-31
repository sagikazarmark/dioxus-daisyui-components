use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

/// A tooltip on a button, and one on a word in a sentence.
///
/// The button is rendered *as* the trigger rather than inside it, through the
/// primitive's `as` prop: the attribute list (the hover and focus handlers and
/// the `aria-describedby`) is handed to a callback that renders the element in
/// the trigger's place, so there is no wrapper between daisyUI's tooltip and
/// the control it belongs to.
///
/// The second trigger is the primitive's own element, which carries
/// `tabindex="0"` and is what makes a tooltip on plain text reachable by
/// keyboard.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-6",
            Tooltip {
                TooltipTrigger {
                    r#as: move |attributes: Vec<Attribute>| rsx! {
                        Button { color: ButtonColor::Primary, attributes, "Deploy" }
                    },
                }
                TooltipContent { "Ships the build on main" }
            }

            p { class: "text-sm",
                "Everything below the "
                Tooltip {
                    TooltipTrigger { class: "underline decoration-dotted", "build" }
                    TooltipContent { "The last commit that compiled" }
                }
                " is already live."
            }
        }
    }
}
