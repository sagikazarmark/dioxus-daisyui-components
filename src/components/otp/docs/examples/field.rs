use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{
    FieldAppearance, FieldDescriptionAppearance, FieldErrorAppearance,
};
use crate::components::otp::{OtpAppearance, OtpField, OtpSize};

/// Field Context supplies the Binding, metadata, and focus request this OTP field resolves.
#[component]
pub fn Example() -> Element {
    let value = use_signal(String::new);
    let binding: Binding<String> = value.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("field-aware-otp")),
        name: Some(Rc::from("verification_code")),
        required: true,
        errors: vec![Rc::from("Enter the six-digit verification code.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();

    rsx! {
        OtpField {
            context,
            label: "Verification code",
            description: "Use the code from your authenticator app.",
            size: OtpSize::Sm,
            appearance: OtpAppearance::Joined,
            length: 6,
            field_appearance: FieldAppearance::None,
            description_appearance: FieldDescriptionAppearance::None,
            error_appearance: FieldErrorAppearance::None,
            class: "rounded-none",
            aria_label: "Authenticator verification code",
            "data-otp-field": "forwarded",
        }
        button {
            id: "focus-field-aware-otp",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                focus_request.request();
            },
            "Focus code"
        }
        output {
            "data-testid": "field-aware-otp-value",
            class: "text-sm opacity-70",
            "Current value: {value}"
        }
    }
}
