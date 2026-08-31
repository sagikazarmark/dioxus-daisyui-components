use dioxus::prelude::*;

use crate::components::breadcrumbs::{Breadcrumbs, BreadcrumbsItem, BreadcrumbsList};

/// A labelled trail with linked ancestors and a non-linked current page.
#[component]
pub fn Example() -> Element {
    rsx! {
        Breadcrumbs { id: "overview-breadcrumbs", aria_label: "Breadcrumb",
            BreadcrumbsList { id: "overview-list",
                BreadcrumbsItem { a { href: "#components", "Components" } }
                BreadcrumbsItem { a { href: "#navigation", "Navigation" } }
                BreadcrumbsItem { "Breadcrumbs" }
            }
        }
    }
}
