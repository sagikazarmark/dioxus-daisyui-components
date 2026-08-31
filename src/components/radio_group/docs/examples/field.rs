use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::components::radio_group::{RadioGroup, RadioItem};

/// Field Context supplies the Binding, metadata, and focus request this radio group resolves.
#[component]
pub fn Example() -> Element {
    let cadence = use_signal(|| String::from("weekly"));
    let binding: Binding<String> = cadence.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-radio-group")),
        name: Some(Rc::from("cadence")),
        required: true,
        errors: vec![Rc::from("Confirm a notification cadence.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();
    let options = [("daily", "Daily"), ("weekly", "Weekly"), ("never", "Never")];

    rsx! {
        form { id: "field-aware-radio-group-form",
            Field { context, class: "max-w-sm",
                FieldLabel { id: "field-aware-radio-group-label", "Notification cadence" }
                RadioGroup { aria_labelledby: "field-aware-radio-group-label",
                    for (index , (value , label)) in options.into_iter().enumerate() {
                        div { class: "flex items-center gap-2",
                            RadioItem {
                                id: "field-aware-radio-{value}",
                                value: value.to_owned(),
                                index,
                                aria_label: label,
                            }
                            span { class: "text-sm", "{label}" }
                        }
                    }
                }
                FieldDescription {
                    "Choose how often notifications should arrive."
                }
                FieldError {}
            }
        }
        button {
            id: "focus-field-aware-radio-group",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus cadence"
        }
        output {
            "data-testid": "field-aware-radio-group-value",
            class: "text-sm opacity-70",
            "Current value: {cadence}"
        }
    }
}
