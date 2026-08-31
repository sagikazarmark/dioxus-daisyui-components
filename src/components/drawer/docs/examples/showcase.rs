use dioxus::prelude::*;

use crate::components::drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerOverlay, DrawerPanel, DrawerPlacement,
    DrawerSide, DrawerTitle,
};

/// Both placements held open in bounded drawers, so one page baseline covers
/// the complete placement axis under each theme.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "grid gap-4 lg:grid-cols-2",
            for placement in DrawerPlacement::ALL.iter().copied() {
                Drawer {
                    placement,
                    open: Some(true),
                    class: "h-72 overflow-hidden rounded-box border border-base-300",
                    style: "position: relative; isolation: isolate;",
                    DrawerContent { class: "grid place-items-center p-6",
                        span { class: "font-medium", "{placement:?} placement" }
                    }
                    DrawerSide {
                        is_modal: false,
                        style: "position: absolute; width: 100%; height: 100%; inset-inline-start: 0; inset-inline-end: 0; grid-column-start: 1;",
                        DrawerOverlay {}
                        DrawerPanel {
                            class: "min-h-full w-48 bg-primary p-5 text-primary-content",
                            style: "translate: 0%;",
                            "data-placement-showcase": "true",
                            DrawerTitle { class: "font-bold", "{placement:?} side" }
                            DrawerDescription { class: "mt-2 text-sm opacity-70",
                                "Held open for the theme baseline."
                            }
                        }
                    }
                }
            }
        }
    }
}
