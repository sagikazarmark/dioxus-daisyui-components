use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin, FieldContext, FieldControlOptions, FieldMeta, FieldSurface,
    merge_attributes, use_binding, use_field_meta, use_focus_registration,
};
use dioxus_primitives::dioxus_attributes::attributes;
use std::rc::Rc;

use crate::components::field::{
    Field, FieldAppearance, FieldDescription, FieldDescriptionAppearance, FieldError,
    FieldErrorAppearance, FieldLabel,
};

/// daisyUI's colour axis for an OTP field.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum OtpColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl OtpColor {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[
        Self::Default,
        Self::Neutral,
        Self::Primary,
        Self::Secondary,
        Self::Accent,
        Self::Info,
        Self::Success,
        Self::Warning,
        Self::Error,
    ];

    /// The daisyUI class name for this value.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Neutral => "otp-neutral",
            Self::Primary => "otp-primary",
            Self::Secondary => "otp-secondary",
            Self::Accent => "otp-accent",
            Self::Info => "otp-info",
            Self::Success => "otp-success",
            Self::Warning => "otp-warning",
            Self::Error => "otp-error",
        }
    }
}

/// daisyUI's size axis for an OTP field.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum OtpSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl OtpSize {
    /// Every value of this axis, from smallest to largest.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "otp-xs",
            Self::Sm => "otp-sm",
            Self::Default => "",
            Self::Lg => "otp-lg",
            Self::Xl => "otp-xl",
        }
    }
}

/// daisyUI's appearance axis for an OTP field.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum OtpAppearance {
    #[default]
    Default,
    Joined,
}

impl OtpAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Joined];

    /// The daisyUI class name for this value.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Joined => "otp-joined",
        }
    }
}

/// A native one-time-code field rendered with daisyUI's `otp` markup.
///
/// daisyUI paints one empty `span` per character box over a single real input.
/// Classes passed by the caller land on the label that daisyUI styles; other
/// global attributes land on the native input.
///
/// Producer-defined invalidity emits `otp-error` when no colour is passed.
/// Binding, metadata, and focus resolve from explicit props, Field Context,
/// then standalone state.
#[component]
pub fn Otp(
    /// An explicit colour, or `None` to derive error colour from Field metadata.
    #[props(default)]
    color: Option<OtpColor>,
    /// daisyUI's size axis.
    #[props(default)]
    size: OtpSize,
    /// Whether the character boxes are separate or joined.
    #[props(default)]
    appearance: OtpAppearance,
    /// The number of character boxes. daisyUI supports one through eight.
    #[props(default = 4)]
    length: usize,
    /// An explicit Field binding, which wins over Field Context.
    binding: Option<Binding<String>>,
    /// Explicit Field metadata, which wins over Field Context.
    meta: Option<FieldMeta>,
    /// The value rendered by the input.
    #[props(default)]
    value: Option<ReadSignal<String>>,
    /// Whether the native input is required.
    #[props(default)]
    required: Option<bool>,
    /// The name under which the input participates in a form.
    #[props(default)]
    name: Option<String>,
    /// Whether the native input is disabled.
    #[props(default)]
    disabled: Option<bool>,
    /// Called with the input's value after user input.
    on_change: Option<EventHandler<String>>,
    /// Called when a complete code or native `change` ends the interaction unit.
    on_commit: Option<EventHandler<()>>,
    /// Called after focus leaves the styled OTP label subtree.
    on_focus_exit: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    assert!(
        (1..=8).contains(&length),
        "Otp length must be between 1 and 8"
    );

    let binding = use_binding(binding, String::new());
    let meta = use_field_meta(meta);
    let pattern = format!("[0-9]{{{length}}}");
    let color = color.map_or_else(
        || {
            if meta.invalid() { "otp-error" } else { "" }
        },
        OtpColor::class,
    );
    let size = size.class();
    let appearance = appearance.class();
    let binding_value = binding.read;
    let resolved_value = value.unwrap_or(binding_value);
    let mut control: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let focus_control = use_callback(move |()| {
        if let Some(control) = control() {
            spawn(async move {
                let _ = control.set_focus(true).await;
            });
        }
    });
    use_focus_registration(focus_control);

    let base = attributes!(label {
        class: "otp {color} {size} {appearance}"
    });
    let inputmode = attributes!(input {
        inputmode: "numeric"
    });
    let mut caller_attributes = merge_attributes(vec![base, attributes, inputmode]);
    let class = take_class(&mut caller_attributes);
    let meta_attributes = meta.attributes_for(
        &FieldControlOptions::new()
            .disabled(disabled)
            .required(required)
            .name(name.map(Rc::from))
            .surface(FieldSurface::NATIVE),
    );
    let merged = merge_attributes(vec![meta_attributes, caller_attributes]);
    let change_binding = binding.clone();
    let input_commit_binding = binding.clone();
    let change_commit_binding = binding.clone();
    let focus_exit_binding = binding;
    let mut committed_on_input = use_signal(|| false);

    rsx! {
        label {
            class,
            onfocusout: move |_| {
                focus_exit_binding.focus_exit();
                if let Some(handler) = &on_focus_exit {
                    handler.call(());
                }
            },
            for _ in 0..length {
                span {}
            }
            input {
                r#type: "text",
                autocomplete: "one-time-code",
                maxlength: length,
                pattern,
                value: resolved_value,
                onmounted: move |event: MountedEvent| control.set(Some(event.data())),
                oninput: move |event| {
                    let next = event.value();
                    change_binding.write(next.clone(), ChangeOrigin::User);
                    if let Some(handler) = &on_change {
                        handler.call(next.clone());
                    }

                    let complete = is_complete_code(&next, length);
                    committed_on_input.set(complete);
                    if complete {
                        input_commit_binding.commit();
                        if let Some(handler) = &on_commit {
                            handler.call(());
                        }
                    }
                },
                onchange: move |_| {
                    if !committed_on_input() {
                        change_commit_binding.commit();
                        if let Some(handler) = &on_commit {
                            handler.call(());
                        }
                    }
                    committed_on_input.set(false);
                },
                ..merged,
            }
        }
    }
}

