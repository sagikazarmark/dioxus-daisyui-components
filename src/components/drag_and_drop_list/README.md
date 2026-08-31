# Drag and drop list

A list whose rows are dragged into a different order, styled with daisyUI's `list` classes and
wrapping the `dioxus-primitives` drag and drop list. The dragging, the keyboard path that does the
same thing without a pointer, the drop targeting, the live-region announcements and the
`sortable list` role description are all the primitive's rather than reimplemented here.

```rust
let items = ["Rise", "Ember", "Lantern"]
    .iter()
    .map(|track| rsx! { div { class: "list-col-grow", "{track}" } })
    .collect();

rsx! {
    DragAndDropList { items, aria_label: "Playlist" }
}
```

The rows are `Element`s rather than values, because the primitive owns the order from the first
render on: it keeps the list and renders each row wherever it has been moved to.

**daisyUI's `list` is a match with nothing to work around.** `.list` is a column, `.list-row` is a
row with a radius and a rule under it, and `.list-col-grow` says which child takes the space left
over. None of them is gated on an element type, none needs a sibling the primitive's tree does not
produce, and none reads a state. The classes go exactly where daisyUI puts them; the standalone
`list` Component exposes the same styling without this Primitive's drag behaviour.

## Composition

The compound parts are `DragAndDropList` (the wrapper), `DragAndDropListItems` (the list itself),
`DragAndDropListItem` (one row), `DragAndDropDropIndicator` (the line a row would land on),
`DragAndDropInstructions` and `DragAndDropLiveRegion`.

**Written with nothing inside them, the first two render the rest.** `DragAndDropList` renders the
instructions, the list and the live region; `DragAndDropListItems` renders a row per item with a
drop line either side of it. That is the primitive's own default composition, put together again
here so that what it renders is *this registry's* rows rather than unstyled ones, the same reason
the calendar composes its own grid (ADR-0022). The axes for the rows and the lines are therefore
props of the two parts that render them.

**The wrapper emits nothing.** daisyUI's `list` belongs on the list, not on what holds it, and what
holds it here is a wrapper around three things, two of which are invisible.

**The instructions and the live region are parts rather than something a caller remembers.** They
are what makes the list usable without a pointer: the instructions name the keys, and every move is
announced as it happens. Both are hidden by the primitive itself, with inline styles rather than a
class, so neither emits anything here.

## State bridging

**Neither tier: every state is a Bridged utility.** daisyUI has a class for a list row and none
at all for a row being carried, so there is no rule for **Tier 1** to match and no modifier class
for **Tier 2** to emit. The primitive reports each state as an attribute on the row, and each is
painted by a Tailwind variant of that attribute:

| State | Attribute | What is emitted |
| --- | --- | --- |
| the row that has been picked up | `data-is-grabbing` | `opacity-60`, `bg-base-200`, the grabbing cursor |
| the row the keyboard is on | `data-focus-visible` | `bg-base-200` |
| a row carried back to where it started | `data-drop-at-origin` | a dashed `outline-primary` |

The last of those exists because the primitive draws no drop line when the drop would leave a row
where it already is (there is no gap to point at) so the row says so instead.

Nothing is recomputed in Rust and the primitive stays the only owner of the drag.

## Axes

- `appearance: DragAndDropListAppearance` on the list: the box's fill, corners and shadow, or
  nothing.
- `appearance: DragAndDropListItemAppearance` on a row: the three states above, or nothing.
- `appearance: DragAndDropIndicatorAppearance` on the drop line: the line, or nothing.

All three are the inverted shape ADR-0004 describes: their `Default` emits and their `None` emits
nothing, because what they carry is utilities this component emits rather than daisyUI component
classes. A utility only ties with a caller's, and a tie is settled by generated-stylesheet order.

`DragAndDropList` and `DragAndDropListItems` carry the axes of the parts they render as well as
their own, for the case where nothing is written inside them.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The new order is not reported to the caller.** The primitive has no callback for it: the order
lives in a signal it owns, and the only way to read it is to look at the DOM. So a list whose order
has to be saved is not something this component can offer today; what it offers is a list whose
order a reader can change. Asserted in the browser specs as the order of the rendered rows, so the
day upstream adds a callback is a day this documentation is wrong rather than a day nobody notices.

**A row can be deleted with the keyboard.** Delete or Backspace on a focused row removes it, which
is the primitive's and is not something this component asks for or can switch off. It is worth
knowing before putting the list somewhere a stray keypress matters.

**The rows are elements, so they are rendered once.** A row that has to change with the state
around it is a component the caller writes into the element they pass, not a value this list
re-reads.

**The drop line is a `div` inside a `ul`.** That is the primitive's markup and it is invalid HTML,
strictly; a list may hold only list items. It is left alone rather than papered over: the element
is the primitive's to render, and the alternative is a wrapper that would sit between `.list` and
`.list-row` and break daisyUI's own rules for both.

## daisyUI classes deliberately not used

- `list-col-grow`, `list-col-wrap`: daisyUI's classes for which part of a row takes the space left
  over and which wraps to a second line. They belong on the caller's own content inside a row,
  which is why the examples use them and this component does not emit them.
- `menu`, `menu-active`: daisyUI's other list. It is a navigation menu: its rows are links with
  their own padding, highlight and active state, none of which a row being dragged wants.
- `table`, `table-zebra`: daisyUI's rows of data. A sortable list has one column of content per
  row rather than a grid of cells, and `.list` is the class daisyUI wrote for exactly that.
- `btn` on a grab handle: a handle is the caller's own element inside a row, so what it looks like
  is theirs. The whole row is draggable either way; a handle is a hint rather than a mechanism.
- The responsive prefixes daisyUI generates for `list` (`sm:list` and the rest): a caller reaches
  those through `class`, which concatenates.
