use dioxus::prelude::*;

use crate::components::drag_and_drop_list::DragAndDropList;

/// The tracks of a playlist, in an order the reader owns.
///
/// A row is dragged with the pointer, or picked up with Enter and moved with
/// the arrow keys; the keyboard path is the primitive's, and it is the reason
/// this component wraps one rather than being a `list` with a drag handler.
/// Every move is announced in a live region, and Escape puts a row back where
/// it started.
///
/// Written with nothing inside it, the list renders those three things itself:
/// the instructions a screen reader reads first, the rows, and the region each
/// move is announced in.
#[component]
pub fn Example() -> Element {
    let items = ["Rise", "Ember", "Lantern", "Tidewater"]
        .iter()
        .enumerate()
        .map(|(position, track)| {
            rsx! {
                div { class: "text-4xl font-thin opacity-30 tabular-nums", "{position + 1}" }
                div { class: "list-col-grow",
                    div { "{track}" }
                    div { class: "text-xs font-semibold uppercase opacity-60", "Kite Season" }
                }
            }
        })
        .collect();

    rsx! {
        div { id: "sortable", class: "w-80",
            DragAndDropList { items, aria_label: "Playlist" }
        }
    }
}
