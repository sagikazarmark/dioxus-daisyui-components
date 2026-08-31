use dioxus::prelude::*;

use crate::components::drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerOverlay, DrawerPanel, DrawerPlacement,
    DrawerSide, DrawerTitle, DrawerTrigger,
};

/// Every value of the placement axis, reached one at a time on the same
/// controlled drawer.
#[component]
pub fn Example() -> Element {
    let mut placement = use_signal(DrawerPlacement::default);
    let mut open = use_signal(|| false);

    rsx! {
        Drawer {
            placement: placement(),
            open: Some(open()),
            on_open_change: move |next| open.set(next),
            DrawerContent {
                div { "data-axis-triggers": "placement", class: "flex flex-wrap gap-2",
                    for value in DrawerPlacement::ALL.iter().copied() {
                        DrawerTrigger {
                            onclick: move |_| placement.set(value),
                            "{value:?}"
                        }
                    }
                }
            }
            DrawerSide { class: "duration-1000",
                DrawerOverlay { id: "placement-overlay" }
                DrawerPanel {
                    id: "placement-panel",
                    class: "is-drawer-open:border-primary is-drawer-close:border-error min-h-full w-64 border-4 bg-base-200 p-6 text-base-content",
                    DrawerTitle { "Placed drawer" }
                    DrawerDescription { "This panel can enter from either inline edge." }
                    button { class: "btn mt-6", onclick: move |_| open.set(false), "Close placed drawer" }
                }
            }
        }
    }
}
