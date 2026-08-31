use dioxus::prelude::*;

use crate::components::switch::{Switch, SwitchColor};

/// Every state a switch renders in, and one the caller drives.
///
/// The on and off pair carry identical classes: daisyUI keys the difference on
/// the `aria-checked` attribute the primitive sets, and the disabled pair on
/// the `disabled` attribute, so nothing is emitted for any of the three, and
/// the disabled attribute also makes the switch inert.
///
/// The last one is controlled by this example rather than by itself, so the
/// state travels out through the change callback and back in through the
/// value prop for a toggle to be visible at all. Every toggle also commits.
#[component]
pub fn Example() -> Element {
    let mut toggled = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Switch { id: "off", aria_label: "Off" }
            Switch { id: "on", default_value: true, aria_label: "On" }

            Switch { disabled: true, aria_label: "Disabled" }
            Switch { disabled: true, default_value: true, aria_label: "Disabled on" }

            Switch {
                color: SwitchColor::Primary,
                value: Some(toggled()),
                on_change: move |on| {
                    toggled.set(on);
                    changes += 1;
                },
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
                aria_label: "Toggle",
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }
            p { class: "text-sm opacity-70",
                "Committed "
                span { "data-testid": "switch-commits", "{commits}" }
                " times"
            }
            output { hidden: true, "data-testid": "switch-focus-exits", "{focus_exits}" }
        }
    }
}
