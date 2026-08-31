use dioxus::prelude::*;
use dioxus_primitives::accordion;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's marker axis for an accordion item, which is the sign it draws in
/// the corner of the title to say the item can be opened.
///
/// [`AccordionItemMarker::Default`] emits no class, which is daisyUI's own
/// unmarked collapse: a title with nothing in its corner.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum AccordionItemMarker {
    #[default]
    Default,
    /// A chevron that turns as the item opens.
    Arrow,
    /// A plus that becomes a minus as the item opens.
    Plus,
}

impl AccordionItemMarker {
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

/// Whether [`Accordion`] emits the utilities that stack its items.
///
/// daisyUI has no class for a set of collapses (its own examples are collapses
/// one after another inside whatever the page already had), so the utilities
/// that make a stack read as one are emitted here instead.
///
/// That inverts the usual convention: [`AccordionAppearance::Default`] emits
/// classes and [`AccordionAppearance::None`] emits nothing. A utility this
/// component emits only ties with a caller's, and a tie is settled by
/// generated-stylesheet order rather than by the class attribute, so switching
/// ours off is the way to win it (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum AccordionAppearance {
    #[default]
    Default,
    None,
}

impl AccordionAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "flex flex-col gap-2",
            Self::None => "",
        }
    }
}

/// Whether [`AccordionItem`] emits the utilities that give it a surface.
///
/// daisyUI's `collapse` lays an item out and animates it but paints nothing:
/// the fill and the border in its own examples are Tailwind utilities on the
/// same element. They are emitted here instead, on the same inverted convention
/// [`AccordionAppearance`] records (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum AccordionItemAppearance {
    #[default]
    Default,
    None,
}

impl AccordionItemAppearance {
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

/// Whether [`AccordionTrigger`] emits the utilities that make a `button` read
/// as a collapse title.
///
/// daisyUI writes `.collapse-title` for the `div` its own CSS-only collapse
/// puts there, and a `button` arrives with a centred label and, through
/// daisyUI's own `cursor: unset`, no pointer. Both are put back here, on the
/// inverted convention [`AccordionAppearance`] records (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum AccordionTriggerAppearance {
    #[default]
    Default,
    None,
}

impl AccordionTriggerAppearance {
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

/// A set of accordion items, one or more of which can be open.
///
/// The root is the primitive's: it owns which items are open, honours
/// `allow_multiple_open` and `collapsible` while doing it, and moves focus
/// between triggers with the arrow keys. daisyUI styles the items rather than
/// the set, so what is emitted here is layout and nothing else, behind an
/// appearance axis that switches it off.
///
/// Classes passed by the caller concatenate with the set's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn Accordion(
    /// Whether to emit the utilities that stack the items.
    #[props(default)]
    appearance: AccordionAppearance,
    /// Whether more than one item may be open at once.
    #[props(default)]
    allow_multiple_open: ReadSignal<bool>,
    /// Whether every item is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the last open item may be closed, leaving nothing open. The
    /// default repeats the primitive's own, since a prop declared here has to
    /// carry one.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    collapsible: ReadSignal<bool>,
    /// Whether the left and right arrow keys move between triggers instead of
    /// up and down. It does not lay the items out: daisyUI has no horizontal
    /// collapse, and this component's own layout is a column either way.
    #[props(default)]
    horizontal: ReadSignal<bool>,
    /// The id of this element.
    #[props(default)]
    id: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        accordion::Accordion {
            id,
            allow_multiple_open,
            disabled,
            collapsible,
            horizontal,
            attributes: merged,
            {children}
        }
    }
}

/// One item of an accordion, carrying daisyUI's `collapse` classes.
///
/// This is where the open state is bridged, and it is **Tier 2**: daisyUI
/// reveals a panel through `.collapse-open` on this element and matches nothing
/// the primitive sets: not the `data-open` attribute it puts here, and not the
/// `aria-expanded` on the trigger.
///
/// The state is **mirrored rather than lifted** (ADR-0011). The lift the dialog
/// and the dropdown perform is not available here: the primitive's item takes
/// no controlled `open` prop at all, because the accordion root owns which
/// items are open and enforces `allow_multiple_open` and `collapsible` while
/// doing it. So this component seeds a signal from `default_open` and updates it
/// from the change callback the primitive already fires on every change; it
/// reads the state rather than owning it, and the primitive stays the only
/// writer.
///
/// [`AccordionTrigger`] and [`AccordionContent`] must stay direct children of
/// this element: daisyUI lays the two out as the rows of a grid and reveals the
/// panel through `.collapse-open > .collapse-content`, so a wrapper around
/// either would leave the item unstyled and the panel shut.
///
/// Classes passed by the caller concatenate with the item's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn AccordionItem(
    /// daisyUI's marker axis.
    #[props(default)]
    marker: AccordionItemMarker,
    /// Whether to emit the utilities that give the item a surface.
    #[props(default)]
    appearance: AccordionItemAppearance,
    /// Where this item falls in the keyboard navigation order.
    index: usize,
    /// Whether this item is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether this item starts open.
    #[props(default)]
    default_open: bool,
    /// Called when this item opens or closes.
    #[props(default)]
    on_change: Callback<bool>,
    /// Called when this item's trigger is clicked, whichever way that leaves
    /// the item.
    #[props(default)]
    on_trigger_click: Callback,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    // Seeded from the same prop the primitive is seeded from, so that the class
    // is right on the first render rather than one change callback later.
    let mut open = use_signal(|| default_open);

    let marker = marker.class();
    let appearance = appearance.class();
    // Tier 2, and the whole reason the state is mirrored: daisyUI's own
    // modifier class, emitted from Rust as a complete literal so that
    // Tailwind's scanner sees it too.
    let state = if open() { "collapse-open" } else { "" };

    let base = attributes!(div {
        class: "collapse {marker} {appearance} {state}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        accordion::AccordionItem {
            index,
            disabled,
            default_open,
            on_change: move |next| {
                open.set(next);
                on_change.call(next);
            },
            on_trigger_click,
            attributes: merged,
            {children}
        }
    }
}

/// The control that opens and closes an item, carrying daisyUI's
/// `collapse-title` class.
///
/// It is a `button` rather than the `div` daisyUI's CSS-only collapse puts
/// here, which is the whole point of wrapping a primitive: the button is
/// focusable, announces the item's state through `aria-expanded`, and points
/// `aria-controls` at the panel it opens. What that costs is cosmetic and is
/// put back by the appearance axis.
#[component]
pub fn AccordionTrigger(
    /// Whether to emit the utilities that make a `button` read as a title.
    #[props(default)]
    appearance: AccordionTriggerAppearance,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive takes one as a prop of its own and puts it on the
    /// element; an id arriving as an attribute would meet the one already
    /// there.
    #[props(default)]
    id: Option<String>,
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
        accordion::AccordionTrigger { id, attributes: merged, {children} }
    }
}

/// The panel an item reveals, carrying daisyUI's `collapse-content` class.
///
/// The primitive mounts this element when the item opens and unmounts it once
/// the exit animation has run, and daisyUI keeps it hidden until
/// `collapse-open` is on the item, so the two agree on when it is there and on
/// when it is visible, and the class coming off while the element is still in
/// the document is what lets the closing animation play at all.
#[component]
pub fn AccordionContent(
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
        accordion::AccordionContent { id, attributes: merged, {children} }
    }
}
