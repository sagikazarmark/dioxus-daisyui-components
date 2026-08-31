use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's colour axis for a file input.
///
/// [`FileInputColor::Default`] emits no class, which is daisyUI's uncoloured
/// file input rather than a synonym for [`FileInputColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum FileInputColor {
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

impl FileInputColor {
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

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Neutral => "file-input-neutral",
            Self::Primary => "file-input-primary",
            Self::Secondary => "file-input-secondary",
            Self::Accent => "file-input-accent",
            Self::Info => "file-input-info",
            Self::Success => "file-input-success",
            Self::Warning => "file-input-warning",
            Self::Error => "file-input-error",
        }
    }
}

/// daisyUI's size axis for a file input.
///
/// [`FileInputSize::Default`] emits no class and renders at the same size as
/// daisyUI's explicit `file-input-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum FileInputSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl FileInputSize {
    /// Every value of this axis, from smallest to largest.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "file-input-xs",
            Self::Sm => "file-input-sm",
            Self::Default => "",
            Self::Lg => "file-input-lg",
            Self::Xl => "file-input-xl",
        }
    }
}

/// daisyUI's appearance axis for a file input.
///
/// [`FileInputAppearance::Default`] emits no class, which is daisyUI's bordered
/// file input rather than a named style.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum FileInputAppearance {
    #[default]
    Default,
    Ghost,
}

impl FileInputAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Ghost];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Ghost => "file-input-ghost",
        }
    }
}

/// A native file picker styled with daisyUI's `file-input` class.
///
/// Classes passed by the caller concatenate with the file input's own. The
/// defining `type="file"` cannot be overridden; file-specific configuration and
/// the change handler are explicit props.
#[component]
pub fn FileInput(
    /// daisyUI's colour axis.
    #[props(default)]
    color: FileInputColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: FileInputSize,
    /// daisyUI's appearance axis.
    #[props(default)]
    appearance: FileInputAppearance,
    /// The file types the picker offers, as the native `accept` attribute.
    accept: Option<String>,
    /// Whether the picker lets more than one file be chosen.
    multiple: Option<bool>,
    /// The name of the input, used in forms.
    name: Option<String>,
    /// The id of the form the input belongs to, when it is not a descendant
    /// of one.
    form: Option<String>,
    /// Whether the native input is required.
    required: Option<bool>,
    /// Whether the native input is disabled.
    disabled: Option<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    /// Called when the chosen files change. Explicit because Dioxus' extended
    /// attributes do not include event handlers.
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let color = color.class();
    let size = size.class();
    let appearance = appearance.class();

    let base = attributes!(input {
        class: "file-input {color} {size} {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        input {
            r#type: "file",
            accept,
            multiple,
            name,
            form,
            required,
            disabled,
            onchange: move |event| {
                if let Some(handler) = &onchange {
                    handler.call(event);
                }
            },
            ..merged,
        }
    }
}
