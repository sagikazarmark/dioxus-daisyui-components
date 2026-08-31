dioxus_registry_preview::component! {
    group: "data-input",
    examples: {
        overview {
            description: "A slider whose value its caller keeps, and one that keeps its own: both of them named, and both moved by the keyboard as well as by the pointer.",
        },
        colors {
            title: "Colours",
            description: "Every value of the colour axis, which daisyUI expresses as two custom properties the parts of this component are drawn from.",
        },
        sizes {
            description: "Every value of the size axis, which is one thumb size the height of everything else is derived from.",
        },
        range {
            title: "Two handles, and none",
            description: "A span between two handles, which daisyUI has no markup for at all, and a slider that refuses to move.",
        },
        field {
            title: "Slider Fields",
            description: "SliderField and RangeSliderField compose visible labels, descriptions, errors, and Field-aware controls whose thumbs share those labels as accessible names.",
        },
        appearance {
            title: "The parts",
            description: "The utilities this component draws the groove, the fill and the handle with, and what each of them looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the root's own, and the parts are how a handle is redrawn.",
        },
    },
}
