dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A checkbox, and the text that names it.",
        },
        colors {
            description: "Every value of the colour axis, rendered checked, which is the state daisyUI expresses colour in.",
        },
        sizes {
            description: "Every value of the size axis, smallest to largest.",
        },
        states {
            description: "Checked, unchecked and disabled, none of which emits a class, plus one checkbox this page controls.",
        },
        field {
            title: "Checkbox Field",
            description: "CheckboxField composes the common label, checkbox, and always-mounted error path around one Field Context.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and beat daisyUI's on cascade layers.",
        },
    },
}
