dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "Two homogeneous arrays as fieldsets of item cards: string items and fixed-object items, with insert, move and remove on each card, append below, and the container's presence operations.",
        },
        schemas {
            title: "The schemas these examples share",
            description: "Written once and imported by the examples below: an optional array of strings with a seed default and a required array of fixed objects.",
        },
        composition {
            description: "The add-on applied by hand: the collection and the shell around the adapter's built-in controls, with the daisyUI presenter in both slots.",
        },
        appearance {
            description: "Every value of the appearance axis on the same arrays: Default emits the layout utilities, None leaves the fieldset, card, btn and join classes alone.",
        },
        buttons {
            description: "A host's own classes on the item actions and the append button. A role given a class drops the package's colour, size and variant; join-item and btn-square stay.",
        },
    },
}
