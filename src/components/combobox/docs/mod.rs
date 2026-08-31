dioxus_registry_preview::component! {
    group: "data-input",
    // The primitive blanks the field whenever the popup opens. Keep the page
    // addressable and tested, but do not direct readers to it until upstream
    // issue DioxusLabs/dioxus-components#294 is fixed.
    listed: false,
    examples: {
        overview {
            description: "A combobox that owns its own state: the field, what is typed into it, and the list it filters.",
        },
        options {
            title: "The options these examples share",
            description: "Written once and imported by every example below, because what the examples vary is the field and the box.",
        },
        colors {
            description: "Every value of the field's colour axis, read off closed comboboxes; what it colours is the field.",
        },
        sizes {
            description: "Every value of the field's size axis, smallest to largest.",
        },
        filtering {
            description: "A query that keeps one option, and one that keeps none, which is what the empty line is for.",
        },
        highlight {
            title: "The option the keyboard is on",
            description: "The one thing the select cannot lend: focus stays in the field, so the highlight is painted from the attribute the primitive reports instead.",
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
            description: "A disabled combobox, one that starts on a value, and a width that reaches the options through the box.",
        },
        field {
            title: "Field Context",
            description: "A Binding and Field metadata drive the selected value and the input's form attributes without value or state props on the control.",
        },
        controlled {
            description: "A combobox whose open state, value and query all belong to its caller.",
        },
    },
}
