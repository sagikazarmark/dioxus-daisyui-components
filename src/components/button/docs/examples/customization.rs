use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};

/// A caller's own classes and attributes, on a component that already has both.
///
/// The class concatenates with the button's own (`btn`, `btn-primary` and
/// `w-64` all land on one element), and every other attribute the caller passes
/// overrides the component's.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Button {
                color: ButtonColor::Primary,
                class: "w-64",
                id: "caller-attributes",
                onclick: move |_| {},
                "Caller classes and attributes"
            }
        }
    }
}
