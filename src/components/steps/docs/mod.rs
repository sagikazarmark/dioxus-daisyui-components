dioxus_registry_preview::component! {
    group: "navigation",
    examples: {
        overview {
            description: "An ordered process using generated counters and caller-owned current-step semantics.",
        },
        directions {
            description: "Every value of the direction Axis, with steps kept as adjacent list items.",
        },
        colors {
            description: "Every value of the colour Axis, repeated so each connector carries the same colour.",
        },
        content {
            title: "Custom markers",
            description: "A data-content marker and a direct StepIcon replacing daisyUI's generated counter.",
        },
        completion {
            title: "Completion semantics",
            description: "The caller supplies meaningful status text and marks only the current item with aria-current.",
        },
        customization {
            title: "Your own classes and attributes",
            description: "Caller classes, data-content, aria-current and other attributes survive on all three parts.",
        },
    },
}
