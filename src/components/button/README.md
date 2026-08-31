# Button

A button styled with daisyUI's `btn` classes.

[Live examples](https://daisyui-components.dioxus.cc/components/button) ·
[their sources](docs/examples/)

`onclick` is the only event handler declared as a prop. Dioxus' `extends` reaches attributes
but never handlers, so each one has to be declared by hand; the rest are added when something
needs them rather than up front.

## State bridging

There is **no state to bridge**. daisyUI's `btn` needs no state to look finished, and there is
no primitive behind this component to express state in the first place; the button renders a
plain `button` element.

`dioxus-primitives` is still declared as a dependency, as every component declares it, because
`merge_attributes` lives there.

## Axes

- `color: ButtonColor`: `btn-neutral`, `btn-primary`, `btn-secondary`, `btn-accent`,
  `btn-info`, `btn-success`, `btn-warning`, `btn-error`.
- `size: ButtonSize`: `btn-xs`, `btn-sm`, `btn-lg`, `btn-xl`.

Both default to a value that emits nothing. `ButtonColor::Default` is daisyUI's uncoloured
button, which is a distinct look from `btn-neutral`; `ButtonSize::Default` renders at the same
size as daisyUI's explicit `btn-md`, so that class is not emitted.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render
every axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

None from daisyUI: this component reproduces daisyUI's own `button.btn` markup exactly. None
from a primitive either, because it wraps none.

## daisyUI classes deliberately not used

- `btn-outline`, `btn-dash`, `btn-soft`, `btn-ghost`, `btn-link`: daisyUI's third,
  appearance axis. Orthogonal to colour and size, and not yet exposed; it would be an
  `appearance` prop, never `style`, which collides with the global HTML attribute.
- `btn-wide`, `btn-block`, `btn-square`, `btn-circle`: shape modifiers. A caller reaches all
  four with utilities on the `class` prop, which concatenate rather than replace.
- `btn-active`: daisyUI's forced-active look. Nothing in this component owns state to key it
  on, and the caller can pass it through `class`.
- `btn-disabled`: a caller sets the `disabled` attribute instead. daisyUI's rule is
  `.btn:is(:disabled, [disabled], [aria-disabled=true])`, so the attribute already draws the
  look, and unlike the class it also makes the button inert.
- `btn-md`: the default size emits nothing, and daisyUI renders an unclassed button at exactly
  that size.
