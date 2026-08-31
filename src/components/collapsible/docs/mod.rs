dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            description: "A title that opens a panel, and a panel that is mounted only while it is open.",
        },
        markers {
            description: "Every value of the marker axis, which daisyUI draws in the corner of the title and turns as the panel opens.",
        },
        states {
            title: "Disabled and kept mounted",
            description: "A disclosure nothing opens, and one whose children stay in the document while it is closed.",
        },
        controlled {
            description: "The open state lifted all the way out to the caller, which is what this component does internally either way.",
        },
        appearance {
            title: "Surface and title",
            description: "The utilities this component emits where daisyUI has no class, and what each of them looks like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and beat them where the two disagree.",
        },
    },
}
