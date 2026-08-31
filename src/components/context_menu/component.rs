use dioxus::prelude::*;
use dioxus_primitives::context_menu;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's size axis for the menu inside a context menu, which sizes the
/// items rather than the box they are in.
///
/// The class strings are the dropdown menu's, duplicated rather than depended
/// on: the registry uses no cross-component dependencies, and the Tailwind
/// contract already requires every one of these to be a literal in the file
/// that emits it.
///
/// [`ContextMenuSize::Default`] emits no class, which renders at the same size
/// as daisyUI's explicit `menu-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ContextMenuSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl ContextMenuSize {
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

/// Whether [`ContextMenuContent`] emits the utilities that draw the box the
/// menu sits in.
///
/// daisyUI has no class for this box at all. Its own floating menus are
/// `dropdown-content`, which positions the element against a trigger, and a
/// context menu is positioned against the *pointer* instead, by the primitive,
/// with coordinates it takes off the event. So what daisyUI writes as utilities
/// beside `dropdown-content` in its own examples (the fill, the corners and the
/// shadow) are all that is left to emit, and they are emitted here.
///
/// That inverts the usual convention: [`ContextMenuContentAppearance::Default`]
/// emits classes and [`ContextMenuContentAppearance::None`] emits nothing. A
/// utility this component emits only ties with a caller's, and a tie is settled
/// by generated-stylesheet order rather than by the class attribute, so
/// switching ours off is the way to win it (ADR-0004).
///
/// daisyUI's own `p-2` is left out, as it is on the dropdown menu: `.menu`
/// already pads by exactly that, and here the two elements would pad in turn
/// rather than agree.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ContextMenuContentAppearance {
    #[default]
    Default,
    None,
}

impl ContextMenuContentAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "bg-base-100 rounded-box shadow-sm",
            Self::None => "",
        }
    }
}

/// The surface a context menu belongs to, holding its trigger and its menu.
///
/// Nothing is emitted here. daisyUI has no element for it: its own menus are
/// written where they appear, and this one is a wrapper the primitive needs so
/// that a right click, an outside click and Escape all reach the same state.
///
/// There is no open state to bridge either, which is what makes this component
/// unlike the dropdown menu. daisyUI hides nothing here (no `dropdown-content`
/// is emitted, because there is no trigger to position against) and the
/// primitive mounts the menu only while it is open. So the state travels
/// through to the primitive untouched, and both a controlled caller and an
/// uncontrolled one get the primitive's own behaviour.
///
/// Classes and attributes passed by the caller land on this element.
#[component]
pub fn ContextMenu(
    /// The controlled open state of the menu.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// The state the menu starts in when it is not controlled. The primitive
    /// closes a menu that nothing in it is focused, so a menu that has to stand
    /// open is one a caller controls; see the component's documentation.
    #[props(default)]
    default_open: bool,
    /// Called when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    /// Whether the menu is disabled, which leaves the surface inert and the
    /// menu unopenable.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether arrow-key navigation wraps around at the ends of the menu. The
    /// default repeats the primitive's own, since a prop declared here has to
    /// carry one.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        context_menu::ContextMenu {
            open,
            default_open,
            on_open_change,
            disabled,
            roving_loop,
            attributes,
            {children}
        }
    }
}

/// The surface a right click opens the menu over.
///
/// It emits nothing. daisyUI has no class for a right-clickable region, and
/// what one looks like is the caller's: a table row, a canvas, a file tile.
/// What the primitive puts here instead is the behaviour: `contextmenu` for a
/// mouse, a long press for touch and pen, `aria-haspopup` and `aria-expanded`
/// so the surface announces that it has a menu, and the pointer coordinates the
/// menu is pinned to.
#[component]
pub fn ContextMenuTrigger(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        context_menu::ContextMenuTrigger { attributes, {children} }
    }
}

/// The box a context menu's items sit in, holding the `menu` list.
///
/// This is the split ADR-0005 exists for, in the second place it applies.
/// daisyUI puts `menu` on a `ul`, which is impossible here: the primitive's
/// content element is a hardcoded `div` (it has to be, because the menu is
/// pinned to the pointer with a `position: fixed` and inline coordinates) and
/// every visual rule `.menu` has for an item is written against a literal `li`.
/// So the box goes on the primitive's element and `menu` goes on a list
/// rendered inside it, each class on the element whose layout it was written to
/// drive.
///
/// The list is marked presentational, as [`ContextMenuItem`]'s own wrapper is:
/// the primitive gives this element the `menu` role and the items the
/// `menuitem` role, and a plain list between them would break the ownership a
/// screen reader announces the menu from.
///
/// Classes passed by the caller concatenate with the box's own, which is the
/// element worth reaching: a width or a fill belongs on the box rather than on
/// the list inside it.
#[component]
pub fn ContextMenuContent(
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: ContextMenuContentAppearance,
    /// daisyUI's size axis for the menu, which sizes the items.
    #[props(default)]
    size: ContextMenuSize,
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
        context_menu::ContextMenuContent { id, attributes: merged,
            // The list is stretched to the box rather than left to size itself,
            // which is the one place the split has to be papered over: `.menu`
            // is `width: fit-content`, and on daisyUI's single element that is
            // the box's width as well, where here it would leave a caller's
            // width on the box with a shrink-wrapped menu inside it. With no
            // width on the box the two are the same thing anyway, since the box
            // shrink-wraps the list in turn.
            ul { role: "none", class: "menu w-full {size}", {children} }
        }
    }
}

/// One item of a context menu, wrapped in the list item daisyUI's `menu` styles
/// it through.
///
/// Nothing is emitted on the item itself: `.menu` reaches it as the child of an
/// `li`, and both the hover highlight and the keyboard one come with that, the
/// second because daisyUI's rule matches `:focus-visible` as well as its own
/// class, and the primitive moves real DOM focus onto this element.
///
/// The wrapper is marked presentational for the reason [`ContextMenuContent`]'s
/// list is, and costs nothing behaviourally: items register with the primitive's
/// focus collection by the `index` they are given rather than by where they sit
/// in the DOM.
///
/// Everything the caller passes travels to the item rather than to the wrapper,
/// which is the element daisyUI styles and the primitive gives the `menuitem`
/// role to.
///
/// The value is a `String` rather than a generic, which is the primitive's
/// choice here and not the dropdown menu's: its items carry a value of the
/// caller's own type. Widening it would mean this component owning a mapping
/// back to the string the primitive wants, which is more than a class mapping.
#[component]
pub fn ContextMenuItem(
    /// What this item is worth, which is what `on_select` is called with.
    value: ReadSignal<String>,
    /// Where this item falls in the keyboard navigation order.
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
    // reports the state as `data-disabled` and `aria-disabled`, which daisyUI
    // matches nowhere.
    let state = if disabled() { "menu-disabled" } else { "" };

    rsx! {
        li { role: "none", class: "{state}",
            context_menu::ContextMenuItem {
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
