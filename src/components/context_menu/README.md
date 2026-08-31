# Context menu

A context menu styled with daisyUI's `menu` classes, wrapping the `dioxus-primitives` context
menu. It opens on a right click and on a touch long press, pins itself to the pointer, dismisses
on Escape and on an outside click, and moves through its items with the arrow keys, none of
which daisyUI's CSS-only menu has, and all of it the primitive's rather than reimplemented here.

[Live examples](https://daisyui-components.dioxus.cc/components/context_menu) ·
[their sources](docs/examples/)

## Composition

The compound parts are `ContextMenu` (the surface the menu belongs to), `ContextMenuTrigger` (the
region a right click opens it over), `ContextMenuContent` (the box, holding the `menu` list) and
`ContextMenuItem` (one command). There is no collapsed component: daisyUI's markup has caller
content in the trigger and in every item, so there is nothing legal to collapse.

`index` orders the keyboard navigation. It is the primitive's prop and is required, because items
register with the focus collection by the index they are given rather than by where they sit in
the DOM, which is what lets daisyUI's own list-item wrappers stand between the menu and its
items.

**This is not the dropdown menu with a different trigger.** A dropdown is opened by a control and
positioned against it; a context menu is opened by a gesture over a region and positioned against
the *pointer*. Everything below follows from that one difference.

## State bridging

**Tier 2** on the disabled item, and **no state to bridge** anywhere else.

daisyUI mutes a disabled item through `menu-disabled` on the list item, or through a `disabled`
attribute on a control. The primitive sets neither: it reports the state as `data-disabled` and
`aria-disabled` on the item, and daisyUI matches neither of those. So the class is emitted from
Rust, on the wrapper, which is where daisyUI wrote it.

The open state needs nothing, which is what makes this component unlike the dropdown menu. There,
`dropdown-content` is hidden until `dropdown-open` is on the outer element, so the class has to be
emitted and the state has to be lifted to emit it. Here no `dropdown` class is emitted at all
(see the deviations) so nothing daisyUI does depends on knowing whether the menu is open, and the
primitive already mounts it only while it is. The state travels through to the primitive
untouched, and a controlled caller and an uncontrolled one both get its own behaviour.

## Axes

- `size: ContextMenuSize` on `ContextMenuContent`: `menu-xs`, `menu-sm`, `menu-lg`, `menu-xl`.
- `appearance: ContextMenuContentAppearance` on `ContextMenuContent`: `bg-base-100 rounded-box
  shadow-sm`.

The size axis is the dropdown menu's, with the class strings duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires
every one of these classes to be a literal in the file that emits it. `ContextMenuSize::Default` emits
nothing and renders at the same size as daisyUI's explicit `menu-md`.

The appearance axis inverts the usual convention: its default value *emits* utilities and its
`None` value emits nothing. daisyUI has no class for this box: its own floating menus are
`dropdown-content`, which is a position rather than a look, with the fill, the corners and the
shadow written beside it as utilities. Those utilities are what is emitted here, so what a caller
would otherwise have to override is a utility rather than a daisyUI class, and two utilities only
tie. Switching ours off is how a caller wins that tie (ADR-0004).

daisyUI's own `p-2` is left out, as it is on the dropdown menu: `.menu` already pads by exactly
that, and here the two elements would pad in turn rather than agree. Its `w-52` is left out too:
a menu with no width sizes to its items, and a caller who wants one adds it through `class`, which
reaches the items because the list is stretched to the box.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**daisyUI's two classes are split across two elements** (ADR-0005). daisyUI puts the box and
`menu` on one `ul`; the primitive's content element is a hardcoded `div`, and it has to be: it
carries the `position: fixed` and the inline coordinates that pin the menu to the pointer. Every
visual rule `.menu` has for an item is written against a literal `li`, so `menu` goes on a list
this component renders inside the box, with each item wrapped in an `li`. Both the list and the
wrappers are marked `role="none"`, so the menu still owns its items in the accessibility tree.

**No `dropdown-*` class is emitted, and there are no placement axes.** daisyUI positions a
dropdown against its trigger, which is what its placement and alignment classes describe. A
context menu has no trigger to be positioned against: it opens where the pointer was, and the
primitive computes those coordinates from the event, including the visual-viewport offset, which
is what keeps it under the finger on a pinched-in mobile page. Exposing daisyUI's placements here
would be offering axes that cannot apply.

**The trigger emits nothing.** What a right-clickable surface looks like is the caller's: daisyUI
has no class for a region, and a table row, a canvas and a file tile all want different ones. What
the primitive puts there instead is `aria-haspopup`, `aria-expanded` and the gesture handling.

**A menu that nothing in it is focused closes itself.** The primitive keeps the open state and the
focus state in step, so a menu opened by a caller rather than by a gesture closes on the next
frame unless the caller holds it open. That is why the preview's rows of menus are controlled, and
why `default_open` on its own is not a way to have a menu standing open.

**An `id` on `ContextMenu` breaks dismissal.** The primitive generates an id for the outer
element and then looks the element up by it twice over: once to tell a click inside the menu from
a click outside (which is how an outside click dismisses) and once to decide which wheel events
to swallow while the menu is open. An `id` passed by a caller lands on the element over that one,
and both lookups then find nothing: the menu stops dismissing on an outside click, and the page
stops scrolling while it is open. There is no prop to declare it through, because the primitive
takes none, so the rule is written here instead: address the trigger, the menu, or a wrapper of
your own, never the `ContextMenu`.

**Scrolling is suppressed while the menu is open.** A `position: fixed` menu pinned to a click
point would drift away from what was clicked as soon as the page scrolled, so the primitive
suppresses wheel and touch scrolling outside the menu while it is open, which is what a native
context menu does. It is the primitive's behaviour rather than this component's, and it is
recorded here because it is the surprising one.

## daisyUI classes deliberately not used

- `menu-disabled` is **not** in this list: it is emitted, and it is the whole of Tier 2 here.
- `dropdown`, `dropdown-content` and the whole placement and alignment family: the deviation
  above.
- `menu-active`: daisyUI's selected item. A context menu's items are commands rather than
  choices; a menu that marks one of its items as current is a listbox, and the select component is
  where that lives.
- `menu-title` and the empty `li` daisyUI draws as a divider: both are markup a caller writes
  inside the menu, and both reach it as ordinary children. This component wraps items, not
  headings and rules.
- `menu-horizontal`: a context menu is a column, which is what `.menu` already is.
- The responsive prefixes daisyUI generates for the size classes (`sm:menu-lg` and the rest): a
  caller reaches those through `class`, which concatenates.
