use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::toggle;

/// daisyUI's colour axis for a toggle, which is the button's own.
///
/// The class strings are the button component's, duplicated rather than
/// depended on: the registry uses no cross-component dependencies, and the
/// Tailwind contract already requires every one of these to be a literal in the
/// file that emits it.
///
/// [`ToggleColor::Default`] emits no class at all, which is daisyUI's
/// uncoloured button rather than a synonym for [`ToggleColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ToggleColor {
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

impl ToggleColor {
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
            Self::Neutral => "btn-neutral",
            Self::Primary => "btn-primary",
            Self::Secondary => "btn-secondary",
            Self::Accent => "btn-accent",
            Self::Info => "btn-info",
            Self::Success => "btn-success",
            Self::Warning => "btn-warning",
            Self::Error => "btn-error",
        }
    }
}

/// daisyUI's size axis for a toggle, which is the button's own and is
/// duplicated for the reason [`ToggleColor`] records.
///
/// [`ToggleSize::Default`] emits no class, which renders at the same size as
/// daisyUI's explicit `btn-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ToggleSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl ToggleSize {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "btn-xs",
            Self::Sm => "btn-sm",
            Self::Default => "",
            Self::Lg => "btn-lg",
            Self::Xl => "btn-xl",
        }
    }
}

/// A button that stays pressed, carrying daisyUI's `btn` classes.
///
/// The pressed state is bridged at **Tier 2**: daisyUI's pressed button is
/// `btn-active`, and `aria-pressed` (which the primitive does set, on this very
/// element) appears nowhere in daisyUI's stylesheet. So the class is emitted
/// from Rust as a complete literal, and the state is **lifted** (ADR-0006) to
/// have something to emit it from: this component seeds a signal from
/// `default_pressed`, always hands the primitive a controlled value, and
/// intercepts the change callback, which leaves a controlled caller and an
/// uncontrolled one both working and this component the only writer.
///
/// The disabled state needs no such thing: the primitive sets the `disabled`
/// attribute on the `button` and daisyUI's rule is `.btn:disabled`.
///
/// Everything the caller passes lands on the button itself, which is both the
/// element daisyUI styles and the one the primitive gives the behaviour to.
/// Classes concatenate with this component's own; every other attribute
/// overrides them.
#[component]
pub fn Toggle(
    /// daisyUI's colour axis.
    #[props(default)]
    color: ToggleColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: ToggleSize,
    /// The controlled pressed state. `Some` makes the toggle controlled.
    #[props(default)]
    pressed: ReadSignal<Option<bool>>,
    /// Whether the toggle starts pressed when it is not controlled.
    #[props(default)]
    default_pressed: bool,
    /// Called when the pressed state changes.
    #[props(default)]
    on_pressed_change: Callback<bool>,
    /// Whether the toggle is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut uncontrolled = use_signal(|| default_pressed);

    // Both are read here rather than inside the markup, and eagerly rather than
    // only when the other is absent, so that this component subscribes to
    // whichever of them is driving and re-renders, which is what puts the
    // modifier class below on the element and takes it off again.
    let is_pressed = pressed().unwrap_or(uncontrolled());

    let color = color.class();
    let size = size.class();
    // Tier 2, and the whole reason the state is lifted.
    let state = if is_pressed { "btn-active" } else { "" };

    let base = attributes!(button {
        class: "btn {color} {size} {state}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        toggle::Toggle {
            pressed: Some(is_pressed),
            on_pressed_change: move |pressed| {
                uncontrolled.set(pressed);
                on_pressed_change.call(pressed);
            },
            disabled,
            attributes: merged,
            {children}
        }
    }
}
