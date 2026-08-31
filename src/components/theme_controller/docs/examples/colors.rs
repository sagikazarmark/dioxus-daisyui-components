use dioxus::prelude::*;

use crate::components::theme_controller::{
    ThemeController, ThemeControllerAppearance, ThemeControllerColor,
};

/// Every value of the colour axis, in each of the scales an appearance names.
///
/// The value is a colour and the appearance decides which class carries it:
/// `toggle-primary`, `checkbox-primary`, `radio-primary`, `btn-primary`,
/// because daisyUI writes one scale per control and none for `theme-controller`
/// itself. `Default` emits no class, which is daisyUI's uncoloured control.
///
/// The first three rows are checked, because that is the state daisyUI paints a
/// toggle's colour in. The buttons are not: daisyUI paints *any* checked button
/// with the primary colour, so a checked row would say less about the axis than
/// an unchecked one does.
///
/// Every control here names a theme daisyUI does not ship, so a row of them can
/// be checked without moving the page they are on.
#[component]
pub fn Example() -> Element {
    let scales = [
        (ThemeControllerAppearance::Toggle, "color-toggle", true),
        (ThemeControllerAppearance::Checkbox, "color-checkbox", true),
        (ThemeControllerAppearance::Radio, "color-radio", true),
        (ThemeControllerAppearance::Button, "color-button", false),
    ];

    rsx! {
        div { class: "flex flex-col gap-4",
            for (appearance , axis , checked) in scales {
                div { key: "{axis}", class: "flex flex-col gap-1",
                    span { class: "text-xs opacity-70", "{appearance:?}" }
                    div { "data-axis": axis, class: "flex flex-wrap items-center gap-3",
                        for color in ThemeControllerColor::ALL.iter().copied() {
                            ThemeController {
                                key: "{color:?}",
                                theme: "parchment",
                                appearance,
                                color,
                                name: "{axis}-{color:?}",
                                aria_label: "{color:?}",
                                default_checked: checked,
                            }
                        }
                    }
                }
            }
        }
    }
}
