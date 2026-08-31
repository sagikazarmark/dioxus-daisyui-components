dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A native text field carrying daisyUI's input class.",
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
            description: "Caller classes join the component's own, and native input attributes pass through.",
        },
        floating {
            slug: "floating-label",
            title: "Floating label",
            description: "The floating-label class belongs to the Label around the Input.",
        },
        form {
            title: "Native form control",
            description: "The browser owns editing, constraint validation and form participation.",
        },
        field {
            title: "Text Field",
            description: "InputField composes the common label, input, and always-mounted error path around one Field Context.",
        },
        states {
            title: "Native states",
            description: "daisyUI reads the disabled state directly from the native attribute.",
        },
    },
}
