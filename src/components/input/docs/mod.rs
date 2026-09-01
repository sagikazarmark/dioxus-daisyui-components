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
        adornments {
            description: "prefix and suffix render inside the control box through the span.input wrapper.",
        },
        adorned_colors {
            slug: "adorned-colors",
            title: "Adorned colours",
            description: "Every value of the colour axis, relocated to the adorned wrapper.",
        },
        adorned_sizes {
            slug: "adorned-sizes",
            title: "Adorned sizes",
            description: "Every value of the size axis on the adorned wrapper, smallest to largest.",
        },
        adorned_slot {
            slug: "adorned-slot",
            title: "Conditional adornment",
            description: "A conditional adornment toggles content inside Some; the empty slot stays mounted and hidden.",
        },
        adorned_field {
            slug: "adorned-field",
            title: "Adorned Text Field",
            description: "InputField forwards prefix, suffix, and wrapper_attributes; box utilities move to the wrapper.",
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
