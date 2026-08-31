dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "Three options and the text that names each, with one chosen to begin with.",
        },
        colors {
            description: "Every value of the colour axis, in one group with nothing chosen, which is the state a row of colours can be read in.",
        },
        states {
            description: "Chosen, unchosen and disabled, none of which emits a class, in a group this page controls.",
        },
        field {
            title: "Field Context",
            description: "A Binding and Field metadata drive the group, while focus requests return to its roving item.",
        },
        orientation {
            description: "Which arrow keys move the selection, and the layout that follows from the same prop.",
        },
        appearance {
            title: "Laying the group out",
            description: "Both values of the group's appearance axis: the utilities this component emits, and a caller's own layout in their place.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and beat daisyUI's on cascade layers.",
        },
    },
}
