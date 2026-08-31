use dioxus::prelude::*;

use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableHeader, TableHeaderCell, TableRow, TableSize,
};

/// Every size, with one body cell exposing daisyUI's padding and type scale.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-start gap-4",
            for size in TableSize::ALL.iter().copied() {
                Table { size, class: "w-auto",
                    TableCaption { class: "sr-only", "{size:?} table size" }
                    TableHeader {
                        TableRow {
                            TableHeaderCell { scope: "col", "Size" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "{size:?}" }
                        }
                    }
                }
            }
        }
    }
}
