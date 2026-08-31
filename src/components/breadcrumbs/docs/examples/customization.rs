use dioxus::prelude::*;

use crate::components::breadcrumbs::{Breadcrumbs, BreadcrumbsItem, BreadcrumbsList};

/// Caller attributes on every part and a linked current page.
#[component]
pub fn Example() -> Element {
    rsx! {
        h3 { id: "account-location", class: "sr-only", "Account location" }
        Breadcrumbs {
            id: "caller-breadcrumbs",
            class: "rounded-box bg-base-200 px-3",
            aria_labelledby: "account-location",
            "data-owner": "root",
            BreadcrumbsList {
                id: "caller-list",
                class: "text-sm",
                start: 4,
                "data-owner": "list",
                BreadcrumbsItem { a { href: "#account", "Account" } }
                BreadcrumbsItem {
                    id: "caller-item",
                    class: "font-semibold",
                    value: 5,
                    "data-owner": "item",
                    a { href: "#profile", aria_current: "page", "Profile" }
                }
            }
        }
    }
}
