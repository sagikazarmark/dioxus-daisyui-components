use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::drag_and_drop_list;
use dioxus_primitives::merge_attributes;

/// Whether [`DragAndDropListItems`] emits the utilities that draw the box the
/// rows sit in.
///
/// daisyUI's `list` lays the rows out and rules between them, and leaves the
/// fill, the corners and the shadow to utilities, which is how its own example
/// is written. They are emitted here instead, so the convention is the inverted
/// one ADR-0004 describes: [`DragAndDropListAppearance::Default`] emits and
/// [`DragAndDropListAppearance::None`] emits nothing.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DragAndDropListAppearance {
    #[default]
    Default,
    None,
}

impl DragAndDropListAppearance {
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

/// Whether [`DragAndDropListItem`] emits the classes that say what is happening
/// to a row.
///
/// daisyUI has a class for a list row and none at all for a row being moved, so
/// every state here is a **Bridged utility**: a Tailwind variant of an attribute
/// the primitive already sets on the row. Nothing is recomputed in Rust and the
/// primitive stays the only owner of the drag.
///
/// The states, and the attribute each is read off:
///
/// - the row that has been picked up: `data-is-grabbing`
/// - the row the keyboard is on: `data-focus-visible`
/// - a row that has been carried back to where it started, which is the one
///   case the primitive draws no drop line for: `data-drop-at-origin`
///
/// The grab cursor goes with them, because a row that can be dragged should
/// look like one.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DragAndDropListItemAppearance {
    #[default]
    Default,
    None,
}

impl DragAndDropListItemAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => concat!(
                "cursor-grab",
                " data-[is-grabbing=true]:cursor-grabbing data-[is-grabbing=true]:opacity-60",
                " data-[is-grabbing=true]:bg-base-200",
                " data-[focus-visible=true]:bg-base-200",
                " data-[drop-at-origin=true]:outline-2 data-[drop-at-origin=true]:outline-dashed",
                " data-[drop-at-origin=true]:outline-primary",
            ),
            Self::None => "",
        }
    }
}

/// Whether [`DragAndDropDropIndicator`] emits the utilities that draw the line
/// a row would land on.
///
/// There is no daisyUI class for it (the whole idea belongs to the primitive)
/// so the line is utilities over daisyUI's own theme colour, and the inverted
/// convention applies for the reason [`DragAndDropListAppearance`] records.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DragAndDropIndicatorAppearance {
    #[default]
    Default,
    None,
}

impl DragAndDropIndicatorAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "h-0.5 bg-primary",
            Self::None => "",
        }
    }
}

/// A list whose rows can be dragged into a different order, styled with
/// daisyUI's `list` classes.
///
/// This element emits nothing. It is the primitive's wrapper around three
/// things (the instructions a screen reader reads, the list itself, and the
/// live region each move is announced in) and daisyUI's `list` belongs on the
/// list rather than on what holds it.
///
/// Written with nothing inside it, it renders those three itself, which is the
/// common case and the one the axes below are here for. A caller who needs
/// something else in the box writes the parts out instead, and then passes the
/// axes to the parts.
///
/// Classes passed by the caller concatenate with this element's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn DragAndDropList(
    /// The rows, in the order they start in. They are elements rather than
    /// values because the primitive owns the order from here on: it renders
    /// them where they have been moved to.
    items: Vec<Element>,
    /// What a screen reader announces the list as.
    #[props(default)]
    aria_label: Option<String>,
    /// Whether to emit the utilities that draw the box, for the list this
    /// renders when nothing is written inside it.
    #[props(default)]
    appearance: DragAndDropListAppearance,
    /// Whether the rows are painted from what is happening to them.
    #[props(default)]
    item_appearance: DragAndDropListItemAppearance,
    /// Whether the line a row would land on is drawn.
    #[props(default)]
    indicator_appearance: DragAndDropIndicatorAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let label = aria_label
        .clone()
        .unwrap_or_else(|| "Sortable list".to_string());

    let children = children.unwrap_or_else(|| {
        rsx! {
            DragAndDropInstructions {}
            DragAndDropListItems {
                aria_label: label.clone(),
                appearance,
                item_appearance,
                indicator_appearance,
            }
            DragAndDropLiveRegion {}
        }
    });

    rsx! {
        drag_and_drop_list::DragAndDropList {
            items,
            aria_label,
            attributes,
            {children}
        }
    }
}

