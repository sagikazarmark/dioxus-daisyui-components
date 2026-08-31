use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarCenter, NavbarContent, NavbarEnd, NavbarItem, NavbarNav, NavbarStart,
    NavbarTrigger, NavbarTriggerSize,
};

/// A navigation bar with start, center and end regions, links and dropdowns.
#[component]
pub fn Example() -> Element {
    rsx! {
        Navbar { aria_label: "Primary", class: "bg-base-200 rounded-box",
            NavbarStart {
                NavbarNav { index: 0usize,
                    NavbarTrigger { size: NavbarTriggerSize::Sm, "Inputs" }
                    NavbarContent { class: "w-44",
                        NavbarItem { index: 0usize, value: "calendar", to: "/components/navbar", "Calendar" }
                        NavbarItem { index: 1usize, value: "slider", disabled: true, to: "/components/navbar", "Slider" }
                        NavbarItem { index: 2usize, value: "checkbox", to: "/components/navbar", "Checkbox" }
                    }
                }
            }

            NavbarCenter {
                NavbarItem {
                    index: 1usize,
                    value: "home",
                    to: "/components/navbar",
                    class: "btn btn-ghost btn-sm",
                    "Home"
                }
            }

            NavbarEnd {
                NavbarNav { index: 2usize,
                    NavbarTrigger { size: NavbarTriggerSize::Sm, "Information" }
                    NavbarContent { class: "w-44",
                        NavbarItem { index: 0usize, value: "tabs", to: "/components/navbar", "Tabs" }
                        NavbarItem { index: 1usize, value: "toast", to: "/components/navbar", "Toast" }
                    }
                }
            }
        }
    }
}
