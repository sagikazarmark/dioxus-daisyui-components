# Dialog

A modal dialog styled with daisyUI's `modal` classes, wrapping the `dioxus-primitives` dialog.
It traps focus while it is open, returns focus to whatever opened it, dismisses on Escape and
on a click outside itself, and announces its own title and description, none of which daisyUI's
CSS-only modal has, and all of which is the primitive's rather than reimplemented here.

```rust
let mut open = use_signal(|| false);

rsx! {
    Button { onclick: move |_| open.set(true), "Delete the project" }

    Dialog {
        open: Some(open()),
        on_open_change: move |next| open.set(next),
        DialogTitle { "Delete the project?" }
        DialogDescription { "Every deployment goes with it. This cannot be undone." }
        DialogActions {
            Button { onclick: move |_| open.set(false), "Cancel" }
            Button { color: ButtonColor::Error, onclick: move |_| open.set(false), "Delete" }
        }
    }
}
```

## Composition

The compound parts are `DialogRoot` (the modal), `DialogContent` (the box), `DialogTitle`,
`DialogDescription` and `DialogActions`. `Dialog` is a **collapsed component** that renders the
first two together, offered alongside the parts rather than instead of them.

The collapse is legal because daisyUI's modal has nothing between those two elements and no
caller content can go there: its markup is the modal, the box, and whatever the caller puts in
the box.

**Caller attributes on `Dialog` land on the box**, including `id`; that is the element worth
reaching, since a class there sizes or repaints the dialog itself. The outer element is reached
through the `placement` axis, or by dropping to the parts:

```rust
DialogRoot { id: "confirm", placement: DialogPlacement::Bottom,
    DialogContent { class: "max-w-sm",
        // ...
    }
}
```

`DialogContent` has to stay a direct child of `DialogRoot`: daisyUI reaches the box from the
open modal through a child combinator, so an element between them would leave the box invisible.

`id` is a prop of its own on every part rather than an attribute that falls through, because the
primitive generates one and then uses it: to find the element to trap focus in, to decide
whether a click landed outside it, and to point `aria-labelledby` and `aria-describedby` at the
title and the description. An id arriving as an attribute would be written over the one the
primitive is still looking for.

There is no close part, because the primitive has none and needs none. A control inside the
dialog closes it through the primitive's context, which this component re-exports:

```rust
#[component]
fn CloseButton() -> Element {
    let ctx: DialogCtx = use_context();

    rsx! {
        Button { onclick: move |_| ctx.set_open(false), "Close" }
    }
}
```

## State bridging

**Tier 2** on the open state, and the first component with **lifted state** (ADR-0006).

daisyUI hides `.modal` outright (`visibility: hidden`, no pointer events, a transparent
backdrop) and reveals it only through `.modal.modal-open`, which matches no ARIA or `data-*`
attribute the primitive sets. The other selectors in that rule are for markup this component
does not produce: `[open]` and `:popover-open` need a native `dialog` element, `:target` needs
navigation by fragment, and `.modal-toggle:checked` needs the CSS-only checkbox. So the class
has to be emitted from Rust (ADR-0002 rules out CSS), which means `DialogRoot` has to know
whether the dialog is open.

It therefore owns that state: it seeds a signal from `default_open`, always hands the primitive
a controlled value, and intercepts `on_open_change` to update its own signal before calling the
caller's. A controlled caller and an uncontrolled one both work, and this component is the only
writer either way.

Two things fall out of doing it this way rather than mounting and unmounting the modal:

- **Exit animations play.** The class comes off while the element is still in the document,
  which is what the primitive's animation-aware unmounting waits on. Entering is daisyUI's
  `@starting-style` on the same rule, which needs the class to be there as the element is
  inserted, and it is, because the class is emitted from the same render that mounts it.
- **A control inside the dialog can still close it.** A call to the primitive's context comes
  back out through `on_open_change`, so the lifted state stays in step with the primitive's.

Nothing else needs bridging. The role, `aria-modal`, `aria-labelledby` and `aria-describedby`
are the primitive's, and daisyUI styles none of them.

## Axes

- `placement: DialogPlacement`: `modal-top`, `modal-middle`, `modal-bottom`, `modal-start`,
  `modal-end`, on the modal.
- `appearance: DialogTitleAppearance` on `DialogTitle`: `text-lg font-bold`.
- `appearance: DialogDescriptionAppearance` on `DialogDescription`: `py-4`.

`DialogPlacement::Default` emits nothing. It is close to `DialogPlacement::Middle` (both centre
the box) but not a synonym for it: daisyUI's explicit middle also caps the box's height at the
viewport less `5em`, where an unclassed modal lets it grow to the full viewport.

The two appearance axes invert the usual convention: their default value *emits* utilities and
their `None` value emits nothing. daisyUI has no class for a modal's title or description, so
what a caller would otherwise have to override is a Tailwind utility this component emitted,
and two utilities only tie, with the tie settled by generated-stylesheet order rather than by
the class attribute. Switching ours off is how a caller wins it (ADR-0004). Neither is named
`style`, which collides with the global HTML attribute.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it and the
browser specs assert over the same rendered set: for `placement`, by opening one dialog per
value, since every value of it is a full-viewport element. That last point narrows the
screenshot baselines: they cover the appearance axes, which are rendered side by side inside
one open dialog, but only the default placement, since a page cannot show six modals at once.

## Deviations

**A closed dialog renders nothing.** The primitive mounts the modal when it opens and unmounts
it once the exit animation has finished, where daisyUI's own modal is always in the document
and merely hidden. Nothing observable rests on the difference (a hidden modal takes no space,
no focus and no pointer events) but it does mean a page cannot be styled or measured against a
closed dialog, and that `.modal` without `modal-open` is a state that only exists on the way
out.

**The modal is a `div`, not a native `dialog` element.** That is the primitive's markup, and
daisyUI's rule covers it: `.modal.modal-open` is one of the five selectors that reveal a modal,
and it is the one that needs no particular element behind it. The top layer and the native
`::backdrop` are given up along with the element; daisyUI does not use either, since it draws
the dim on the modal itself and sets a `z-index` rather than promoting the element.

## daisyUI classes deliberately not used

- `modal-backdrop`: a form or an anchor rendered behind the box to catch clicks and close the
  modal by navigation. The primitive already dismisses on a click outside the box, so this would
  be a second dismissal mechanism competing with the first, and an element between the modal and
  the box for no styling.
- `modal-toggle`: the hidden checkbox the CSS-only modal is opened by. This component emits the
  modifier class instead, which is what the lifted state exists to make possible.
- `modal-open` is **not** in this list: it is emitted, and it is the whole of Tier 2 here.
- The responsive prefixes daisyUI generates for all of the above (`sm:modal`, `md:modal-box`,
  and so on): a caller reaches them through `class`, which concatenates.
