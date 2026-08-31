# List

A daisyUI-styled list: rows of content in a vertical stack.

```rust
List {
    ListRow {
        div { "01" }
        img { src: "/cover.jpg", alt: "", class: "size-10 rounded-box" }
        ListColumn { grow: ListColumnGrow::Grow,
            div { class: "font-semibold", "Aster" }
            div { class: "text-xs uppercase opacity-60", "Field notes" }
        }
        button { class: "btn btn-ghost", "Open" }
    }
    ListRow {
        img { src: "/cover.jpg", alt: "", class: "size-10 rounded-box" }
        div { "Lantern" }
        ListColumn { wrap: ListColumnWrap::Wrap,
            "A description on its own line below the row."
        }
        button { class: "btn btn-ghost", "Open" }
    }
}
```

This is a Presentational component and wraps no Primitive. Its Compound parts follow daisyUI's
documented markup: `List` renders the `ul.list`, `ListRow` renders each `li.list-row`, and
`ListColumn` renders caller content as a direct child of the row.

`ListColumn` exists because daisyUI puts `list-col-grow` and `list-col-wrap` on a row's children,
not on `list-row`. Attributes passed to `ListRow` can reach only the `li`; they cannot put a class
on one of its caller-owned children. The column part provides that boundary while keeping the
modifier on the direct child daisyUI's selectors expect. It renders a `div`; callers that need a
different element can write that child directly and apply the daisyUI class themselves.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. A list has
no focus, keyboard interaction or ARIA wiring for a Primitive to provide.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `grow: ListColumnGrow` on `ListColumn`: `list-col-grow`, or nothing. daisyUI gives the second
  child the remaining width by default; this Axis moves that width to the marked child.
- `wrap: ListColumnWrap` on `ListColumn`: `list-col-wrap`, or nothing. The marked child moves to a
  grid row below the other children.

The Axes are independent because they change different parts of daisyUI's grid and both classes
may be put on one child. Each enum's `Default` emits nothing, and each enum exposes `ALL` so the
Preview and browser specs render every value. Caller classes concatenate on all three parts.

The `list`, `list-row`, `list-col-grow` and `list-col-wrap` class literals are deliberately
duplicated from `drag_and_drop_list` rather than shared. Registry Components are independently
installed and Tailwind's scanner must see the complete literals in the Component that emits them.
The drag and drop Component has a Recomposed default because its Primitive's collapsed row cannot
receive classes on what it renders inside. This standalone list has no Primitive or collapsed
composition, so callers write its rows and columns directly and no Recomposed default is needed.

## Deviations

None from daisyUI: the parts reproduce its documented list structure and put each modifier on a
direct row child. The Component wraps no Primitive because a presentational list has no behaviour
for one to provide.

## daisyUI classes deliberately not used

- Responsive-prefixed list classes such as `sm:list`: callers add them through `class`.
- `menu`, `menu-active`: daisyUI's other list is interactive navigation, not a vertical stack of
  arbitrary content.
- `table`, `table-zebra`: those classes describe tabular rows and columns rather than list items.
