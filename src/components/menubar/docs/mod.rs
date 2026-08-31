dioxus_registry_preview::component! {
    group: "navigation",
    examples: {
        overview {
            description: "Two menus in a bar that is one tab stop, walked with the arrow keys and closed with Escape.",
        },
        colors {
            description: "Every value of the trigger colour axis, which is the button's own because the bar is a row of buttons.",
        },
        sizes {
            description: "Every value of the trigger size axis, smallest first.",
        },
        menus {
            title: "Menu sizes",
            description: "Every value of the popup's size axis, which sizes the items inside a menu; open them one at a time.",
        },
        states {
            title: "Disabled",
            description: "A menu nothing opens, a bar that is disabled outright, and an item the keyboard skips.",
        },
        appearance {
            description: "Both sets of utilities this component emits where daisyUI has no class, and what each looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, on the bar, on a trigger and on a popup.",
        },
    },
}
