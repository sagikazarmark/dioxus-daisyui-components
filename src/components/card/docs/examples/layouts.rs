use dioxus::prelude::*;

use crate::components::card::{Card, CardBody, CardLayout, CardTitle};

const LANDSCAPE: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%20400%20220'%3E%3Crect%20width='400'%20height='220'%20fill='%236d5dfc'/%3E%3Ccircle%20cx='320'%20cy='55'%20r='30'%20fill='%23f9d72f'/%3E%3Cpath%20d='M0%20175L95%2080l65%2055%2075-90%20165%20130v45H0z'%20fill='%2329a36a'/%3E%3Cpath%20d='M0%20190l120-85%2070%2050%2050-35%20160%20100H0z'%20fill='%23186946'/%3E%3C/svg%3E";

/// Every value of the layout Axis, each with the image it arranges.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "layout", class: "flex flex-wrap items-start gap-4",
            for layout in CardLayout::ALL.iter().copied() {
                Card { class: "w-72 bg-base-100 shadow-sm overflow-hidden", layout,
                    figure {
                        img {
                            class: "h-36 w-full object-cover",
                            src: LANDSCAPE,
                            alt: "Stylized hills under a sun"
                        }
                    }
                    CardBody {
                        CardTitle { "{layout:?}" }
                        p { "The same image and body in each layout." }
                    }
                }
            }
        }
    }
}
