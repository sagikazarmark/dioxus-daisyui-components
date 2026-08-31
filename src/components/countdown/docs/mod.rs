dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "A fixed value and a signal-driven value updating through ordinary Dioxus reactivity.",
        },
        digits {
            description: "Every value of the digits Axis, showing natural width and leading zeroes.",
        },
        clock {
            description: "Several direct value children sharing one root with caller-provided separators.",
        },
        customization {
            description: "Clamped inputs, a descriptive label, and caller styles composed with authoritative custom properties.",
        },
    },
}
