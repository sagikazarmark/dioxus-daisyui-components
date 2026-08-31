use dioxus::prelude::*;

use crate::components::kbd::Kbd;

/// A keyboard key used as semantic inline content.
#[component]
pub fn Example() -> Element {
    rsx! {
        p {
            "Press "
            Kbd { id: "default-kbd", "Enter" }
            " to continue."
        }
    }
}
