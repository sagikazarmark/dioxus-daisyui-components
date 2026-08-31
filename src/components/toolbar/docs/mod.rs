dioxus_registry_preview::component! {
    group: "navigation",
    examples: {
        overview {
            description: "Two groups of controls with a rule between them, and the arrow keys moving along the row.",
        },
        colors {
            description: "Every value of the button colour axis, which is on the control rather than on the toolbar.",
        },
        sizes {
            description: "Every value of the button size axis, smallest first.",
        },
        separators {
            description: "Every value of the separator colour axis, and a rule that turns with the toolbar it is in.",
        },
        states {
            title: "Disabled",
            description: "One control the arrow keys stop at, and a toolbar that is disabled outright, neither of which emits a class.",
        },
        appearance {
            description: "The layout utilities this component emits where daisyUI has no class, and what the row looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and the layout utilities switch off for daisyUI's join.",
        },
    },
}
