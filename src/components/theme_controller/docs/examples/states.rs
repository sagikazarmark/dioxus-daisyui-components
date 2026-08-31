use dioxus::prelude::*;

use crate::components::theme_controller::{
    ThemeController, ThemeControllerAppearance, ThemeControllerColor,
};

/// Every state a controller renders in, and what the page can hear about it.
///
/// The on and off pair carry identical classes: daisyUI keys the difference on
/// the input's own `:checked`, and the disabled pair on its `disabled`
/// attribute, so nothing is emitted for either, and the disabled attribute
/// also makes the control inert.
///
/// The last one reports through `onchange`, which is what a caller listens to
/// in order to remember a choice the CSS applies but nothing stores. The theme
/// is already on the document by the time this runs; writing it down is the
/// only part left.
///
/// What the event carries depends on the control, as it does for any input: a
/// checkbox reports its state (`value()` is `"true"` or `"false"`, which is
/// what `checked()` reads) so the theme is the one the caller named. A radio
/// reports its `value`, which for a controller *is* the theme, and is how a set
/// says which one was picked.
#[component]
pub fn Example() -> Element {
    let mut remembered = use_signal(|| String::from("none"));

    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            ThemeController { id: "states-off", theme: "parchment", aria_label: "Off" }
            ThemeController {
                id: "states-on",
                theme: "parchment",
                default_checked: true,
                aria_label: "On",
            }

            ThemeController {
                theme: "parchment",
                disabled: true,
                aria_label: "Disabled",
            }
            ThemeController {
                theme: "parchment",
                disabled: true,
                default_checked: true,
                aria_label: "Disabled on",
            }

            ThemeController {
                id: "states-remembered",
                theme: "midnight",
                appearance: ThemeControllerAppearance::Checkbox,
                color: ThemeControllerColor::Primary,
                aria_label: "Midnight",
                onchange: move |event: FormEvent| {
                    remembered.set(String::from(if event.checked() { "midnight" } else { "none" }));
                },
            }

            p { class: "text-sm opacity-70",
                "Remembered: "
                span { "data-testid": "remembered", "{remembered}" }
            }
        }
    }
}
