use dioxus::prelude::*;

use crate::components::list::{List, ListColumn, ListColumnWrap, ListRow};

/// Every value of the wrap Axis, with the modifier on the row's first child.
#[component]
pub fn Example() -> Element {
    rsx! {
        List { "data-axis": "wrap", class: "w-full max-w-md bg-base-100 rounded-box shadow-sm",
            for wrap in ListColumnWrap::ALL.iter().copied() {
                ListRow {
                    ListColumn { wrap, "{wrap:?} column" }
                    div { class: "badge badge-ghost", "First row" }
                }
            }
        }
    }
}
