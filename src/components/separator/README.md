# Separator

A separator styled with daisyUI's `divider` classes, wrapping the `dioxus-primitives` separator.

```rust
Separator { color: SeparatorColor::Primary, "OR" }

div { class: "flex",
    p { "Before" }
    Separator { horizontal: false }
    p { "After" }
}
```

Children are optional and land between the two halves of the rule, which is where daisyUI puts a
divider's text. The primitive's `horizontal` and `decorative` props are passed through.

## State bridging

**Tier 2** on the orientation, which is the only thing about a separator that varies. The
primitive reports it twice (`aria-orientation` on the element and `data-orientation` beside it)
and daisyUI matches neither: it has classes of its own, `divider-horizontal` and
`divider-vertical`, and no attribute selectors at all. So the class is emitted from Rust, as a
complete literal, from the same prop the primitive is given.

There is no other state. A separator is never open, checked, selected or disabled, so this is the
whole of the bridging.

## Axes

- `color: SeparatorColor`: `divider-neutral`, `divider-primary`, `divider-secondary`,
  `divider-accent`, `divider-info`, `divider-success`, `divider-warning`, `divider-error`.
- `placement: SeparatorPlacement`: `divider-start`, `divider-end`.

`SeparatorColor::Default` emits nothing, which is daisyUI's own rule colour: the page's text
colour mixed down to a tenth. `SeparatorPlacement::Default` emits nothing and keeps both halves
of the rule, which is content in the middle.

Both enums expose `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

Orientation is not an axis. It is the primitive's `horizontal` prop, passed through: the
primitive puts it in the accessibility tree and this component puts it in the class attribute,
from one value rather than two.

## Deviations

**daisyUI's orientation names are the inverse of the primitive's, and the component keeps the
primitive's.** ARIA names a separator after the line it draws: `aria-orientation="horizontal"` is
a horizontal rule between two stacked sections. daisyUI names a divider after the layout it sits
in: `divider-horizontal` is the one for a horizontal row of content, and it draws a **vertical**
rule down the middle of it.

So the mapping crosses over:

| `horizontal` | the line | `aria-orientation` | the daisyUI class |
| ------------ | -------- | ------------------ | ----------------- |
| `true`       | across   | `horizontal`       | `divider-vertical` |
| `false`      | down     | `vertical`         | `divider-horizontal` |

The prop follows the primitive rather than daisyUI, because the prop is also what lands in the
accessibility tree, and a prop that announced the opposite of what it emitted would be a bug
whichever way it was documented. What this costs is that a reader arriving from daisyUI's
documentation, where `divider-horizontal` is written on the element in a row, has to write
`horizontal: false` instead. It is written out here rather than smoothed over, since the two
readings differ by exactly ninety degrees and nothing about the rendered result says which one
you asked for.

**A class is emitted for both orientations**, following ADR-0008. `divider-vertical` restates
what an unclassed `divider` already does, so it changes nothing on its own; what it buys is that
the orientation is legible on the element rather than inferred from an absence, and that both
values of a single prop can be shown to apply.

It costs one thing worth knowing. daisyUI's own answer to a layout that changes at a breakpoint
is a responsive prefix (`divider lg:divider-horizontal`) and a caller writing that now meets a
`divider-vertical` this component already emitted. The caller still wins: Tailwind generates
responsive variants after the base utilities they vary, so the prefixed class is later in the
stylesheet and settles the tie in the caller's favour. That is a source-order argument rather
than a specificity one (ADR-0004), which is why the preview renders exactly that case and the
browser specs assert it, rather than the documentation asserting it alone.

## daisyUI classes deliberately not used

- Nothing in daisyUI's divider is left out: every colour, both placements and both orientations
  are reachable.
- The responsive prefixes daisyUI generates for all of them (`lg:divider-horizontal` and the
  rest): a caller reaches those through `class`, which concatenates, and the deviation above is
  what makes that work.
