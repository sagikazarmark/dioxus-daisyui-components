dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "A default card, and three independent Axes set together.",
        },
        sizes {
            description: "Every value of the size Axis, smallest to largest.",
        },
        borders {
            description: "Every value of the border Axis. The default emits no class.",
        },
        layouts {
            description: "Every value of the layout Axis, each with the image it arranges.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join every part's own, and a caller's attributes override them.",
        },
    },
}
