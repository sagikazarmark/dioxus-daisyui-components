# Collapsible

A disclosure styled with daisyUI's `collapse` classes, wrapping the `dioxus-primitives`
collapsible. Its title is a real button that announces what it opens, and its panel is mounted
only while it is open, neither of which daisyUI's CSS-only collapse has, and both of them the
primitive's rather than reimplemented here.

```rust
Collapsible { marker: CollapsibleMarker::Arrow,
    CollapsibleTrigger { "Recent activity" }
    CollapsibleContent { "Fixed a bug in the collapsible component." }
}
```

## Composition

The compound parts are `Collapsible` (the disclosure), `CollapsibleTrigger` (its title) and
`CollapsibleContent` (its panel). There is no collapsed component: daisyUI's own collapse markup
has all three elements and caller content goes in two of them, so there is nothing legal to
collapse.

The trigger and the content must stay direct children of the disclosure. daisyUI lays the two out
as the rows of one grid and reveals the panel through `.collapse-open > .collapse-content`, so a
wrapper around either would leave the disclosure unstyled and the panel shut.

**This is the accordion's item without the accordion.** The two carry the same daisyUI classes and
the same axes, and a caller who wants several of these to answer to each other (one open at a
time, arrow keys between the titles) wants the accordion instead. A collapsible answers to
nobody, which is why it has no `index`.

## State bridging

**Tier 2** on the open state.

daisyUI reveals a panel through `.collapse-open` on the root (it sets the grid row the panel
lives in from `0fr` to `1fr` and lifts the `content-visibility: hidden` off it), and it matches
nothing the primitive sets. The `data-open` attribute the primitive puts on all three elements is
in no daisyUI selector, and neither is the `aria-expanded` on the trigger. The other ways daisyUI
opens a collapse are for markup this component does not produce: `[open]` needs a native
`details`, the `:checked` rules need its CSS-only checkbox, and the `:focus-within` rules need the
root to carry a `tabindex`. So the class is emitted from Rust (ADR-0002 rules out CSS), which
means `Collapsible` has to know whether it is open.

**The state is lifted, where the accordion item's is mirrored** (ADR-0006, ADR-0011). That is the
primitive's difference rather than a change of convention: an accordion item takes no controlled
`open` prop, because the set owns which items are open so that it can enforce
`allow_multiple_open` and `collapsible`, and an item that owned its own state could disagree with
the set about it. A collapsible answers to nobody and takes one. So this component does what the
dialog and the dropdown do: it seeds a signal from `default_open`, always hands the primitive a
controlled value, and intercepts `on_open_change` to update that signal before calling the
caller's. A controlled caller and an uncontrolled one both work, and this component is the only
writer either way.

The disabled state needs no bridging and gets none: the primitive puts the `disabled` attribute on
the trigger `button`, which is a native attribute rather than an ARIA one, and daisyUI has no
disabled collapse to match it with either way. See the deviations.

## Axes

- `marker: CollapsibleMarker`: `collapse-arrow`, `collapse-plus`.
- `appearance: CollapsibleAppearance` on `Collapsible`: `border border-base-300 bg-base-100`.
- `appearance: CollapsibleTriggerAppearance` on `CollapsibleTrigger`: `cursor-pointer text-start
  font-medium`.

Every one of these is the accordion's, with the class strings duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires
every one of these classes to be a literal in the file that emits it.

`CollapsibleMarker::Default` emits nothing, which is daisyUI's unmarked collapse: a title with
nothing in its corner.

The two appearance axes invert the usual convention: their default value *emits* utilities and
their `None` value emits nothing. daisyUI's `collapse` lays a disclosure out and animates it but
paints nothing, so what a caller would otherwise have to override is a Tailwind utility this
component emitted, and two utilities only tie, with the tie settled by generated-stylesheet order
rather than by the class attribute. Switching ours off is how a caller wins it (ADR-0004). Neither
is named `style`, which collides with the global HTML attribute.

The trigger's axis exists because of the element swap below: a `button` centres its label and,
through daisyUI's own `.collapse > .collapse-title { cursor: unset }`, arrives without a pointer.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The title is a `button`, where daisyUI's is a `div`.** That is the whole reason for wrapping a
primitive here: daisyUI's collapse is opened by a hidden checkbox or by `details`, and neither
announces itself as a control that expands something. The primitive's trigger is focusable,
carries `aria-expanded` and points `aria-controls` at the panel. What the swap costs is the
`div`'s text alignment and pointer, which the trigger's appearance axis puts back.

**A closed panel is an empty panel, not an absent one.** The primitive always renders the panel
element (it is a row of daisyUI's grid, and the transition runs on it) but mounts what the
caller wrote inside it only while the disclosure is open. daisyUI's own collapse keeps the content
in the document and hides it. `keep_mounted` is the primitive's prop for the difference and is
passed through: with it set, the children stay mounted while closed, which is what a caller who
measures the panel, pre-renders it, or holds state inside it wants.

What a kept panel *looks* like while closed is daisyUI's business: the row it lives in is
`0fr` tall and its contents are `content-visibility: hidden`. Engines disagree about the box they
report for an element inside that (WebKit gives it one where Chromium and Firefox do not), so
code that has to know whether a kept panel is on screen should ask the disclosure whether it is
open rather than measure the panel.

**A disabled collapsible has no daisyUI look.** daisyUI has no disabled state for a collapse at
all. The primitive disables the trigger button and reports `data-disabled` on all three elements,
so the control is inert and skipped by the keyboard, and a caller who wants it to *look* inert
reaches for `data-[disabled=true]:opacity-50` or similar through `class`. Emitting one here would
be this registry inventing a look daisyUI does not have.

**There is no horizontal collapsible.** daisyUI has no horizontal collapse and the primitive
offers no orientation, so nothing is lost; it is recorded here only because the accordion, which
does take an orientation, documents that daisyUI ignores it.

## daisyUI classes deliberately not used

- `collapse-open` is **not** in this list: it is emitted, and it is the whole of Tier 2 here.
- `collapse-close`: daisyUI's way of forcing a collapse shut against its own `:focus-within`
  rules. Those rules only apply to a root carrying a `tabindex`, which this component never gives
  it, so there is nothing to force.
- `collapse-title`'s sibling form for the CSS-only collapse: the `input[type=checkbox]` and
  `input[type=radio]` markup daisyUI opens a collapse with, and the `details`/`summary` markup it
  offers as the alternative. Both are ways of getting behaviour without JavaScript, which is
  exactly what the primitive is here for.
- `join` and `join-vertical`: daisyUI's way of welding several collapses into one block with
  shared corners. That is a decision about a *set*, which this component is not; a caller stacking
  collapsibles reaches it through `class`, which concatenates.
- The responsive prefixes daisyUI generates for all of the above (`sm:collapse-arrow` and the
  rest): a caller reaches them through `class`.
