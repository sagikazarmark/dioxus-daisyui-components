use dioxus::prelude::*;

use crate::components::drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerOverlay, DrawerPanel, DrawerSide, DrawerTitle,
    DrawerTrigger,
};

/// An uncontrolled drawer whose page remains in place while its dialog-backed
/// side opens and closes.
#[component]
pub fn Example() -> Element {
    rsx! {
        Drawer { class: "min-h-64 rounded-box border border-base-300",
            DrawerContent { class: "flex min-h-64 items-center justify-center p-6",
                DrawerTrigger { "Open navigation" }
            }
            DrawerSide {
                DrawerOverlay {}
                DrawerPanel { class: "min-h-full w-80 bg-base-200 p-6 text-base-content",
                    DrawerTitle { class: "text-xl font-bold", "Navigation" }
                    DrawerDescription { class: "mt-2 opacity-70",
                        "Choose a section of the application."
                    }
                    nav { class: "mt-6 flex flex-col gap-2",
                        a { href: "#dashboard", class: "btn btn-ghost justify-start", "Dashboard" }
                        a { href: "#settings", class: "btn btn-ghost justify-start", "Settings" }
                    }
                }
            }
        }
    }
}
