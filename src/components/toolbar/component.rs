use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::toolbar;

/// daisyUI's colour axis for a toolbar button, which is the button's own.
///
/// The class strings are the button component's, duplicated rather than
/// depended on: the registry uses no cross-component dependencies, and the
/// Tailwind contract already requires every one of these to be a literal in the
/// file that emits it.
///
/// [`ToolbarButtonColor::Default`] emits no class at all, which is daisyUI's
/// uncoloured button rather than a synonym for
/// [`ToolbarButtonColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ToolbarButtonColor {
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

impl ToolbarButtonColor {
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

/// daisyUI's size axis for a toolbar button, which is the button's own and is
/// duplicated for the reason [`ToolbarButtonColor`] records.
///
/// [`ToolbarButtonSize::Default`] emits no class, which renders at the same
/// size as daisyUI's explicit `btn-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ToolbarButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl ToolbarButtonSize {
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

/// Whether [`Toolbar`] emits the utilities that lay the row out.
///
/// daisyUI has no toolbar. It has `join`, which fuses a row of controls into
/// one shape and is what the toggle group is; a toolbar is the other thing:
/// separate controls, in groups, with rules between the groups, so the row
/// itself is Tailwind utilities and every part inside it is a daisyUI class.
///
/// That inverts the usual convention: [`ToolbarAppearance::Default`] emits
/// classes and [`ToolbarAppearance::None`] emits nothing. A utility this
/// component emits only ties with a caller's, and a tie is settled by
/// generated-stylesheet order rather than by the class attribute, so switching
/// ours off is the way to win it (ADR-0004).
///
/// The direction is not part of this axis: it follows the primitive's
/// `horizontal` prop, which is also what the arrow keys go by, and is emitted
/// whichever way the axis is set.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ToolbarAppearance {
    #[default]
    Default,
    None,
}

impl ToolbarAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "inline-flex flex-wrap items-center gap-1",
            Self::None => "",
        }
    }
}

/// daisyUI's colour axis for a toolbar separator, which is the separator
/// component's own: the rule's colour rather than that of anything written
/// along it.
///
/// The class strings are duplicated for the reason [`ToolbarButtonColor`]
/// records.
///
/// [`ToolbarSeparatorColor::Default`] emits no class, which is daisyUI's own
/// rule: the page's text colour mixed down to a tenth, so it reads as a line
/// rather than as a border.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ToolbarSeparatorColor {
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

impl ToolbarSeparatorColor {
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

/// Which way the toolbar runs, as [`Toolbar`] holds it for its separators to
/// read.
///
/// The primitive's own context is private, and a separator that is not told an
/// orientation takes the opposite of the toolbar's, so the class this component
/// emits for it needs the same fact the primitive is using, from the only place
/// it can be had.
#[derive(Copy, Clone)]
struct ToolbarDirection {
    horizontal: ReadSignal<bool>,
}

/// A row of grouped controls, laid out with Tailwind utilities.
///
/// daisyUI has no class for this element, so what it carries is layout and
/// nothing else, all of it Defeatable (ADR-0004). What the primitive puts here
/// instead is the behaviour: the arrow keys move between the buttons, Home
/// returns to the first, and the toolbar role tells a screen reader that the
/// controls belong together. What it does *not* do (a roving tab stop, an End
/// key, skipping past a disabled control) is recorded in the component's
/// documentation rather than added here, since keyboard behaviour is upstream's
/// and this registry's business is the class mapping.
///
/// The direction class is emitted for both values rather than only for the one
/// that departs from Tailwind's default, following ADR-0008. Here it does more
/// than name what is already true: the primitive's `horizontal` prop is which
/// arrow keys move focus, and the class is which way the row runs, so emitting
/// both keeps the two from disagreeing.
///
/// Classes passed by the caller concatenate with the toolbar's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn Toolbar(
    /// Whether to emit the utilities that lay the row out.
    #[props(default)]
    appearance: ToolbarAppearance,
    /// Whether every control in the toolbar is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the toolbar runs across rather than down, which is both how it
    /// is laid out and which arrow keys move through it. The default repeats
    /// the primitive's own.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    horizontal: ReadSignal<bool>,
    /// What the toolbar is announced as, which a toolbar of icons needs and one
    /// of words rarely does.
    #[props(default)]
    aria_label: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    use_context_provider(|| ToolbarDirection { horizontal });

