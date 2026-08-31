dioxus_registry_preview::component! {
    group: "overlays",
    examples: {
        overview {
            description: "A tooltip on a button, and one on a word, both openable by pointer and by keyboard.",
        },
        colors {
            description: "Every value of the colour axis, each tooltip held open; daisyUI has no neutral class, because an unclassed tooltip already is one.",
        },
        placements {
            description: "Every value of the side axis, which places the tail as well as the bubble.",
        },
        alignments {
            description: "Every value of the align axis, on triggers wide enough to tell them apart.",
        },
        states {
            description: "A tooltip this page holds open (which is what the lifted state exists for) and one that is switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the bubble's own, and beat daisyUI's on cascade layers.",
        },
    },
}
