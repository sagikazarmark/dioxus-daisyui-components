use dioxus::prelude::*;

use crate::components::toast::{ToastColor, ToastOptions, ToastProvider, use_toast};

/// Every value of the colour axis, one toast each, up for as long as the page
/// is.
///
/// This is the axis nobody passes: the kind is decided at the call that
/// dispatches the toast, and the class follows from it; `ToastColor::of` is
/// the whole of the bridging, since daisyUI's alert colours match no data
/// attribute and the primitive reports the kind as one.
///
/// These are permanent, so the page has the rendered set on it rather than for
/// five seconds after a click. Their close buttons still dismiss them.
#[component]
pub fn Example() -> Element {
    rsx! {
        ToastProvider { "data-region": "types",
            Dispatcher {}
        }
    }
}

/// One toast per colour, sent once when the page arrives.
///
/// The dispatch is in an effect rather than in the body, because a toast is a
/// write to the provider's queue and a component may not write to a signal
/// while rendering. The flag keeps a re-render from sending them twice, and is
/// read with `peek` so that reading it is not what causes one.
#[component]
fn Dispatcher() -> Element {
    let toast = use_toast();
    let mut sent = use_signal(|| false);

    use_effect(move || {
        if *sent.peek() {
            return;
        }
        sent.set(true);

        for color in ToastColor::ALL.iter().copied() {
            toast.show(
                format!("{color:?}"),
                color.toast_type(),
                ToastOptions::new()
                    .description("A toast of this kind")
                    .permanent(true),
            );
        }
    });

    rsx! {
        p { class: "text-sm opacity-70",
            "One toast of each kind, pinned to the corner of the viewport for as long as this page is open."
        }
    }
}
