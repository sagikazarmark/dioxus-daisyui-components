use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::popover;
use dioxus_primitives::{ContentAlign, ContentSide};

/// daisyUI's side axis for a popover, which is the side of the trigger the
/// panel opens on.
///
/// Every value emits a class, including the default one: the inverse of the
/// usual convention, and for the reason ADR-0008 records for the dropdown menu:
/// an unclassed `.dropdown-content` lands wherever it would have fallen in flow,
/// which stops being under the trigger as soon as a caller writes anything else
/// inside [`Popover`].
///
/// The value is also what the primitive is told, so that the `data-side` it
/// reports and the side the panel is actually on are the same thing.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum PopoverSide {
    /// Above the trigger.
    Top,
    /// Under the trigger, which is where daisyUI puts an unplaced dropdown and
    /// where a popover usually goes.
    #[default]
    Bottom,
    /// To the left of the trigger, in either writing direction; daisyUI's
    /// horizontal placements are physical rather than logical.
    Left,
    /// To the right of the trigger, mirroring [`PopoverSide::Left`].
    Right,
}

impl PopoverSide {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Top, Self::Bottom, Self::Left, Self::Right];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Top => "dropdown-top",
            Self::Bottom => "dropdown-bottom",
            Self::Left => "dropdown-left",
            Self::Right => "dropdown-right",
        }
    }

    /// What the primitive is told, so that the `data-side` it reports says the
    /// same thing the class above did.
    const fn side(self) -> ContentSide {
        match self {
            Self::Top => ContentSide::Top,
            Self::Bottom => ContentSide::Bottom,
            Self::Left => ContentSide::Left,
            Self::Right => ContentSide::Right,
        }
    }
}

/// daisyUI's align axis for a popover, which is where the panel sits along
/// the side [`PopoverSide`] opened it on.
///
/// The two compose the way daisyUI's own classes do: an alignment on a vertical
/// placement moves the panel across the trigger, and on a horizontal one it
/// moves the panel up and down it.
///
/// Every value emits a class here too, for the reason [`PopoverSide`]
/// records, and every value is handed to the primitive as well.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum PopoverAlign {
    /// The inline start edge, which follows the writing direction: the left in
    /// a left-to-right document, the right in a right-to-left one. This is
    /// where daisyUI puts an unaligned dropdown.
    #[default]
    Start,
    /// Centred on the trigger.
    Center,
    /// The inline end edge, mirroring [`PopoverAlign::Start`].
    End,
}

impl PopoverAlign {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Start, Self::Center, Self::End];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Start => "dropdown-start",
            Self::Center => "dropdown-center",
            Self::End => "dropdown-end",
        }
    }

    /// What the primitive is told, so that the `data-align` it reports says the
    /// same thing the class above did.
    const fn align(self) -> ContentAlign {
        match self {
            Self::Start => ContentAlign::Start,
            Self::Center => ContentAlign::Center,
            Self::End => ContentAlign::End,
        }
    }
}

/// Whether [`PopoverContent`] emits the utilities that draw the box the panel's
/// content sits in.
///
/// daisyUI's `dropdown-content` only positions the element: the fill, the
/// corners, the padding and the shadow in its own examples are Tailwind
/// utilities on the same element. They are emitted here instead, which inverts
/// the usual convention: [`PopoverContentAppearance::Default`] emits classes and
/// [`PopoverContentAppearance::None`] emits nothing. A utility this component
/// emits only ties with a caller's, and a tie is settled by generated-stylesheet
/// order rather than by the class attribute, so switching ours off is the way to
/// win it (ADR-0004).
///
/// The padding is kept here where the dropdown menu drops it. There it would pad
/// in turn with the `menu` inside; a popover holds whatever the caller wrote,
/// and nothing in it pads on daisyUI's behalf.
///
/// daisyUI's `z-1` and `w-52` are left out for the reasons the dropdown menu
/// records: the first is beneath the `z-index` `.dropdown-content` already sets,
/// and the second is a width for the menu that example was written around rather
/// than for every panel.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum PopoverContentAppearance {
    #[default]
    Default,
    None,
}

impl PopoverContentAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "bg-base-100 rounded-box p-4 shadow-sm",
            Self::None => "",
        }
    }
}

