dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "The classless defaults, visible surrounding meaning, and caller-owned semantics.",
        },
        colors {
            description: "Every value of the colour Axis. The default emits no modifier.",
        },
        sizes {
            description: "Every value of the size Axis, from extra small through extra large.",
        },
        animations {
            title: "Caller animations",
            description: "Ping and bounce are caller-composed Tailwind utilities rather than Status state.",
        },
        customization {
            title: "Your own classes",
            description: "Caller classes and semantic attributes join the selected colour and size.",
        },
    },
}
