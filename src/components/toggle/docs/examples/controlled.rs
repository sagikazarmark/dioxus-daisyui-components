use dioxus::prelude::*;

use crate::components::toggle::{Toggle, ToggleColor};

/// The pressed state lifted all the way out to the caller, which is what this
/// component does internally either way.
///
/// The toggle reports through the callback and this example writes the state
/// back, so what is on screen is what the page decided, including the half an
/// uncontrolled toggle cannot do, which is a button elsewhere releasing it.
#[component]
pub fn Example() -> Element {
    let mut pressed = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-3",
            Toggle {
                id: "controlled",
                color: ToggleColor::Primary,
                pressed: Some(pressed()),
                on_pressed_change: move |on| {
                    pressed.set(on);
                    changes += 1;
                },
                aria_label: "Recording",
                "Recording"
            }

            button {
                id: "release",
                class: "btn btn-sm",
                onclick: move |_| pressed.set(false),
                "Release from outside"
            }

            p { class: "text-sm opacity-70",
                span { "data-testid": "state", if pressed() { "pressed" } else { "released" } }
                " · changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }
        }
    }
}
