dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "Three items, one open, which is the primitive's default, where opening one closes the last.",
        },
        markers {
            description: "Every value of the marker axis, which daisyUI draws on the title from a class on the item.",
        },
        states {
            description: "Every item open at once, one of them disabled, and a tally of what the set reported.",
        },
        appearance {
            description: "All three appearance axes: the utilities this component emits where daisyUI has no class, and what switching them off leaves.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and beat daisyUI's on cascade layers.",
        },
    },
}
