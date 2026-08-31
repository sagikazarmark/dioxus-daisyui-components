use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger, MenubarTriggerSize,
};

/// A caller's own classes, on the bar, on a trigger and on a popup.
///
/// The bar takes a surface, which is always the caller's: daisyUI has no menubar
/// to paint. The trigger takes `btn-ghost`, which is the look an application
/// menu bar usually wants and which this component does not choose for anybody.
/// The popup takes a width, which lands on the box rather than on the `menu`
/// inside it: the list is stretched to the box, so the width reaches the items
/// too.
#[component]
pub fn Example() -> Element {
    rsx! {
        Menubar { id: "caller-attributes", class: "bg-base-200 rounded-box p-1",
            MenubarMenu { index: 0usize,
                MenubarTrigger {
                    id: "caller-trigger",
                    size: MenubarTriggerSize::Sm,
                    class: "btn-ghost",
                    "View"
                }
                MenubarContent { id: "caller-popup", class: "w-56",
                    MenubarItem { index: 0usize, value: "zoom-in", "Zoom in" }
                    MenubarItem { index: 1usize, value: "zoom-out", "Zoom out" }
                }
            }

            MenubarMenu { index: 1usize,
                MenubarTrigger { size: MenubarTriggerSize::Sm, class: "btn-ghost", "Help" }
                MenubarContent { class: "w-56",
                    MenubarItem { index: 0usize, value: "docs", "Documentation" }
                }
            }
        }
    }
}
