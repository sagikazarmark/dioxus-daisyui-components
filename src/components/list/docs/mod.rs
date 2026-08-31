dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "One row with a growing title, and another with a description wrapped below it.",
        },
        grow {
            title: "Growing columns",
            description: "Every value of the grow Axis. The default leaves the remaining width to the second child.",
        },
        wrap {
            title: "Wrapping columns",
            description: "Every value of the wrap Axis, either in the first grid row or below it.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join every part's own, including both column modifiers.",
        },
    },
}
