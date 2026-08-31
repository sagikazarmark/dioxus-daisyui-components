use dioxus::prelude::*;
use dioxus_field::Binding;

use crate::components::checkbox::{Checkbox, CheckboxColor, CheckboxState};

/// Every state a checkbox renders in, and one the caller drives.
///
/// The checked and unchecked pair carry identical classes: daisyUI keys the
/// difference on the `aria-checked` attribute the primitive sets, and the
/// disabled pair on the `disabled` attribute, so nothing is emitted for any of
/// the three, and the disabled attribute also makes the checkbox inert.
///
/// The last one is controlled by this example rather than by itself, so the
/// state travels out through the change callback and back in through the value
/// prop for a toggle to be visible at all. Every toggle also commits.
#[component]
pub fn Example() -> Element {
    let mut toggled = use_signal(|| CheckboxState::Unchecked);
    let mut changes = use_signal(|| 0_u32);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);
    let disabled = use_signal(|| false);
    let disabled_binding: Binding<bool> = disabled.into();

    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Checkbox { id: "unchecked", aria_label: "Unchecked" }
            Checkbox {
                id: "checked",
                default_value: CheckboxState::Checked,
                aria_label: "Checked",
            }
            Checkbox {
                id: "indeterminate",
                default_value: CheckboxState::Indeterminate,
                aria_label: "Indeterminate",
            }

            Checkbox {
                bool_binding: disabled_binding,
                disabled: true,
                aria_label: "Disabled",
            }
            Checkbox {
                disabled: true,
                default_value: CheckboxState::Checked,
                aria_label: "Disabled checked",
            }

            Checkbox {
                color: CheckboxColor::Primary,
                value: Some(toggled()),
                on_change: move |state| {
                    toggled.set(state);
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
                span { "data-testid": "checkbox-commits", "{commits}" }
                " times"
            }
            output { hidden: true, "data-testid": "checkbox-focus-exits", "{focus_exits}" }
            output { hidden: true, "data-testid": "disabled-bool-value", "{disabled}" }
        }
    }
}
