use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{FieldDescriptionAppearance, FieldErrorAppearance};
use crate::components::textarea::{TextareaAppearance, TextareaField, TextareaSize};

/// Field Context supplies the Binding, metadata, and focus request this textarea resolves.
#[component]
pub fn Example() -> Element {
    let value = use_signal(String::new);
    let binding: Binding<String> = value.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-textarea")),
        name: Some(Rc::from("notes")),
        required: true,
        errors: vec![Rc::from("Add implementation notes.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();

    rsx! {
        TextareaField {
            context,
            label: "Implementation notes",
            description: "Include constraints and important tradeoffs.",
            size: TextareaSize::Sm,
            appearance: TextareaAppearance::Ghost,
            description_appearance: FieldDescriptionAppearance::None,
            error_appearance: FieldErrorAppearance::None,
            class: "max-w-sm",
            rows: 5,
            placeholder: "Describe the implementation",
        }
        button {
            id: "focus-field-aware-textarea",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus notes"
        }
        output {
            "data-testid": "field-aware-textarea-value",
            class: "text-sm opacity-70",
            "Current value: {value}"
        }
    }
}
