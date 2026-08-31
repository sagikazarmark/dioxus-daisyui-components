dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A select that owns its own state: the field, the value it shows and the list under it.",
        },
        options {
            title: "The options these examples share",
            description: "Written once and imported by every example below, because what the examples vary is the field and the box.",
        },
        colors {
            description: "Every value of the trigger's colour axis, read off closed selects; what it colours is the field.",
        },
        sizes {
            description: "Every value of the trigger's size axis, smallest to largest.",
        },
        placeholder {
            description: "Whether a placeholder is faded apart from a chosen value, which is a utility this component emits and a caller can switch off.",
        },
        placements {
            description: "Every value of the side axis, held open by the caller.",
        },
        alignments {
            description: "Every value of the align axis, which moves a popup along the side it opened on.",
        },
        list {
            title: "List size and appearance",
            description: "The list's own two axes: the size that sizes the options, and the appearance that paints the box.",
        },
        states {
            title: "Disabled, chosen, and your own classes",
            description: "A disabled select, one that starts on a value, and a width that reaches the options through the box.",
        },
        controlled {
            description: "A select whose open state and value both belong to its caller, with keyboard navigation and typeahead.",
        },
        field {
            title: "Field Context",
            description: "A Binding and Field metadata drive the select through its root and trigger parts.",
        },
    },
}
