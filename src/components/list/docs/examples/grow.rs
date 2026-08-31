use dioxus::prelude::*;

use crate::components::list::{List, ListColumn, ListColumnGrow, ListRow};

/// Every value of the grow Axis, with the modifier on the row's first child.
#[component]
pub fn Example() -> Element {
    rsx! {
        List { "data-axis": "grow", class: "w-full max-w-md bg-base-100 rounded-box shadow-sm",
            for grow in ListColumnGrow::ALL.iter().copied() {
                ListRow {
                    ListColumn { grow, "{grow:?} column" }
                    div { class: "badge badge-ghost", "Fixed" }
                }
            }
        }
    }
}
