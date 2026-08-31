use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};
use crate::components::dialog::{Dialog, DialogActions, DialogCtx, DialogDescription, DialogTitle};

/// A dialog opened from a button, built from the collapsed [`Dialog`]
/// component.
///
/// The registry offers no trigger part: the open state is the caller's, and a
/// button that sets it is a button like any other. A control *inside* the
/// dialog reaches the primitive's context instead, which is why the close
/// button below is a component of its own.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Button { color: ButtonColor::Primary, onclick: move |_| open.set(true), "Open dialog" }

            Dialog {
                open: Some(open()),
                on_open_change: move |next| open.set(next),
                DialogTitle { "Deploy to production?" }
                DialogDescription { "The build on main will replace what is live now." }
                DialogActions {
                    DismissButton { "Not now" }
                    Button { color: ButtonColor::Primary, onclick: move |_| open.set(false), "Deploy" }
                }
            }
        }
    }
}

/// A button that closes the dialog it is inside.
///
/// The primitive's context is public and the registry re-exports it, so a
/// control within the dialog can close it without the caller threading its own
/// state down.
#[component]
fn DismissButton(children: Element) -> Element {
    let ctx: DialogCtx = use_context();

    rsx! {
        Button { onclick: move |_| ctx.set_open(false), {children} }
    }
}
