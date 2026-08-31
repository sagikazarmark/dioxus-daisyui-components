use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use dioxus_primitives::dialog;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// The primitive's own context, re-exported so that a caller needs one import.
///
/// It is how a control inside the dialog closes it: an uncontrolled dialog
/// has no other handle on its state, and the call travels back out through
/// [`DialogRoot`]'s own change callback, so the lifted state stays in step
/// either way.
pub use dioxus_primitives::dialog::DialogCtx;

/// daisyUI's placement axis for a dialog, which places the box within the
/// modal.
///
/// [`DialogPlacement::Default`] emits no class. It is close to
/// [`DialogPlacement::Middle`] (both centre the box) but not a synonym for
/// it: daisyUI's explicit middle also caps the box's height at the viewport
/// less `5em`, where an unclassed modal lets it grow to the full viewport.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DialogPlacement {
    #[default]
    Default,
    Top,
    Middle,
    Bottom,
    /// The inline start edge, which follows the writing direction: the left in
    /// a left-to-right document, the right in a right-to-left one.
    Start,
    /// The inline end edge, mirroring [`DialogPlacement::Start`].
    End,
}

impl DialogPlacement {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[
        Self::Default,
        Self::Top,
        Self::Middle,
        Self::Bottom,
        Self::Start,
        Self::End,
    ];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Top => "modal-top",
            Self::Middle => "modal-middle",
            Self::Bottom => "modal-bottom",
            Self::Start => "modal-start",
            Self::End => "modal-end",
        }
    }
}

/// Whether [`DialogTitle`] emits the utilities that make it look like a title.
///
/// daisyUI has no class for a modal's title, so the ones it uses in its own
/// examples are emitted here instead. That inverts the usual convention:
/// [`DialogTitleAppearance::Default`] emits classes and
/// [`DialogTitleAppearance::None`] emits nothing, because a utility this
/// component emits only ties with a caller's, and a tie is settled by
/// generated-stylesheet order rather than by the class attribute. Switching
/// ours off is the way to win it (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DialogTitleAppearance {
    #[default]
    Default,
    None,
}

impl DialogTitleAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "text-lg font-bold",
            Self::None => "",
        }
    }
}

/// Whether [`DialogDescription`] emits the utilities that space it from the
/// title.
///
/// The inverse convention of the other axes, for the reason
/// [`DialogTitleAppearance`] records.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DialogDescriptionAppearance {
    #[default]
    Default,
    None,
}

impl DialogDescriptionAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "py-4",
            Self::None => "",
        }
    }
}

/// The outer element of a dialog, carrying daisyUI's `modal` classes.
///
/// This is where the open state is **lifted** (ADR-0006). daisyUI hides
/// `.modal` outright unless `modal-open` is on it, and matches no attribute
/// the primitive sets, so the class has to be emitted from Rust, and this
/// component has to know the state to emit it. It seeds a signal from
/// `default_open`, always hands the primitive a controlled value, and
/// intercepts the change callback, which leaves a controlled caller and an
/// uncontrolled one both working and this component the only writer.
///
/// The dim behind the box is drawn by this element rather than by a backdrop
/// of its own, which is how daisyUI draws it.
///
/// Classes passed by the caller concatenate with this element's own; every
/// other attribute the caller passes overrides them.
#[component]
pub fn DialogRoot(
    /// daisyUI's placement axis.
    #[props(default)]
    placement: DialogPlacement,
    /// The id of this element. Declared rather than left to the attribute
    /// list, because the primitive generates one and then looks the element up
    /// by it; an id that arrived as an attribute would be written over the
    /// one it is looking for.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    /// Whether the dialog traps focus while it is open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    is_modal: ReadSignal<bool>,
    /// The controlled open state of the dialog.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// The state the dialog starts in when it is not controlled.
    #[props(default)]
    default_open: bool,
    /// Called when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut uncontrolled = use_signal(|| default_open);

    // Both are read here rather than inside the markup, and eagerly rather
    // than only when the other is absent, so that this component subscribes to
    // whichever of them is driving and re-renders, which is what puts the
    // modifier class below on the element and takes it off again.
    let is_open = open().unwrap_or(uncontrolled());

    let placement = placement.class();
    // Tier 2, and the whole reason the state is lifted: daisyUI's own modifier
    // class, emitted from Rust as a complete literal so that Tailwind's
    // scanner sees it too.
    let state = if is_open { "modal-open" } else { "" };

    let base = attributes!(div {
        class: "modal {placement} {state}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        dialog::DialogRoot {
            id,
            is_modal,
            open: Some(is_open),
            on_open_change: move |open| {
                uncontrolled.set(open);
                on_open_change.call(open);
            },
            attributes: merged,
            {children}
        }
    }
}

