# Dropdown menu

A dropdown menu styled with daisyUI's `dropdown` and `menu` classes, wrapping the
`dioxus-primitives` dropdown menu. It opens and closes on state rather than on focus, moves
through its items with the arrow keys, skips disabled ones, dismisses on Escape, on a click
outside and on Tab, all of it the primitive's rather than reimplemented here.

```rust
rsx! {
    DropdownMenu {
        DropdownMenuTrigger { "Actions" }
        DropdownMenuContent {
            DropdownMenuItem::<String> {
                value: "edit".to_string(),
                index: 0usize,
                on_select: move |value| tracing::info!("selected {value}"),
                "Edit"
            }
            DropdownMenuItem::<String> {
                value: "archive".to_string(),
                index: 1usize,
                disabled: true,
                "Archive"
            }
        }
    }
}
```

## Composition

The compound parts are `DropdownMenu` (the dropdown), `DropdownMenuTrigger` (the button),
`DropdownMenuContent` (the box, with the menu inside it) and `DropdownMenuItem` (one item).
There is no collapsed component: the trigger and the content are two elements a caller writes
between, so there is nothing to collapse.

An item's `index` is its position in the keyboard navigation order, and it is explicit because
the primitive's focus collection is ordered by it rather than by the DOM, which is what makes
the wrappers below free.

**The classes split across two elements** (ADR-0005). daisyUI puts `dropdown-content` and `menu`
on a single `<ul>`; here `dropdown-content` and the box's utilities go on the primitive's content
`<div>`, `menu` goes on a `<ul>` rendered inside it, and each item is wrapped in an `<li>`:

```html
<div class="dropdown dropdown-bottom dropdown-start dropdown-open">
  <button class="btn" aria-haspopup="listbox" aria-expanded="true">Actions</button>
  <div class="dropdown-content bg-base-100 rounded-box shadow-sm" role="listbox">
    <ul class="menu w-full" role="none">
      <li role="none"><div role="option">Edit</div></li>
    </ul>
  </div>
</div>
```

Three things force it. `DropdownMenuContent` renders a hardcoded `div` and, unlike the trigger,
has no `as` prop. `DropdownMenuContext` is private, so a replacement content component could
never reach the open state, the trigger id or the focus collection. And every visual rule
daisyUI's `.menu` applies to an item is of the form `.menu :where(li:not(.menu-title)>…)`, so
literal `li` elements are required. Given two elements are unavoidable, each class goes on the
element whose layout it was written to drive.

Both wrappers carry `role="none"`, so they are out of the accessibility tree and the listbox
still owns its options directly, which is what option counts and positions are announced from.

**Where daisyUI's example utilities went.** Its list is
`dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm`, and splitting it was
settled by measuring in the preview rather than by copying:

- `bg-base-100 rounded-box shadow-sm` are on the box, which is the element that draws it. They
  are what the `appearance` axis switches off.
- `p-2` is not emitted. `.menu` already pads by the same `.5rem`, and on two elements the two
  paddings would sit inside one another rather than agree; the menu's own padding leaves the
  items inset from the box's edge by exactly what daisyUI's example produces.
- `z-1` is not emitted. daisyUI's own `.dropdown .dropdown-content` rule already sets
  `z-index: 999`.
- `w-52` is not emitted. `.menu` is `width: fit-content` and the box shrink-wraps it, so a menu
  with no width sizes to its items. A caller's width goes on the box through `class`.
- `w-full` **is** emitted, on the list. It is the one place the split has to be papered over: a
  width on daisyUI's single element is the menu's width too, where here it would leave the items
  shrink-wrapped inside a wider box. With no width on the box the two are the same thing anyway.

`w-full` is the one utility here with no axis switching it off, and ADR-0004 asks for one. What
that ADR exists to prevent is a caller's utility *tying* with one of ours, since a tie is settled
by generated-stylesheet order rather than by the class attribute. There is no tie to lose here: a
caller's classes land on the box, this one is on an element no caller reaches, and the two
cooperate: the width the caller sets is the width the items take. Switching it off would only
reintroduce the difference between daisyUI's one element and this component's two, which is what
it is there to close.

