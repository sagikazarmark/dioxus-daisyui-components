# Hover card

A hover card styled with daisyUI's `card` classes, wrapping the `dioxus-primitives` hover card: a
panel of content (a picture, a heading, a paragraph) that opens when a link is pointed at or
focused. It opens on keyboard focus as well as on hover, and describes its trigger while it is
up, neither of which daisyUI's CSS-only dropdown has, and both of them the primitive's rather than
reimplemented here.

```rust
HoverCard {
    HoverCardTrigger { class: "link", "Ada Lovelace" }
    HoverCardPanel { class: "w-64", side: HoverCardSide::Bottom,
        HoverCardTitle { "Ada Lovelace" }
        p { "Wrote the first algorithm intended for a machine." }
    }
}
```

## Composition

The compound parts are `HoverCard` (the box the panel is positioned against), `HoverCardTrigger`
(what the card opens from), `HoverCardContent` (the panel, which is the `card`), `HoverCardBody`
(daisyUI's `card-body`) and `HoverCardTitle` (its `card-title`). `HoverCardPanel` is the collapsed
component over the panel and its body, which is the pair every card has.

The collapse is legal: daisyUI has nothing between `.card` and `.card-body`, and no caller content
goes there: its own examples put a figure *beside* the body rather than between the two, which is
a sibling and so still reachable through the parts.

**The body is not optional.** daisyUI pads a card in `.card-body` rather than in `.card`, so
content written straight into the panel is content with no padding. That is daisyUI's arrangement
rather than this component's, and it is why the collapsed component exists.

**The trigger emits nothing.** What a hover card hangs off is the caller's (a link, a name, an
avatar) and daisyUI has no class for it. The primitive's element is a `div` with `tabindex="0"`
and `role="button"`, which is what makes a card on plain text reachable by keyboard.

## State bridging

**There is no state to bridge.**

daisyUI's card is a box. It has no open state, no hidden state, and no class that reveals or
conceals anything; every one of its classes describes what the box looks like. The primitive
mounts the panel when the card opens and unmounts it once the exit animation has run, so there is
nothing for a modifier class to say. The open state travels through to the primitive untouched,
and a controlled caller and an uncontrolled one both get its own behaviour.

What daisyUI *would* have supplied is the other half of a floating panel, where it goes, and it
supplies that only for dropdowns. ADR-0015 records why a hover card cannot be one: daisyUI takes
pointer events off the first child of an open dropdown, and that first child here is the trigger
the card opens and closes on. So the positioning is Tailwind utilities, which is the other thing
this registry does where daisyUI has no class to reach for (ADR-0004).

## Axes

- `side: HoverCardSide` on the panel: `bottom-full`, `left-full`, `top-full`, `right-full`.
- `align: HoverCardAlign` on the panel: `left-0`, `left-1/2 -translate-x-1/2`, `right-0`,
  `top-0`, `top-1/2 -translate-y-1/2`, `bottom-0`.
- `size: HoverCardSize` on the panel: `card-xs`, `card-sm`, `card-lg`, `card-xl`.
- `border: HoverCardBorder` on the panel: `card-border`, `card-dash`.
- `positioning: HoverCardPositioning` on the panel: `absolute z-10`, plus the side and alignment
  utilities above.
- `appearance: HoverCardContentAppearance` on the panel: `bg-base-100 shadow-sm`.
- `appearance: HoverCardAppearance` on the root: `relative inline-block`.

The first two are utilities rather than daisyUI classes, for ADR-0015's reason. **The alignment
utility depends on the side as well**: `left-0` aligns the start edge of a panel that is above or
below the trigger, and `top-0` aligns the start edge of one that is beside it. The two stay
separate axes because that is how a caller thinks about them.

Both are also handed to the primitive, so the `data-side` and `data-align` it reports say the same
thing the utilities did. A caller who switches the positioning axis off keeps those attributes and
can position the panel from them.

`HoverCardSize::Default` emits nothing and renders at the same size as daisyUI's explicit
`card-md`. `HoverCardBorder::Default` emits nothing, which is daisyUI's borderless card.

The three appearance-shaped axes (`positioning`, and both `appearance`s) invert the usual
convention: their default value *emits* utilities and their `None` value emits nothing. daisyUI's
`card` rounds a box and lays it out but paints nothing, and it has no class at all for a
positioning context, so what a caller would otherwise have to override is a utility this component
emitted. Two utilities only tie, and a tie is settled by generated-stylesheet order rather than by
the class attribute, so switching ours off is how a caller wins it (ADR-0004). None of them is
named `style`, which collides with the global HTML attribute.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set, with every
card in those rows held open, since a closed one renders no panel to measure.

## Deviations

**No width is emitted.** Every card in daisyUI's own documentation carries one, because a card in
the flow of a page has no natural width and a floating one has none either. It is left to the
caller because a width is the single most content-dependent thing about a panel, and because a
utility emitted here would have to be defeated by every caller who disagreed rather than simply
set.

**No offset between the trigger and the panel.** daisyUI's dropdown has none, and here one would
be actively wrong: a gap is a gap the pointer crosses, and a pointer that has left the trigger
without arriving on the panel has closed the card.

**The title is a `div`, not a heading.** The primitive gives the panel `role="tooltip"` and
announces it through the trigger's `aria-describedby`, so its content is read out with the trigger
rather than walked as a document outline, and a heading element inside it would land in the
page's outline at whatever level it was given. A caller who wants a real heading writes one as a
child of the title, or in place of it.

**A closed card renders no panel.** The primitive mounts the panel when the card opens and
unmounts it once the exit animation has run, where daisyUI's own dropdown keeps its content in the
document at `opacity: 0`. Nothing observable rests on the difference (a faded panel takes no
pointer events and is not read out) but it does mean a page cannot be measured against a closed
card.

**The pointer cannot travel from the trigger into the panel.** The primitive closes the card the
moment the pointer leaves the trigger and unmounts the panel with it, so the panel's own
`mouseenter` (which is written to keep the card open) arrives at an element that is no longer
there. There is no grace period to widen and no gap to close: the panel is flush against the
trigger, and moving onto it still closes the card. Every engine behaves the same way, and the
browser spec asserts it rather than wishing otherwise.

**A hover card is therefore for reading, not for reaching into.** It closes on blur and on the
pointer leaving, it traps no focus, and its content is reachable neither by pointer nor by tabbing
into it. Anything a user has to *use* (a button, a link they must follow) belongs in a dialog or
a popover instead. What belongs here is what a reader wants to *see*: a name, a picture, a
sentence of context.

## daisyUI classes deliberately not used

- `dropdown`, `dropdown-content` and the positioning family: ADR-0015. This is the one thing
  daisyUI has for a floating box, and the reason it is unusable here is written down rather than
  left to be rediscovered.
- `card-side`: a card whose figure sits beside its body rather than above it. It is a layout for
  the caller to choose and it reaches the panel through `class`, which concatenates.
- `image-full`: daisyUI's card with the picture behind the text. Same reason.
- `card-actions`: the row a card's buttons sit in. A hover card is not a place to put buttons
  (see the deviations), and a caller who has decided otherwise writes the class themselves.
- `card-md`: the size value the default renders at, which is what emitting nothing already does.
- The responsive prefixes daisyUI generates for the size and border classes (`sm:card-lg` and the
  rest): a caller reaches those through `class`, which concatenates.
