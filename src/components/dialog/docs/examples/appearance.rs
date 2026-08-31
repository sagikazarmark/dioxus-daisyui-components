use dioxus::prelude::*;

use crate::components::button::Button;
use crate::components::dialog::{
    Dialog, DialogActions, DialogCtx, DialogDescription, DialogDescriptionAppearance, DialogTitle,
    DialogTitleAppearance,
};

/// Both appearance axes, side by side inside one dialog that opens on arrival.
///
/// It opens from `default_open` alone, with no caller holding its state: a
/// closed dialog renders no elements at all (the primitive mounts them when it
/// opens and unmounts them once the exit animation has run) so there would
/// otherwise be nothing here to look at. Close it to reach the rest of the
/// page.
///
/// The first value of each row is left without an id of its own, so that it
/// keeps the one the primitive names and describes the dialog by: two elements
/// cannot share it, and a row where every element had an id would leave the
/// dialog pointing at nothing.
#[component]
pub fn Example() -> Element {
    rsx! {
        Dialog { id: "showcase", class: "rounded-none", default_open: true,
            div { "data-axis": "title", class: "flex flex-wrap items-baseline gap-4",
                for appearance in DialogTitleAppearance::ALL.iter().copied() {
                    DialogTitle { appearance, id: title_id(appearance), "{appearance:?}" }
                }
            }

            div { "data-axis": "description", class: "flex flex-wrap items-baseline gap-4",
                for appearance in DialogDescriptionAppearance::ALL.iter().copied() {
                    DialogDescription {
                        appearance,
                        id: description_id(appearance),
                        "{appearance:?}"
                    }
                }
            }

            DialogActions {
                CloseButton {}
            }
        }
    }
}

/// The id for one title in the appearance row, and none for the value the row
/// starts with, which is the one the dialog is named by.
fn title_id(appearance: DialogTitleAppearance) -> Option<String> {
    (appearance != DialogTitleAppearance::Default).then(|| format!("title-{appearance:?}"))
}

/// The same, for the description the dialog is described by.
fn description_id(appearance: DialogDescriptionAppearance) -> Option<String> {
    (appearance != DialogDescriptionAppearance::Default)
        .then(|| format!("description-{appearance:?}"))
}

/// A button that closes the dialog it is inside.
///
/// This is how a dialog that owns its own state is closed from within it: the
/// primitive's context is public and the registry re-exports it, and the call
/// travels back out through the styled wrapper's change callback, so the lifted
/// state stays in step with the primitive's.
#[component]
fn CloseButton() -> Element {
    let ctx: DialogCtx = use_context();

    rsx! {
        Button { onclick: move |_| ctx.set_open(false), "Close" }
    }
}
