use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};
use crate::components::dialog::{
    DialogActions, DialogContent, DialogDescription, DialogPlacement, DialogRoot, DialogTitle,
};

/// Every value of the placement axis, reached one at a time.
///
/// The triggers are the row rather than the dialogs themselves, because six
/// modals are six full-viewport elements stacked on one another. What the
/// buttons vary is the placement of the one dialog under them, which is built
/// from the compound parts and controlled by this example, so a dismissal has
/// to travel out through the change callback for anything to close.
#[component]
pub fn Example() -> Element {
    let mut placement = use_signal(DialogPlacement::default);
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-col items-start gap-4",
            div { "data-axis-triggers": "placement", class: "flex flex-wrap items-center gap-2",
                for value in DialogPlacement::ALL.iter().copied() {
                    Button {
                        onclick: move |_| {
                            placement.set(value);
                            open.set(true);
                        },
                        "{value:?}"
                    }
                }
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }

            DialogRoot {
                id: "positioned",
                placement: placement(),
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                DialogContent { id: "positioned-box",
                    DialogTitle { "Delete the project?" }
                    DialogDescription { "Every deployment goes with it. This cannot be undone." }
                    DialogActions {
                        Button { onclick: move |_| open.set(false), "Cancel" }
                        Button {
                            color: ButtonColor::Error,
                            onclick: move |_| open.set(false),
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
