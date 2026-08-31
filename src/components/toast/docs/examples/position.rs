use dioxus::prelude::*;

use crate::components::button::Button;
use crate::components::toast::{
    ToastBlock, ToastOptions, ToastInline, ToastProvider, use_toast,
};

/// Every value of both position axes, reached one at a time.
///
/// The triggers are the rows rather than the regions themselves, because a
/// region is pinned to the viewport: nine of them at once would be nine stacks
/// of toasts overlapping in the corners of the page rather than a rendered set
/// to read. What the buttons vary is the position of the one provider under
/// them.
///
/// daisyUI has two axes here and so does this component: the inline edge
/// (`toast-start`, `toast-center`, `toast-end`) and the block edge
/// (`toast-top`, `toast-middle`, `toast-bottom`). Every value emits a class,
/// the defaults included (ADR-0008).
#[component]
pub fn Example() -> Element {
    let inline = use_signal(ToastInline::default);
    let block = use_signal(ToastBlock::default);

    rsx! {
        ToastProvider {
            id: "positioned",
            inline: inline(),
            block: block(),
            Triggers { inline, block }
        }
    }
}

/// The rows that move the provider above, and send it something to show once it
/// has moved.
#[component]
fn Triggers(inline: Signal<ToastInline>, block: Signal<ToastBlock>) -> Element {
    let toast = use_toast();
    let mut inline = inline;
    let mut block = block;

    rsx! {
        div { class: "flex flex-col items-start gap-3",
            div { "data-axis-triggers": "inline", class: "flex flex-wrap items-center gap-2",
                for value in ToastInline::ALL.iter().copied() {
                    Button {
                        onclick: move |_| {
                            inline.set(value);
                            toast
                                .info(
                                    format!("{value:?}"),
                                    ToastOptions::new().permanent(true),
                                );
                        },
                        "{value:?}"
                    }
                }
            }

            div { "data-axis-triggers": "block", class: "flex flex-wrap items-center gap-2",
                for value in ToastBlock::ALL.iter().copied() {
                    Button {
                        onclick: move |_| {
                            block.set(value);
                            toast
                                .info(
                                    format!("{value:?}"),
                                    ToastOptions::new().permanent(true),
                                );
                        },
                        "{value:?}"
                    }
                }
            }
        }
    }
}
