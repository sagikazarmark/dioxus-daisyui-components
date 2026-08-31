use dioxus::prelude::*;

use crate::components::slider::{RangeSlider, Slider, SliderColor};

/// A slider with two handles, and one that is disabled.
///
/// daisyUI has no two-thumb range at all (a native input cannot be one) so the
/// span here is the primitive's shape wearing daisyUI's clothes: the same
/// classes on the root, the same parts inside it, and a fill that runs between
/// the handles rather than from the start. Each handle is bounded by the other,
/// which is the primitive's doing.
///
/// The disabled one shows what daisyUI's `.range:disabled` would have done, put
/// back through a data attribute because a `div` is never `:disabled`
/// (ADR-0016). Its handle also leaves the tab order and announces itself as
/// disabled while the primitive refuses its keys.
#[component]
pub fn Example() -> Element {
    let mut span = use_signal(|| 20.0..70.0);
    let mut changes = use_signal(|| 0usize);
    let mut commits = use_signal(|| 0usize);
    let mut focus_exits = use_signal(|| 0usize);
    let mut disabled = use_signal(|| true);

    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-2",
                p { class: "text-sm opacity-70",
                    "Between "
                    span { "data-testid": "span-start", "{span().start}" }
                    " and "
                    span { "data-testid": "span-end", "{span().end}" }
                }
                RangeSlider {
                    id: "span",
                    label: "Price range",
                    color: SliderColor::Accent,
                    value: Some(span()),
                    on_change: move |next| {
                        span.set(next);
                        changes += 1;
                    },
                    on_commit: move |()| commits += 1,
                    on_focus_exit: move |()| focus_exits += 1,
                    class: "w-64",
                }
                p { class: "text-xs opacity-60",
                    "Changes: "
                    span { "data-testid": "range-slider-changes", "{changes}" }
                    ", commits: "
                    span { "data-testid": "range-slider-commits", "{commits}" }
                }
                output {
                    class: "hidden",
                    "data-testid": "range-slider-focus-exits",
                    "{focus_exits}"
                }
            }

            div { class: "flex flex-col gap-2",
                button {
                    id: "toggle-disabled-slider",
                    r#type: "button",
                    aria_label: if disabled() { "Enable slider" } else { "Disable slider" },
                    class: "text-left text-sm opacity-70",
                    onclick: move |_| disabled.set(!disabled()),
                    if disabled() { "Disabled" } else { "Enabled" }
                }
                Slider {
                    id: "disabled",
                    label: "Unavailable",
                    default_value: 50.0,
                    disabled: disabled(),
                    class: "w-64",
                }
            }
        }
    }
}
