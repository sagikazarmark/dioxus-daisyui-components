use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's appearance axis for a theme controller, which is the control it is
/// drawn as.
///
/// `theme-controller` paints nothing (it carries the theme and nothing else)
/// so every one of daisyUI's own theme controllers borrows the look of another
/// control. This axis is which one it borrows, and the colour and size axes
/// follow it, because daisyUI writes one scale per control rather than one for
/// this class.
///
/// The value also decides the input's `type`, since daisyUI's scales are gated
/// on it: `.toggle-lg[type=checkbox]` and `.radio-lg[type=radio]`. Pairing the
/// two here is what keeps a controller from carrying a size class that cannot
/// match (ADR-0010): each value is one of daisyUI's own pairings rather than a
/// combination the caller assembles.
///
/// [`ThemeControllerAppearance::None`] emits no class at all, for a control the
/// caller draws themselves; daisyUI's `swap` is written that way, with a bare
/// input inside a label the caller owns.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ThemeControllerAppearance {
    /// daisyUI's `toggle`, on a checkbox: one theme against the one the
    /// document is already under.
    #[default]
    Toggle,
    /// daisyUI's `checkbox`, on a checkbox: the same choice, drawn as a box.
    Checkbox,
    /// daisyUI's `radio`, on a radio: one theme out of a named set.
    Radio,
    /// daisyUI's `btn`, on a radio: a row or a menu of themes. The input has no
    /// children, so daisyUI prints its `aria-label` as the button's text.
    Button,
    /// No class at all, on a checkbox, for a control the caller draws.
    None,
}

impl ThemeControllerAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[
        Self::Toggle,
        Self::Checkbox,
        Self::Radio,
        Self::Button,
        Self::None,
    ];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Toggle => "toggle",
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            Self::Button => "btn",
            Self::None => "",
        }
    }

    /// The `type` the input carries under this appearance, which is what
    /// daisyUI's size rules are gated on and what decides whether the control
    /// is one of a set.
    pub const fn input_type(self) -> &'static str {
        match self {
            Self::Toggle | Self::Checkbox | Self::None => "checkbox",
            Self::Radio | Self::Button => "radio",
        }
    }
}

/// daisyUI's colour axis for a theme controller, which is the colour of
/// whichever control the appearance draws.
///
/// The value names a colour and the appearance decides which class carries it,
/// because daisyUI writes a scale per control (`toggle-primary`,
/// `checkbox-primary`, `radio-primary`, `btn-primary`) and there is no
/// `theme-controller-primary` for it to write one against.
///
/// [`ThemeControllerColor::Default`] emits no class at all, which is daisyUI's
/// own uncoloured control rather than a synonym for
/// [`ThemeControllerColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ThemeControllerColor {
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

impl ThemeControllerColor {
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

    /// The daisyUI class name for this value under one appearance, as a
    /// complete string literal so Tailwind's scanner can see it.
    ///
    /// The appearance that emits no paint takes no colour either: every one of
    /// these classes only sets a custom property that the paint reads, so one
    /// on its own would change nothing.
    pub const fn class(self, appearance: ThemeControllerAppearance) -> &'static str {
        match appearance {
            ThemeControllerAppearance::Toggle => match self {
                Self::Default => "",
                Self::Neutral => "toggle-neutral",
                Self::Primary => "toggle-primary",
                Self::Secondary => "toggle-secondary",
                Self::Accent => "toggle-accent",
                Self::Info => "toggle-info",
                Self::Success => "toggle-success",
                Self::Warning => "toggle-warning",
                Self::Error => "toggle-error",
            },
            ThemeControllerAppearance::Checkbox => match self {
                Self::Default => "",
                Self::Neutral => "checkbox-neutral",
                Self::Primary => "checkbox-primary",
                Self::Secondary => "checkbox-secondary",
                Self::Accent => "checkbox-accent",
                Self::Info => "checkbox-info",
                Self::Success => "checkbox-success",
                Self::Warning => "checkbox-warning",
                Self::Error => "checkbox-error",
            },
            ThemeControllerAppearance::Radio => match self {
                Self::Default => "",
                Self::Neutral => "radio-neutral",
                Self::Primary => "radio-primary",
                Self::Secondary => "radio-secondary",
                Self::Accent => "radio-accent",
                Self::Info => "radio-info",
                Self::Success => "radio-success",
                Self::Warning => "radio-warning",
                Self::Error => "radio-error",
            },
            ThemeControllerAppearance::Button => match self {
                Self::Default => "",
                Self::Neutral => "btn-neutral",
                Self::Primary => "btn-primary",
                Self::Secondary => "btn-secondary",
                Self::Accent => "btn-accent",
                Self::Info => "btn-info",
                Self::Success => "btn-success",
                Self::Warning => "btn-warning",
                Self::Error => "btn-error",
            },
            ThemeControllerAppearance::None => "",
        }
    }
}

