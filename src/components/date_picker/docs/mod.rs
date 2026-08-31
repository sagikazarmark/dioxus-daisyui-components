dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A date typed into a field a segment at a time, or chosen from the calendar the button opens.",
        },
        fields {
            title: "The field",
            description: "The daisyUI input's colour and size, and the axes on the segments, the button inside the field and the box the popup is positioned against.",
        },
        popup {
            title: "The popup",
            description: "Which side the calendar opens on, where it sits along it, and what draws the box and the month: one picker per value, opened by its own button.",
        },
        states {
            title: "Disabled, read-only and limited",
            description: "A picker that is off, one the calendar is the only way into, and one that accepts a fortnight with a week missing from it.",
        },
    },
}
