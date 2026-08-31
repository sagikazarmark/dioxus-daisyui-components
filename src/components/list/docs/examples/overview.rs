use dioxus::prelude::*;

use crate::components::list::{List, ListColumn, ListColumnGrow, ListColumnWrap, ListRow};

/// A list with one growing title and one description wrapped below its row.
#[component]
pub fn Example() -> Element {
    rsx! {
        List { id: "default-list", class: "w-full max-w-md bg-base-100 rounded-box shadow-sm",
            ListRow { id: "default-row",
                div { class: "text-2xl", "01" }
                div { class: "size-10 rounded-box bg-primary/15" }
                ListColumn { id: "default-grow-column", grow: ListColumnGrow::Grow,
                    div { class: "font-semibold", "Aster" }
                    div { class: "text-xs uppercase opacity-60", "Field notes" }
                }
                button { class: "btn btn-sm btn-ghost", "Open" }
            }
            ListRow {
                div { class: "size-10 rounded-box bg-secondary/15" }
                div {
                    div { class: "font-semibold", "Lantern" }
                    div { class: "text-xs uppercase opacity-60", "Night journal" }
                }
                ListColumn { id: "default-wrap-column", wrap: ListColumnWrap::Wrap,
                    "A compact description placed below the rest of the row."
                }
                button { class: "btn btn-sm btn-ghost", "Open" }
            }
        }
    }
}
