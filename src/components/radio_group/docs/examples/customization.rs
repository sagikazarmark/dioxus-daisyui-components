use dioxus::prelude::*;

use crate::components::radio_group::{RadioGroup, RadioItem, RadioItemColor};

/// A caller's own classes and attributes, and the size daisyUI's own classes
/// cannot give this element.
///
/// The first item squares off a corner radius daisyUI itself sets (a radio is
/// round by default) which is the case worth demonstrating: the two are both
/// single class selectors, and the caller's wins on cascade layers rather than
/// on specificity.
///
/// The third sets the custom property daisyUI's `radio-lg` would have set. This
/// component exposes no size axis, because daisyUI puts a radio's size behind
/// `.radio-lg[type=radio]` and the primitive renders a `button`; the plain
/// `.radio-lg` arm sets only the padding around the dot, so the classes would
/// leave the control the same size with a smaller dot in it. Reaching for the
/// property is the caller's decision to take on a daisyUI internal, not one the
/// registry takes for them.
#[component]
pub fn Example() -> Element {
    rsx! {
        RadioGroup {
            horizontal: true,
            default_value: "square".to_string(),
            aria_label: "Caller classes and attributes",
            RadioItem {
                value: "square".to_string(),
                index: 0usize,
                color: RadioItemColor::Primary,
                class: "rounded-none",
                id: "caller-attributes",
                aria_label: "Caller classes and attributes",
            }
            RadioItem {
                value: "round".to_string(),
                index: 1usize,
                color: RadioItemColor::Primary,
                aria_label: "Untouched",
            }
            RadioItem {
                value: "large".to_string(),
                index: 2usize,
                color: RadioItemColor::Primary,
                class: "[--size:calc(var(--size-selector,0.25rem)*8)]",
                id: "caller-size",
                aria_label: "Caller size",
            }
        }
    }
}
