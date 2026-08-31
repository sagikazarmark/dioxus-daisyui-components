use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::FieldAppearance;
use crate::components::input::{InputAppearance, InputSize, InputField};

/// Field Context supplies the Binding and metadata this input resolves.
#[component]
pub fn Example() -> Element {
    let value = use_signal(String::new);
    let binding: Binding<String> = value.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-input")),
        name: Some(Rc::from("account")),
        required: true,
        invalid: Some(true),
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();

    rsx! {
        InputField {
            context,
            label: "Account",
            size: InputSize::Sm,
            appearance: InputAppearance::Ghost,
            field_appearance: FieldAppearance::None,
            r#type: "email",
            placeholder: "Account name",
        }
        button {
            id: "focus-field-aware-input",
            r#type: "button",
            class: "btn btn-sm",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus account"
        }
        output {
            "data-testid": "field-aware-input-value",
            class: "text-sm opacity-70",
            "Current value: {value}"
        }
    }
}
