use dioxus::prelude::*;

use crate::components::list::{List, ListColumn, ListColumnGrow, ListColumnWrap, ListRow};

/// A caller's classes joined with every part's own.
#[component]
pub fn Example() -> Element {
    rsx! {
        List { id: "caller-list", class: "w-full max-w-md rounded-none bg-base-100 shadow-sm",
            ListRow { id: "caller-row", class: "p-2",
                div { "01" }
                ListColumn {
                    id: "caller-column",
                    class: "italic",
                    grow: ListColumnGrow::Grow,
                    wrap: ListColumnWrap::Wrap,
                    "Caller-styled content that grows and wraps."
                }
            }
        }
    }
}
