# Table

A semantic native table with daisyUI size, zebra, and pinning Axes.

[Live examples](https://daisyui-components.dioxus.cc/components/table) ·
[their sources](docs/examples/)

`Table`, `TableCaption`, `TableHeader`, `TableBody`, `TableFooter`, `TableRow`,
`TableHeaderCell`, and `TableCell` render `table`, `caption`, `thead`, `tbody`, `tfoot`, `tr`, `th`,
and `td` respectively. This is a Presentational component and wraps no Primitive. Its Compound
parts preserve the native elements daisyUI's selectors and browser table semantics expect.

## Native semantics

A caption is the table's accessible name and should describe what its data represents. Use
`scope="col"` or `scope="row"` for straightforward headings. More complex tables can give header
cells ids and put a space-separated `headers` list on each related data cell. Dioxus exposes
`scope`, `colspan`, and `rowspan` as typed native attributes; its current table attribute list
omits `headers`, so write that standard attribute with quoted custom-attribute syntax as shown
above. Global attributes, including ids, ARIA and data attributes, reach the native elements too.

The caller owns valid table ordering. A `caption` comes first, followed by any caller-written
`colgroup`, then an optional `thead`, one or more `tbody` groups, and an optional `tfoot`. A table
section contains `tr` rows, and each row contains `th` or `td` cells. The Component deliberately
does not repair or validate invalid nesting; browsers may move invalid table markup while parsing
it. Native `colgroup` and `col` remain ordinary caller markup because daisyUI gives neither a
table-specific class or Axis.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. Native table
semantics come from the browser, while striping and pinning are CSS presentation selected by Axes.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` combines the root's classes and attributes.

## Axes

- `size: TableSize` - `table-xs`, `table-sm`, `table-lg`, or `table-xl`.
- `zebra: TableZebra` - `table-zebra`, or nothing.
- `row_pinning: TableRowPinning` - `table-pin-rows`, or nothing.
- `column_pinning: TableColumnPinning` - `table-pin-cols`, or nothing.

All four Axes are independent, non-exhaustive enums with `ALL`. Row and column pinning can be
enabled together. Each enum's `Default` emits nothing. In particular, `TableSize::Default` renders
at daisyUI's medium size without emitting redundant `table-md`.

Pinning only supplies daisyUI's sticky positioning. It becomes visible when a caller puts the
table inside a constrained scrolling ancestor. The Component does not render that wrapper because
its dimensions and place in the surrounding layout belong to the caller.

Caller classes concatenate with `Table`'s daisyUI classes and pass through unchanged on every
other part. Responsive overflow, borders, backgrounds, active-row colour, widths and other layout
remain caller styling. For example, put `overflow-x-auto` on an outer `div`, border and background
utilities on that wrapper or table, and an active colour utility such as `bg-base-200` on a
`TableRow`.

## Deviations

None from daisyUI's documented table structure or modifiers. The parts preserve native table
elements rather than replacing table semantics with generic containers.

## daisyUI classes deliberately not used

- `table-md` - the unmodified table already has the same medium size.
- `row-hover` - daisyUI 5.7.17 still defines this row class in its source but does not document it;
  hover treatment remains caller styling rather than a typed Axis. A caller may still pass
  `class: "row-hover"` to `TableRow` deliberately.
- Responsive-prefixed table classes - callers add them through `class`.
- `overflow-x-auto` - this utility belongs on a caller-owned ancestor, not the native table.
- Border, background, active-row and sizing utilities shown in daisyUI examples - these are caller
  styling rather than Table Axes.
