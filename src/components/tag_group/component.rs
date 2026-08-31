use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::tag_group;

/// daisyUI's colour axis for a tag.
///
/// [`TagColor::Default`] emits no class, which is daisyUI's uncoloured badge: a
/// tag that reads as a tag without claiming to mean anything.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum TagColor {
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

impl TagColor {
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
            Self::Neutral => "badge-neutral",
            Self::Primary => "badge-primary",
            Self::Secondary => "badge-secondary",
            Self::Accent => "badge-accent",
            Self::Info => "badge-info",
            Self::Success => "badge-success",
            Self::Warning => "badge-warning",
            Self::Error => "badge-error",
        }
    }
}

/// daisyUI's size axis for a tag.
///
/// [`TagSize::Default`] emits no class, which renders at the same size as
/// daisyUI's explicit `badge-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum TagSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl TagSize {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "badge-xs",
            Self::Sm => "badge-sm",
            Self::Default => "",
            Self::Lg => "badge-lg",
            Self::Xl => "badge-xl",
        }
    }
}

/// Whether the group emits the utilities that stack its label above its tags.
///
/// daisyUI has no class for a set of badges (its own examples are badges one
/// after another inside whatever the page already had) so the utilities that
/// make a group read as one are emitted here.
///
/// That inverts the usual convention: [`TagGroupAppearance::Default`] emits
/// classes and [`TagGroupAppearance::None`] emits nothing. A utility this
/// component emits only ties with a caller's, and a tie is settled by
/// generated-stylesheet order rather than by the class attribute, so switching
/// ours off is the way to win it (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum TagGroupAppearance {
    #[default]
    Default,
    None,
}

impl TagGroupAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "flex flex-col items-start gap-2",
            Self::None => "",
        }
    }
}

/// Whether [`TagList`] emits the utilities that lay the tags out.
///
/// The primitive's list is a `role="grid"` of one-cell rows, which has no layout
/// of its own and no daisyUI class either. What is emitted here is the row of
/// wrapping tags daisyUI's own badge examples are written as, on the inverted
/// convention [`TagGroupAppearance`] records (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum TagListAppearance {
    #[default]
    Default,
    None,
}

impl TagListAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "flex flex-wrap items-center gap-2",
            Self::None => "",
        }
    }
}

/// Whether [`TagOption`] emits the utilities that mark it selected and focused.
///
/// daisyUI's badge has colours, sizes and styles, and **no selected state at
/// all**, nor a focus ring, since a badge is not something you focus. Both are
/// emitted here, and both are written as variants of the attributes the
/// primitive already sets, so the state stays the primitive's and no class has
/// to be recomputed to follow it.
///
/// The ring is painted in `--badge-color`, which is the property daisyUI's own
/// colour classes set, so a selected tag is ringed in its own colour rather than
/// in one this registry chose. An uncoloured tag falls back to `currentColor`,
/// which is what daisyUI paints its border with in the same case.
///
/// On the usual inverted convention: [`TagOptionAppearance::Default`] emits
/// classes and [`TagOptionAppearance::None`] emits nothing (ADR-0004), which is
/// what a caller who would rather show selection as a colour switches to.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum TagOptionAppearance {
    #[default]
    Default,
    None,
}

impl TagOptionAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "cursor-pointer focus-visible:outline-2 focus-visible:outline-offset-2 data-[selected=true]:ring-2 data-[selected=true]:ring-offset-2 data-[selected=true]:ring-[color:var(--badge-color,currentColor)] data-[selected=true]:ring-offset-base-100"
            }
            Self::None => "",
        }
    }
}

/// Whether [`TagGroupEmpty`] emits the utilities that mute what it says.
///
/// daisyUI has no empty state for anything, so what an emptied list says is
/// written the way daisyUI writes secondary text in its own examples, and is
/// switched off the same way as every other utility here (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum TagGroupEmptyAppearance {
    #[default]
    Default,
    None,
}

impl TagGroupEmptyAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "text-sm opacity-60",
            Self::None => "",
        }
    }
}

/// A group of tags, one of which can be selected.
///
/// The selection is the primitive's throughout (it owns the value, enforces
/// `allow_empty_selection`, clears on Escape and moves between tags with the
/// arrow keys) and nothing about it is lifted here, because daisyUI has no
/// class that says a badge is selected. What marks one is a utility on the tag
/// itself; see [`TagOptionAppearance`].
///
/// Classes passed by the caller concatenate with the group's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn TagGroup<T: Clone + PartialEq + 'static>(
    /// Whether to emit the utilities that stack the label above the tags.
    #[props(default)]
    appearance: TagGroupAppearance,
    /// The controlled selected value. `Some` makes the group controlled.
    #[props(default)]
    value: Option<ReadSignal<Option<T>>>,
    /// What is selected when the group is not controlled.
    #[props(default)]
    default_value: Option<T>,
    /// Called when the selected value changes.
    #[props(default)]
    on_value_change: Callback<Option<T>>,
    /// Whether every tag in the group is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether tags can be selected at all. With this off they stay focusable
    /// and removable, which is a list of tags rather than a choice between them.
    /// The default repeats the primitive's own, since a prop declared here has
    /// to carry one.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    selectable: ReadSignal<bool>,
    /// Whether selecting the selected tag again clears the selection. The
    /// default repeats the primitive's own.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    allow_empty_selection: ReadSignal<bool>,
    /// Whether Escape clears the selection. The default repeats the primitive's
    /// own.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    escape_clears_selection: ReadSignal<bool>,
    /// Whether arrow-key navigation wraps around at the ends of the list. The
    /// default repeats the primitive's own.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagGroup::<T> {
            value,
            default_value,
            on_value_change,
            disabled,
            selectable,
            allow_empty_selection,
            escape_clears_selection,
            roving_loop,
            attributes: merged,
            {children}
        }
    }
}