/// The list itself, carrying daisyUI's `list` class.
///
/// daisyUI's list is a match with nothing to work around: `.list` is a column,
/// `.list-row` is a row with a radius and a rule under it, and neither is gated
/// on an element type or on a sibling that the primitive's tree does not
/// produce. So the class goes where daisyUI puts it, on the `ul`.
///
/// Written with nothing inside it, it renders a row per item, with a drop line
/// either side of each, which is the primitive's own default composition, put
/// together again here so that the rows are this registry's rather than
/// unstyled ones.
#[component]
pub fn DragAndDropListItems(
    /// What a screen reader announces the list as.
    aria_label: String,
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: DragAndDropListAppearance,
    /// Whether the rows are painted from what is happening to them, for the
    /// rows this renders when nothing is written inside it.
    #[props(default)]
    item_appearance: DragAndDropListItemAppearance,
    /// Whether the line a row would land on is drawn.
    #[props(default)]
    indicator_appearance: DragAndDropIndicatorAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(ul {
        class: "list {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    let children = children.unwrap_or_else(|| {
        rsx! {
            for item in drag_and_drop_list::use_drag_and_drop_list_items() {
                Fragment { key: "{item.key}",
                    DragAndDropDropIndicator {
                        index: item.index,
                        position: "before",
                        appearance: indicator_appearance,
                    }
                    DragAndDropListItem {
                        index: item.index,
                        item_key: item.key.clone(),
                        appearance: item_appearance,
                        {item.children}
                    }
                    DragAndDropDropIndicator {
                        index: item.index,
                        position: "after",
                        appearance: indicator_appearance,
                    }
                }
            }
        }
    });

    rsx! {
        drag_and_drop_list::DragAndDropListItems { aria_label, attributes: merged, {children} }
    }
}

/// One row, carrying daisyUI's `list-row` class.
///
/// The row is what is dragged, what the keyboard picks up and puts down, and
/// what the arrow keys move, all of it the primitive's. What is emitted here is
/// `list-row` and a variant per state the primitive reports.
///
/// Everything the caller puts inside it is theirs; daisyUI's own list gives the
/// second child of a row the space left over, and `list-col-grow` moves that to
/// another one.
#[component]
pub fn DragAndDropListItem(
    /// Where this row currently is in the list.
    index: usize,
    /// What this row is, whatever it is moved to. It is the same value passed
    /// as the row's `key`.
    #[props(default)]
    item_key: Option<String>,
    /// Whether the row is painted from what is happening to it.
    #[props(default)]
    appearance: DragAndDropListItemAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(li {
        class: "list-row {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        drag_and_drop_list::DragAndDropListItem {
            index,
            item_key,
            attributes: merged,
            {children}
        }
    }
}

/// The line a row would land on, drawn where the drop would put it.
///
/// It is in the document only while a move is in flight and only beside the row
/// the drop is aimed at (the primitive renders nothing at all otherwise) so
/// this part is a line and nothing else.
#[component]
pub fn DragAndDropDropIndicator(
    /// The row this line belongs to.
    index: usize,
    /// Which side of that row it is on: `"before"` or `"after"`.
    position: &'static str,
    /// Whether to emit the utilities that draw the line.
    #[props(default)]
    appearance: DragAndDropIndicatorAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        drag_and_drop_list::DragAndDropDropIndicator { index, position, attributes: merged }
    }
}

/// What a screen reader is told about how to reorder the list without a
/// pointer.
///
/// Nothing is emitted here, and nothing needs to be: the primitive hides the
/// element itself, and it is not something anybody sees.
#[component]
pub fn DragAndDropInstructions(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        drag_and_drop_list::DragAndDropInstructions { attributes }
    }
}

/// Where each move is announced as it happens, for the same reason and in the
/// same way.
#[component]
pub fn DragAndDropLiveRegion(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        drag_and_drop_list::DragAndDropLiveRegion { attributes }
    }
}
