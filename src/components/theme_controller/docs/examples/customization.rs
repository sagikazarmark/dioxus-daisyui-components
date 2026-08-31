use dioxus::prelude::*;

use crate::components::theme_controller::{
    ThemeController, ThemeControllerAppearance, ThemeControllerColor,
};

/// A caller's own classes and attributes.
///
/// The first squares off the corner radius daisyUI's `toggle` sets, which is the
/// case worth demonstrating: the two are both single class selectors, and the
/// caller's wins on cascade layers rather than on specificity.
///
/// The second is the pairing the appearance axis does not offer (a button that
/// is a checkbox rather than one of a set) reached the way any look this
/// component does not name is reached: `None` leaves the element a bare
/// checkbox, and the caller's classes concatenate onto it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-3",
            ThemeController {
                id: "caller-attributes",
                theme: "midnight",
                color: ThemeControllerColor::Primary,
                class: "rounded-none",
                aria_label: "Squared off",
            }

            ThemeController {
                id: "caller-button-checkbox",
                theme: "midnight",
                appearance: ThemeControllerAppearance::None,
                class: "btn btn-sm",
                aria_label: "Midnight",
            }
        }
    }
}
