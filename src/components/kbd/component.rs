use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's size Axis for a keyboard key.
///
/// [`KbdSize::Default`] emits no class and renders at the same medium size as
/// daisyUI's explicit `kbd-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum KbdSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl KbdSize {
    /// Every value of this Axis, from smallest to largest.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "kbd-xs",
            Self::Sm => "kbd-sm",
            Self::Default => "",
            Self::Lg => "kbd-lg",
            Self::Xl => "kbd-xl",
        }
    }
}

/// Native `kbd` markup styled with daisyUI's keyboard-key classes.
///
/// The element denotes user input but is not an interactive control and adds
/// no focus, keyboard or ARIA behaviour. Classes passed by the caller
/// concatenate with the key's own; every other caller attribute overrides the
/// key's.
#[component]
pub fn Kbd(
    /// daisyUI's size axis.
    #[props(default)]
    size: KbdSize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let size = size.class();

    let base = attributes!(kbd {
        class: "kbd {size}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        kbd { ..merged, {children} }
    }
}
