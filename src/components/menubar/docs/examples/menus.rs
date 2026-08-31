use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarMenuSize, MenubarTrigger,
    MenubarTriggerSize,
};

/// Every value of the popup's size axis, which sizes the items inside a menu
/// rather than the trigger above it.
///
/// One menu per value, opened one at a time: the primitive closes a menu that
/// nothing in it is focused, and offers no way to pin one open, so these are
/// reached by opening them rather than by asking for them.
///
/// `Default` emits no class and renders at the same size as daisyUI's explicit
/// `menu-md`.
#[component]
pub fn Example() -> Element {
    rsx! {
        Menubar { "data-axis-triggers": "menu-size",
            for (index , size) in MenubarMenuSize::ALL.iter().copied().enumerate() {
                MenubarMenu { index,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "{size:?}" }
                    MenubarContent { size, class: "w-40",
                        MenubarItem { index: 0usize, value: "{size:?}", "An item" }
                        MenubarItem { index: 1usize, value: "{size:?}-second", "Another" }
                    }
                }
            }
        }
    }
}
