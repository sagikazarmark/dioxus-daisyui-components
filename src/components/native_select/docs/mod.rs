dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A native enum picker carrying daisyUI's select class.",
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
        placeholder {
            description: "The disabled placeholder option, selected while the value is None.",
        },
        controlled {
            title: "Controlled value",
            description: "A controlled select follows external writes, before and after the user picks.",
        },
        customization {
            title: "Your own classes and attributes",
            description: "Caller classes join the component's own, and native select attributes pass through.",
        },
        form {
            title: "Native form control",
            description: "The browser owns constraint validation and form participation; explicit form values replace the positional default.",
        },
        field {
            title: "Native Select Field",
            description: "NativeSelectField composes the common label, select, description, and always-mounted error path.",
        },
        states {
            title: "Native states",
            description: "daisyUI reads the disabled state directly from the native attribute.",
        },
    },
}
