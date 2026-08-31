dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        placements {
            description: "Both required logical placements with image, header and time, bubble, and footer as direct children.",
        },
        colors {
            title: "Bubble colours",
            description: "The unclassed default and every semantic colour on bubbles at alternating placements.",
        },
        rtl {
            title: "Logical tails and RTL",
            description: "Start and end move to the opposite physical edges under RTL while each bubble tail keeps pointing outward.",
        },
        customization {
            title: "Your own classes",
            description: "Caller classes and attributes merge on every part, including raw Avatar styling on the image part.",
        },
    },
}
