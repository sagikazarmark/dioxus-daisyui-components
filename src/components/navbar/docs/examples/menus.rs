use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarContent, NavbarItem, NavbarMenuSize, NavbarNav, NavbarStart, NavbarTrigger,
    NavbarTriggerSize,
};

/// Every value of the popup menu's size axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        Navbar {
            NavbarStart { "data-axis-triggers": "menu-size", class: "w-full flex-wrap gap-2",
                for (index , size) in NavbarMenuSize::ALL.iter().copied().enumerate() {
                    NavbarNav { index,
                        NavbarTrigger { size: NavbarTriggerSize::Sm, "{size:?}" }
                        NavbarContent { size, class: "w-40",
                            NavbarItem { index: 0usize, value: "{size:?}", to: "/components/navbar", "An item" }
                            NavbarItem { index: 1usize, value: "{size:?}-second", to: "/components/navbar", "Another" }
                        }
                    }
                }
            }
        }
    }
}