## State bridging

**Tier 2** on the open state, with **lifted state** (ADR-0006), and here the lift is mandatory
rather than cosmetic: daisyUI hides `.dropdown-content` outright (`display: none`, no opacity,
scaled down) unless the element above it carries `dropdown-open`, and that class matches no ARIA
or `data-*` attribute the primitive sets. Its other selectors are for markup this component does
not produce: `details`, `.dropdown-hover:hover`, and a `[popover]` element.

`DropdownMenu` therefore owns that state: it seeds a signal from `default_open`, always hands the
primitive a controlled value, and intercepts `on_open_change` to update its own signal before
calling the caller's. A controlled caller and an uncontrolled one both work, and this component
is the only writer either way. The class comes off while the content is still mounted, which is
what daisyUI's exit transition and the primitive's animation-aware unmounting both need.

daisyUI's remaining route in is `:focus-within`, which reveals the content of a dropdown that
holds focus. It never shows anything on its own here, because a closed menu has no content
element at all, but it is why an open menu being operated would look right even without the
modifier, and why the class has to be there for the case that matters: a menu that is open while
focus is somewhere else.

**Tier 2** on an item's disabled state as well, emitted onto the item's wrapper. daisyUI mutes a
disabled item through `.menu-disabled` on the list item or a `disabled` attribute on the item
itself; the primitive reports the state as `data-disabled`, which daisyUI matches nowhere.

Nothing else needs bridging, and two things deliberately get no class at all:

- **The hover highlight**, which daisyUI draws from `li > *:hover` inside a `.menu`.
- **The keyboard focus highlight**, which daisyUI's own rule matches through `:focus-visible` as
  well as through its `.menu-focus` class, and the primitive moves real DOM focus onto the item.

The trigger's `aria-expanded`, the content's `listbox` role and its `aria-labelledby` are the
primitive's, and daisyUI styles none of them.

## Axes

- `side: DropdownMenuSide` on `DropdownMenu`: `dropdown-top`, `dropdown-bottom`,
  `dropdown-left`, `dropdown-right`.
- `align: DropdownMenuAlign` on `DropdownMenu`: `dropdown-start`, `dropdown-center`,
  `dropdown-end`.
- `size: DropdownMenuSize` on `DropdownMenuContent`: `menu-xs`, `menu-sm`, `menu-lg`, `menu-xl`,
  on the list. It sizes the items rather than the box, which has no size of its own.
- `appearance: DropdownMenuContentAppearance` on `DropdownMenuContent`:
  `bg-base-100 rounded-box shadow-sm`.

The side and align axes compose the way daisyUI's classes do: an alignment on a vertical side
moves the menu across the trigger, and on a horizontal one it moves the menu up and down it.

**Both of them emit a class for every value, the default one included** (ADR-0008), the inverse
of the convention the other components follow. daisyUI's unclassed dropdown places its menu
wherever it would have fallen in flow, which is under the trigger and aligned to its start only
for as long as the trigger is the one thing written before it. `dropdown-bottom dropdown-start`
says the same thing about the element rather than about the flow, stays true when a caller writes
anything else inside `DropdownMenu`, and differs even now in the edge the menu grows from as it
opens.

`appearance` inverts the convention the other way, the way the dialog's title and description do:
its default value *emits* utilities and its `None` value emits nothing. daisyUI has no class for
the box a dropdown's menu sits in, so what a caller would otherwise have to override is a
Tailwind utility this component emitted, and two utilities only tie, with the tie settled by
generated-stylesheet order rather than by the class attribute. Switching ours off is how a caller
wins it (ADR-0004). `DropdownMenuSize::Default` emits nothing, which renders at the same size as
daisyUI's explicit `menu-md`.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it and the browser
specs read the same rendered set.

## Deviations

