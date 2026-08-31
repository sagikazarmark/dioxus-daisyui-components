use dioxus::prelude::*;

use crate::components::navbar::{
    Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarStart, NavbarTrigger, NavbarTriggerSize,
};

/// A disabled dropdown, a disabled navbar and an item the keyboard skips.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            Navbar { id: "states",
                NavbarStart { class: "gap-2",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { size: NavbarTriggerSize::Sm, "Open me" }
                        NavbarContent { class: "w-40",
                            NavbarItem { index: 0usize, value: "first", to: "/components/navbar", "An item" }
                            NavbarItem { index: 1usize, value: "second", disabled: true, to: "/components/navbar", "A disabled item" }
                        }
                    }
                    NavbarNav { index: 1usize, disabled: true,
                        NavbarTrigger { size: NavbarTriggerSize::Sm, "Disabled menu" }
                        NavbarContent { class: "w-40",
                            NavbarItem { index: 0usize, value: "never", to: "/components/navbar", "Never reached" }
                        }
                    }
                    NavbarItem {
                        index: 2usize,
                        value: "disabled-link",
                        disabled: true,
                        to: "/components/button",
                        class: "btn btn-sm",
                        "Disabled link"
                    }
                }
            }

            Navbar { id: "disabled", disabled: true,
                NavbarStart { class: "gap-2",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { size: NavbarTriggerSize::Sm, "File" }
                        NavbarContent { class: "w-40",
                            NavbarItem { index: 0usize, value: "new", to: "/components/navbar", "New" }
                        }
                    }
                    NavbarNav { index: 1usize,
                        NavbarTrigger { size: NavbarTriggerSize::Sm, "Edit" }
                        NavbarContent { class: "w-40",
                            NavbarItem { index: 0usize, value: "cut", to: "/components/navbar", "Cut" }
                        }
                    }
                    NavbarItem {
                        index: 2usize,
                        value: "disabled-navbar-link",
                        to: "/components/button",
                        class: "btn btn-sm",
                        "Disabled navbar link"
                    }
                }
            }
        }
    }
}
