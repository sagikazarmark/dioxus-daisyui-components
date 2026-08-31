dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A native multi-line text field carrying daisyUI's textarea class.",
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
        customization {
            title: "Your own classes and attributes",
            description: "Caller classes join the component's own, and native textarea attributes pass through.",
        },
        form {
            title: "Native form control",
            description: "The browser owns editing, constraint validation and form participation.",
        },
        field {
            title: "Textarea Field",
            description: "TextareaField composes the common label, textarea, description, and always-mounted error path.",
        },
        states {
            title: "Native states",
            description: "daisyUI reads the disabled state directly from the native attribute.",
        },
    },
}
