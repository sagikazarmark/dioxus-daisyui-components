# Card

A daisyUI-styled card: a titled box with a body and a row of actions.

[Live examples](https://daisyui-components.dioxus.cc/components/card) ·
[their sources](docs/examples/)

This is a Presentational component and wraps no Primitive. Its Compound parts mirror daisyUI's
documented markup rather than a Primitive tree: `Card`, `CardBody`, `CardTitle` and `CardActions`
exist because daisyUI styles `.card`, `.card-body`, `.card-title` and `.card-actions` as separate
elements. A `figure` remains caller markup because daisyUI gives it no card-specific structural
class.

`CardTitle` renders daisyUI's canonical `h2`. Callers that need a different heading level can put
their own heading inside `CardBody` and apply `card-title` themselves.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. A card has
no focus, keyboard interaction or ARIA wiring for a Primitive to provide.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `size: CardSize`: `card-xs`, `card-sm`, `card-lg`, `card-xl`.
- `border: CardBorder`: `card-border`, `card-dash`.
- `layout: CardLayout`: `card-side`, `image-full`.

All three Axes are orthogonal. Each enum's `Default` emits nothing, and each enum exposes `ALL` so
the Preview and browser specs render every value. `CardSize::Default` renders identically to
explicit `card-md`; `CardBorder::Default` is borderless; and `CardLayout::Default` puts an image
above the body.

The size and border class strings are deliberately duplicated from `hover_card` rather than
shared. Cross-Component dependencies are for public composition, while Tailwind's scanner must see
every complete class literal in the Component file that emits it.

Caller classes concatenate on all four parts. Width, fill, colour and shadow remain caller
utilities rather than Card Axes.

## Deviations

None from daisyUI: the parts reproduce daisyUI's documented card structure. The Component wraps
no Primitive because a card has no behaviour for one to provide.

## daisyUI classes deliberately not used

- `card-md`: the default size emits nothing, and daisyUI renders an unclassed card at exactly
  that size.
- Responsive-prefixed card classes: callers add them through `class`.
- Width, background and shadow utilities from daisyUI's examples: these are caller styling, not
  classes or Axes owned by the card.
