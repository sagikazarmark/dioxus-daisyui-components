use dioxus::prelude::*;

use crate::components::input::Input;

/// A conditional adornment toggles content inside `Some`, never the option.
///
/// Presence of `prefix`/`suffix` selects the wrapper structure, so switching
/// the option between `Some` and `None` across renders would remount the
/// native input mid-session. The stable slot renders empty instead — hidden,
/// so it leaves no gap in the box — and the content appears in place without
/// disturbing focus, caret, or the Focus Exit contract.
#[component]
pub fn Example() -> Element {
    let mut verified = use_signal(|| false);

    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Input {
                id: "slot-input",
                suffix: rsx! {
                    if verified() {
                        span { class: "badge badge-success badge-sm", "verified" }
                    }
                },
                aria_label: "Account handle",
                placeholder: "handle",
            }
            button {
                id: "toggle-slot",
                r#type: "button",
                class: "btn btn-sm",
                onclick: move |_| verified.toggle(),
                "Toggle badge"
            }
        }
    }
}
