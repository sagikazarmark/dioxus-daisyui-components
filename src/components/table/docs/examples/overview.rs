use dioxus::prelude::*;

use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHeader, TableHeaderCell, TableRow,
};

/// Native table semantics and caller attributes on every Compound part.
#[component]
pub fn Example() -> Element {
    rsx! {
        Table {
            id: "semantic-table",
            class: "rounded-none",
            "data-owner": "caller",
            TableCaption {
                id: "sales-caption",
                class: "caption-top pb-3 text-left font-semibold",
                "Quarterly subscriptions by plan"
            }
            TableHeader { id: "semantic-head", class: "bg-base-200/50",
                TableRow { id: "heading-row",
                    TableHeaderCell {
                        id: "plan-heading",
                        class: "whitespace-nowrap",
                        scope: "col",
                        rowspan: 2,
                        "Plan"
                    }
                    TableHeaderCell {
                        id: "quarter-heading",
                        scope: "colgroup",
                        colspan: 2,
                        "Quarter totals"
                    }
                }
                TableRow {
                    TableHeaderCell { id: "accounts-heading", scope: "col", "Accounts" }
                    TableHeaderCell { id: "revenue-heading", scope: "col", "Revenue" }
                }
            }
            TableBody { id: "semantic-body", class: "text-base-content",
                TableRow { id: "active-row", class: "bg-base-200",
                    TableHeaderCell { id: "starter-heading", scope: "row", "Starter" }
                    TableCell {
                        id: "starter-accounts",
                        class: "font-medium",
                        "headers": "starter-heading accounts-heading",
                        "128"
                    }
                    TableCell { "headers": "starter-heading revenue-heading", "$2,560" }
                }
                TableRow {
                    TableHeaderCell { id: "team-heading", scope: "row", "Team" }
                    TableCell { "headers": "team-heading accounts-heading", "64" }
                    TableCell { "headers": "team-heading revenue-heading", "$6,400" }
                }
            }
            TableFooter { id: "semantic-foot", class: "bg-base-200/50",
                TableRow { id: "total-row",
                    TableHeaderCell { id: "total-heading", scope: "row", "Total" }
                    TableCell {
                        id: "total-cell",
                        "headers": "total-heading accounts-heading revenue-heading",
                        colspan: 2,
                        "192 accounts / $8,960"
                    }
                }
            }
        }
    }
}
