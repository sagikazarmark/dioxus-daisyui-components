dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A month of days that holds its own choice: click one, walk them with the arrow keys, change the month from the header.",
        },
        dates {
            title: "The dates these examples share",
            description: "The month, the date drawn as today and the date chosen, pinned once, because a calendar that read the clock would be a different screenshot every day.",
        },
        range {
            title: "A range",
            description: "The same calendar choosing a run of days, with the two ends and the middle painted apart.",
        },
        days {
            description: "The day's size axis, and whether a day is painted from what it is at all.",
        },
        navigation {
            title: "The month buttons",
            description: "The daisyUI button's colour, size and look, on the two buttons that change the month.",
        },
        appearance {
            title: "The box",
            description: "The utilities that draw the box around a month, and the value that emits none of them.",
        },
        layout {
            title: "The parts that lay a month out",
            description: "The view, the navigation row, the title and the grid, each drawn with utilities and each switchable off.",
        },
        months {
            title: "Two months",
            description: "A second view is a second month, offset from the first and moved by the same buttons.",
        },
        states {
            title: "Unavailable and disabled",
            description: "A week that cannot be chosen, a range the calendar cannot navigate out of, and a calendar that is off altogether.",
        },
        controlled {
            description: "A calendar whose chosen date and month on show both belong to its caller.",
        },
    },
}
