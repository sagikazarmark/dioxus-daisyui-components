dioxus_registry_preview::component! {
    group: "actions",
    examples: {
        overview {
            description: "A row of themes to pick one of. Picking one re-declares that theme on the document root, in CSS, with nothing in Rust to run, which is what the switcher in this page's header does. The controls on this page name themes daisyUI does not ship, so they show the component without theming the page they document; the header offers every theme it does.",
        },
        appearance {
            description: "Every value of the appearance axis, which is the control daisyUI draws, and the input type its rules are written against.",
        },
        colors {
            title: "Colours",
            description: "Every value of the colour axis, in each of the four scales an appearance names.",
        },
        sizes {
            description: "Every value of the size axis, smallest to largest, in the same four scales.",
        },
        states {
            description: "On, off and disabled, all of them daisyUI reading the input's own state, and the change a page listens for when it wants to remember the choice.",
        },
        swap {
            title: "Your own control",
            description: "The axis value that emits nothing, inside daisyUI's swap: the component contributes the input, the caller draws the rest.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the component's own, and beat them where the two disagree.",
        },
    },
}
