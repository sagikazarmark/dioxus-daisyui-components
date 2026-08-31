use dioxus::prelude::*;

use crate::components::alert_dialog::{
    AlertDialog, AlertDialogActions, AlertDialogButtonColor, AlertDialogCancel,
    AlertDialogDescription, AlertDialogDescriptionAppearance, AlertDialogTitle,
    AlertDialogTitleAppearance,
};
use crate::components::button::Button;

/// Every axis the parts inside the box carry, reached one at a time.
///
/// The title and the description are rendered once each rather than as a row of
/// values, which is this primitive's doing: it names and describes the dialog by
/// two ids it holds itself and puts them on whatever title and description are
/// rendered, so a second of either would repeat an id that has to be unique.
/// The triggers vary the one dialog under them instead, the way the placement
/// axis is reached.
///
/// The buttons have no such constraint, so every value of their colour axis is
/// one row inside the dialog. They all cancel: whichever part a button is, it
/// closes the dialog before the caller's handler runs.
#[component]
pub fn Example() -> Element {
    let mut title = use_signal(AlertDialogTitleAppearance::default);
    let mut description = use_signal(AlertDialogDescriptionAppearance::default);
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col items-start gap-4",
            div { "data-axis-triggers": "title", class: "flex flex-wrap items-center gap-2",
                for value in AlertDialogTitleAppearance::ALL.iter().copied() {
                    Button {
                        onclick: move |_| {
                            title.set(value);
                            open.set(true);
                        },
                        "Title: {value:?}"
                    }
                }
            }

            div { "data-axis-triggers": "description", class: "flex flex-wrap items-center gap-2",
                for value in AlertDialogDescriptionAppearance::ALL.iter().copied() {
                    Button {
                        onclick: move |_| {
                            description.set(value);
                            open.set(true);
                        },
                        "Description: {value:?}"
                    }
                }
            }

            AlertDialog {
                id: "showcase",
                open: Some(open()),
                on_open_change: move |next| open.set(next),
                AlertDialogTitle { appearance: title(), "Title: {title():?}" }
                AlertDialogDescription { appearance: description(),
                    "Description: {description():?}"
                }
                AlertDialogActions { class: "flex-wrap",
                    div { "data-axis": "button", class: "flex flex-wrap items-center gap-2",
                        for color in AlertDialogButtonColor::ALL.iter().copied() {
                            AlertDialogCancel { color, "{color:?}" }
                        }
                    }
                }
            }
        }
    }
}
