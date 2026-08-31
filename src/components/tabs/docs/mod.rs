dioxus_registry_preview::component! {
    group: "navigation",
    examples: {
        overview {
            description: "A trigger and the panel it reveals are adjacent siblings, both direct children of the set: daisyUI's own tabs markup (ADR-0003).",
        },
        appearances {
            description: "Every value of the appearance axis. Three of the four need the panel to be the active tab's sibling, which is what the tree is arranged around.",
        },
        sizes {
            description: "Every value of the size axis, as tab bars with no panels under them, which is a shape daisyUI ships as well as the height the axis sets.",
        },
        panels {
            title: "Panel surface",
            description: "The panel's own appearance axis, whose switched-off value leaves the surface to the caller.",
        },
        controlled {
            description: "A set whose active value belongs to its caller, with keyboard navigation and a disabled tab.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the set's own, and beat daisyUI's on cascade layers.",
        },
    },
}
