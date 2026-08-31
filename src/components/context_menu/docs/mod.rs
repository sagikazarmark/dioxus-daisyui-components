dioxus_registry_preview::component! {
    group: "overlays",
    examples: {
        overview {
            description: "A surface with a menu behind it, opened by a right click or a long press and pinned where the pointer was.",
        },
        items {
            description: "The commands every menu on this page holds, written once and shared between the examples.",
        },
        sizes {
            description: "Every value of the size axis, which daisyUI applies to the items rather than to the box.",
        },
        appearance {
            title: "The box",
            description: "The utilities this component emits where daisyUI has only a position, and what they look like switched off.",
        },
        customization {
            title: "Your own classes",
            description: "A caller's classes join the box's own, and replace them where the box's are switched off.",
        },
    },
}