/// The box a dialog's content sits in, carrying daisyUI's `modal-box` class.
///
/// This is the element that traps focus while the dialog is open, listens for
/// Escape, and dismisses on a click outside itself, all of it the primitive's,
/// none of it reimplemented here.
///
/// It must stay a direct child of [`DialogRoot`]: daisyUI's open rule reaches
/// the box through a child combinator, so a wrapper between them would leave
/// the box invisible.
#[component]
pub fn DialogContent(
    /// The id of this element, declared for the reason [`DialogRoot`]'s is.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: "modal-box" });
    let mut merged = merge_attributes(vec![base, attributes]);

    // The merged class travels through the primitive's own `class` prop, which
    // is the one part that takes one. Left in the attribute list it would
    // arrive at an element that already has a class attribute on it, and which
    // of the two lands is a question about the renderer rather than about this
    // component.
    let class = take_class(&mut merged);

    rsx! {
        dialog::DialogContent { id, class, attributes: merged, {children} }
    }
}

/// A dialog's title, which is what the dialog is announced by.
///
/// The primitive points the dialog's `aria-labelledby` at this element, so a
/// dialog with no title is an unnamed one.
#[component]
pub fn DialogTitle(
    /// Whether to emit the utilities that make this look like a title.
    #[props(default)]
    appearance: DialogTitleAppearance,
    /// The id of this element. Declared rather than left to the attribute
    /// list, because the primitive names the dialog by this element's id;
    /// one that arrived as an attribute would replace the id `aria-labelledby`
    /// still points at.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(h2 {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        dialog::DialogTitle { id, attributes: merged, {children} }
    }
}

/// A dialog's description, which is what the dialog is announced by after its
/// title.
///
/// The primitive points the dialog's `aria-describedby` at this element.
#[component]
pub fn DialogDescription(
    /// Whether to emit the utilities that space this from the title.
    #[props(default)]
    appearance: DialogDescriptionAppearance,
    /// The id of this element, declared for the reason [`DialogTitle`]'s is.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(p {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        dialog::DialogDescription { id, attributes: merged, {children} }
    }
}

/// The row a dialog's buttons sit in, carrying daisyUI's `modal-action` class.
///
/// There is no primitive behind this one and none is needed: the class lays
/// the row out and nothing about it is behavioural.
#[component]
pub fn DialogActions(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "modal-action",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

/// A dialog, as one component rather than as [`DialogRoot`] wrapped around
/// [`DialogContent`].
///
/// The collapse is legal because daisyUI's modal has nothing between those two
/// elements and no caller content can go there: its markup is the modal, the
/// box, and whatever the caller puts in the box.
///
/// **Caller attributes land on the box**, which is the element worth reaching:
/// a class here sizes or repaints the box. The outer element is reached
/// through `placement`, or by dropping to the parts, which is what makes
/// offering both worthwhile.
#[component]
pub fn Dialog(
    /// daisyUI's placement axis, which styles the outer element.
    #[props(default)]
    placement: DialogPlacement,
    /// The id of the box.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    /// Whether the dialog traps focus while it is open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    is_modal: ReadSignal<bool>,
    /// The controlled open state of the dialog.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// The state the dialog starts in when it is not controlled.
    #[props(default)]
    default_open: bool,
    /// Called when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        DialogRoot {
            placement,
            is_modal,
            open,
            default_open,
            on_open_change,
            DialogContent { id, attributes, {children} }
        }
    }
}

/// Takes the class out of a merged attribute list, so that it can be passed to
/// a primitive that takes one as a prop of its own.
///
/// `merge_attributes` has already concatenated the caller's class with this
/// component's by the time this runs, so there is exactly one to take, as
/// long as it is text, which is the only kind of class `rsx!` produces and the
/// only kind that could have been concatenated in the first place. Anything
/// else is left where it is, to travel on as an attribute.
fn take_class(attributes: &mut Vec<Attribute>) -> String {
    let class = attributes.iter().position(|attribute| {
        attribute.name == "class" && matches!(attribute.value, AttributeValue::Text(_))
    });

    match class.map(|index| attributes.remove(index).value) {
        Some(AttributeValue::Text(class)) => class,
        _ => String::new(),
    }
}
