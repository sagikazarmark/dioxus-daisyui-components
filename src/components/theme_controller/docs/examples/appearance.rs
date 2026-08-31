use dioxus::prelude::*;

use crate::components::theme_controller::{ThemeController, ThemeControllerAppearance};

/// Every value of the appearance axis, which is the control daisyUI draws.
///
/// `theme-controller` paints nothing of its own, so each of these borrows the
/// look of another daisyUI control and, with it, the input type that control's
/// rules are written against. `None` borrows nothing and leaves a bare browser
/// checkbox, which is what a caller drawing their own control starts from.
///
/// Each is its own named group rather than a set to choose from: they are five
/// looks at one theme, not five themes. That theme is not one of daisyUI's, so
/// the row shows the controls without theming the page.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-4",
            for appearance in ThemeControllerAppearance::ALL.iter().copied() {
                ThemeController {
                    key: "{appearance:?}",
                    theme: "parchment",
                    appearance,
                    name: "appearance-{appearance:?}",
                    aria_label: "{appearance:?}",
                }
            }
        }
    }
}
