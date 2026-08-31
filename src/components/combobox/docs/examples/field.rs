use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::combobox::{Combobox, ComboboxInput, ComboboxList};
use crate::components::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::examples::combobox::options::Options;

/// Field Context supplies the selected-value Binding, metadata, and focus request.
#[component]
pub fn Example() -> Element {
    let selected = use_signal(|| Option::<String>::None);
    let mut focus_exits = use_signal(|| 0_u32);
    let binding: Binding<Option<String>> = selected.into();
    let binding = binding.with_focus_exit(Callback::new(move |()| focus_exits += 1));
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-combobox")),
        name: Some(Rc::from("fruit")),
        required: true,
        errors: vec![Rc::from("Choose a fruit.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();

    rsx! {
        Field { context, class: "max-w-sm",
            FieldLabel { "Fruit" }
            Combobox::<String> {
                ComboboxInput { placeholder: "Pick a fruit" }
                ComboboxList {
                    Options {}
                }
            }
            FieldDescription {
                "Type to filter the available fruit."
            }
            FieldError {}
        }
        button {
            id: "focus-field-aware-combobox",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus fruit"
        }
        output {
            "data-testid": "field-aware-combobox-value",
            class: "text-sm opacity-70",
            "Current value: {selected:?}"
        }
        output {
            "data-testid": "field-aware-combobox-focus-exits",
            class: "hidden",
            "{focus_exits}"
        }
    }
}
