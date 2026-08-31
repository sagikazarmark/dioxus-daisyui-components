use dioxus::prelude::*;

use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogButtonColor, AlertDialogCancel,
    AlertDialogContent, AlertDialogDescription, AlertDialogPlacement, AlertDialogRoot,
    AlertDialogTitle,
};
use crate::components::button::Button;

/// Every value of the placement axis, reached one at a time.
///
/// The triggers are the row rather than the dialogs themselves, because six
/// modals are six full-viewport elements stacked on one another. What the
/// buttons vary is the placement of the one dialog under them, which is built
/// from the compound parts and controlled by this example.
#[component]
pub fn Example() -> Element {
    let mut placement = use_signal(AlertDialogPlacement::default);
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-col items-start gap-4",
            div { "data-axis-triggers": "placement", class: "flex flex-wrap items-center gap-2",
                for value in AlertDialogPlacement::ALL.iter().copied() {
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

            AlertDialogRoot {
                id: "positioned",
                placement: placement(),
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                AlertDialogContent { id: "positioned-box",
                    AlertDialogTitle { "Roll back to the last release?" }
                    AlertDialogDescription {
                        "Everything deployed since Tuesday will stop serving traffic."
                    }
                    AlertDialogActions {
                        AlertDialogCancel { "Stay here" }
                        AlertDialogAction { color: AlertDialogButtonColor::Warning, "Roll back" }
                    }
                }
            }
        }
    }
}
