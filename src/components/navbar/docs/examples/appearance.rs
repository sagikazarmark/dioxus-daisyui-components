use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarContent, NavbarContentAppearance, NavbarItem, NavbarNav, NavbarStart,
    NavbarTrigger, NavbarTriggerOpenAppearance, NavbarTriggerSize,
};

/// The popup's default box utilities and the value that emits none.
#[component]
pub fn Example() -> Element {
    rsx! {
        Navbar {
            NavbarStart { class: "w-full flex-wrap gap-2",
                div { "data-axis-triggers": "content", class: "flex gap-2",
                    for (index , appearance) in NavbarContentAppearance::ALL.iter().copied().enumerate() {
                        NavbarNav { index,
                            NavbarTrigger { size: NavbarTriggerSize::Sm, "Popup: {appearance:?}" }
                            NavbarContent { appearance, class: "w-44",
                                NavbarItem { index: 0usize, value: "{appearance:?}", to: "/components/navbar", "An item" }
                            }
                        }
                    }
                }

                div { "data-axis-triggers": "trigger-open", class: "flex gap-2",
                    for (offset , open_appearance) in NavbarTriggerOpenAppearance::ALL.iter().copied().enumerate() {
                        NavbarNav { index: offset + NavbarContentAppearance::ALL.len(),
                            NavbarTrigger {
                                open_appearance,
                                size: NavbarTriggerSize::Sm,
                                "Open: {open_appearance:?}"
                            }
                            NavbarContent { class: "w-44",
                                NavbarItem { index: 0usize, value: "{open_appearance:?}", to: "/components/navbar", "An item" }
                            }
                        }
                    }
                }
            }
        }
    }
}
