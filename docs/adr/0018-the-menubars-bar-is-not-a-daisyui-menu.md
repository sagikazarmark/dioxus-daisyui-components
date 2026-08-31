# The menubar's bar is a row of buttons, not a daisyUI menu

`Menubar` lays its bar out with Tailwind utilities and gives each trigger daisyUI's `btn`.
daisyUI's own menu bar is `menu menu-horizontal` on a `<ul>`, and the registry uses `menu` only
inside each menu's popup, where the split ADR-0005 describes applies for the third time.

daisyUI reaches a menu item as the child of a literal list item:

```css
.menu :where(li:not(.menu-title) > :not(ul, menu, details, .menu-title, .btn)) { … }
```

The primitive's tree has an element the bar cannot get rid of. `Menubar` renders the bar,
`MenubarMenu` renders a wrapper per menu, and the trigger and the popup are both inside that
wrapper. Wrapping each menu in an `li` (the trick the dropdown menu, the context menu and the
select all use for their items) would therefore put daisyUI's item styling on the *wrapper*:

- The padding, the radius and the hover highlight would land on a box that also contains the
  popup, so hovering an open menu's popup would keep the bar item highlighted.
- Worse, the keyboard highlight would not land anywhere. daisyUI's is
  `:focus-visible` on the same selector, and `:focus-visible` matches the element that has focus,
  which is the trigger inside the wrapper, an element that selector never reaches. A keyboard user
  moving along the bar would see nothing move.

The second point is decisive: it is an accessibility regression rather than a cosmetic difference,
and the registry keeps the primitives precisely for the keyboard.

daisyUI's exclusion list is what makes the alternative work. `.btn` is excluded from item styling
by name (daisyUI expects buttons inside menus), so a trigger with `btn` on it carries daisyUI's
own button paint, its focus ring included, wherever it sits.

## Consequences

- The bar emits layout utilities only, all Defeatable (ADR-0004). daisyUI has no bar to paint, so
  a caller who wants one paints it through `class`.
- Each trigger carries `btn` and the button's colour and size axes, duplicated as every other
  component that borrows them duplicates them.
- The popups are the ADR-0005 split: box and placement on the primitive's content element, `menu`
  on a `ul` inside it, each item wrapped in a presentational `li`. Everything `.menu` gives an item
  (the padding, the hover, the `:focus-visible` highlight, `menu-disabled`) works there, because
  the items *are* `li > *`.
- The popup is placed with utilities rather than `dropdown-content`, because there is no
  `.dropdown` for it to be inside: the bar and the menus are the primitive's elements. This is the
  hover card's situation (ADR-0015) arrived at from the other direction.
- The trigger's open look is a **Bridged utility** over the `data-state` the primitive sets on the
  menu's wrapper. It cannot be Tier 2: `btn-active` would have to be emitted from state this
  component is never told, since the primitive's menubar has no controlled open prop and no change
  callback to lift one through.
- A caller who wants daisyUI's `menu menu-horizontal` bar exactly (the CSS-only one, with
  `details`/`summary` menus) writes it by hand and gets no keyboard behaviour, which is the trade
  this component exists to avoid.
