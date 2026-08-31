use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarStart, NavbarTrigger, NavbarTriggerColor,
};

/// Every value of the trigger colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        Navbar {
            NavbarStart { "data-axis": "color", class: "w-full flex-wrap gap-2",
                for (index , color) in NavbarTriggerColor::ALL.iter().copied().enumerate() {
                    NavbarNav { index,
                        NavbarTrigger { color, "{color:?}" }
                        NavbarContent { class: "w-40",
                            NavbarItem { index: 0usize, value: "{color:?}", to: "/components/navbar", "An item" }
                        }
                    }
                }
            }
        }
    }
}
