dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A set of labels, selected several at a time and removed with a key or a button of their own.",
        },
        selection {
            title: "One at a time",
            description: "The single-selection root, with a disabled tag the arrow keys skip and daisyUI has no look for.",
        },
        colors {
            title: "Colours",
            description: "Every value of the colour axis, which daisyUI applies per badge rather than per set.",
        },
        sizes {
            description: "Every value of the size axis, which daisyUI derives a badge's height and padding from.",
        },
        appearance {
            title: "Selection, layout and the empty state",
            description: "The utilities this component emits where daisyUI has no class, and what each of them looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the tag's own, and are how selection becomes a fill rather than a ring.",
        },
    },
}
