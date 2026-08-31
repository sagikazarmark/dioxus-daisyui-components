# Popover

A panel of your own content opened from a button, styled with daisyUI's `dropdown` classes,
wrapping the `dioxus-primitives` popover.

[Live examples](https://daisyui-components.dioxus.cc/components/popover) ·
[their sources](docs/examples/)

This is the dropdown menu's structure with nothing prescribed inside the box: no `menu`, no items,
no selection. What the primitive adds over a `details` element is dismissal (Escape and a click
outside both close it) and, when the popover is modal, a focus trap that keeps the keyboard
inside the panel while it is open.

## State bridging

**Tier 2** on the open state, and it is mandatory rather than cosmetic: daisyUI hides
`.dropdown-content` outright unless `dropdown-open` is on the root, and matches no attribute the
primitive sets: `data-state` appears nowhere in its stylesheet.

The state is therefore **lifted** (ADR-0006), the way the dropdown menu's is. `Popover` seeds a
signal from `default_open`, always hands the primitive a controlled value, and intercepts the
change callback, so a controlled caller and an uncontrolled one both work and this component is
the only writer. A benefit falls out of it: daisyUI's open and close transitions run, because the
class is removed while the element is still mounted.

Nothing else is bridged. The trigger is a `button` with daisyUI's `btn` on it, and the panel's
roles, its `aria-labelledby` and its focus trap are the primitive's.

## Axes

- `side: PopoverSide`: `dropdown-top`, `dropdown-bottom`, `dropdown-left`,
  `dropdown-right`.
- `align: PopoverAlign`: `dropdown-start`, `dropdown-center`, `dropdown-end`.
- `appearance: PopoverContentAppearance`: the utilities that draw the box, or nothing.

Side and align emit a class for **every** value, their defaults included, which is the
shape ADR-0008 records for the dropdown menu and the reason is the same: an unclassed
`.dropdown-content` lands wherever it would have fallen in flow, which is only under the trigger
for as long as the trigger is the last thing written before it.

Both are handed to the primitive as well as emitted as classes, so the `data-side` and
`data-align` it reports on the panel say what daisyUI's classes did, and a caller who switches
this component's classes off still has them to position from.

`PopoverContentAppearance` is the inverted shape ADR-0004 describes: `Default` emits utilities and
`None` emits nothing. daisyUI's `dropdown-content` only positions the element; the fill, the
corners, the padding and the shadow are Tailwind utilities in daisyUI's own examples, so they are
emitted here and switched off rather than out-ranked.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**A daisyUI dropdown is used here where the hover card may not use one**, which is recorded as
ADR-0017 rather than left to look like an inconsistency. daisyUI takes the pointer events off a
`[tabindex]:first-child` of an open dropdown (fatal to a card that opens on hover) and the
popover's trigger is a plain `button` with no `tabindex` attribute, so the rule does not match it.
The interaction the rule exists for, closing on a second click, is also what a popover wants.

**The trigger cannot be rendered as another element.** The primitive offers no `as` prop here, and
this component does not invent one: the shape that would break daisyUI's dropdown is exactly a
trigger carrying `tabindex`.

**The padding is kept where the dropdown menu drops it.** There, `.menu` inside the box pads
already and the two would pad in turn; here the box holds whatever the caller wrote, so the
padding is the box's own.

**A modal popover traps focus, and that is the default**: the primitive's, repeated rather than
chosen. It is the right default for a panel with controls in it, and the wrong one for a panel
that is only text next to a page a reader is still using, which is what `is_modal: false` is for.

## daisyUI classes deliberately not used

- `menu`, `menu-title`, `menu-active`, `menu-disabled`: the list inside a daisyUI dropdown, which
  is the dropdown menu's and the context menu's. A popover holds the caller's own content, so
  there is no list to style and no split (ADR-0005) to make.
- `card`, `card-body`, `card-title`: daisyUI's other box, which its own popover examples sometimes
  use. It brings its own layout and a required inner element, which would be this component
  prescribing what goes inside the panel; a caller who wants a card writes one inside the content.
- `dropdown-hover`: daisyUI's CSS-only way of opening a dropdown on hover. A popover opens on
  click, and a panel that opens on hover is the hover card.
- `dropdown-open` as a caller-facing axis: it is what this component emits for the open state,
  and a caller who set it themselves would be lying about the state to daisyUI while the primitive
  said otherwise.
- `dropdown-close`, and daisyUI's `[popover]` path: the first is for a dropdown a caller wants
  shut against the CSS-only rules that would open it, and neither applies to a panel whose open
  state is already the primitive's.
- `z-1`, `w-52` from daisyUI's example class list: the first is beneath the `z-index`
  `.dropdown-content` already sets, and the second is a width for the menu that example was
  written around rather than for every panel.