/// A group of tags, any number of which can be selected.
///
/// Everything [`TagGroup`] documents holds here, with a set of values in place
/// of one: `allow_empty_selection` decides whether the last selected tag can be
/// deselected, and Escape clears all of them at once.
#[component]
pub fn TagGroupMulti<T: Clone + PartialEq + 'static>(
    /// Whether to emit the utilities that stack the label above the tags.
    #[props(default)]
    appearance: TagGroupAppearance,
    /// The controlled selected values. `Some` makes the group controlled.
    #[props(default)]
    values: ReadSignal<Option<Vec<T>>>,
    /// What is selected when the group is not controlled.
    #[props(default)]
    default_values: Vec<T>,
    /// Called when the selected values change.
    #[props(default)]
    on_values_change: Callback<Vec<T>>,
    /// Whether every tag in the group is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether tags can be selected at all.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    selectable: ReadSignal<bool>,
    /// Whether the last selected tag can be deselected.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    allow_empty_selection: ReadSignal<bool>,
    /// Whether Escape clears the selection.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    escape_clears_selection: ReadSignal<bool>,
    /// Whether arrow-key navigation wraps around at the ends of the list.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagGroupMulti::<T> {
            values,
            default_values,
            on_values_change,
            disabled,
            selectable,
            allow_empty_selection,
            escape_clears_selection,
            roving_loop,
            attributes: merged,
            {children}
        }
    }
}

/// What the group is called, carrying daisyUI's `label` class.
///
/// The primitive points the list's `aria-labelledby` at this element, so the
/// grid of tags is announced by the name written here rather than by whatever
/// text happened to be above it. daisyUI's `label` is the class it writes for
/// exactly that job (a muted line naming the control under it) and it applies
/// to any element rather than only to a `label`, which is what makes it usable
/// on the `div` the primitive renders.
#[component]
pub fn TagGroupLabel(
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then points the list's
    /// `aria-labelledby` at it; an id arriving as an attribute would replace
    /// the one the list is still naming.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: "label" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagGroupLabel { id, attributes: merged, {children} }
    }
}

/// The tags themselves, laid out with utilities because daisyUI has no class
/// for a set of badges.
///
/// The primitive gives this element `role="grid"` and one row per tag, which is
/// what lets a screen reader say how many tags there are and which one it is on
/// because a badge is not usually anything you can focus, and a list of removable
/// tokens is.
#[component]
pub fn TagList(
    /// Whether to emit the utilities that lay the tags out.
    #[props(default)]
    appearance: TagListAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagList { attributes: merged, {children} }
    }
}

/// One tag, carrying daisyUI's `badge` classes.
///
/// The badge lands on the primitive's row element, which is where daisyUI's
/// markup wants it: the cell inside it is `display: contents`, so the label and
/// the remove button are the flex children of the badge itself, exactly as they
/// would be in daisyUI's own markup, with the grid semantics on top of it.
///
/// Selection and focus are marked by [`TagOptionAppearance`]'s utilities rather
/// than by a daisyUI class, because daisyUI has neither. A disabled tag is not
/// marked at all: daisyUI has no disabled badge, and the primitive already makes
/// it inert and skips it; see the component's documentation.
#[component]
pub fn TagOption<T: Clone + PartialEq + 'static>(
    /// daisyUI's colour axis.
    #[props(default)]
    color: TagColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: TagSize,
    /// Whether to emit the utilities that mark this tag selected and focused.
    #[props(default)]
    appearance: TagOptionAppearance,
    /// What this tag is worth, which is what the group reports as selected and
    /// what a remove names.
    value: ReadSignal<T>,
    /// What the remove button calls this tag when its value is not a string.
    #[props(default)]
    text_value: ReadSignal<Option<String>>,
    /// Where this tag falls in the keyboard navigation order, which is also the
    /// row number it announces.
    index: ReadSignal<usize>,
    /// The id of this element.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    /// Whether this tag is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = color.class();
    let size = size.class();
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "badge {color} {size} {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagOption::<T> {
            value,
            text_value,
            index,
            id,
            disabled,
            attributes: merged,
            {children}
        }
    }
}

/// The control that removes the tag it is written in, carrying daisyUI's `btn`
/// classes.
///
/// The class strings are the button component's, duplicated rather than depended
/// on, as the toast's close button is: the registry uses no cross-component
/// dependencies, and the Tailwind contract already requires every one of them to
/// be a literal in the file that emits it.
///
/// Writing this button is what makes the tag removable at all: by click and by
/// Delete or Backspace on the tag itself. The primitive names it after the tag
/// it belongs to, so a row of them is not a row of buttons all called "remove".
#[component]
pub fn TagRemoveButton(
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: "btn btn-ghost btn-xs btn-circle",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagRemoveButton { attributes: merged, {children} }
    }
}

/// What the list says once every tag in it has been removed.
///
/// The primitive renders it only then, and renders it as a row of the same grid,
/// so the list keeps its shape while it is empty.
#[component]
pub fn TagGroupEmpty(
    /// Whether to emit the utilities that mute it.
    #[props(default)]
    appearance: TagGroupEmptyAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        tag_group::TagGroupEmpty { attributes: merged, {children} }
    }
}
