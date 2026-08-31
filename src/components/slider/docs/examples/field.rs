use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::{
    field::{FieldAppearance, FieldDescriptionAppearance, FieldErrorAppearance},
    slider::{RangeSliderField, SliderAppearance, SliderField, SliderSize},
};

/// The closed Field wrappers share each slider's Binding, metadata, and focus request.
#[component]
pub fn Example() -> Element {
    let volume = use_signal(|| 35.0);
    let volume_binding: Binding<f64> = volume.into();
    let volume_meta = use_field_meta_state(FieldMetaValues {
        name: Some(Rc::from("volume")),
        required: true,
        errors: vec![Rc::from("Choose a supported volume")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins each context's focus slot, so keep both stable across value updates.
    let volume_context = use_hook(move || FieldContext::new(volume_binding).with_meta(volume_meta));
    let volume_focus = volume_context.focus_request();

    let span = use_signal(|| 20.0..70.0);
    let span_binding: Binding<std::ops::Range<f64>> = span.into();
    let span_meta = use_field_meta_state(FieldMetaValues {
        name: Some(Rc::from("price")),
        required: true,
        errors: vec![Rc::from("Choose a supported price range")],
        touched: true,
        ..FieldMetaValues::default()
    });
    let span_context = use_hook(move || FieldContext::new(span_binding).with_meta(span_meta));
    let span_focus = span_context.focus_request();

    rsx! {
        div { class: "grid max-w-md gap-8",
            SliderField {
                context: volume_context,
                label: "Volume",
                description: "Set the playback volume.",
                size: SliderSize::Sm,
                appearance: SliderAppearance::None,
                field_appearance: FieldAppearance::None,
                description_appearance: FieldDescriptionAppearance::None,
                error_appearance: FieldErrorAppearance::None,
                class: "relative w-64",
                "data-testid": "field-aware-slider",
            }
            button {
                id: "focus-field-aware-slider",
                r#type: "button",
                class: "btn btn-sm w-fit",
                onclick: move |_| {
                    volume_focus.request();
                },
                "Focus volume"
            }
            output {
                "data-testid": "field-aware-slider-value",
                class: "text-sm opacity-70",
                "Current value: {volume}"
            }

            RangeSliderField {
                context: span_context,
                label: "Price range",
                description: "Set the acceptable price span.",
                size: SliderSize::Lg,
                class: "w-64",
                "data-testid": "field-aware-range-slider",
            }
            button {
                id: "focus-field-aware-range-slider",
                r#type: "button",
                class: "btn btn-sm w-fit",
                onclick: move |_| {
                    span_focus.request();
                },
                "Focus price range"
            }
            output {
                "data-testid": "field-aware-range-slider-value",
                class: "text-sm opacity-70",
                "Current range: {span().start} to {span().end}"
            }
        }
    }
}
