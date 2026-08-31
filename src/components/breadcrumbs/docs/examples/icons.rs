use dioxus::prelude::*;

use crate::components::breadcrumbs::{Breadcrumbs, BreadcrumbsItem, BreadcrumbsList};

/// Icons supplied by the caller inside the item's direct child.
#[component]
pub fn Example() -> Element {
    rsx! {
        Breadcrumbs { id: "icon-breadcrumbs", aria_label: "File location",
            BreadcrumbsList {
                BreadcrumbsItem {
                    a { href: "#workspace",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            class: "size-4 stroke-current",
                            "aria-hidden": "true",
                            path {
                                d: "M3 7v10a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-6l-2-2H5a2 2 0 0 0-2 2Z",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                            }
                        }
                        "Workspace"
                    }
                }
                BreadcrumbsItem {
                    a { href: "#documents",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            class: "size-4 stroke-current",
                            "aria-hidden": "true",
                            path {
                                d: "M7 3h7l5 5v13H7a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2Z M14 3v6h5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                            }
                        }
                        "Documents"
                    }
                }
                BreadcrumbsItem {
                    span {
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            class: "size-4 stroke-current",
                            "aria-hidden": "true",
                            path {
                                d: "M9 13h6m-3-3v6m5 5H7a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6l6 6v10a2 2 0 0 1-2 2Z",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                            }
                        }
                        "New document"
                    }
                }
            }
        }
    }
}
