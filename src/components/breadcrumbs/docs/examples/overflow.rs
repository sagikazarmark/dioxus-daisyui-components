use dioxus::prelude::*;

use crate::components::breadcrumbs::{Breadcrumbs, BreadcrumbsItem, BreadcrumbsList};

/// A long hierarchy constrained to a horizontally scrolling trail.
#[component]
pub fn Example() -> Element {
    rsx! {
        Breadcrumbs {
            id: "overflow-trail",
            class: "max-w-xs rounded-box bg-base-200 px-3 text-sm",
            aria_label: "Deep location",
            BreadcrumbsList {
                BreadcrumbsItem { a { href: "#workspace", "Workspace" } }
                BreadcrumbsItem { a { href: "#design-systems", "Design systems" } }
                BreadcrumbsItem { a { href: "#component-library", "Component library" } }
                BreadcrumbsItem { a { href: "#navigation", "Navigation patterns" } }
                BreadcrumbsItem { a { href: "#hierarchy", "Hierarchical context" } }
                BreadcrumbsItem { "Breadcrumb documentation" }
            }
        }
    }
}
