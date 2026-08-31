use dioxus::prelude::*;
use std::time::Duration;

use crate::components::button::{Button, ButtonColor};
use crate::components::toast::{ToastBlock, ToastOptions, ToastProvider, Toasts, use_toast};

/// A provider, and the buttons that send it something to show.
///
/// The dispatcher is a component of its own rather than the body of this one,
/// and it has to be: `use_toast` reads the provider's context, and a component
/// cannot read a context it provides itself. In an app the provider wraps
/// everything and the question never comes up; here the two are a few lines
/// apart, which is where it does.
#[component]
pub fn Example() -> Element {
    rsx! {
        // Pinned to the top rather than left at daisyUI's own bottom corner,
        // because this page mounts a provider per example and an app mounts
        // one: the kinds below keep a stack in the bottom corner for as long
        // as the page is open, and two regions in one corner would sit on top
        // of each other.
        // The stacking is the caller's too: daisyUI's toast sets no z-index,
        // and this site has a sticky header, so a region pinned to the top
        // needs a utility to come out over it.
        ToastProvider { block: ToastBlock::Top, class: "z-40",
            Dispatcher {}
        }
    }
}

/// Everything under the provider, which is the only place a toast can be sent
/// from.
#[component]
fn Dispatcher() -> Element {
    // The handle the provider hands out, written out rather than inferred: it
    // is the primitive's own, re-exported by this component so that a caller
    // needs one import.
    let toast: Toasts = use_toast();

    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Button {
                color: ButtonColor::Success,
                onclick: move |_| {
                    toast
                        .success(
                            "Deployed".to_string(),
                            ToastOptions::new().description("Live in eu-west-1"),
                        );
                },
                "Success"
            }

            Button {
                color: ButtonColor::Error,
                onclick: move |_| {
                    toast
                        .error(
                            "Deploy failed".to_string(),
                            ToastOptions::new().description("The health check never passed"),
                        );
                },
                "Error"
            }

            Button {
                onclick: move |_| {
                    toast
                        .info(
                            "Still building".to_string(),
                            ToastOptions::new()
                                .description("This one stays up for fifteen seconds")
                                .duration(Duration::from_secs(15)),
                        );
                },
                "Info, for longer"
            }

            Button {
                onclick: move |_| {
                    toast
                        .warning(
                            "Certificate expires tomorrow".to_string(),
                            ToastOptions::new()
                                .description("This one stays until it is dismissed")
                                .permanent(true),
                        );
                },
                "Warning, permanent"
            }
        }
    }
}
