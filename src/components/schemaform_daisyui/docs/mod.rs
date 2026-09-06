dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A form bound through every seam at once: the controls as daisyUI fields, the array as a fieldset of item cards, the shell with a primary submit button, the summary as an alert.",
        },
        schemas {
            title: "The schemas these examples share",
            description: "Written once and imported by the examples below: a data schema exercising every control kind, a UI schema naming the two widget symbols, and the arrays schema.",
        },
        controls {
            description: "Every control kind and both widget symbols: inputs, the native and registry checkboxes, the replacement select of a write-only boolean, a native select, a radio group, the compound select, read-only output and a constant.",
        },
        arrays {
            description: "Two homogeneous arrays: string items and fixed-object items, with insert, move and remove on each card, append below, and the container's presence operations.",
        },
        composition {
            description: "The seams composed by hand: the daisyUI collection and shell around the adapter's built-in controls, with the daisyUI presenter in both slots.",
        },
        appearance {
            description: "Every value of the appearance axis on the same form: Default emits the package's layout utilities, None leaves daisyUI's component classes alone.",
        },
    },
}
