# Toggle group

A group of toggle buttons styled with daisyUI's `join` and `btn` classes, wrapping the
`dioxus-primitives` toggle group.

```rust
ToggleGroup { horizontal: true, allow_multiple_pressed: true,
    ToggleItem { index: 0usize, color: ToggleItemColor::Primary, aria_label: "Bold", "B" }
    ToggleItem { index: 1usize, color: ToggleItemColor::Primary, aria_label: "Italic", "I" }
    ToggleItem { index: 2usize, color: ToggleItemColor::Primary, aria_label: "Underline", "U" }
}
```

Items are identified by `index`, which is both their keyboard order and how the group names them
in the pressed set. The group is a single tab stop: focus enters it and the arrow keys move
between items, which is why the index is explicit rather than taken from the DOM.

With `allow_multiple_pressed` off (the default) the group behaves like a set of radios that
happen to look like buttons: pressing one releases the last.

## State bridging

**Tier 2** on the pressed state. daisyUI's pressed button is `btn-active`, and it matches no ARIA
attribute at all: `aria-pressed`, which the primitive sets on every item, appears nowhere in
daisyUI's stylesheet, not once in a megabyte of it. So the class is emitted from Rust as a
complete literal.

The state is **lifted** (ADR-0006), the way the select's value is. `ToggleGroup` seeds a signal
from `default_pressed`, always hands the primitive a controlled set, and intercepts the change
callback, so a controlled caller and an uncontrolled one both work and this component is the only
writer. The set then travels to the items through a context of the component's own, because the
primitive's context is private and the class belongs on the item rather than on the element that
knows the set.

The disabled state is not bridged and does not need to be: the primitive sets the `disabled`
attribute on each `button`, and daisyUI's rule is `.btn:disabled`. It is a native attribute rather
than an ARIA one, so it is not Tier 1 either; there is nothing to bridge.

The orientation is a third case again. It is not state, but daisyUI expresses it with a class and
matches nothing the primitive sets, so the group emits `join-horizontal` or `join-vertical` from
the same prop the primitive is given.

## Axes

- `color: ToggleItemColor`: `btn-neutral`, `btn-primary`, `btn-secondary`, `btn-accent`,
  `btn-info`, `btn-success`, `btn-warning`, `btn-error`.
- `size: ToggleItemSize`: `btn-xs`, `btn-sm`, `btn-lg`, `btn-xl`.

Both are the button component's axes, with the class strings duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires
every one of these classes to be a literal in the file that emits it.

Both are on the **item** rather than on the group. daisyUI's classes are per-button (a joined row
is a row of buttons that happen to be joined) so a group-wide colour would be this component
inventing an API daisyUI does not have, and a row of differently coloured items would then need
an escape hatch out of it.

`ToggleItemColor::Default` emits nothing, which is daisyUI's uncoloured button.
`ToggleItemSize::Default` emits nothing and renders at the same size as `btn-md`.

Both enums expose `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

`btn-active` is not an axis. It is what this component emits for the pressed state, and a caller
who set it themselves would be lying about the state to daisyUI while `aria-pressed` said
otherwise.

## Deviations

**Items must be direct children of the group.** daisyUI rounds a joined row's ends through
`@scope`d `:scope > :first-child` and `:scope > :last-child` rules, and pulls the shared borders
together with a negative margin on `.join-item`. The primitive renders the group's element and
each item's `button` with nothing in between, so daisyUI's canonical markup comes out unchanged,
but an item wrapped in anything is an item those selectors cannot find, and the row loses its
rounded ends and doubles its inner borders. This is the same constraint the accordion documents
for its own parts, and it is not enforceable in the type system: it is written here instead.

**The orientation class is emitted for both values.** ADR-0008 is the precedent, and here it
earns more than legibility. The primitive's group is vertical by default (up and down are the
arrow keys it listens for, and `data-orientation` says `vertical`) while an unclassed `.join` is
a row. Emitting the class means the layout and the keyboard always agree, whichever way the group
is set.

**The group's own default is a column**, which is the primitive's and not daisyUI's. A prop
declared here has to carry a default, and repeating the primitive's is the rule the registry
follows everywhere; a toolbar is `horizontal: true`.

## daisyUI classes deliberately not used

- `btn-active` as a caller-facing axis: it is what the pressed state emits, above.
- `btn-outline`, `btn-soft`, `btn-dash`, `btn-ghost`, `btn-link`, `btn-wide`, `btn-block`,
  `btn-square`, `btn-circle`: the button component's other looks, which are not exposed here for
  the same reason they are not exposed there: a caller reaches them through `class`, which
  concatenates. `btn-square` and `btn-circle` in particular are what an icon-only toolbar wants,
  and a circle inside a join is a shape daisyUI's own radii would fight.
- `join-item` on anything but an item: daisyUI's join is a general-purpose row and its item class
  is documented for inputs and selects too. This component joins toggle buttons and nothing else;
  a caller mixing controls into the same row is writing their own `join`.
- The responsive prefixes daisyUI generates for the colour and size classes (`sm:btn-lg` and the
  rest): a caller reaches those through `class`, which concatenates.
