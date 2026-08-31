use dioxus::prelude::*;

use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogButtonColor, AlertDialogCancel,
    AlertDialogContent, AlertDialogDescription, AlertDialogPlacement, AlertDialogRoot,
    AlertDialogTitle, AlertDialogTitleAppearance,
};
use crate::components::button::Button;

/// A caller's own classes, on the parts rather than on the collapsed component.
///
/// Dropping to the parts is what reaching the outer element means: the placement
/// axis is on `AlertDialogRoot`, a width and a corner radius are on
/// `AlertDialogContent`, and the title switches this component's utilities off
/// and brings its own (ADR-0004).
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Button { onclick: move |_| open.set(true), "Revoke the token" }

            AlertDialogRoot {
                placement: AlertDialogPlacement::Bottom,
                open: Some(open()),
                on_open_change: move |next| open.set(next),
                AlertDialogContent { id: "caller-attributes", class: "max-w-sm rounded-none",
                    AlertDialogTitle {
                        appearance: AlertDialogTitleAppearance::None,
                        class: "text-2xl font-black uppercase",
                        "Revoke the token?"
                    }
                    AlertDialogDescription { "Anything using it stops working immediately." }
                    AlertDialogActions {
                        AlertDialogCancel { "Keep it" }
                        AlertDialogAction { color: AlertDialogButtonColor::Error, "Revoke" }
                    }
                }
            }
        }
    }
}
