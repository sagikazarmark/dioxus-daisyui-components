dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A plain native fieldset whose first direct legend names the group.",
        },
        fields {
            title: "Multiple labeled controls",
            description: "Caller-owned box styling around multiple controls with explicit label associations.",
        },
        native {
            title: "Native disabled group",
            description: "The browser owns form association, disabled descendants, and the first-legend exception.",
        },
        customization {
            title: "Your own classes and attributes",
            description: "Caller classes and attributes join both native Compound parts.",
        },
    },
}
