dioxus_registry_preview::component! {
    group: "actions",
    examples: {
        overview {
            description: "A button, and the two axes daisyUI styles one along.",
        },
        colors {
            description: "Every value of the colour axis. The default emits no class, which is daisyUI's own uncoloured button rather than a synonym for neutral.",
        },
        sizes {
            description: "Every value of the size axis, smallest to largest.",
        },
        interactive {
            title: "Activation and the disabled state",
            description: "A click handler, and the same handler on a button the disabled attribute has made inert.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and a caller's attributes override them.",
        },
    },
}