/// daisyUI's size axis for a theme controller, which is the size of whichever
/// control the appearance draws.
///
/// Read the same way as the colour axis: the value names a size and the
/// appearance decides which scale carries it. Three of those scales are gated
/// on the input's `type`, which is why the appearance owns that too.
///
/// [`ThemeControllerSize::Default`] emits no class and renders at the same size
/// as daisyUI's explicit `-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ThemeControllerSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl ThemeControllerSize {
    /// Every value of this axis, from the smallest to the largest, which is the
    /// order the preview renders them in.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value under one appearance, as a
    /// complete string literal so Tailwind's scanner can see it.
    pub const fn class(self, appearance: ThemeControllerAppearance) -> &'static str {
        match appearance {
            ThemeControllerAppearance::Toggle => match self {
                Self::Xs => "toggle-xs",
                Self::Sm => "toggle-sm",
                Self::Default => "",
                Self::Lg => "toggle-lg",
                Self::Xl => "toggle-xl",
            },
            ThemeControllerAppearance::Checkbox => match self {
                Self::Xs => "checkbox-xs",
                Self::Sm => "checkbox-sm",
                Self::Default => "",
                Self::Lg => "checkbox-lg",
                Self::Xl => "checkbox-xl",
            },
            ThemeControllerAppearance::Radio => match self {
                Self::Xs => "radio-xs",
                Self::Sm => "radio-sm",
                Self::Default => "",
                Self::Lg => "radio-lg",
                Self::Xl => "radio-xl",
            },
            ThemeControllerAppearance::Button => match self {
                Self::Xs => "btn-xs",
                Self::Sm => "btn-sm",
                Self::Default => "",
                Self::Lg => "btn-lg",
                Self::Xl => "btn-xl",
            },
            ThemeControllerAppearance::None => "",
        }
    }
}

/// A control that puts a daisyUI theme on the document, styled with daisyUI's
/// `theme-controller` class.
///
/// This was the first component the registry rendered itself rather than
/// wrapping a primitive, and it has to be an `input`. daisyUI switches the theme
/// in CSS: every theme it emits is written
/// `:root:has(input.theme-controller[value=dark]:checked)` as well as
/// `[data-theme=dark]`, so the state the rule reads is a real form control's
/// `:checked`, which no primitive here renders. See ADR-0019.
///
/// There is therefore **no state to bridge, and nothing to lift**: the browser
/// owns the checked state, daisyUI reads it off the element it is on, and this
/// component neither keeps a copy nor emits a class for it. `onchange` reports
/// what the browser did, for a caller who wants to remember the choice; the
/// theme itself is applied whether anyone listens or not.
///
/// The theme is the input's `value`, and it has to name a theme the app enabled
/// in its `@plugin "daisyui"` block: daisyUI emits the rule above only for the
/// themes named there, and a name it never emitted leaves the document on the
/// theme it was already under.
///
/// The control renders no children, because an `input` has none. A caption is
/// the caller's, beside it or wrapped around it; under
/// [`ThemeControllerAppearance::Button`] daisyUI prints the `aria-label` as the
/// button's own text.
///
/// Classes passed by the caller concatenate with the controller's own; every
/// other attribute the caller passes overrides them. `type` and `value` are not
/// among them: this component renders the element, and those two are what make
/// it a theme controller at all, so they are props rather than attributes a
/// caller could contradict.
#[component]
pub fn ThemeController(
    /// The theme this control puts on the document, which is the input's
    /// `value` and the name daisyUI's rule matches on. Required rather than
    /// defaulted: a theme controller that names no theme is an ordinary
    /// checkbox.
    theme: ReadSignal<String>,
    /// daisyUI's appearance axis, which is also the input's `type`.
    #[props(default)]
    appearance: ThemeControllerAppearance,
    /// daisyUI's colour axis.
    #[props(default)]
    color: ThemeControllerColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: ThemeControllerSize,
    /// The name the control is grouped and submitted under. A set of themes to
    /// pick one of is one name shared by every controller in it, which is what
    /// makes the browser uncheck the last choice when the next is made.
    name: Option<String>,
    /// Whether the control starts checked, which is how the one matching the
    /// theme the document loaded under is marked. It seeds the input and
    /// nothing more; the browser owns the state from there, so this does not
    /// fight a reader who unchecks it.
    #[props(default)]
    default_checked: bool,
    /// Whether the control is disabled.
    disabled: Option<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    /// Called with the native `change` event after the browser toggles the
    /// control, for a caller who wants to remember the choice; the theme is
    /// applied whether anyone listens or not. Declared rather than left to the
    /// attribute list because `extends` reaches attributes only, never event
    /// handlers, so a handler has to be its own prop.
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let paint = appearance.class();
    let color = color.class(appearance);
    let size = size.class(appearance);

    // `theme-controller` after the paint, which is the order daisyUI's own
    // examples write the two in, and the order they read in: what the control
    // looks like, then what it does.
    let base = attributes!(input {
        class: "{paint} theme-controller {color} {size}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        input {
            type: appearance.input_type(),
            value: theme,
            name,
            disabled,
            // `initial_checked` rather than `checked`: it sets the input's
            // `defaultChecked`, which seeds the control and then leaves the
            // browser to own it. `checked` would be re-asserted on every render
            // that reached this element, putting the control back where the
            // caller started it and taking the reader's choice with it.
            initial_checked: default_checked,
            onchange: move |event| {
                if let Some(handler) = &onchange {
                    handler.call(event);
                }
            },
            ..merged,
        }
    }
}
