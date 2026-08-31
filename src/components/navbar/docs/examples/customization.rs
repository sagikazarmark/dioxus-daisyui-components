use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarStart, NavbarTrigger, NavbarTriggerSize,
};

/// Caller classes on the bar, a region, a trigger, a popup and an item.
#[component]
pub fn Example() -> Element {
    rsx! {
        Navbar { id: "caller-attributes", class: "bg-base-200 rounded-box p-1",
            NavbarStart { id: "caller-start", class: "gap-2",
                NavbarNav { index: 0usize,
                    NavbarTrigger {
                        id: "caller-trigger",
                        size: NavbarTriggerSize::Sm,
                        class: "btn-ghost",
                        "View"
                    }
                    NavbarContent { id: "caller-popup", class: "w-56",
                        NavbarItem {
                            index: 0usize,
                            value: "docs",
                            to: "/components/navbar",
                            class: "font-semibold",
                            "Documentation"
                        }
                    }
                }
            }
        }
    }
}
