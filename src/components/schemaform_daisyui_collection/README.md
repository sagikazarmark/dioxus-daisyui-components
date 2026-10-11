# Schemaform DaisyUI Collection

A `schemaform_daisyui` add-on that presents homogeneous arrays as a daisyUI
fieldset of item cards, with each item's actions as a `join` of square icon
buttons and the append affordance below.

[Live examples](https://daisyui-components.dioxus.cc/components/schemaform_daisyui_collection) ·
[their sources](docs/examples/)

Install it beside `schemaform_daisyui` only when a form reaches a homogeneous
array. Without it a form keeps the adapter's built-in collection: accessible,
unstyled output that a host themes through the adapter's `schemaform-*` class
hooks. With it, add `CollectionAddon` to `SchemaformDaisyui`'s `addons`, or
apply `Addon::structure` to a structure bundle composed by hand; the
collection then takes the form's `Options`: its appearance and button classes.

## What it renders

| Node | Presentation |
| --- | --- |
| homogeneous array | `fieldset` carrying the adapter's element id, focusable for the container presence operations; `fieldset-legend`; help; the incompatible-value readout and container presence `Button`s; a dashed empty state while there are no items; the adapter-keyed item hosts in a grid; the append affordance as an outline `Button`; the adapter's live region; the local findings |
| array item | `card card-border card-sm` with `role="group"`, labelled by a title reading the item noun and its position; the insert, move up, move down, and remove affordances as a `join` of `btn-square` icon buttons whose `aria-label` is the affordance's positional accessible name and whose `title` is its label; the item's controls follow |

Every button carries its affordance id, so focus after a move lands on the same
button in the moved row. Identity, keying, focus after a mutation, and
announcements stay the adapter's: the renderer places what it is handed and
never composes affordances itself.

`DaisyuiCollectionFieldset` and `DaisyuiCollectionItem` are the two renderings
as components, for a host whose own `CollectionRenderer` keeps them for some
arrays. `ButtonClasses::item` and `ButtonClasses::append` restyle the item
actions and the append button; `join-item btn-square` stay on the actions.

A built-in fixed object rendered as an array item keeps whatever frame the
host's theme gives `.schemaform-group`; the collection knows nothing of it. A
theme that frames groups is the one to flatten them inside a card, as
schemaform's demo `forms.css` does under
`[data-schemaform-daisyui="collection-item"]`.

## State bridging

The component has no state to bridge: the core owns the array, and the adapter
owns identity, keying, and the affordances the renderer places.

## Deviations

- **Fixed prose.** The empty state's text, "Nothing here yet.", is a fixed
  English string: the localized item noun the context carries is a name, not a
  word a sentence can be built around. An item card's title is the item noun
  followed by its position (`Tags item 2`), a fixed word order.
- **Its own ids.** Beside the ids the adapter reserves, the component derives
  `{element_id}-legend` and `{row_id}-title` from the adapter's ids.
- **It is an add-on.** It builds on `schemaform_daisyui`'s presence buttons,
  icon, and button classes, which the base shares as crate-visible items
  rather than public API (ADR-0035).

## daisyUI classes deliberately not used

- `tooltip` on the item action buttons: their full name is already exposed
  through `aria-label`, and the visible label through `title`; a tooltip would
  add a third, hover-only copy.
- `list`: an item is a group of controls, not a row of text, and a card frames
  that group as daisyUI intends.
