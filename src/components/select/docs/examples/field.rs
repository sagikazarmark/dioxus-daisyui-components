use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::components::select::{Select, SelectList, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// Field Context supplies the Binding, metadata, and focus request this select resolves.
#[component]
pub fn Example() -> Element {
    let value = use_signal(|| Option::<String>::None);
    let mut focus_exits = use_signal(|| 0_u32);
    let binding: Binding<Option<String>> = value.into();
    let binding = binding.with_focus_exit(Callback::new(move |()| focus_exits += 1));
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-select")),
        name: Some(Rc::from("fruit")),
        required: true,
        errors: vec![Rc::from("Choose a fruit.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();
    let selected = value().unwrap_or_else(|| String::from("nothing"));

    rsx! {
        Field { context, class: "max-w-sm",
            FieldLabel { "Fruit" }
            Select::<String> {
                SelectTrigger {
                    SelectValue { placeholder: "Pick a fruit" }
                }
                SelectList {
                    Options {}
                }
            }
            FieldDescription {
                "Choose one fruit for the release basket."
            }
            FieldError {}
        }
        button {
            id: "focus-field-aware-select",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus fruit"
        }
        output {
            "data-testid": "field-aware-select-value",
            class: "text-sm opacity-70",
            "Current value: {selected}"
        }
        output {
            "data-testid": "field-aware-select-focus-exits",
            class: "hidden",
            "{focus_exits}"
        }
    }
}
