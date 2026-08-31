dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "A default badge, and three independent axes set together.",
        },
        colors {
            description: "Every value of the colour axis. The default emits no class.",
        },
        sizes {
            description: "Every value of the size axis, smallest to largest.",
        },
        appearance {
            description: "Every value of the appearance axis on one fixed colour.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and a caller's attributes override them.",
        },
    },
}
