use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::separator;

/// daisyUI's colour axis for a separator, which is the colour of the rule
/// rather than of anything written along it.
///
/// [`SeparatorColor::Default`] emits no class, which is daisyUI's own rule: the
/// page's text colour mixed down to a tenth, so it reads as a line rather than
/// as a border.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum SeparatorColor {
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

impl SeparatorColor {
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
            Self::Neutral => "divider-neutral",
            Self::Primary => "divider-primary",
            Self::Secondary => "divider-secondary",
            Self::Accent => "divider-accent",
            Self::Info => "divider-info",
            Self::Success => "divider-success",
            Self::Warning => "divider-warning",
            Self::Error => "divider-error",
        }
    }
}

/// daisyUI's placement axis, which is where along the rule the separator's own
/// content sits.
///
/// daisyUI draws a separator as two rules with a gap between them, so placing
/// the content is a matter of dropping one of the two: `divider-start` hides
/// the rule before it and `divider-end` the rule after it.
///
/// [`SeparatorPlacement::Default`] emits no class and keeps both, which is
/// content in the middle. A separator with nothing in it renders the same under
/// every value (there is nothing to place) but the rule it drops is still
/// gone, so half the line disappears with it.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum SeparatorPlacement {
    #[default]
    Default,
    /// The inline start edge, which follows the writing direction: the left in
    /// a left-to-right document, the right in a right-to-left one.
    Start,
    /// The inline end edge, mirroring [`SeparatorPlacement::Start`].
    End,
}

impl SeparatorPlacement {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Start, Self::End];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Start => "divider-start",
            Self::End => "divider-end",
        }
    }
}

/// A separator styled with daisyUI's `divider` classes.
///
/// The orientation is bridged at **Tier 2**: the primitive reports it as
/// `aria-orientation` and `data-orientation`, daisyUI matches neither, and
/// expresses it with a class of its own, so the class is emitted from Rust.
///
/// **The two names are inverted, and this is the one trap in the component.**
/// ARIA names a separator after the line it draws, so a horizontal separator is
/// a horizontal rule. daisyUI names a divider after the layout it sits in, so
/// `divider-horizontal` is the one that runs a rule *down* a row of content.
/// This component keeps the primitive's `horizontal` prop with the primitive's
/// meaning and emits whichever daisyUI class draws that line, which is the
/// other one's name.
///
/// A class is emitted for both orientations rather than only for the one that
/// departs from daisyUI's default, which follows ADR-0008: what is on the
/// element then says the orientation outright rather than by omission. The two
/// classes are not equally load-bearing, though: `divider-horizontal` turns
/// the rule, where `divider-vertical` restates what an unclassed `divider`
/// already does.
///
/// Whatever the caller writes as children is laid out between the two halves of
/// the rule, which is where daisyUI puts a divider's text.
///
/// Classes passed by the caller concatenate with the separator's own; every
/// other attribute the caller passes overrides them.
#[component]
pub fn Separator(
    /// daisyUI's colour axis.
    #[props(default)]
    color: SeparatorColor,
    /// daisyUI's placement axis for this separator's own content.
    #[props(default)]
    placement: SeparatorPlacement,
    /// Whether the rule runs across the content or down it. The default repeats
    /// the primitive's own, since a prop declared here has to carry one.
    #[props(default = true)]
    horizontal: bool,
    /// Whether the separator is decorative, which keeps it out of the
    /// accessibility tree: a rule that divides nothing a screen reader has to
    /// be told about.
    #[props(default = false)]
    decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = color.class();
    let placement = placement.class();
    // Tier 2, and the inversion: `horizontal` is the primitive's word for the
    // line, and `divider-vertical` is daisyUI's word for the same line.
    let orientation = if horizontal {
        "divider-vertical"
    } else {
        "divider-horizontal"
    };

    let base = attributes!(div {
        class: "divider {orientation} {color} {placement}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        separator::Separator {
            horizontal,
            decorative,
            attributes: merged,
            {children}
        }
    }
}
