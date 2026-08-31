use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor, ButtonSize};

/// A button with nothing set on it, and two axes set at once.
///
/// The axes are independent props rather than one variant enum, because
/// daisyUI's own classes are independent: `btn-primary` and `btn-lg` combine,
/// and so do the props that emit them.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Button { "Button" }
            Button { color: ButtonColor::Primary, "Primary" }
            Button { color: ButtonColor::Accent, size: ButtonSize::Lg, "Accent, large" }
        }
    }
}
