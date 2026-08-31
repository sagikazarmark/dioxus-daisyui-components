use dioxus::prelude::*;

use crate::components::toggle::{Toggle, ToggleColor};

/// A caller's own classes and attributes.
///
/// The first squares off the corner radius daisyUI's `btn` sets, which is the
/// case worth demonstrating: the two are both single class selectors, and the
/// caller's wins on cascade layers rather than on specificity.
///
/// The second is the icon toggle daisyUI's `swap` exists for, written the way
/// this component leaves it to the caller: `btn-circle` shapes the button and
/// the caller swaps the glyph from the state they already control, since a
/// controlled toggle hands them the state this component would otherwise be
/// guessing at.
#[component]
pub fn Example() -> Element {
    let mut muted = use_signal(|| false);

    rsx! {
        div { class: "flex flex-wrap items-center gap-3",
            Toggle {
                id: "caller-attributes",
                color: ToggleColor::Primary,
                default_pressed: true,
                class: "rounded-none",
                aria_label: "Squared off",
                "Squared off"
            }

            Toggle {
                id: "caller-swap",
                class: "btn-circle",
                pressed: Some(muted()),
                on_pressed_change: move |on| muted.set(on),
                aria_label: "Mute",
                if muted() {
                    "🔇"
                } else {
                    "🔊"
                }
            }
        }
    }
}
