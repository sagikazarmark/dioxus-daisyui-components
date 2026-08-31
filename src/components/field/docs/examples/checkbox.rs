use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::checkbox::{Checkbox, CheckboxState};
use crate::components::field::{Field, FieldDescription, FieldError, FieldLabel};

/// Field metadata associates the Compound parts with a primitive-backed Checkbox.
#[component]
pub fn Example() -> Element {
    let checked = use_signal(|| CheckboxState::Unchecked);
    let binding: Binding<CheckboxState> = checked.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("product-updates")),
        name: Some(Rc::from("product_updates")),
        errors: vec![Rc::from("Choose whether to receive product updates.")],
        ..FieldMetaValues::default()
    });
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));

    rsx! {
        Field { id: "product-updates-field", context, class: "max-w-sm",
            FieldLabel { id: "product-updates-label", "Product updates" }
            Checkbox {}
            FieldDescription {
                "Receive a short email when a release ships."
            }
            FieldError {}
        }
        output { "data-testid": "product-updates-value", "Current state: {checked:?}" }
    }
}
