dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "First, middle, and last connector arrangements with content on both sides of the line.",
        },
        directions {
            description: "Every value of the direction Axis, with horizontal and vertical connector geometry.",
        },
        compact {
            description: "Every value of the compact Axis, moving start content onto the common side.",
        },
        snap {
            title: "Snapped markers",
            description: "Every value of the snap Axis, centring markers or moving them toward the item start.",
        },
        content_appearance {
            title: "Content boxes",
            description: "Every shared content appearance on both the start and end parts.",
        },
        line_colors {
            title: "Caller-owned line colours",
            description: "Background utilities paint individual connector segments; the Component owns no colour Axis.",
        },
        accessibility {
            title: "Chronology and semantics",
            description: "DOM order remains chronological while decorative connectors and marker icons stay silent.",
        },
        customization {
            title: "Your own classes",
            description: "Caller classes and attributes merge on all six parts, including connector styling.",
        },
    },
}
