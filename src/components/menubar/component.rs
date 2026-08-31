use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::menubar;
use dioxus_primitives::merge_attributes;

/// daisyUI's colour axis for a menubar trigger, which is the button's own.
///
/// The class strings are the button component's, duplicated rather than
/// depended on: the registry uses no cross-component dependencies, and the
/// Tailwind contract already requires every one of these to be a literal in the
/// file that emits it.
///
/// [`MenubarTriggerColor::Default`] emits no class at all, which is daisyUI's
/// uncoloured button rather than a synonym for
/// [`MenubarTriggerColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum MenubarTriggerColor {
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

impl MenubarTriggerColor {
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

/// daisyUI's size axis for a menubar trigger, which is the button's own and is
/// duplicated for the reason [`MenubarTriggerColor`] records.
///
/// [`MenubarTriggerSize::Default`] emits no class, which renders at the same
/// size as daisyUI's explicit `btn-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum MenubarTriggerSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl MenubarTriggerSize {
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

/// daisyUI's size axis for the menu inside a menubar's popup, which sizes the
/// items rather than the box they are in.
///
/// The class strings are the dropdown menu's, duplicated for the reason
/// [`MenubarTriggerColor`] records.
///
/// [`MenubarMenuSize::Default`] emits no class, which renders at the same size
/// as daisyUI's explicit `menu-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum MenubarMenuSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl MenubarMenuSize {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "menu-xs",
            Self::Sm => "menu-sm",
            Self::Default => "",
            Self::Lg => "menu-lg",
            Self::Xl => "menu-xl",
        }
    }
}

/// Whether [`Menubar`] emits the utilities that lay the bar out.
///
/// daisyUI has no class for this element. Its own menu bar is `menu
/// menu-horizontal` on a list, which this component cannot be (ADR-0018 records
/// why) so the bar is a row of buttons laid out with Tailwind utilities, and
/// daisyUI's `menu` is used where its markup can be reproduced: inside the
/// popups.
///
/// That inverts the usual convention: [`MenubarAppearance::Default`] emits
/// classes and [`MenubarAppearance::None`] emits nothing. A utility this
/// component emits only ties with a caller's, and a tie is settled by
/// generated-stylesheet order rather than by the class attribute, so switching
/// ours off is the way to win it (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum MenubarAppearance {
    #[default]
    Default,
    None,
}

impl MenubarAppearance {
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

/// Whether [`MenubarContent`] emits the utilities that place and draw the popup.
///
/// daisyUI's own floating box is `dropdown-content`, which positions an element
/// inside a `.dropdown`, and there is no `.dropdown` here, because the bar and
/// each menu are elements the primitive renders. So the popup is placed with
/// Tailwind utilities, the way the hover card's panel is (ADR-0015), and painted
/// with the same utilities daisyUI writes beside `dropdown-content` in its own
/// examples.
///
/// This is the inverted shape again: [`MenubarContentAppearance::Default`] emits
/// classes and [`MenubarContentAppearance::None`] emits nothing, so a caller who
/// positions the popup themselves (with anchor positioning, or a
/// floating-element library) switches ours off rather than out-ranking it
/// (ADR-0004).
///
/// daisyUI's own `p-2` is left out, as it is on the dropdown menu: `.menu`
/// already pads by exactly that, and here the two elements would pad in turn
/// rather than agree.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum MenubarContentAppearance {
    #[default]
    Default,
    None,
}

impl MenubarContentAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    ///
    /// The `z-10` is this component's rather than daisyUI's: `dropdown-content`
    /// brings a `z-index` of its own and nothing here does, so a popup over the
    /// content below it needs one written.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "absolute start-0 top-full z-10 mt-1 bg-base-100 rounded-box shadow-sm"
            }
            Self::None => "",
        }
    }
}

/// A bar of menus, laid out with Tailwind utilities.
///
/// daisyUI's own menu bar is `menu menu-horizontal` on a list, and this element
/// cannot be one: the primitive renders a wrapper per menu between the bar and
/// the trigger, so daisyUI's item rules (which reach an item as `li > *`) would
/// land on that wrapper rather than on the trigger inside it, taking the
/// keyboard highlight with them. ADR-0018 records it in full. What the bar
/// carries instead is layout, all of it Defeatable (ADR-0004), and each trigger
/// carries `btn`.
///
/// There is no open state to bridge here, and nothing to lift: the primitive
/// has no controlled open prop and no change callback (which menu is open lives
/// in a context of its own) so this component never learns it. What the
/// primitive does report is `data-state` on each menu's wrapper, which is what
/// [`MenubarTrigger`]'s open look is written against.
///
/// Classes passed by the caller concatenate with the bar's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn Menubar(
    /// Whether to emit the utilities that lay the bar out.
    #[props(default)]
    appearance: MenubarAppearance,
    /// Whether every menu in the bar is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether arrow-key navigation wraps around at the ends of the bar. The
    /// default repeats the primitive's own, since a prop declared here has to
    /// carry one.
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
        menubar::Menubar { disabled, roving_loop, attributes: merged, {children} }
    }
}

