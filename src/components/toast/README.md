# Toast

Toasts styled with daisyUI's `toast` and `alert` classes, wrapping the `dioxus-primitives` toast
provider.

```rust
// Once, around everything that dispatches:
ToastProvider { inline: ToastInline::End, block: ToastBlock::Bottom,
    App {}
}

// And anywhere under it, in a component of its own:
let toast = use_toast();

rsx! {
    Button {
        onclick: move |_| {
            toast.success(
                "Deployed".to_string(),
                ToastOptions::new().description("Live in eu-west-1"),
            );
        },
        "Deploy"
    }
}
```

This is the only component in the registry that is not written where it appears. A caller mounts
the provider once and dispatches from anywhere below it; the toasts are rendered through a portal,
so where they land on screen is the provider's classes rather than where the call came from.

**Whatever dispatches has to be a component under the provider.** `use_toast` reads the
provider's context, and a component cannot read a context it provides itself, so the button in a
caller's `App` works, and a button written beside `ToastProvider` in the same function does not.
The preview's examples each have a small dispatcher component for exactly this reason.

**One provider per app.** Each one pins its own region to the viewport and registers the `F6`
shortcut that focuses it, so two of them are two stacks in a corner and a shortcut whose target
depends on which listener runs last. The preview mounts one per example because each example
configures its own; an app mounts one, around everything.

The primitive's dispatch API is re-exported here (`use_toast`, `Toasts`, `ToastOptions` and
`ToastType`) so a caller needs one import.

## State bridging

**Tier 2** on the kind of toast. The primitive reports it as `data-type="success"` on the toast
element, and daisyUI's alert colours match no data attribute at all, so the class is emitted from
Rust through `ToastColor::of`.

There is no state to lift and none to mirror, which makes this the plainest Tier 2 in the
registry: the queue is the primitive's, and it hands each toast's kind to the render callback as a
prop. This component reads a prop and returns a class.

Two things that look like state are not. A toast's *duration* is a timer the primitive owns and
daisyUI has no styling for. Its *position in the stack* (`data-top`, `data-toast-even`,
`data-toast-odd`, `--toast-index`) is offered for stacked-card effects the registry does not
build; daisyUI's toast is a plain column.

## Axes

- `inline: ToastInline`: `toast-start`, `toast-center`, `toast-end`.
- `block: ToastBlock`: `toast-top`, `toast-middle`, `toast-bottom`.
- `ToastColor`: `alert-info`, `alert-success`, `alert-warning`, `alert-error`.
- `appearance: ToastListAppearance`: whether the provider emits the utilities that lay the
  toasts out.
- `appearance: ToastTitleAppearance`, `appearance: ToastDescriptionAppearance`: whether those
  parts emit the utilities daisyUI has no classes for.

The two position axes emit a class for every value, the defaults included, following ADR-0008.
`ToastColor` is a **derived axis**: nobody passes it, because a toast's kind is decided at the
call that dispatches it and the element it styles is rendered by this component rather than by
the caller. It is an axis in every other way (a class per value, and a variant list the preview
renders every one of) and `ToastColor::of` is the bridging itself.

Every enum exposes `ALL`. The preview iterates it to render every axis value and the browser specs
assert computed styles over the same rendered set.

## Deviations

**The provider styles a list it cannot reach.** daisyUI's `.toast` stacks and spaces its *own
children*, and the primitive puts an `ol` and a `li` between the region and each toast, neither
of which takes an attribute from anywhere. The container class stays on the region, where the
fixed positioning belongs, and the column and the gap are re-emitted as utilities that reach
through the list. Recorded as ADR-0014.

**The per-toast animation is lost.** daisyUI animates `.toast > *`, which here is the list rather
than each toast, so a toast arriving while another is up slides nothing. The registry adds no
animation of its own (ADR-0002), so this is documented rather than replaced.

**The toast component takes the primitive's props rather than its own.** `Toast` is a plain
function over `ToastProps`, not a component with a props list of its own, because the provider
builds those props for every toast in its queue and `render_toast` hands them over; a component
with different props could not be handed them. That is also why its axes are on its parts rather
than on it: there is no call site to pass them at.

**daisyUI's toast has no stacking order of its own**, which is worth knowing before you meet it:
`.toast` is `position: fixed` with no `z-index`, so a sticky header or a positioned sidebar can
come out over the toasts. Nothing is emitted for it here (a `z-index` is a decision about the
app's own layers rather than about a toast) and a caller who needs one passes `class: "z-40"` on
the provider. The preview does exactly that for the region it pins to the top.

**The close button is chrome rather than an axis.** It carries `btn btn-ghost btn-xs btn-circle`,
duplicated from the button component as the spec requires, and exposes no colour or size of its
own. A caller who wants another look passes `class`; a caller who wants another *button* passes
their own `render_toast`.

## daisyUI classes deliberately not used

- `alert-soft`, `alert-outline`, `alert-dash`: daisyUI's quieter alert styles. A toast is an
  interruption and the solid fill is what makes it read as one; a caller who disagrees passes
  `class` on the toast through their own `render_toast`.
- `alert-vertical` and `alert-horizontal`: daisyUI's own responsive helpers for an alert whose
  content stacks. A toast's content is a title and a line under it, which the block box inside the
  alert already stacks.
- `toast` on anything but the region: daisyUI's container is one element, and this component has
  exactly one.
- The responsive prefixes daisyUI generates for the inline, block and colour classes
  (`sm:toast-start` and the rest): a caller reaches those through `class`, which concatenates.
