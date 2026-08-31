dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A native file picker carrying daisyUI's file-input class.",
        },
        colors {
            description: "Every value of the colour axis. The default emits no class.",
        },
        sizes {
            description: "Every value of the size axis, smallest to largest.",
        },
        appearance {
            description: "Every value of the appearance axis.",
        },
        configuration {
            title: "File configuration and your own classes",
            description: "Caller classes join the Component's own; accept and multiple configure the native picker.",
        },
        form {
            title: "Native form control",
            description: "The browser owns file selection, constraint validation and form participation.",
        },
        states {
            title: "Native states",
            description: "daisyUI reads the disabled state directly from the native attribute.",
        },
    },
}
