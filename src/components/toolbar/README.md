# Toolbar

A row of grouped controls, styled with daisyUI's `btn` and `divider` classes, wrapping the
`dioxus-primitives` toolbar.

```rust
Toolbar { aria_label: "Text formatting",
    ToolbarButton { index: 0usize, on_click: move |_| {}, "Cut" }
    ToolbarButton { index: 1usize, on_click: move |_| {}, "Copy" }
    ToolbarSeparator {}
    ToolbarButton { index: 2usize, on_click: move |_| {}, "Undo" }
}
```

Controls are identified by `index`, which is the order the arrow keys move through. It is
explicit rather than taken from the DOM because the primitive matches a key press to a control by
index rather than by position; see the gaps below for what that order does and does not buy.

A separator does not take an index. It is not focusable, and nothing navigates to it.

## State bridging

**Tier 2** on the separator's orientation, and nothing else.

daisyUI expresses a rule's orientation with a class and matches nothing the primitive sets, so the
class is emitted from Rust, with the inversion the separator component documents: ARIA names a
separator after the line it draws, and daisyUI names a divider after the layout it sits in, so the
class that draws a rule *down* a row is `divider-horizontal`.

What is different here is where the orientation comes from. The primitive already inverts the
toolbar's own direction for a separator that is not told one, so this component reads the
toolbar's direction from a context of its own (the primitive's is private) and follows the same
rule rather than deciding it a second time.

Nothing else needs bridging. The buttons are `button` elements with the native `disabled`
attribute on them, which is what `.btn:disabled` matches; the keyboard highlight is `.btn`'s own
`:focus-visible` rule, on the element the primitive moves real DOM focus to. There is no open
state, no selection and no pressed state in a toolbar: a toolbar of toggles is toggles inside a
toolbar, and the toggle component is what those are.

## Axes

- `appearance: ToolbarAppearance`: the utilities that lay the row out, or nothing.
- `color: ToolbarButtonColor` on a button: `btn-neutral`, `btn-primary`, `btn-secondary`,
  `btn-accent`, `btn-info`, `btn-success`, `btn-warning`, `btn-error`.
- `size: ToolbarButtonSize` on a button: `btn-xs`, `btn-sm`, `btn-lg`, `btn-xl`.
- `color: ToolbarSeparatorColor` on a separator: `divider-neutral`, `divider-primary`,
  `divider-secondary`, `divider-accent`, `divider-info`, `divider-success`, `divider-warning`,
  `divider-error`.

The button and separator axes are the button component's and the separator component's, with the
class strings duplicated rather than depended on: cross-Component dependencies are for public
composition, while the Tailwind contract requires every one of these classes to be a literal in the
file that emits it.

Both are on the **part** rather than on the toolbar. daisyUI's classes are per-control, so a
toolbar-wide colour would be this component inventing an API daisyUI does not have.

`ToolbarAppearance` is the inverted shape ADR-0004 describes (`Default` emits utilities and
`None` emits nothing) because these are the registry's own utilities rather than daisyUI classes,
and a caller who wants to lay the row out themselves needs ours out of the way rather than
outranked.

The direction is not an axis at all. It follows the primitive's `horizontal` prop, which is also
which arrow keys move focus, and a class is emitted for both of its values (ADR-0008) so that what
the row looks like and what the keyboard does cannot disagree.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**daisyUI has no toolbar, so the row is Tailwind utilities.** What daisyUI has is `join`, which
fuses a row of controls into one shape with shared borders and rounded ends; that is the toggle
group, and it is a different thing: a toolbar holds separate controls, in groups, with rules
between the groups. So the root emits layout utilities and nothing else, every one of them
Defeatable (ADR-0004), and each part inside it carries a daisyUI class of its own. A caller who
wants the joined look writes `join` and `join-item` through `class`, which concatenates; daisyUI's
join is a general-purpose row and nothing here stops it.

**A separator brings daisyUI's own margins.** `.divider` sets `margin: 1rem` along the axis it
runs, which is generous next to the `gap-1` this component lays the row out with. It is not
trimmed, because trimming it would be the registry making a spacing decision on daisyUI's behalf,
and because the margin is what makes a rule read as a group boundary rather than as another
control. A caller who wants it tighter passes a margin utility through `class`, which
concatenates.

**A vertical toolbar stretches its controls.** `flex-col` on its own leaves each button as wide as
its own text, which reads as a ragged column rather than as a toolbar, so `items-stretch` is
emitted with it. Both are part of the same Defeatable utility set, so a caller who wants the
ragged column switches the appearance axis off.

## The primitive's keyboard, as it actually is

These are the primitive's rather than this component's, and they are written down because a
toolbar is usually assumed to behave the way a radio group or a toggle group does, and this one
does not. Nothing here is worked around: adding a key handler or a tabindex would be the registry
taking on behaviour that belongs upstream.

**Every control is its own tab stop.** The primitive puts `tabindex="0"` on each button, so Tab
walks through the toolbar one control at a time. A roving tab stop (Tab enters the row once, the
arrow keys move inside it) is what the toggle group and the radio group have, and what ARIA's
toolbar pattern asks for; this toolbar has the arrow keys *as well as* Tab rather than instead of
it.

**The arrow keys stop at a disabled control rather than skipping it.** The primitive moves focus
by asking for the neighbouring index, and a disabled index resolves to no control, so focus stays
where it was and pressing again does not get past. Put disabled controls at the end of a group, or
leave them out.

**Home works and End does not.** Home asks for the first index; End asks for an index far beyond
the last one, which resolves to no control at all, so nothing moves.

## daisyUI classes deliberately not used

- `join`, `join-item`, `join-horizontal`, `join-vertical`: the joined row, which is the toggle
  group's shape and is above.
- `btn-active`: what a pressed control emits, which is the toggle's and the toggle group's. A
  toolbar button is a button that does something rather than one that stays down; a toolbar of
  toggles is toggles written inside a toolbar.
- `btn-outline`, `btn-soft`, `btn-dash`, `btn-ghost`, `btn-link`, `btn-square`, `btn-circle`: the
  button component's other looks, not exposed here for the same reason they are not exposed there:
  a caller reaches them through `class`, which concatenates. `btn-ghost` and `btn-square` in
  particular are what an icon toolbar usually wants.
- `divider-start`, `divider-end`: where daisyUI places a divider's own content along the rule. A
  toolbar separator has no content: the primitive renders it as an empty element with a separator
  role, and daisyUI's placement classes only drop one half of the rule when there is nothing
  between them to place.
- The responsive prefixes daisyUI generates for these classes (`sm:btn-lg` and the rest): a caller
  reaches those through `class`, which concatenates.
