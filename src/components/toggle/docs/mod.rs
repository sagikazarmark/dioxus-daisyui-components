dioxus_registry_preview::component! {
    group: "actions",
    examples: {
        overview {
            description: "Three buttons that stay pressed, one of them pressed to start with.",
        },
        colors {
            description: "Every value of the colour axis, rendered pressed, which is the state daisyUI darkens.",
        },
        sizes {
            description: "Every value of the size axis, smallest first.",
        },
        states {
            description: "Pressed and released, which differ in a class, and disabled, which does not.",
        },
        controlled {
            description: "The pressed state lifted all the way out to the caller, which is what this component does internally either way.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and the icon toggle daisyUI's swap is left to the caller for.",
        },
    },
}
