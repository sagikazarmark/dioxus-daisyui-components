use dioxus::prelude::*;
use dioxus_primitives::collapsible;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's marker axis for a collapsible, which is the sign it draws in the
/// corner of the title to say the panel can be opened.
///
/// The class strings are the accordion's, duplicated rather than depended on:
/// the registry uses no cross-component dependencies, and the Tailwind contract
/// already requires every one of these to be a literal in the file that emits
/// it.
///
/// [`CollapsibleMarker::Default`] emits no class, which is daisyUI's own
/// unmarked collapse: a title with nothing in its corner.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CollapsibleMarker {
    #[default]
    Default,
    /// A chevron that turns as the panel opens.
    Arrow,
    /// A plus that becomes a minus as the panel opens.
    Plus,
}

impl CollapsibleMarker {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Arrow, Self::Plus];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Arrow => "collapse-arrow",
            Self::Plus => "collapse-plus",
        }
    }
}

/// Whether [`Collapsible`] emits the utilities that give it a surface.
///
/// daisyUI's `collapse` lays a disclosure out and animates it but paints
/// nothing: the fill and the border in its own examples are Tailwind utilities
/// on the same element. They are emitted here instead, which inverts the usual
/// convention: [`CollapsibleAppearance::Default`] emits classes and
/// [`CollapsibleAppearance::None`] emits nothing. A utility this component
/// emits only ties with a caller's, and a tie is settled by
/// generated-stylesheet order rather than by the class attribute, so switching
/// ours off is the way to win it (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CollapsibleAppearance {
    #[default]
    Default,
    None,
}

impl CollapsibleAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "border border-base-300 bg-base-100",
            Self::None => "",
        }
    }
}

/// Whether [`CollapsibleTrigger`] emits the utilities that make a `button` read
/// as a collapse title.
///
/// daisyUI writes `.collapse-title` for the `div` its own CSS-only collapse
/// puts there, and a `button` arrives with a centred label and, through
/// daisyUI's own `.collapse > .collapse-title { cursor: unset }`, no pointer.
/// Both are put back here, on the inverted convention
/// [`CollapsibleAppearance`] records (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CollapsibleTriggerAppearance {
    #[default]
    Default,
    None,
}

impl CollapsibleTriggerAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "cursor-pointer text-start font-medium",
            Self::None => "",
        }
    }
}

/// One disclosure, carrying daisyUI's `collapse` classes.
///
/// This is where the open state is bridged, and it is **Tier 2**: daisyUI
/// reveals a panel through `.collapse-open` on this element (it sets the grid
/// row the panel lives in from `0fr` to `1fr` and lifts the
/// `content-visibility: hidden` off it), and it matches nothing the primitive
/// sets, neither the `data-open` it puts on all three elements nor the
/// `aria-expanded` on the trigger.
///
/// The state is **lifted** (ADR-0006), where the accordion item's is mirrored
/// (ADR-0011). The difference is the primitive's, not a change of heart: an
/// accordion item takes no controlled `open` prop, because the set owns which
/// items are open, while a collapsible answers to nobody and takes one. So this
/// component seeds a signal from `default_open`, always hands the primitive a
/// controlled value, and intercepts the change callback, which leaves a
/// controlled caller and an uncontrolled one both working, and this component
/// the only writer.
///
/// [`CollapsibleTrigger`] and [`CollapsibleContent`] must stay direct children
/// of this element: daisyUI lays the two out as the rows of one grid and
/// reveals the panel through `.collapse-open > .collapse-content`, so a wrapper
/// around either would leave the disclosure unstyled and the panel shut.
///
/// Classes passed by the caller concatenate with this element's own; every
/// other attribute the caller passes overrides them.
#[component]
pub fn Collapsible(
    /// daisyUI's marker axis.
    #[props(default)]
    marker: CollapsibleMarker,
    /// Whether to emit the utilities that give the disclosure a surface.
    #[props(default)]
    appearance: CollapsibleAppearance,
    /// The controlled open state. `Some` makes the collapsible controlled.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// Whether the panel starts open when the collapsible is not controlled.
    #[props(default)]
    default_open: bool,
    /// Called when the panel opens or closes.
    #[props(default)]
    on_open_change: Callback<bool>,
    /// Whether the trigger is inert and the panel unopenable.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the panel's children stay in the document while it is closed.
    /// The default repeats the primitive's own, since a prop declared here has
    /// to carry one.
    #[props(default)]
    keep_mounted: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut uncontrolled = use_signal(|| default_open);

    // Both are read here rather than inside the markup, and eagerly rather than
    // only when the other is absent, so that this component subscribes to
    // whichever of them is driving and re-renders, which is what puts the
    // modifier class below on the element and takes it off again.
    let is_open = open().unwrap_or(uncontrolled());

    let marker = marker.class();
    let appearance = appearance.class();
    // Tier 2, and the whole reason the state is lifted: daisyUI's own modifier
    // class, emitted from Rust as a complete literal so that Tailwind's scanner
    // sees it too.
    let state = if is_open { "collapse-open" } else { "" };

    let base = attributes!(div {
        class: "collapse {marker} {appearance} {state}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        collapsible::Collapsible {
            open: Some(is_open),
            on_open_change: move |open| {
                uncontrolled.set(open);
                on_open_change.call(open);
            },
            disabled,
            keep_mounted,
            attributes: merged,
            {children}
        }
    }
}

/// The control that opens and closes the panel, carrying daisyUI's
/// `collapse-title` class.
///
/// It is a `button` rather than the `div` daisyUI's CSS-only collapse puts
/// here, which is the whole point of wrapping a primitive: the button is
/// focusable, announces the state through `aria-expanded`, and points
/// `aria-controls` at the panel it opens. What that costs is cosmetic and is
/// put back by the appearance axis.
///
/// `as` is the primitive's, passed through: it hands the whole merged attribute
/// list (this component's class included) to a callback that renders the
/// element in this one's place. The part's children do not travel with it,
/// because the primitive's `as` path does not carry them; an element rendered
/// that way brings its own, and should be focusable by being the kind of
/// element that already is.
#[component]
pub fn CollapsibleTrigger(
    /// Whether to emit the utilities that make a `button` read as a title.
    #[props(default)]
    appearance: CollapsibleTriggerAppearance,
    /// Renders the trigger as an element of the caller's rather than as a
    /// button, with every attribute this component and the primitive would have
    /// put on it.
    #[props(default)]
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(button {
        class: "collapse-title {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        collapsible::CollapsibleTrigger { r#as, attributes: merged, {children} }
    }
}

/// The panel the trigger reveals, carrying daisyUI's `collapse-content` class.
///
/// The element itself is always in the document (it is a row of daisyUI's grid
/// and the transition runs on it) while what the caller wrote inside it is
/// mounted only while the panel is open, or always if `keep_mounted` is set on
/// [`Collapsible`]. daisyUI hides the row until `collapse-open` is on the root,
/// so the two agree on when the panel is visible.
#[component]
pub fn CollapsibleContent(
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then points the trigger's
    /// `aria-controls` at it; an id arriving as an attribute would replace the
    /// one the trigger is still naming.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "collapse-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        collapsible::CollapsibleContent { id, attributes: merged, {children} }
    }
}
