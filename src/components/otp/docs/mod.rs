dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "One native input painted as a row of one-time-code boxes by daisyUI.",
        },
        colors {
            description: "Every value of the colour axis. The default emits no class.",
        },
        sizes {
            description: "Every value of the size axis, smallest to largest.",
        },
        appearance {
            description: "Separate boxes or daisyUI's joined row.",
        },
        customization {
            title: "Your own classes and attributes",
            description: "Caller classes style the row; global attributes identify and describe its native input.",
        },
        form {
            title: "Native form control",
            description: "The browser owns editing and form participation; completion or native change commits the interaction.",
        },
        field {
            title: "OTP Field",
            description: "OtpField composes the common label, OTP input, description, and always-mounted error path.",
        },
        states {
            title: "Native states",
            description: "The browser owns whether the real input can receive focus and edits.",
        },
    },
}
