use dioxus::prelude::*;

use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableHeader, TableHeaderCell, TableRow,
};

/// Horizontal scrolling supplied by the caller rather than by the Component.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            id: "caller-overflow-wrapper",
            class: "w-full max-w-72 overflow-x-auto rounded-box border border-base-content/10 bg-base-100",
            Table { id: "wide-table", class: "min-w-[48rem]",
                TableCaption { class: "caption-top px-4 text-left font-semibold", "Regional revenue" }
                TableHeader {
                    TableRow {
                        TableHeaderCell { scope: "col", "Region" }
                        TableHeaderCell { scope: "col", "Owner" }
                        TableHeaderCell { scope: "col", "Forecast" }
                        TableHeaderCell { scope: "col", "Actual" }
                    }
                }
                TableBody {
                    TableRow {
                        TableHeaderCell { scope: "row", "North America" }
                        TableCell { "Account team A" }
                        TableCell { "$120,000" }
                        TableCell { "$126,000" }
                    }
                    TableRow {
                        TableHeaderCell { scope: "row", "Europe" }
                        TableCell { "Account team B" }
                        TableCell { "$98,000" }
                        TableCell { "$101,000" }
                    }
                }
            }
        }
    }
}
