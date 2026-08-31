use dioxus::prelude::*;

use crate::components::theme_controller::{
    ThemeController, ThemeControllerAppearance, ThemeControllerSize,
};

/// Every value of the size axis, smallest to largest, in each of the scales an
/// appearance names.
///
/// Three of the four scales are gated on the input's `type`:
/// `.toggle-lg[type=checkbox]`, `.radio-lg[type=radio]`, which is why the
/// appearance owns the type rather than leaving it to be paired up by hand.
/// This is the one component in the registry those rules reach: everywhere else
/// the styled element is a `button` a primitive rendered, and ADR-0010 records
/// the axes that go missing because of it.
///
/// `Default` emits no class and renders at the same size as an explicit `-md`.
#[component]
pub fn Example() -> Element {
    let scales = [
        (ThemeControllerAppearance::Toggle, "size-toggle"),
        (ThemeControllerAppearance::Checkbox, "size-checkbox"),
        (ThemeControllerAppearance::Radio, "size-radio"),
        (ThemeControllerAppearance::Button, "size-button"),
    ];

    rsx! {
        div { class: "flex flex-col gap-4",
            for (appearance , axis) in scales {
                div { key: "{axis}", class: "flex flex-col gap-1",
                    span { class: "text-xs opacity-70", "{appearance:?}" }
                    div { "data-axis": axis, class: "flex flex-wrap items-center gap-3",
                        for size in ThemeControllerSize::ALL.iter().copied() {
                            ThemeController {
                                key: "{size:?}",
                                theme: "parchment",
                                appearance,
                                size,
                                name: "{axis}-{size:?}",
                                aria_label: "{size:?}",
                            }
                        }
                    }
                }
            }
        }
    }
}
