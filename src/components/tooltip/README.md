# Tooltip

A tooltip styled with daisyUI's `tooltip` classes, wrapping the `dioxus-primitives` tooltip. It
opens on hover *and* on keyboard focus, dismisses on Escape, and is announced through
`aria-describedby`, none of which daisyUI's CSS-only tooltip has, and all of which is the
primitive's rather than reimplemented here.

```rust
Tooltip {
    color: TooltipColor::Primary,
    side: TooltipSide::Bottom,
    TooltipTrigger {
        r#as: move |attributes: Vec<Attribute>| rsx! {
            Button { attributes, "Deploy" }
        },
    }
    TooltipContent { "Ships the build on main" }
}
```

## Composition

The compound parts are `Tooltip` (the outer element), `TooltipTrigger` (what the tooltip is
attached to) and `TooltipContent` (the bubble). There is no collapsed component: daisyUI's own
markup has all three elements and caller content goes in two of them, so there is nothing legal
to collapse.

`TooltipContent` must stay a direct child of `Tooltip`. Every rule that paints, positions or
reveals the bubble reaches it through `.tooltip > .tooltip-content`.

The trigger emits nothing at all. daisyUI's tooltip styles the bubble and positions it against
the element carrying `tooltip`, and has nothing to say about what is inside, so a trigger that
should look like something is the caller's. `as` is how a control becomes the trigger rather
than sitting inside one: the primitive hands the whole attribute list to a callback that renders
the element in the trigger's place. The primitive's own element is a `div` with `tabindex="0"`,
which is what makes a tooltip on plain text reachable by keyboard; an element rendered through
`as` should be focusable by being the kind of element that already is.

**Where the bubble sits is a prop of `Tooltip`, not of `TooltipContent`.** daisyUI decides it
from classes on the outer element, so that is where the axes are, and the values travel down to
the primitive from there, so the `data-side` and `data-align` it reports on the bubble say the
same thing daisyUI did. A caller says it once.

## State bridging

**Tier 2** on the open state, with **lifted state** (ADR-0006).

daisyUI reveals a bubble through three selectors on the outer element: `.tooltip-open`,
`:hover`, and `:has(:focus-visible)`. The last two agree with when the primitive opens on its
own (it opens on `mouseenter` and on `focus`, and the trigger is inside this element) so a
tooltip that is only ever pointed at or tabbed to would look right with no class emitted at all.

That is not enough. A tooltip a caller *controls*, and one that opens with the page, reach
neither selector: the pointer is elsewhere and nothing is focused, so daisyUI would keep the
bubble at `opacity: 0` while the primitive had it mounted and announced. So the class is emitted
from Rust (ADR-0002 rules out CSS), and `Tooltip` owns the state to emit it: it seeds a signal
from `default_open`, always hands the primitive a controlled value, and intercepts
`on_open_change` to update its own signal before calling the caller's. A controlled caller and
an uncontrolled one both work, and this component is the only writer either way.

The exit animation falls out of it, as it does for the dialog: the class comes off while the
bubble is still in the document, which is what the primitive's animation-aware unmounting waits
on.

Nothing else needs bridging. The `role="tooltip"`, the `aria-describedby` and the Escape handler
are the primitive's, and daisyUI styles none of them.

## Axes

- `color: TooltipColor`: `tooltip-primary`, `tooltip-secondary`, `tooltip-accent`,
  `tooltip-info`, `tooltip-success`, `tooltip-warning`, `tooltip-error`.
- `side: TooltipSide`: `tooltip-top`, `tooltip-bottom`, `tooltip-left`,
  `tooltip-right`.
- `align: TooltipAlign`: `tooltip-start`, `tooltip-center`, `tooltip-end`.

**The colour axis has no neutral value.** daisyUI fills an unclassed tooltip with
`--color-neutral` and ships no `tooltip-neutral` to ask for it by name, so `TooltipColor::Default`
*is* the neutral one, which is the opposite of what `Default` means on every other component
here, and is daisyUI's decision rather than this registry's.

**Side and align emit a class for every value, including their defaults.** That is the
inverse of the usual convention and rhymes with ADR-0008 without being the same case. daisyUI's
base rule places the *bubble* exactly where `tooltip-top` places it, so on the bubble alone the
class would be redundant. The *tail* is a pseudo-element of the outer element, and the base rule
gives it no position at all; every inset it has comes from a side or align class. An
unplaced tooltip therefore draws a correct bubble with its tail somewhere else, which is worse
than either half being wrong on its own.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render
every axis value and the browser specs assert computed styles over the same rendered set, with
every tooltip in those rows held open, since a closed one renders no bubble to measure.

## Deviations

**A closed tooltip renders no bubble.** The primitive mounts the bubble when the tooltip opens
and unmounts it once the exit animation has run, where daisyUI's own tooltip keeps it in the
document at `opacity: 0`. Nothing observable rests on the difference (a faded bubble takes no
pointer events and is not read out) but it does mean a page cannot be measured against a closed
tooltip, and that `.tooltip-content` without `.tooltip-open` is a state that only exists on the
way out.

**The bubble is an element, not `content: attr(data-tip)`.** daisyUI offers both, and only one
of them can hold markup, be given an id, or be pointed at by `aria-describedby`. The `data-tip`
form is left to callers who want a decoration rather than a described control.

**The bubble is capped at `20rem`.** daisyUI sets a `max-width` on it, which is not something a
caller's `width` utility can out-rank: the two are different properties, so the cap wins however
the cascade resolves. A wider bubble takes `max-w-none` alongside the width. Everything else
daisyUI puts on the bubble is an ordinary class that loses to a caller's on cascade layers.

**A tooltip is not a live region.** The bubble is announced because the trigger is described by
it, which means it is read when the trigger takes focus rather than when the bubble appears. A
message that has to interrupt is not a tooltip.

## daisyUI classes deliberately not used

- `tooltip-open` is **not** in this list: it is emitted, and it is the whole of Tier 2 here.
- `data-tip`: not a class, but daisyUI's other way of writing a tooltip, and the reason
  `.tooltip-content` exists at all. See the deviation above.
- The responsive prefixes daisyUI generates for all of the above (`sm:tooltip-right`,
  `lg:tooltip-primary`, and so on): a caller reaches them through `class`, which concatenates.
  A responsive placement is worth knowing about: it is the one way to move a bubble that would
  otherwise leave a narrow viewport.
