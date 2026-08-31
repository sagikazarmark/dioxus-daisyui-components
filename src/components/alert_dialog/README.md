# Alert dialog

An alert dialog styled with daisyUI's `modal` classes, wrapping the `dioxus-primitives` alert
dialog. It is the dialog's markup applied to a decision that has to be taken: it traps focus,
announces itself as an `alertdialog` with its title and description, and offers exactly two ways
out, neither of which is a click on the backdrop.

[Live examples](https://daisyui-components.dioxus.cc/components/alert_dialog) ·
[their sources](docs/examples/)

## Composition

The compound parts are `AlertDialogRoot` (the modal), `AlertDialogContent` (the box),
`AlertDialogTitle`, `AlertDialogDescription`, `AlertDialogActions`, `AlertDialogAction` and
`AlertDialogCancel`. `AlertDialog` is a **collapsed component** that renders the first two
together, offered alongside the parts rather than instead of them, legal for the reason the
dialog's is: daisyUI's modal has nothing between those two elements and no caller content can go
there.

**Caller attributes on `AlertDialog` land on the box**, including `id`. The outer element is
reached through the `placement` axis, or by dropping to the parts.

`AlertDialogContent` has to stay a direct child of `AlertDialogRoot`: daisyUI reaches the box
from the open modal through a child combinator.

There is no close part and no context to reach for one. The action and the cancel *are* the way
out: both close the dialog themselves, before the caller's `on_click` runs, which is what
tells this component apart from the dialog, where a caller wires a close button of their own.
Which of the two is the dangerous one is the caller's to say, through the colour axis.

**There can be only one title and one description.** The primitive holds the two ids it names and
describes the dialog by, and puts them on whatever title and description are rendered, so a
second of either repeats an id that has to be unique. That is why the preview varies their
appearance axes one dialog at a time rather than in a row.

## State bridging

**Tier 2** on the open state, with **lifted state** (ADR-0006), exactly as the dialog has it.

daisyUI hides `.modal` outright (`visibility: hidden`, no pointer events, a transparent
backdrop) and reveals it only through `.modal.modal-open`, which matches no ARIA or `data-*`
attribute the primitive sets. Its other selectors are for markup this component does not
produce: `[open]` and `:popover-open` need a native `dialog` element, `:target` needs navigation
by fragment, and `.modal-toggle:checked` needs the CSS-only checkbox. So the class is emitted
from Rust (ADR-0002 rules out CSS), and `AlertDialogRoot` owns the state to emit it: it seeds a
signal from `default_open`, always hands the primitive a controlled value, and intercepts
`on_open_change`.

Nothing else needs bridging. The `role="alertdialog"`, `aria-modal`, `aria-labelledby`,
`aria-describedby` and the focus trap are the primitive's, and daisyUI styles none of them.

## Axes

- `placement: AlertDialogPlacement`: `modal-top`, `modal-middle`, `modal-bottom`, `modal-start`,
  `modal-end`, on the modal.
- `color: AlertDialogButtonColor` on `AlertDialogAction` and `AlertDialogCancel`: `btn-neutral`,
  `btn-primary`, `btn-secondary`, `btn-accent`, `btn-info`, `btn-success`, `btn-warning`,
  `btn-error`.
- `appearance: AlertDialogTitleAppearance` on `AlertDialogTitle`: `text-lg font-bold`.
- `appearance: AlertDialogDescriptionAppearance` on `AlertDialogDescription`: `py-4`.

`AlertDialogPlacement::Default` emits nothing. It is close to `AlertDialogPlacement::Middle` (both
centre the box) but not a synonym for it: daisyUI's explicit middle also caps the box's height
at the viewport less `5em`.

The button colour axis is the button component's classes written out again rather than depended
on. Cross-Component dependencies are for public composition, not class sharing. A bare dependency
name also resolves against the upstream default registry rather than this one and would silently
install somebody else's Component. Since the Tailwind contract already requires every class name
to be a literal in scanned source, there is nothing to share but the literal.

The two appearance axes invert the usual convention: their default value *emits* utilities and
their `None` value emits nothing, for the reason ADR-0004 records. Neither is named `style`,
which collides with the global HTML attribute.

Each enum exposes `ALL`. The preview iterates it and the browser specs assert over the same
rendered set: for `placement`, and for the title and description axes, by opening one dialog per
value, since every value of the first is a full-viewport element and the other two cannot be
repeated in one dialog.

## Deviations

**A click outside the box does not dismiss.** That is the whole point of an alert dialog and it
is the primitive's behaviour rather than something switched off here. Escape still closes it, so
a keyboard user is never trapped.

**A closed alert dialog renders nothing at all**, and unlike the dialog, *nothing* means the
outer element too. The dialog's primitive keeps its root mounted and unmounts the box; this one
unmounts the root, so `.modal` without `modal-open` is a state that exists only during the exit
animation. The class is still emitted rather than left off, because it is what reveals the modal
in the render that mounts it and what animates it out when it is removed.

**The modal is a `div`, not a native `dialog` element.** That is the primitive's markup, and
daisyUI's rule covers it: `.modal.modal-open` is the one selector of the five that needs no
particular element behind it. The top layer and the native `::backdrop` go with the element;
daisyUI uses neither, since it draws the dim on the modal itself.

**The two buttons carry `btn` and nothing else.** No size axis, no appearance axis: a dialog's
buttons are the caller's to shape through `class`, which concatenates, and the colour is the one
that changes what the choice *means*.

## daisyUI classes deliberately not used

- `modal-open` is **not** in this list: it is emitted, and it is the whole of Tier 2 here.
- `modal-backdrop`: a form or an anchor rendered behind the box to close the modal by
  navigation. An alert dialog is the one modal that must not close that way.
- `modal-toggle`: the hidden checkbox the CSS-only modal is opened by. This component emits the
  modifier class instead, which is what the lifted state exists to make possible.
- `alert`, `alert-warning`, `alert-error`: daisyUI's *alert* is a strip of text in a page, not a
  modal, and nothing about this component is one. The name they share is daisyUI's and ARIA's
  meeting at a word.
- The responsive prefixes daisyUI generates for all of the above (`sm:modal-bottom`,
  `md:modal-box`, and so on): a caller reaches them through `class`, which concatenates.
