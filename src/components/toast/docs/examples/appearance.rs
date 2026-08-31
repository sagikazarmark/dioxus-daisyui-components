use dioxus::prelude::*;
use dioxus_primitives::toast as primitive;

use crate::components::button::Button;
use crate::components::toast::{
    ToastCloseButton, ToastContent, ToastDescription, ToastDescriptionAppearance,
    ToastListAppearance, ToastOptions, ToastInline, ToastProps, ToastPropsWithOwner,
    ToastProvider, ToastTitle, ToastTitleAppearance, use_toast,
};

/// Every value of all three appearance axes, which is also how a caller renders
/// a toast of their own.
///
/// The first provider is the component as it arrives: it lays the toasts out in
/// a column with a gap, and the title and the description carry the utilities
/// daisyUI has no classes for: the first value of each axis.
///
/// The second switches them off, which is what an axis over utilities is for
/// (ADR-0004). The layout is the caller's, and so is the toast: `render_toast`
/// takes a component over the primitive's own props, which is what the provider
/// builds for every toast in its queue, so it is written as a plain function
/// rather than with a props list of its own. The registry's parts are still
/// used inside it.
#[component]
pub fn Example() -> Element {
    // The two providers below are the two values of the layout axis, in
    // variant-list order, named here so the page says which is which.
    let layouts: Vec<String> = ToastListAppearance::ALL
        .iter()
        .map(|value| format!("{value:?}"))
        .collect();

    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            p { class: "w-full text-sm opacity-70", "Layout: {layouts.join(\", then \")}." }

            // Each provider is pinned to an edge of its own, for the reason
            // the overview's is: this page has four of them and an app has one.
            ToastProvider {
                "data-region": "default-parts",
                inline: ToastInline::Start,
                appearance: ToastListAppearance::Default,
                Trigger { label: "The component's own", toasts: 1usize }
            }

            ToastProvider {
                "data-region": "custom-parts",
                inline: ToastInline::Center,
                appearance: ToastListAppearance::None,
                class: "[&>ol]:grid [&>ol]:gap-4",
                render_toast: Callback::new(|props: ToastPropsWithOwner| rsx! {
                    PlainToast { ..props }
                }),
                Trigger {
                    label: "Parts of your own",
                    toasts: ToastTitleAppearance::ALL.len(),
                }
            }
        }
    }
}

/// One button per provider, which is where a toast can be sent from.
///
/// The custom provider asks for one toast per value of the part axes, because
/// the toast it renders picks its appearance off the variant lists by index;
/// one toast cannot show two titles.
#[component]
fn Trigger(label: String, toasts: usize) -> Element {
    let toast = use_toast();
    let title = label.clone();

    rsx! {
        Button {
            onclick: move |_| {
                for number in 0..toasts {
                    toast
                        .info(
                            format!("{title} {number}"),
                            ToastOptions::new()
                                .description("Sent from the provider beside it")
                                .permanent(true),
                        );
                }
            },
            "{label}"
        }
    }
}

/// A toast rendered by the caller rather than by the registry.
///
/// It is the primitive's toast element (which is what carries the timer, the
/// ids and the ARIA) with this page's own classes on it and the registry's
/// parts inside.
///
/// Each toast shows one value of the part axes, taken off the variant lists by
/// its position in the stack. That is what makes this a rendered set rather
/// than one arrangement: a title has one appearance at a time, so the values
/// are spread across the toasts instead of across one.
#[allow(non_snake_case)]
fn PlainToast(props: ToastProps) -> Element {
    let title = ToastTitleAppearance::ALL[props.index % ToastTitleAppearance::ALL.len()];
    let description =
        ToastDescriptionAppearance::ALL[props.index % ToastDescriptionAppearance::ALL.len()];

    rsx! {
        primitive::Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            duration: props.duration,
            class: "alert border-2 border-dashed",
            ToastContent {
                ToastTitle { appearance: title, class: "uppercase tracking-wide" }
                ToastDescription { appearance: description, class: "opacity-70" }
            }
            ToastCloseButton {}
        }
    }
}
