use dioxus::prelude::*;

use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableHeader, TableHeaderCell, TableRow, TableZebra,
};

/// Both zebra values, each with an even row for daisyUI to paint or leave plain.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "zebra", class: "grid gap-4 md:grid-cols-2",
            for zebra in TableZebra::ALL.iter().copied() {
                Table { zebra,
                    TableCaption { class: "caption-top text-left font-semibold", "{zebra:?}" }
                    TableHeader {
                        TableRow {
                            TableHeaderCell { scope: "col", "Plan" }
                            TableHeaderCell { scope: "col", "Accounts" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Starter" }
                            TableCell { "128" }
                        }
                        TableRow {
                            TableCell { "Team" }
                            TableCell { "64" }
                        }
                    }
                }
            }
        }
    }
}
