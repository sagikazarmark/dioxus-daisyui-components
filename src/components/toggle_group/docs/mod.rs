dioxus_registry_preview::component! {
    group: "actions",
    examples: {
        overview {
            description: "A formatting toolbar, and a group where only one item can be on.",
        },
        colors {
            description: "Every value of the colour axis, which is the button's own and lives on the item.",
        },
        sizes {
            description: "Every value of the size axis, which a joined row survives being mixed on.",
        },
        orientation {
            description: "A row and a column: one prop deciding both the layout and the arrow keys.",
        },
        states {
            description: "Pressed, unpressed and disabled (one of which needs a class of ours) plus a group this page controls.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, on the group and on the items.",
        },
    },
}