/// One menu of a menubar: its trigger and the popup that trigger opens.
///
/// This element is the positioning context the popup is placed against, which is
/// the whole of what it emits: `relative`, and a `group` marker so that
/// [`MenubarTrigger`] can be styled from the `data-state` the primitive sets
/// here. Both are Defeatable (ADR-0004): the appearance axis that switches the
/// popup's placement off switches this off with it, on the axis' own part.
///
/// Classes passed by the caller concatenate with this element's own.
#[component]
pub fn MenubarMenu(
    /// Where this menu falls in the bar's order, which is both the keyboard
    /// navigation order and how the primitive identifies it. It is explicit
    /// rather than taken from the DOM because the bar is one tab stop, so the
    /// order is the caller's to state.
    index: ReadSignal<usize>,
    /// Whether this menu is disabled, which leaves its trigger inert and the
    /// menu unopenable.
    #[props(default)]
    disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "group relative",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        menubar::MenubarMenu { index, disabled, attributes: merged, {children} }
    }
}

/// The control one menu opens from, carrying daisyUI's `btn` class.
///
/// The open look is a **Bridged utility**: the primitive reports the state as
/// `data-state` on the menu's wrapper, and the utility is written as a variant
/// of that attribute on the element below it (`group-data-[state=open]:…`)
/// rather than recomputed in Rust. Nothing else can do it here: daisyUI's
/// `btn-active` would have to be emitted from state this component is never
/// told, since the primitive's menubar has no controlled open prop and no change
/// callback to lift one through.
///
/// Everything the caller passes lands on the button, which is both the element
/// daisyUI styles and the one the primitive gives the `menuitem` role and the
/// keyboard behaviour to.
#[component]
pub fn MenubarTrigger(
    /// daisyUI's colour axis.
    #[props(default)]
    color: MenubarTriggerColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: MenubarTriggerSize,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = color.class();
    let size = size.class();

    let base = attributes!(button {
        class: "btn {color} {size} group-data-[state=open]:ring-2 group-data-[state=open]:ring-offset-2 group-data-[state=open]:ring-offset-base-100",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        menubar::MenubarTrigger { attributes: merged, {children} }
    }
}

/// The popup one menu opens, holding the `menu` list.
///
/// This is the split ADR-0005 exists for, in the third place it applies. daisyUI
/// puts `menu` on a `ul`, which is impossible here: the primitive's content
/// element is a hardcoded `div`, and every visual rule `.menu` has for an item is
/// written against a literal `li`. So the placement and the box go on the
/// primitive's element and `menu` goes on a list rendered inside it, each class
/// on the element whose layout it was written to drive.
///
/// The list is marked presentational, as [`MenubarItem`]'s own wrapper is: the
/// primitive gives this element the `menu` role and the items `menuitem`, and a
/// plain list between them would break the ownership a screen reader announces
/// the menu from.
///
/// Classes passed by the caller concatenate with the popup's own, which is the
/// element worth reaching: a width or a fill belongs on the box rather than on
/// the list inside it.
#[component]
pub fn MenubarContent(
    /// Whether to emit the utilities that place and draw the popup.
    #[props(default)]
    appearance: MenubarContentAppearance,
    /// daisyUI's size axis for the menu, which sizes the items.
    #[props(default)]
    size: MenubarMenuSize,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then looks the element up by it;
    /// an id that arrived as an attribute would be written over the one it is
    /// looking for.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();
    let size = size.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        menubar::MenubarContent { id, attributes: merged,
            // The list is stretched to the box rather than left to size itself,
            // which is the one place the split has to be papered over: `.menu`
            // is `width: fit-content`, and on daisyUI's single element that is
            // the box's width as well, where here it would leave a caller's
            // width on the box with a shrink-wrapped menu inside it.
            ul { role: "none", class: "menu w-full {size}", {children} }
        }
    }
}

/// One item of a menu, wrapped in the list item daisyUI's `menu` styles it
/// through.
///
/// Nothing is emitted on the item itself: `.menu` reaches it as the child of an
/// `li`, and both the hover highlight and the keyboard one come with that, the
/// second because daisyUI's rule matches `:focus-visible` as well as its own
/// class, and the primitive moves real DOM focus onto this element.
///
/// The wrapper is marked presentational for the reason [`MenubarContent`]'s list
/// is, and costs nothing behaviourally: items register with the primitive's
/// focus collection by the `index` they are given rather than by where they sit
/// in the DOM.
///
/// The value is a `String` rather than a generic, which is the primitive's
/// choice here and not the dropdown menu's. Widening it would mean this
/// component owning a mapping back to the string the primitive wants, which is
/// more than a class mapping.
#[component]
pub fn MenubarItem(
    /// What this item is worth, which is what `on_select` is called with.
    value: String,
    /// Where this item falls in the keyboard navigation order within its menu.
    index: ReadSignal<usize>,
    /// Whether this item is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Called with this item's value when it is selected.
    #[props(default)]
    on_select: Callback<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    // Tier 2 on the disabled state, on the wrapper rather than on the item:
    // daisyUI mutes a disabled item through `.menu-disabled` on the list item or
    // a `disabled` attribute on the item, and the primitive sets neither: it
    // reports the state as `data-disabled`, which daisyUI matches nowhere.
    let state = if disabled() { "menu-disabled" } else { "" };

    rsx! {
        li { role: "none", class: "{state}",
            menubar::MenubarItem {
                value,
                index,
                disabled,
                on_select,
                attributes,
                {children}
            }
        }
    }
}
