use dioxus::prelude::*;

use crate::components::switch::{Switch, SwitchColor};

/// A caller's own classes and attributes, and the size daisyUI's own classes
/// cannot give this element.
///
/// The first switch squares off a corner radius daisyUI itself sets, which is
/// the case worth demonstrating: the two are both single class selectors, and
/// the caller's wins on cascade layers rather than on specificity.
///
/// The second sets the custom property daisyUI's `toggle-lg` would have set, on
/// the element that already carries `toggle`. This component exposes no size
/// axis, because every one of daisyUI's size rules is gated on the element
/// being, or containing, a checkbox input, and the primitive renders a
/// `button` with the input beside it. Reaching for the property is the caller's
/// decision to take on a daisyUI internal, not one the registry takes for them.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Switch {
                color: SwitchColor::Primary,
                default_value: true,
                class: "rounded-none",
                id: "caller-attributes",
                aria_label: "Caller classes and attributes",
            }

            Switch {
                color: SwitchColor::Primary,
                default_value: true,
                class: "[--size:calc(var(--size-selector,0.25rem)*8)]",
                id: "caller-size",
                aria_label: "Caller size",
            }
        }
    }
}
