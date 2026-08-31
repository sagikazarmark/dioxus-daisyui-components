use dioxus::prelude::*;

use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableColumnPinning, TableFooter, TableHeader,
    TableHeaderCell, TableRow, TableRowPinning,
};

/// Every pinning value in a scroll region, plus both pinning Axes together.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            section { class: "flex flex-col gap-3",
                h3 { class: "font-semibold", "Row pinning" }
                div { "data-axis": "row-pinning", class: "grid gap-4 md:grid-cols-2",
                    for row_pinning in TableRowPinning::ALL.iter().copied() {
                        div { class: "h-44 overflow-auto rounded-box border border-base-content/10",
                            PinningTable {
                                caption: format!("{row_pinning:?} rows"),
                                row_pinning,
                            }
                        }
                    }
                }
            }

            section { class: "flex flex-col gap-3",
                h3 { class: "font-semibold", "Column pinning" }
                div { "data-axis": "column-pinning", class: "grid gap-4 md:grid-cols-2",
                    for column_pinning in TableColumnPinning::ALL.iter().copied() {
                        div { class: "w-full max-w-72 overflow-x-auto rounded-box border border-base-content/10",
                            PinningTable {
                                caption: format!("{column_pinning:?} columns"),
                                column_pinning,
                            }
                        }
                    }
                }
            }

            section { class: "flex flex-col gap-3",
                h3 { class: "font-semibold", "Rows and columns together" }
                div {
                    id: "combined-pinning-wrapper",
                    class: "h-44 w-full max-w-72 overflow-auto rounded-box border border-base-content/10",
                    PinningTable {
                        caption: "Combined pinning",
                        row_pinning: TableRowPinning::Pinned,
                        column_pinning: TableColumnPinning::Pinned,
                    }
                }
            }
        }
    }
}

#[component]
fn PinningTable(
    caption: String,
    #[props(default)] row_pinning: TableRowPinning,
    #[props(default)] column_pinning: TableColumnPinning,
) -> Element {
    rsx! {
        Table {
            class: "min-w-[40rem]",
            row_pinning,
            column_pinning,
            TableCaption { class: "sr-only", "{caption}" }
            TableHeader {
                TableRow {
                    TableHeaderCell { scope: "col", "Plan" }
                    TableHeaderCell { scope: "col", "Owner" }
                    TableHeaderCell { scope: "col", "Monthly revenue" }
                }
            }
            TableBody {
                for number in 1..=8 {
                    TableRow {
                        TableHeaderCell { scope: "row", "Plan {number}" }
                        TableCell { "Account team {number}" }
                        TableCell { "${number},000" }
                    }
                }
            }
            TableFooter {
                TableRow {
                    TableHeaderCell { scope: "row", "Total" }
                    TableCell { "Eight plans" }
                    TableCell { "$36,000" }
                }
            }
        }
    }
}
