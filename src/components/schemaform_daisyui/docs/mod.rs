dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A form bound through every seam at once: the controls as daisyUI fields, the topics as a checkbox group, the shell with a primary submit button, the summary as an alert.",
        },
        schemas {
            title: "The schemas these examples share",
            description: "Written once and imported by the examples below, and by the add-ons' pages: a data schema exercising every control kind, and the overview's smaller form.",
        },
        controls {
            description: "Every control kind: inputs, native checkboxes (indeterminate while they hold no boolean), the replacement select of a write-only boolean, a native select, read-only output and a constant.",
        },
        formats {
            description: "The adapter's format-to-widget mapping: email, URL, date and time inputs, with text fallbacks and password precedence for write-only values.",
        },
        multiple_choice {
            description: "A checkbox per option, with array presence actions and an incompatible-value readout. Reset shows the checkboxes following the form's node; submit shows the host's received data.",
        },
        advisory {
            description: "The host accepts data with findings in advisory mode. Switch to gated mode to see the same findings block submission and use error presentation.",
        },
        composition {
            description: "The seams composed by hand: the daisyUI shell around the adapter's built-in controls, with the daisyUI presenter in both slots.",
        },
        appearance {
            description: "Every value of the appearance axis on the same form: Default emits the package's layout utilities, None leaves daisyUI's component classes alone.",
        },
        density {
            description: "Default keeps presence operations below each field. Compact puts them beside labels, preserves explicit empty values, and wraps multiple-choice options.",
        },
        buttons {
            description: "A host's own classes on the buttons the renderers draw: presence operations and submit. A role given a class drops the package's colour, size and variant.",
        },
    },
}
