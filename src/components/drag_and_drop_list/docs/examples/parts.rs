use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list::use_drag_and_drop_list_items;

use crate::components::drag_and_drop_list::{
    DragAndDropDropIndicator, DragAndDropInstructions, DragAndDropList, DragAndDropListItem,
    DragAndDropListItems, DragAndDropLiveRegion,
};

/// The same list with its parts written out, which is what the collapsed form
/// above renders for you.
///
/// It is worth writing out when something else has to go in the box (a heading
/// over the rows, a count under them) or when a row needs more than the item
/// it was given: here each row carries a grab handle beside its content, which
/// is a caller's element rather than one this registry emits.
#[component]
pub fn Example() -> Element {
    let items = ["Sourdough", "Rye", "Focaccia"]
        .iter()
        .map(|loaf| {
            rsx! {
                span { class: "opacity-40", "⠿" }
                div { class: "list-col-grow", "{loaf}" }
            }
        })
        .collect();

    rsx! {
        div { id: "written-out", class: "w-72",
            DragAndDropList { items, aria_label: "Bakes",
                DragAndDropInstructions {}
                DragAndDropListItems { aria_label: "Bakes",
                    Rows {}
                }
                DragAndDropLiveRegion {}
            }
        }
    }
}

/// The rows, which have to be their own component: the list they come from is a
/// context the list element provides, so a hook reading it belongs to something
/// rendered *inside* that element rather than to whoever wrote the list.
///
/// `use_drag_and_drop_list_items` is the primitive's list in its current order.
/// A drop line is rendered either side of each row; only the one a drop is
/// aimed at is ever in the document.
#[component]
fn Rows() -> Element {
    rsx! {
        for item in use_drag_and_drop_list_items() {
            Fragment { key: "{item.key}",
                DragAndDropDropIndicator { index: item.index, position: "before" }
                DragAndDropListItem {
                    index: item.index,
                    item_key: item.key.clone(),
                    {item.children}
                }
                DragAndDropDropIndicator { index: item.index, position: "after" }
            }
        }
    }
}
