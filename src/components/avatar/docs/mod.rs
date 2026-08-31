dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "A portrait, the initials shown when there is none, and the name every avatar needs.",
        },
        status {
            description: "Every value of the status axis, which daisyUI draws as a dot in the corner.",
        },
        states {
            description: "Loaded, empty and failed, two of which carry daisyUI's placeholder class, mirrored off the primitive's own state.",
        },
        appearance {
            title: "Frame and placeholder",
            description: "The utilities this component emits where daisyUI has no class, and what each of them looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, on the frame, including a size smaller than the one it emits.",
        },
    },
}
