# Kbd

A native keyboard-input element styled with daisyUI's `kbd` classes.

[Live examples](https://daisyui-components.dioxus.cc/components/kbd) ·
[their sources](docs/examples/)

`Kbd` renders a native `kbd` element. In HTML, `kbd` denotes textual user input, commonly a key
or key name. It does not create a button, enter the tab order, handle key presses or add an ARIA
role. An application shortcut remains application behaviour outside this Component.

For a multi-key shortcut, render sibling `Kbd` components and supply separators such as `+` as
ordinary caller content. Each key remains its own semantic and styled element.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. In
ADR-0023's terms this is a Presentational component: native `kbd` supplies semantics, but no
interaction, focus or ARIA state exists for a Primitive to manage.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `size: KbdSize` - `kbd-xs`, `kbd-sm`, `kbd-lg`, and `kbd-xl`.

`KbdSize` is non-exhaustive and exposes `ALL` in smallest-to-largest order for the Preview.
`KbdSize::Default` emits nothing because bare `kbd` renders at the same medium size as explicit
`kbd-md`. Every emitted class is a complete literal in this Component's source so Tailwind can
discover the Component when it is installed alone.

## Deviations

None from daisyUI: the Component reproduces its documented native `kbd.kbd` markup and size
modifiers. It wraps no Primitive because the element is noninteractive and has no state or
behaviour for one to provide.

## daisyUI classes deliberately not used

- `kbd-md` - the classless default renders at the same medium size.
- Responsive-prefixed kbd classes - callers add them through `class`, which concatenates.
