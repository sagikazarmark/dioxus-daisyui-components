dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "The explicit end/top default, fused with locally written badge classes.",
        },
        positions {
            description: "All nine combinations of the independent inline and block Axes, decorating one container.",
        },
        content {
            title: "Caller content",
            description: "Locally styled badge, status, text, and control content with no Registry cross-Component imports.",
        },
        rtl {
            title: "Logical edges and RTL",
            description: "Inline start and end mirror with writing direction; the explicit default remains logical.",
        },
        customization {
            title: "Your own classes",
            description: "Caller classes and attributes join both parts' own, with multiple items on one container.",
        },
    },
}
