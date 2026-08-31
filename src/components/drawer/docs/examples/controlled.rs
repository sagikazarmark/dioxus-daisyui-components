use dioxus::prelude::*;

use crate::components::drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerOverlay, DrawerPanel, DrawerSide, DrawerTitle,
    DrawerTrigger,
};

/// A controlled drawer with caller classes and attributes on every part.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        Drawer {
            id: "controlled-drawer",
            class: "rounded-none",
            "data-part": "root",
            open: Some(open()),
            on_open_change: move |next| {
                open.set(next);
                changes += 1;
            },
            DrawerContent {
                id: "controlled-content",
                class: "p-1",
                "data-part": "content",
                div { class: "flex flex-wrap items-center gap-3",
                    DrawerTrigger {
                        id: "controlled-trigger",
                        class: "btn-primary",
                        "data-part": "trigger",
                        "Open account drawer"
                    }
                    span { "data-testid": "controlled-state", if open() { "open" } else { "closed" } }
                    span { "data-testid": "controlled-changes", "{changes}" }
                }
            }
            DrawerSide {
                id: "controlled-side",
                class: "z-20",
                "data-part": "side",
                DrawerOverlay {
                    id: "controlled-overlay",
                    class: "bg-secondary",
                    "data-part": "overlay",
                }
                DrawerPanel {
                    id: "controlled-panel",
                    class: "min-h-full w-72 bg-base-200 p-6 text-base-content",
                    "data-part": "panel",
                    DrawerTitle {
                        id: "controlled-title",
                        class: "text-2xl font-bold",
                        "data-part": "title",
                        "Account navigation"
                    }
                    DrawerDescription {
                        id: "controlled-description",
                        class: "mt-2 italic",
                        "data-part": "description",
                        "Links for managing your account."
                    }
                    button { class: "btn mt-6", onclick: move |_| open.set(false), "Close account drawer" }
                }
            }
        }
    }
}
