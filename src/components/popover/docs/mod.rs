dioxus_registry_preview::component! {
    group: "overlays",
    examples: {
        overview {
            description: "A trigger, and a panel of whatever the caller wrote, with the dismissal a details element has none of.",
        },
        placements {
            description: "Every value of the side axis, one popover per value and all of them held open.",
        },
        alignments {
            description: "Every value of the align axis, which is where the panel sits along the side it opened on.",
        },
        modality {
            title: "Modal and not",
            description: "Where the keyboard can go while the panel is open, which is the one thing is_modal decides.",
        },
        controlled {
            description: "The open state lifted all the way out to the caller, which is what this component does internally either way.",
        },
        appearance {
            description: "The box utilities this component emits where daisyUI has no class, and the panel with them switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, on the trigger and on the panel alike.",
        },
    },
}
