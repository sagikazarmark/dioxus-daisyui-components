use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarStart, NavbarTrigger, NavbarTriggerSize,
};

/// Every value of the trigger size axis, smallest first.
#[component]
pub fn Example() -> Element {
    rsx! {
        Navbar {
            NavbarStart { "data-axis": "size", class: "w-full flex-wrap items-end gap-2",
                for (index , size) in NavbarTriggerSize::ALL.iter().copied().enumerate() {
                    NavbarNav { index,
                        NavbarTrigger { size, "{size:?}" }
                        NavbarContent { class: "w-40",
                            NavbarItem { index: 0usize, value: "{size:?}", to: "/components/navbar", "An item" }
                        }
                    }
                }
            }
        }
    }
}
