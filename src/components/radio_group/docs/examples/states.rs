use dioxus::prelude::*;

use crate::components::radio_group::{RadioGroup, RadioItem, RadioItemColor};

/// Chosen, unchosen and disabled, plus a group this page controls.
///
/// The chosen and unchosen items carry identical classes: daisyUI keys the
/// difference on the `aria-checked` attribute the primitive sets, and the
/// disabled one on the `disabled` attribute, so nothing is emitted for any of
/// them, and the disabled attribute also takes the item out of the keyboard
/// order.
///
/// The value travels out through the change callback and back in through the
/// value prop, so what is rendered below is the group's state as this example
/// holds it rather than as the primitive keeps it.
#[component]
pub fn Example() -> Element {
    let mut chosen = use_signal(|| String::from("standard"));
    let mut changes = use_signal(|| 0_u32);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    let options = [
        ("standard", "Standard", false),
        ("priority", "Priority", false),
        ("overnight", "Overnight, unavailable here", true),
    ];

    rsx! {
        div { class: "flex flex-col gap-3",
            RadioGroup {
                horizontal: true,
                name: "shipping",
                value: Some(chosen()),
                on_change: move |value: String| {
                    chosen.set(value);
                    changes += 1;
                },
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
                aria_label: "Shipping",
                for (index , (value , label , disabled)) in options.into_iter().enumerate() {
                    div { class: "flex items-center gap-2",
                        RadioItem {
                            value: value.to_string(),
                            index,
                            disabled,
                            color: RadioItemColor::Primary,
                            aria_labelledby: "states-{value}",
                        }
                        span { id: "states-{value}", class: "text-sm", "{label}" }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                "Chose "
                span { "data-testid": "value", "{chosen}" }
                ", changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }
            output { hidden: true, "data-testid": "radio-group-commits", "{commits}" }
            output { hidden: true, "data-testid": "radio-group-focus-exits", "{focus_exits}" }
        }
    }
}
