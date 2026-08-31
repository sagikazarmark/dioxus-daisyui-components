# Alert

A coloured box stating something the reader needs to know, styled with daisyUI's `alert` classes.

[Live examples](https://daisyui-components.dioxus.cc/components/alert) ·
[their sources](docs/examples/)

The component renders one `div.alert`. It does not assign `role="alert"` or any other live-region
semantics. A standalone alert is normally present when the document is loaded and is read in
document order; making every such box a live region would cause content to interrupt a screen
reader unnecessarily. A caller whose alert appears dynamically can add the appropriate role and
ARIA attributes through the component's global attributes.

This differs deliberately from the toast. A toast appears without the reader asking for it, so the
toast Primitive gives its content `role="alert"` and `aria-atomic`. That announcement behaviour
belongs to the toast's arrival, not to the shared daisyUI `alert` classes.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. There is no
alert Primitive to wrap, and none is appropriate for presentational markup with no focus, keyboard
interaction or ARIA wiring of its own.

`dioxus-primitives` is still declared as a dependency, as every component declares it, because
`merge_attributes` lives there.

## Axes

- `color: AlertColor` - `alert-info`, `alert-success`, `alert-warning`, `alert-error`.
- `appearance: AlertAppearance` - `alert-outline`, `alert-dash`, `alert-soft`.
- `direction: AlertDirection` - `alert-horizontal`, `alert-vertical`.

All three axes are orthogonal, and every enum exposes `ALL` so the Preview and browser specs render
every value. The appearance prop is daisyUI's style axis; it is not named `style` because that name
belongs to the global HTML attribute.

`AlertColor::Default` emits nothing and is daisyUI's uncoloured alert. Unlike the toast's colour,
this is an ordinary Axis a caller passes: a standalone alert has no dispatched kind from which to
derive it. The class strings are deliberately duplicated from the toast rather than shared because
cross-Component dependencies are for public composition, while Tailwind's scanner must see every
complete literal in the Component file that emits it.

`AlertAppearance::Default` emits nothing and is daisyUI's filled alert. The default direction is
`AlertDirection::Horizontal`, which emits `alert-horizontal`; both direction values are explicit so
the component's direction is carried by its class rather than inferred from an absence.

## Deviations

The component omits the `role="alert"` found in daisyUI's alert examples. That role is announcement
behaviour rather than styling, and adding it to content that is already in the document would be an
accessibility regression. Callers add live-region semantics only when their use requires them.

It wraps no Primitive because an alert has no behaviour for one to provide.

## daisyUI classes deliberately not used

- Responsive-prefixed alert classes - callers add them through `class`.
- Toast placement classes - those belong to the separate toast Component, not to an alert Axis.
