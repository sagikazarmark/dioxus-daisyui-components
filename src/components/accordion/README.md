# Accordion

An accordion styled with daisyUI's `collapse` classes, wrapping the `dioxus-primitives`
accordion. Its triggers are real buttons that announce what they open and move between each
other with the arrow keys, none of which daisyUI's CSS-only collapse has, and all of it the
primitive's rather than reimplemented here.

[Live examples](https://daisyui-components.dioxus.cc/components/accordion) ·
[their sources](docs/examples/)

## Composition

The compound parts are `Accordion` (the set), `AccordionItem` (one collapse), `AccordionTrigger`
(its title) and `AccordionContent` (its panel). There is no collapsed component: daisyUI's own
collapse markup has all three elements and caller content goes in two of them, so there is
nothing legal to collapse.

The trigger and the content must stay direct children of the item. daisyUI lays the two out as
the rows of one grid and reveals the panel through `.collapse-open > .collapse-content`, so a
wrapper around either would leave the item unstyled and the panel shut.

`index` orders the keyboard navigation. It is the primitive's prop and is required, because
triggers register with the focus collection by the index they are given rather than by where
they sit in the DOM.

Whether opening one item closes another is the set's decision, through the primitive's
`allow_multiple_open` and `collapsible` props. An item never decides it.

## State bridging

**Tier 2** on the open state, per item.

daisyUI reveals a panel through `.collapse-open` on the item (it sets the grid row the panel
lives in from `0fr` to `1fr` and lifts the `content-visibility: hidden` off it), and it matches
nothing the primitive sets. The `data-open` attribute the primitive puts on the item is not in
any daisyUI selector, and neither is the `aria-expanded` on the trigger. The other ways daisyUI
opens a collapse are for markup this component does not produce: `[open]` needs a native
`details`, the `:checked` rules need its CSS-only checkbox, and the `:focus-within` rules need
the item to carry a `tabindex`. So the class has to be emitted from Rust (ADR-0002 rules out
CSS), which means `AccordionItem` has to know whether it is open.

**The state is mirrored, not lifted** (ADR-0011). Every other component in this registry that
emits a modifier class *owns* the state it emits from: it seeds a signal from a `default_*`
prop, hands the primitive a controlled value, and intercepts the change callback. That is not
available here, because the primitive's item takes no controlled `open` prop at all; the
accordion root owns which items are open, so that it can enforce `allow_multiple_open` and
`collapsible`. An item that owned its own state could disagree with the set about it.

So the item does the reverse: it seeds a signal from `default_open` (the same prop the
primitive is seeded from, so the class is right on the first render rather than one callback
later), and updates that signal from `on_change`, which the primitive fires on every change.
The primitive stays the only writer, and this component only reads.

Two things fall out of it, both of which the lifted components get for the same reason:

- **The closing animation plays.** The class comes off while the panel is still in the document,
  which is what the primitive's animation-aware unmounting waits on.
- **A caller's own `on_change` still fires**, because this component intercepts the callback and
  calls it rather than replacing it.

Nothing else needs bridging. The roles, `aria-expanded` and `aria-controls` are the primitive's,
and daisyUI styles none of them.

## Axes

- `marker: AccordionItemMarker` on `AccordionItem`: `collapse-arrow`, `collapse-plus`.
- `appearance: AccordionAppearance` on `Accordion`: `flex flex-col gap-2`.
- `appearance: AccordionItemAppearance` on `AccordionItem`: `border border-base-300
  bg-base-100`.
- `appearance: AccordionTriggerAppearance` on `AccordionTrigger`: `cursor-pointer text-start
  font-medium`.

`AccordionItemMarker::Default` emits nothing, which is daisyUI's unmarked collapse: a title with
nothing in its corner.

The three appearance axes invert the usual convention: their default value *emits* utilities and
their `None` value emits nothing. daisyUI's `collapse` lays an item out and animates it but
paints nothing, and it has no class at all for a set of items, so what a caller would otherwise
have to override is a Tailwind utility this component emitted, and two utilities only tie, with
the tie settled by generated-stylesheet order rather than by the class attribute. Switching ours
off is how a caller wins it (ADR-0004). None of them is named `style`, which collides with the
global HTML attribute.

The trigger's axis exists because of the element swap below: a `button` centres its label and,
through daisyUI's own `.collapse > .collapse-title { cursor: unset }`, arrives without a
pointer.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render
every axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The title is a `button`, where daisyUI's is a `div`.** That is the whole reason for wrapping a
primitive here: daisyUI's collapse is opened by a hidden checkbox or by `details`, and neither
announces itself as a control that expands something. The primitive's trigger is focusable,
carries `aria-expanded` and points `aria-controls` at the panel. What the swap costs is the
`div`'s text alignment and pointer, which the trigger's appearance axis puts back.

**A closed panel renders nothing.** The primitive mounts a panel when its item opens and
unmounts it once the closing animation has run, where daisyUI's own collapse keeps the panel in
the document and hides it. Nothing observable rests on the difference (a hidden panel takes no
space and no focus), but it does mean a page cannot be measured against a closed panel, and that
`.collapse-content` without `.collapse-open` is a state that only exists on the way out.

**A disabled item has no daisyUI look.** daisyUI has no disabled state for a collapse at all.
The primitive disables the trigger and reports `data-disabled` on the item, so the item is inert
and skipped by the arrow keys, and a caller who wants it to *look* inert reaches for
`data-[disabled=true]:opacity-50` or similar through `class`. Emitting one here would be this
registry inventing a look daisyUI does not have.

**`on_change` reports a state, not a change.** The primitive fires it from an effect over the
set's own state, so every item reports whenever that state is re-evaluated: on the first
render, and when some *other* item was the one that moved. This component's mirror is idempotent
and does not care, and a caller's handler has to be written the same way: write the value into a
slot rather than counting transitions with it. The preview's own example does exactly that, and
says so.

**`horizontal` does not lay the items out.** It is the primitive's prop and it decides which
arrow keys move between triggers. daisyUI has no horizontal collapse, so the layout this
component emits is a column either way, the same trade the tabs component records for its own
orientation, and documented here rather than silently ignored.

## daisyUI classes deliberately not used

- `collapse-open` is **not** in this list: it is emitted, and it is the whole of Tier 2 here.
- `collapse-close`: daisyUI's way of forcing a collapse shut against its own `:focus-within`
  rules. Those rules only apply to an item carrying a `tabindex`, which this component never
  gives it, so there is nothing to force.
- `collapse-title`'s sibling form for the CSS-only collapse: the `input[type=checkbox]` and
  `input[type=radio]` markup daisyUI opens a collapse with, and the `details`/`summary` markup
  it offers as the alternative. Both are ways of getting behaviour without JavaScript, which is
  exactly what the primitive is here for.
- `join` and `join-vertical`: daisyUI's way of welding items into one block with shared
  corners. It is a layout choice for the caller to make on the set, and it reaches the set
  through `class`, which concatenates.
- The responsive prefixes daisyUI generates for all of the above (`sm:collapse-arrow` and the
  rest): a caller reaches them through `class`.
