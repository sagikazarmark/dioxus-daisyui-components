use dioxus::prelude::*;

use crate::components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogActions, AlertDialogButtonColor, AlertDialogCancel,
    AlertDialogDescription, AlertDialogTitle,
};
use crate::components::button::{Button, ButtonColor};

/// A decision that cannot be dismissed by accident, from the collapsed
/// [`AlertDialog`] component.
///
/// There is no trigger part: the open state is the caller's, and a button that
/// sets it is a button like any other. Inside the dialog there is no close part
/// either: the action and the cancel *are* the way out, and both close the
/// dialog themselves before the caller's handler runs.
///
/// A click outside the box does nothing here, which is the difference from the
/// dialog: the decision has to be taken rather than dismissed.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut outcome = use_signal(|| String::from("nothing yet"));

    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Button { color: ButtonColor::Error, onclick: move |_| open.set(true), "Delete the project" }

            p { class: "text-sm opacity-70",
                "Chose "
                span { "data-testid": "outcome", "{outcome}" }
            }

            AlertDialog {
                id: "confirm",
                open: Some(open()),
                on_open_change: move |next| open.set(next),
                AlertDialogTitle { "Delete the project?" }
                AlertDialogDescription {
                    "Every deployment goes with it, and the name is released. This cannot be undone."
                }
                AlertDialogActions {
                    AlertDialogCancel { on_click: move |_| outcome.set(String::from("cancel")), "Keep it" }
                    AlertDialogAction {
                        color: AlertDialogButtonColor::Error,
                        on_click: move |_| outcome.set(String::from("delete")),
                        "Delete it"
                    }
                }
            }
        }
    }
}
