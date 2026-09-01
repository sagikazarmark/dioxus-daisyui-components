use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::FieldAppearance;
use crate::components::native_select::{NativeSelectField, NativeSelectOption, NativeSelectSize};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Flavor {
    Orange,
    Lemon,
    Cherry,
}

/// Field Context supplies the Binding and metadata this select resolves.
#[component]
pub fn Example() -> Element {
    let value: Signal<Option<Flavor>> = use_signal(|| None);
    let binding: Binding<Option<Flavor>> = value.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-native-select")),
        name: Some(Rc::from("flavor")),
        required: true,
        invalid: Some(true),
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();

    rsx! {
        NativeSelectField {
            context,
            label: "Flavor",
            size: NativeSelectSize::Sm,
            field_appearance: FieldAppearance::None,
            placeholder: "Pick a flavor",
            options: vec![
                NativeSelectOption::new(Flavor::Orange, "Orange"),
                NativeSelectOption::new(Flavor::Lemon, "Lemon"),
                NativeSelectOption::new(Flavor::Cherry, "Cherry"),
            ],
        }
        button {
            id: "focus-field-aware-native-select",
            r#type: "button",
            class: "btn btn-sm",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus flavor"
        }
        output { "data-testid": "field-aware-native-select-value",
            class: "text-sm opacity-70",
            {format!("Current value: {:?}", value())}
        }
    }
}