**A menu cannot start open on its own.** The primitive keeps the open state in step with its
focus collection, and closes a menu that nothing in it is focused, so `default_open: true` is
undone as soon as the page settles. A menu that is to be open without being operated is one its
caller controls, holding `open: Some(true)`, which is how the preview's axis rows stand open.

**Focus is not returned to the trigger when the menu is dismissed from inside it.** Escape from
an item closes the menu, and the item goes out of the document with the focus still on it, which
leaves it on the document body. Escape with the trigger focused (the common case, since the
trigger is what opened it) leaves focus where it was. The gap is the primitive's; nothing here
restores focus on its behalf, because doing so means deciding whether a menu closed because the
user dismissed it or because they clicked something else, which is the state the primitive holds
and this component does not. A browser spec pins the gap down rather than leaving it to this
paragraph, so the day upstream restores focus is the day that spec fails and this component
inherits the fix.

**An `id` on the trigger breaks the menu's name.** Every other part in the registry declares `id`
as a prop of its own, because the primitive behind it generates one and then uses it.
`DropdownMenuTrigger` cannot: the primitive's trigger takes no `id` prop at all, generates one,
and points the menu's `aria-labelledby` at it, so a caller's `id` lands over it and leaves the
menu named after an element that is no longer in the document. Address the dropdown itself
instead, which is the element `DropdownMenu` puts a caller's attributes on:

```rust
DropdownMenu { id: "actions",
    DropdownMenuTrigger { "Actions" }
    // ...
}
```

**A trigger rendered through `as` must be focusable without a `tabindex`.** daisyUI takes the
pointer events off a `[tabindex]` first child of an open dropdown (which is how its own CSS-only
dropdown closes on a second click) so a `div` made focusable by a tabindex, which is what
daisyUI's own examples use, stops answering clicks the moment it has focus. A `button` or an `a`
has no such trouble.

**The listbox and option roles are left alone**, despite this component being called a menu.
They are the primitive's: changing them would mean overriding `aria-haspopup` on the trigger too
and diverging from the roles its keyboard handling was built around, and a screen reader
announcing "list box" for a menu is a smaller cost than a control whose roles and behaviour
disagree. If upstream corrects them, this component inherits that fix as well.

**A closed menu renders nothing.** The primitive mounts the content when it opens and unmounts it
once the exit animation has finished, where daisyUI's own dropdown is always in the document and
merely hidden. Nothing observable rests on the difference, but it does mean a page cannot be
measured against a closed menu.

## daisyUI classes deliberately not used

- `dropdown-hover`: opens the menu on hover, with no state behind it. The open state here is the
  primitive's, and a menu that opened on hover would leave the class and the state disagreeing.
- `dropdown-close`: forces a dropdown closed whatever else is on it. It exists for markup that
  has no state to close, which is the opposite of this component's problem.
- `dropdown-open` is **not** in this list: it is emitted, and it is the whole of Tier 2 on the
  open state. Nor is `menu-disabled`, which is the other half.
- `menu-horizontal` and `menu-vertical`: the primitive navigates a dropdown with the up and down
  arrows, so a horizontal menu would announce and navigate one way while rendering another. A
  vertical menu is what `.menu` already is.
- `menu-title`, `menu-active`, `menu-dropdown` and `menu-dropdown-toggle`: group labels, a
  selected item and nested submenus. The primitive's dropdown has no part for any of them; the
  first two arrive with the select component, which borrows this structure.
- `menu-focus`: daisyUI's own class for the item the keyboard is on. The same rule matches
  `:focus-visible`, and the primitive moves real focus, so emitting it would be a second way of
  saying what the DOM already says.
- `disabled` on a list item: daisyUI's hover rules exclude it but nothing defines it;
  `menu-disabled` is the class that does the work.
- The responsive prefixes daisyUI generates for all of the above (`sm:dropdown`,
  `md:dropdown-end`, and so on): a caller reaches them through `class`, which concatenates.
