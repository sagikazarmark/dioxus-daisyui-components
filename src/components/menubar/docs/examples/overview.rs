use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger, MenubarTriggerSize,
};

/// An application menu bar: two menus, each a trigger and a popup.
///
/// The bar is one tab stop. Left and right move along it, down opens a menu and
/// steps into it, Escape closes it again, and an item reports its value and
/// closes the menu when it is chosen.
///
/// Every menu states its `index` and every item states one within its menu,
/// because the order is the keyboard's rather than the DOM's.
#[component]
pub fn Example() -> Element {
    let mut chosen = use_signal(|| String::from("nothing yet"));

    rsx! {
        div { class: "flex flex-col gap-3",
            Menubar { id: "overview",
                MenubarMenu { index: 0usize,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "File" }
                    MenubarContent { class: "w-44",
                        MenubarItem {
                            index: 0usize,
                            value: "new".to_string(),
                            on_select: move |value: String| chosen.set(value),
                            "New"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "open".to_string(),
                            on_select: move |value: String| chosen.set(value),
                            "Open"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "save".to_string(),
                            on_select: move |value: String| chosen.set(value),
                            "Save"
                        }
                    }
                }

                MenubarMenu { index: 1usize,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "Edit" }
                    MenubarContent { class: "w-44",
                        MenubarItem {
                            index: 0usize,
                            value: "cut".to_string(),
                            on_select: move |value: String| chosen.set(value),
                            "Cut"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "copy".to_string(),
                            on_select: move |value: String| chosen.set(value),
                            "Copy"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "paste".to_string(),
                            disabled: true,
                            on_select: move |value: String| chosen.set(value),
                            "Paste"
                        }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                "Chosen: "
                span { "data-testid": "chosen", "{chosen}" }
            }
        }
    }
}
