use dioxus::prelude::*;
use dioxus_field::FieldContext;

use crate::components::{
    field::Field,
    radio_group::{RadioGroup, RadioItem},
};

/// Offscreen controls for focus behavior that requires a real browser.
#[component]
pub fn RadioGroupFocusFixtures() -> Element {
    let disabled_context = use_hook(FieldContext::empty);
    let disabled_focus_request = disabled_context.focus_request();
    let all_disabled_context = use_hook(FieldContext::empty);
    let all_disabled_focus_request = all_disabled_context.focus_request();
    let reserved_locator_context = use_hook(FieldContext::empty);
    let reserved_locator_focus_request = reserved_locator_context.focus_request();

    rsx! {
        div {
            id: "radio-group-focus-fixtures",
            inert: true,
            aria_hidden: "true",
            style: "position: fixed; left: -10000px; top: 0;",
            Field { context: disabled_context,
                RadioGroup {
                    id: "disabled-field-radio-group",
                    disabled: true,
                    aria_label: "Disabled group",
                    RadioItem {
                        value: "disabled".to_owned(),
                        index: 0usize,
                        aria_label: "Disabled group option",
                    }
                }
            }
            button {
                id: "focus-disabled-field-radio-group",
                r#type: "button",
                onclick: move |_| {
                    disabled_focus_request.request();
                },
                "Focus disabled group"
            }
            Field { context: all_disabled_context,
                RadioGroup {
                    id: "all-items-disabled-radio-group",
                    aria_label: "All options disabled",
                    RadioItem {
                        value: "first".to_owned(),
                        index: 0usize,
                        disabled: true,
                        aria_label: "Disabled first",
                    }
                    RadioItem {
                        value: "second".to_owned(),
                        index: 1usize,
                        disabled: true,
                        aria_label: "Disabled second",
                    }
                }
            }
            button {
                id: "focus-all-items-disabled-radio-group",
                r#type: "button",
                onclick: move |_| {
                    all_disabled_focus_request.request();
                },
                "Focus group whose items are disabled"
            }
            Field { context: reserved_locator_context,
                RadioGroup {
                    id: "reserved-locator-radio-group",
                    "data-field-group": "caller-locator",
                    aria_label: "Reserved locator group",
                    RadioItem {
                        id: "reserved-locator-radio-item",
                        value: "enabled".to_owned(),
                        index: 0usize,
                        aria_label: "Enabled item",
                    }
                }
            }
            button {
                id: "focus-reserved-locator-radio-group",
                r#type: "button",
                onclick: move |_| {
                    reserved_locator_focus_request.request();
                },
                "Focus group with caller locator"
            }
        }
    }
}