/// The outer element of a popover, carrying daisyUI's `dropdown` classes.
///
/// This is where the open state is **lifted** (ADR-0006), and here the lift is
/// mandatory rather than cosmetic: daisyUI hides `.dropdown-content` outright
/// unless `dropdown-open` is on this element, and matches no attribute the
/// primitive sets. So the class has to be emitted from Rust, and this component
/// has to know the state to emit it. It seeds a signal from `default_open`,
/// always hands the primitive a controlled value, and intercepts the change
/// callback, which leaves a controlled caller and an uncontrolled one both
/// working and this component the only writer.
///
/// A daisyUI dropdown is available here where it is not to the hover card
/// (ADR-0017): the rule that takes the pointer events off an open dropdown's
/// first child matches `[tabindex]` elements, and the primitive's trigger is a
/// plain `button`, and even where it matched, closing on the second click is
/// what a popover wants and what the rule was written to do.
///
/// Classes passed by the caller concatenate with this element's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn Popover(
    /// daisyUI's side axis.
    #[props(default)]
    side: PopoverSide,
    /// daisyUI's align axis.
    #[props(default)]
    align: PopoverAlign,
    /// The controlled open state of the popover.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// Whether the popover starts open when it is not controlled.
    #[props(default)]
    default_open: bool,
    /// Called when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    /// Whether focus is trapped inside the panel while it is open. The default
    /// repeats the primitive's own, which is a modal popover.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    is_modal: ReadSignal<bool>,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive dismisses the popover on a click outside *this*
    /// element and finds it by the id it generated; an id that arrived as an
    /// attribute would be written over the one it is looking for.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut uncontrolled = use_signal(|| default_open);

    // Both are read here rather than inside the markup, and eagerly rather than
    // only when the other is absent, so that this component subscribes to
    // whichever of them is driving and re-renders, which is what puts the
    // modifier class below on the element and takes it off again.
    let is_open = open().unwrap_or(uncontrolled());

    // The parts need the placement the root was given: daisyUI's classes live
    // out here, and what the primitive is told lives on the content.
    use_context_provider(|| PopoverPosition { side, align });

    let side = side.class();
    let align = align.class();
    // Tier 2, and the whole reason the state is lifted: daisyUI's own modifier
    // class, emitted from Rust as a complete literal so that Tailwind's scanner
    // sees it too.
    let state = if is_open { "dropdown-open" } else { "" };

    let base = attributes!(div {
        class: "dropdown {side} {align} {state}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        popover::PopoverRoot {
            id,
            is_modal,
            open: Some(is_open),
            default_open,
            on_open_change: move |open| {
                uncontrolled.set(open);
                on_open_change.call(open);
            },
            attributes: merged,
            {children}
        }
    }
}

/// The control a popover opens from, carrying daisyUI's `btn` class.
///
/// The button's own axes are not repeated here (a colour or a size reaches this
/// element through `class`, which concatenates) and the primitive offers no way
/// to render the trigger as anything else, so the element is always a `button`.
/// That is what keeps daisyUI's dropdown usable here (ADR-0017): a `button` with
/// no `tabindex` attribute is not what daisyUI takes the pointer events off.
///
/// This part takes no `id` prop. The primitive puts an id of its own on the
/// element and points the panel's `aria-labelledby` at it, so a trigger a caller
/// needs to address is one they reach through [`Popover`]'s id instead.
#[component]
pub fn PopoverTrigger(
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button { class: "btn" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        popover::PopoverTrigger { attributes: merged, {children} }
    }
}

/// The panel a popover opens, carrying daisyUI's `dropdown-content` class.
///
/// Nothing is rendered inside it: a popover holds whatever the caller wrote,
/// which is what makes this the one component that borrows daisyUI's dropdown
/// without the split ADR-0005 describes. There is no `menu` to put on a list and
/// no items to wrap, so the box and the positioning stay on the primitive's own
/// element.
///
/// The side and the alignment are handed to the primitive from the root's axes,
/// so the `data-side` and `data-align` it reports say what daisyUI's classes
/// did. A caller who switches this component's classes off keeps those
/// attributes and can position from them.
///
/// Classes passed by the caller concatenate with the panel's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn PopoverContent(
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: PopoverContentAppearance,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then looks the element up by it
    /// to trap focus inside it.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let position = use_context::<PopoverPosition>();
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "dropdown-content {appearance}",
    });
    let mut merged = merge_attributes(vec![base, attributes]);

    // The merged class travels through the primitive's own `class` prop, which
    // is the one part that takes one. Left in the attribute list it would arrive
    // at an element that already has a class attribute on it, and which of the
    // two lands is a question about the renderer rather than about this
    // component.
    let class = take_class(&mut merged);

    rsx! {
        popover::PopoverContent {
            id,
            class,
            side: position.side.side(),
            align: position.align.align(),
            attributes: merged,
            {children}
        }
    }
}

/// Where the panel goes, as [`Popover`] holds it for [`PopoverContent`] to read.
///
/// The axes are the root's, because daisyUI's placement classes belong on the
/// element that carries `dropdown`. The primitive wants the same two facts on
/// the content, where it turns them into `data-side` and `data-align`, and its
/// own context is private, so they travel through a context of this component's
/// rather than being asked for twice.
#[derive(Copy, Clone)]
struct PopoverPosition {
    side: PopoverSide,
    align: PopoverAlign,
}

/// Takes the class out of a merged attribute list, so that it can be passed to a
/// primitive that takes one as a prop of its own.
///
/// `merge_attributes` has already concatenated the caller's class with this
/// component's by the time this runs, so there is exactly one to take, as long
/// as it is text, which is the only kind of class `rsx!` produces and the only
/// kind that could have been concatenated in the first place. Anything else is
/// left where it is, to travel on as an attribute.
fn take_class(attributes: &mut Vec<Attribute>) -> String {
    let class = attributes.iter().position(|attribute| {
        attribute.name == "class" && matches!(attribute.value, AttributeValue::Text(_))
    });

    match class.map(|index| attributes.remove(index).value) {
        Some(AttributeValue::Text(class)) => class,
        _ => String::new(),
    }
}