    let appearance = appearance.class();
    // Emitted from Rust as a complete literal, from the same prop the primitive
    // is given, so that what the row looks like and what the keyboard does are
    // one decision rather than two.
    let direction = if horizontal() {
        "flex-row"
    } else {
        "flex-col items-stretch"
    };

    let base = attributes!(div {
        class: "{appearance} {direction}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        toolbar::Toolbar {
            disabled,
            horizontal,
            aria_label,
            attributes: merged,
            {children}
        }
    }
}

/// One control of a toolbar, carrying daisyUI's `btn` class.
///
/// Nothing is bridged here. The primitive sets the native `disabled` attribute
/// on the `button`, which is what daisyUI's `.btn:disabled` matches, and the
/// keyboard highlight is the browser's focus ring on the element the primitive
/// moves real focus to; `.btn` styles `:focus-visible` itself.
///
/// Everything the caller passes lands on the button, which is both the element
/// daisyUI styles and the one the primitive gives the behaviour to.
#[component]
pub fn ToolbarButton(
    /// daisyUI's colour axis.
    #[props(default)]
    color: ToolbarButtonColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: ToolbarButtonSize,
    /// Where this control falls in the toolbar's order, which is the order the
    /// arrow keys move through. It is explicit rather than taken from the DOM,
    /// because the primitive matches a key press to a control by index rather
    /// than by position.
    index: ReadSignal<usize>,
    /// Whether this control is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Called when the control is activated.
    #[props(default)]
    on_click: Callback<()>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = color.class();
    let size = size.class();

    let base = attributes!(button {
        class: "btn {color} {size}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        toolbar::ToolbarButton {
            index,
            disabled,
            on_click,
            attributes: merged,
            {children}
        }
    }
}

/// A rule between two groups of a toolbar, carrying daisyUI's `divider`
/// classes.
///
/// The orientation is bridged at **Tier 2**, as it is in the separator
/// component and with the same inversion: ARIA names a separator after the line
/// it draws, so a separator across a column is `horizontal`, while daisyUI names
/// a divider after the layout it sits in, so the class that draws a rule *down*
/// a row is `divider-horizontal`.
///
/// What is different here is where the orientation comes from. The primitive
/// already inverts the toolbar's own when a separator is not told one (a rule
/// in a row runs down it) so this component follows what the primitive decided
/// rather than deciding it a second time, and a caller who overrides it
/// overrides both at once.
#[component]
pub fn ToolbarSeparator(
    /// daisyUI's colour axis.
    #[props(default)]
    color: ToolbarSeparatorColor,
    /// Whether the rule runs across the toolbar's content or down it. Left
    /// unset, it is the opposite of the toolbar's own direction, which is the
    /// primitive's default and the only one a rule between groups wants.
    #[props(default)]
    horizontal: Option<bool>,
    /// Whether the rule is decorative, which keeps it out of the accessibility
    /// tree.
    #[props(default = false)]
    decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<ToolbarDirection>();
    let color = color.class();
    // The same inversion the separator component documents, over an orientation
    // the primitive works out rather than one this component is told: unset, a
    // separator is the opposite of the toolbar it is in.
    let orientation = if horizontal.unwrap_or(!(ctx.horizontal)()) {
        "divider-vertical"
    } else {
        "divider-horizontal"
    };

    let base = attributes!(div {
        class: "divider {orientation} {color}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        toolbar::ToolbarSeparator { horizontal, decorative, attributes: merged }
    }
}
