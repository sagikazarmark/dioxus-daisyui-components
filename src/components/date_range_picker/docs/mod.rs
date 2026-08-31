dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A run of days chosen from the calendar the button opens (two clicks, one range) shown in a field the caller writes.",
        },
        fields {
            title: "The field",
            description: "The daisyUI input's colour and size, the button inside the field, and the box the popup is positioned against.",
        },
        popup {
            title: "The popup",
            description: "Which side the calendar opens on, where it sits along it, and what draws the box and the month: one picker per value, opened by its own button.",
        },
        states {
            title: "Disabled, read-only and limited",
            description: "A picker that is off, one the calendar is the only way into, and one that accepts a fortnight with three days missing from it.",
        },
    },
}
