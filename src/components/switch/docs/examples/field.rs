use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::switch::SwitchField;

/// Field Context supplies the Binding, metadata, and focus request this switch resolves.
#[component]
pub fn Example() -> Element {
    let enabled = use_signal(|| false);
    let binding: Binding<bool> = enabled.into();
    let meta = use_field_meta_state(FieldMetaValues {
        name: Some(Rc::from("telemetry")),
        required: true,
        errors: vec![Rc::from("Choose whether to send anonymous telemetry.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();

    rsx! {
        form { id: "field-aware-switch-form",
            SwitchField {
                context,
                label: "Anonymous telemetry",
                description: "Send anonymous usage data to help improve the product.",
            }
        }
        button {
            id: "focus-field-aware-switch",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus telemetry"
        }
        output {
            "data-testid": "field-aware-switch-value",
            class: "text-sm opacity-70",
            "Current state: {enabled}"
        }
    }
}
