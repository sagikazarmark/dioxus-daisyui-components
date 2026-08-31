dioxus_registry_preview::component! {
    group: "overlays",
    examples: {
        overview {
            description: "A panel of content that opens when its trigger is pointed at or tabbed to, and closes again as soon as the pointer leaves it.",
        },
        placements {
            title: "Sides and alignment",
            description: "Every value of both positioning axes, which are utilities rather than daisyUI classes; see ADR-0015.",
        },
        sizes {
            title: "Sizes and borders",
            description: "daisyUI's own two axes for a card, with the panels left in the flow so that they read as a row.",
        },
        appearance {
            title: "Paint and positioning",
            description: "The utilities this component emits where daisyUI has no class, and what each of them looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the panel's own, and the parts are how the body is reached.",
        },
    },
}
