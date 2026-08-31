dioxus_registry_preview::component! {
    group: "overlays",
    examples: {
        overview {
            description: "A menu that owns its own open state: a dropdown, its trigger and the box its items sit in.",
        },
        items {
            title: "The items these examples share",
            description: "Written once and imported by every example below, because what the examples vary is the box rather than the items.",
        },
        placements {
            description: "Every value of the side axis, held open by the caller; the primitive closes a menu that nothing in it is focused.",
        },
        alignments {
            description: "Every value of the align axis, which moves a menu along the side it opened on.",
        },
        sizes {
            description: "Every value of the menu's size axis, which sizes the items rather than the box.",
        },
        appearance {
            title: "Box appearance",
            description: "The switched-off value leaves the menu positioned by daisyUI and painted by nobody.",
        },
        controlled {
            description: "A menu whose open state belongs to its caller, with keyboard navigation, dismissal and selection.",
        },
        customization {
            title: "Your own classes and trigger",
            description: "A width that reaches the items through the box, and a trigger element the caller renders.",
        },
    },
}
