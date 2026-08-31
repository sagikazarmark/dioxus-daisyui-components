# Menubar

An application menu bar styled with daisyUI's `btn` and `menu` classes, wrapping the
`dioxus-primitives` menubar.

[Live examples](https://daisyui-components.dioxus.cc/components/menubar) ·
[their sources](docs/examples/)

Menus are identified by `index` and items by an `index` within their menu, which is the keyboard
order in both cases. The bar is a single tab stop: focus enters it once, the left and right arrows
move along it, and down opens a menu and steps into it.

## State bridging

**Tier 2** on an item's disabled state, and a **Bridged utility** on the trigger's open state.
Nothing is lifted and nothing is mirrored, because there is nothing to lift it from.

The primitive's `Menubar` has no controlled open prop and no change callback: which menu is open
lives in a context of its own, and no wrapper can read it or write it. So the trigger's open look
cannot be `btn-active`, which would have to be emitted from Rust. What the primitive *does* report
is `data-state` on each menu's wrapper element, so the trigger's ring is written as a variant of
that attribute (`group-data-[state=open]:…`) on the element below it. Nothing is recomputed, and
the primitive stays the only owner of the state, which is what makes it a Bridged utility rather
than either tier.

The popup needs no visibility bridging at all: the primitive mounts it only while its menu is
open, so there is no daisyUI class hiding it and none to emit. This is the opposite of the
dropdown menu, where `dropdown-open` is mandatory.

A disabled item is Tier 2, on the wrapper rather than on the item: daisyUI mutes a disabled item
through `.menu-disabled` on the list item, and the primitive sets `data-disabled` on the item,
which daisyUI matches nowhere.

Everything else is Tier 1 or nothing. `.menu`'s hover highlight and its `:focus-visible` highlight
both reach an item as `li > *`, and the primitive moves real DOM focus onto the item, so the
keyboard highlight is daisyUI's own, with nothing emitted for it.

## Axes

- `appearance: MenubarAppearance`: the utilities that lay the bar out, or nothing.
- `color: MenubarTriggerColor` on a trigger: `btn-neutral`, `btn-primary`, `btn-secondary`,
  `btn-accent`, `btn-info`, `btn-success`, `btn-warning`, `btn-error`.
- `size: MenubarTriggerSize` on a trigger: `btn-xs`, `btn-sm`, `btn-lg`, `btn-xl`.
- `appearance: MenubarContentAppearance` on a popup: the utilities that place and draw it, or
  nothing.
- `size: MenubarMenuSize` on a popup: `menu-xs`, `menu-sm`, `menu-lg`, `menu-xl`.

The trigger's axes are the button component's and the popup's size axis is the dropdown menu's,
with the class strings duplicated rather than depended on: cross-Component dependencies are for
public composition, while the Tailwind contract requires every one of these classes to be a literal
in the file that emits it.

Both appearance axes are the inverted shape ADR-0004 describes (`Default` emits utilities and
`None` emits nothing) because these are the registry's own utilities rather than daisyUI classes.
The popup's covers its placement as well as its paint, so a caller using anchor positioning or a
floating-element library switches all of it off at once and positions the popup themselves.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The bar is not a daisyUI `menu`.** ADR-0018 records this in full. daisyUI's own menu bar styles
an item as `li > *`, and the primitive renders a wrapper element per menu between the bar and the
trigger, so daisyUI's padding and hover would land on that wrapper, and its `:focus-visible`
highlight would land nowhere at all, because the element that takes focus is the trigger inside
it. Giving each trigger `btn` instead keeps a keyboard user's place visible, and `.btn` is on
daisyUI's own exclusion list for menu items, so the two never fight.

**The popup is placed with utilities rather than `dropdown-content`.** There is no `.dropdown` for
it to be inside (the bar and the menus are the primitive's elements) so `absolute start-0
top-full mt-1` does the placing, with a `z-10` that `dropdown-content` would have brought itself.
The menu's wrapper carries `relative` to be the context those offsets resolve against. This is the
hover card's situation (ADR-0015) reached from the other direction.

**The popup does not collide-detect.** Neither does daisyUI's dropdown, so nothing is lost against
it; a menu near the end of a viewport opens off the edge rather than flipping, which is what the
appearance axis and a floating-element library are for.

**A trigger does not announce that it has a menu.** The primitive gives it `role="menuitem"` and
moves focus, but sets neither `aria-haspopup` nor `aria-expanded`, so a screen reader is told a
menu bar with items in it rather than a menu bar with menus under them. Nothing here adds the
attributes: they would have to be kept in step with an open state this component is never told,
and getting them wrong is worse than not having them. It is upstream's to fix, and the registry
inherits the fix when it lands.

**An open menu closes when focus leaves it.** That is the primitive's: it closes on `blur` of the
trigger or of an item, so a menu cannot be pinned open by a caller (there is no open prop) and a
menu shown in documentation is shown by opening it rather than by asking for it.

## daisyUI classes deliberately not used

- `menu-horizontal`, `menu-vertical` on the bar: the bar is not a `.menu`, above.
- `menu-dropdown`, `menu-dropdown-toggle`, `menu-dropdown-show`: daisyUI's CSS-only submenu
  toggling, which is state daisyUI drives from a class the caller flips. Here the state is the
  primitive's and the popup is mounted only while it is open, so nothing is left for them to do.
- `dropdown`, `dropdown-content`, and the placement family: daisyUI's floating box, unusable here
  for the structural reason above rather than for the hover card's behavioural one (ADR-0015).
- `btn-active` on a trigger: the open look, which cannot be emitted from state this component is
  never told; the Bridged utility is what stands in for it.
- `menu-title`: daisyUI's group heading inside a menu. The primitive has no group part in a
  menubar menu, so a heading would be a `li` the registry rendered with no primitive behind it; a
  caller who wants one writes it inside the popup.
- `menu-active`: daisyUI's selected item, which is the select's. A menubar item is a command
  rather than a choice, and the primitive marks none of them selected.
- The responsive prefixes daisyUI generates for these classes (`sm:btn-lg`, `md:menu-lg` and the
  rest): a caller reaches those through `class`, which concatenates.
