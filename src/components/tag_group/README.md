# Tag group

A group of tags styled with daisyUI's `badge` classes, wrapping the `dioxus-primitives` tag
group: the chips a filter bar, a label list or a token field is made of. They take focus, move
under the arrow keys, select with Enter and Space, clear on Escape, and remove with Delete or
with a button of their own, none of which daisyUI's badge has, since a badge is a decoration.

[Live examples](https://daisyui-components.dioxus.cc/components/tag_group) ·
[their sources](docs/examples/)

## Composition

The compound parts are `TagGroup` and `TagGroupMulti` (the roots, one selection or several),
`TagGroupLabel` (what the set is called), `TagList` (the tags themselves), `TagOption` (one tag),
`TagRemoveButton` (the control that removes the tag it is written in) and `TagGroupEmpty` (what
the list says once every tag is gone). There is no collapsed component: caller content goes in the
label, in every tag and in the empty state, so there is nothing legal to collapse.

`index` orders the keyboard navigation and is also the row number a tag announces. It is the
primitive's prop and is required.

**Writing a remove button is what makes a tag removable.** The primitive counts the buttons
mounted inside a tag and turns Delete and Backspace on for it while there is one, so removability
is a fact about the markup rather than a prop that could disagree with it.

The value type travels through the parts as the primitive's generic. `TagGroup::<String>` and its
`TagOption::<String>` must agree; the primitive panics on a mismatch, and it says so.

## State bridging

Neither tier applies, and the reason is worth the sentence: **Tier 1** would need daisyUI to match
what the primitive sets, and **Tier 2** would need daisyUI to have a class to emit. daisyUI's badge
has colours, sizes and styles and no selected state at all (its own filter examples swap classes
in the framework around it) so there is nothing to bridge to.

What is emitted instead is a **Bridged utility**: a Defeatable one (ADR-0004) written as a variant
of the attribute the primitive already sets: `data-[selected=true]:ring-2` and the three utilities
that go with it. The class never changes, no state reaches Rust, and the primitive stays the only
thing that knows what is selected. It is the arrangement Tier 1 describes (a rule that matches
the primitive's own attribute) with the rule emitted by this registry rather than found in
daisyUI.

The ring is painted in `--badge-color`, which is the custom property daisyUI's own colour classes
set, so a selected tag is ringed in its own colour rather than in one this registry chose. An
uncoloured tag falls back to `currentColor`, which is what daisyUI borders one with in the same
case.

The focus ring is emitted the same way and for the same reason: a badge is not usually anything
you can focus, so daisyUI wrote no `:focus-visible` rule for one.

**The disabled state is not bridged and not drawn.** daisyUI has no disabled badge either, and
here, unlike selection, nothing is emitted in its place: the primitive already makes a disabled
tag inert, unselectable and skipped by the arrow keys, and a look for it would be this registry
inventing one. A caller who wants it to *look* inert reaches for `data-[disabled=true]:opacity-50`
through `class`, which is the same escape hatch the accordion documents.

## Axes

- `color: TagColor` on `TagOption`: `badge-neutral`, `badge-primary`, `badge-secondary`,
  `badge-accent`, `badge-info`, `badge-success`, `badge-warning`, `badge-error`.
- `size: TagSize` on `TagOption`: `badge-xs`, `badge-sm`, `badge-lg`, `badge-xl`.
- `appearance: TagOptionAppearance` on `TagOption`: the selection and focus utilities above.
- `appearance: TagGroupAppearance` on both roots: `flex flex-col items-start gap-2`.
- `appearance: TagListAppearance` on `TagList`: `flex flex-wrap items-center gap-2`.
- `appearance: TagGroupEmptyAppearance` on `TagGroupEmpty`: `text-sm opacity-60`.

Both daisyUI axes are on the **tag** rather than on the group, because daisyUI's classes are
per-badge: a group-wide colour would be this component inventing an API daisyUI does not have, and
a set of differently coloured tags (which is what a label list usually is) would then need an
escape hatch out of it.

`TagColor::Default` emits nothing, which is daisyUI's uncoloured badge. `TagSize::Default` emits
nothing and renders at the same size as daisyUI's explicit `badge-md`.

The four appearance axes invert the usual convention: their default value *emits* utilities and
their `None` value emits nothing (ADR-0004). None of them is named `style`, which collides with the
global HTML attribute.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**A tag is a grid row, where daisyUI's badge is a `span`.** The primitive renders
`div[role=grid] > div[role=row] > div[role=gridcell]`, so that a screen reader can say how many
tags there are and which one it is on, which a row of `span`s cannot. daisyUI's markup survives
it intact: the cell is `display: contents`, so the label and the remove button are the flex
children of the badge itself, exactly as they would be in daisyUI's own markup.

**The remove button carries the button's classes, duplicated.** `btn btn-ghost btn-xs btn-circle`
is written out here rather than depended on because cross-Component dependencies are for public
composition, while the Tailwind contract requires every class to be a literal in the file that
emits it. It is the same set the toast's close button carries.

**A removed tag is gone from the list, not from the caller's data.** The primitive hides it and
stops navigating to it; the array it was rendered from is the caller's to update in the callback.
A caller who re-renders the same list without removing anything gets the tag back.

**The empty state only appears after the first render.** The primitive waits for the list to be
mounted before deciding it is empty, so a group that starts with no tags shows nothing for one
frame rather than flashing its empty state during hydration.

**A tag group is not a form control.** There is no hidden input and no `name`; the selection
reaches an app through `on_value_change` or `on_values_change`. That is the primitive's shape, and
it is the same for every other control in this registry.

## daisyUI classes deliberately not used

- `badge-outline`, `badge-dash`, `badge-soft`, `badge-ghost`: daisyUI's other badge looks. They
  are not exposed for the same reason the button component does not expose its own: a caller
  reaches them through `class`, which concatenates. `badge-soft` in particular is what an
  unselected filter chip often wants, and pinning that meaning here would be this registry
  deciding what selection looks like.
- `badge-md`: the size value the default renders at, which is what emitting nothing already does.
- `indicator` and `indicator-item`: daisyUI's way of hanging a badge off the corner of something
  else. That is a badge as decoration, which is the case this component is not for.
- The responsive prefixes daisyUI generates for the colour and size classes (`sm:badge-lg` and the
  rest): a caller reaches those through `class`, which concatenates.