/// The common Field composition for a native one-time-code input.
///
/// This Composition sugar intentionally has no children. Use [`Field`] and its
/// Compound parts when content or attributes must land between the parts. Caller
/// classes are forwarded to the styled OTP label; other global attributes are
/// forwarded to the native input.
#[component]
pub fn OtpField(
    /// The context supplied to the input and every Field part.
    #[props(into)]
    context: FieldContext,
    /// The input's visible label.
    label: String,
    /// Supporting text rendered between the input and its error region.
    #[props(default)]
    description: Option<String>,
    /// An explicit colour, or `None` to derive error colour from Field metadata.
    #[props(default)]
    color: Option<OtpColor>,
    /// daisyUI's size axis.
    #[props(default)]
    size: OtpSize,
    /// Whether the character boxes are separate or joined.
    #[props(default)]
    appearance: OtpAppearance,
    /// The number of character boxes. daisyUI supports one through eight.
    #[props(default = 4)]
    length: usize,
    /// Whether the surrounding Field emits its default layout utilities.
    #[props(default)]
    field_appearance: FieldAppearance,
    /// Whether supporting text emits its default wrapping utilities.
    #[props(default)]
    description_appearance: FieldDescriptionAppearance,
    /// Whether the error region emits its default semantic colour.
    #[props(default)]
    error_appearance: FieldErrorAppearance,
    /// An explicit Field binding, which wins over `context` for the input.
    binding: Option<Binding<String>>,
    /// Explicit Field metadata, which wins over `context` for the input.
    meta: Option<FieldMeta>,
    /// The value rendered by the input.
    #[props(default)]
    value: Option<ReadSignal<String>>,
    /// Whether the native input is required.
    #[props(default)]
    required: Option<bool>,
    /// The name under which the input participates in a form.
    #[props(default)]
    name: Option<String>,
    /// Whether the native input is disabled.
    #[props(default)]
    disabled: Option<bool>,
    /// Called with the input's value after user input.
    on_change: Option<EventHandler<String>>,
    /// Called when a complete code or native `change` ends the interaction unit.
    on_commit: Option<EventHandler<()>>,
    /// Called after focus leaves the styled OTP label subtree.
    on_focus_exit: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Field { context, appearance: field_appearance,
            FieldLabel { {label} }
            Otp {
                color,
                size,
                appearance,
                length,
                binding,
                meta,
                value,
                required,
                name,
                disabled,
                on_change,
                on_commit,
                on_focus_exit,
                attributes,
            }
            if let Some(description) = description {
                FieldDescription { appearance: description_appearance, {description} }
            }
            FieldError { appearance: error_appearance }
        }
    }
}

fn is_complete_code(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|digit| digit.is_ascii_digit())
}

/// Moves the merged class to the styled label, leaving global attributes for
/// the real input they identify and describe.
fn take_class(attributes: &mut Vec<Attribute>) -> String {
    let class = attributes.iter().position(|attribute| {
        attribute.name == "class" && matches!(attribute.value, AttributeValue::Text(_))
    });

    match class.map(|index| attributes.remove(index).value) {
        Some(AttributeValue::Text(class)) => class,
        _ => String::new(),
    }
}
