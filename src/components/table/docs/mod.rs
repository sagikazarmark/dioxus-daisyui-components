dioxus_registry_preview::component! {
    group: "data-display",
    examples: {
        overview {
            title: "Native table semantics",
            description: "A captioned table with valid sections, scoped and spanned headings, explicit header associations, and caller classes on every part.",
        },
        sizes {
            description: "Every value of the size Axis, from extra small through the classless medium default to extra large.",
        },
        zebra {
            title: "Zebra rows",
            description: "Both values of the zebra Axis, with striping independent of size and pinning.",
        },
        pinning {
            description: "Every row- and column-pinning value in caller-owned scroll regions, followed by both pinning modes combined.",
        },
        overflow {
            title: "Caller-owned overflow",
            description: "A wide table remains native while its caller supplies the horizontal scrolling wrapper.",
        },
    },
}
