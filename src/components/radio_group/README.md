# Radio group

A radio group styled with daisyUI's `radio` classes, wrapping the `dioxus-primitives` radio
group. One tab stop for the whole group, arrow keys to move the selection, disabled items
skipped, all of it the primitive's rather than reimplemented here.

```rust
RadioGroup {
    default_value: "weekly".to_string(),
    on_change: move |value| {},
    on_commit: move |()| {},
    aria_labelledby: "digest",

    div { class: "flex items-center gap-2",
        RadioItem {
            value: "daily".to_string(),
            index: 0usize,
            color: RadioItemColor::Primary,
            aria_labelledby: "digest-daily",
        }
        span { id: "digest-daily", "Every day" }
    }
    // …
}
```

## Composition

The compound parts are `RadioGroup` and `RadioItem`. There is no collapsed component: a group
is a list of items, so there is nothing to collapse.

An item renders nothing inside itself. daisyUI draws the dot from a `::before` on the item, so
the text beside it is the caller's markup, which is also what lets a caller lay a row out any
way they like. The item is named by `aria_label` or by `aria_labelledby`, and the group by
either as well.

`index` orders the keyboard navigation. It is the primitive's prop and is required, because the
primitive registers items by the index they are given rather than by where they sit in the DOM.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, the Component resolves
Field Context and then standalone state. Field metadata belongs to the `role="radiogroup"` root:
it supplies the one value's id, ARIA required and disabled state, ARIA relationships, and `data-*`
state. Its name is omitted from that `div` and applied to registry-owned hidden form participants.
Per-option `RadioItem` ids remain independent. Existing explicit props and caller attributes win
over metadata. `data-field-group` is the one reserved attribute: it is a generated internal locator
for Field focus requests rather than caller state.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<String>` wins over Field Context;
without either, the control owns standalone state seeded by `default_value`. The lower-level trio
remains available: a `Some` value in `value: ReadSignal<Option<String>>` overrides the rendered
Binding value, `on_change` observes each user choice, and `on_commit` observes every completed
choice. Writes and commits still reach the resolved Binding. Choosing one radio is one complete
interaction unit, so change and commit have the same cadence. Global attributes continue to spread
onto the Primitive's group root.

The complete logical focus scope is every `RadioItem` owned by the group. A choice Commits
immediately while focus remains on an item. Arrow, Home, and End movement between owned items stays
inside the scope and does not report Focus Exit; leaving all items does. Each actual exit calls the
resolved Binding's Focus Exit capability and then the optional dependency-free `on_focus_exit`
prop, once each. Focus Exit remains independent from Commit and does not itself infer touched,
blurred, or validation semantics.

## State bridging

**Tier 1** on the chosen state: the primitive sets `aria-checked` on the `button` it renders for
each item, and daisyUI's rule is `.radio:checked, .radio[aria-checked=true]`. The attribute
daisyUI already matches is exactly there, so there is nothing to bridge.

The disabled state needs no bridging either, though it is not Tier 1: that tier is about ARIA
attributes, and this is a native one. The primitive sets `disabled` on the `button` and
daisyUI's rule is `.radio:disabled`.

**Tier 2** on producer-defined invalidity: when a RadioItem's colour Axis is omitted and Field
metadata is invalid, the Component emits `radio-error` on that item. Passing any colour value
explicitly, including `RadioItemColor::Default`, wins over metadata. The Registry does not compute
invalidity.

The selected and disabled states emit **no class at all**. Only producer-defined invalidity needs a
modifier class and, per ADR-0002, the Component ships no CSS of its own.

## Axes

- `color: Option<RadioItemColor>` on `RadioItem`: `radio-neutral`, `radio-primary`,
  `radio-secondary`, `radio-accent`, `radio-info`, `radio-success`, `radio-warning`, `radio-error`;
  omission permits invalid Field metadata to emit `radio-error`.
- `appearance: RadioGroupAppearance` on `RadioGroup`: `flex gap-3
  data-[orientation=vertical]:flex-col`.

`RadioItemColor::Default` emits nothing, which is daisyUI's uncoloured radio, a distinct look
from `radio-neutral`.

**There is no size axis**, which the next section is about.

The appearance axis inverts that convention: its default value *emits* utilities and its `None`
value emits nothing. daisyUI has no class for the group, so what a caller would otherwise have
to override is a Tailwind utility this component emitted, and two utilities only tie, with the
tie settled by generated-stylesheet order rather than by the class attribute. Switching ours off
is how a caller wins it (ADR-0004). It is not named `style`, which collides with the global HTML
attribute.

The layout follows the `horizontal` prop without reading it, through a variant on the
`data-orientation` attribute the primitive already sets, so the axis stays one string literal
and the orientation stays one prop, rather than the two having to be kept in step.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render
every axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The Primitive's callback name is adapted to the Binding trio.** Its `on_value_change` callback is
exposed as `on_change`, and the Component emits `on_commit` immediately after each change because a
radio choice is one complete interaction unit. This is a deliberate breaking rename for the
Registry's trio contract.

**Field metadata lands on the group, not on its items.** The Binding represents the group's one
chosen value, and `role="radiogroup"` is where its id, ARIA required and invalid state, description
and error relationships belong. Repeating them on items would duplicate ids and misstate
per-choice validity. A `RadioItem`'s explicit `id` remains its per-option id; metadata only reaches
items as invalid paint when their colour is omitted. Because the group root is a `div`, its name is
reserved for the hidden form participants and `FieldLabel` names the group through
`aria-labelledby`; the Field focus request, rather than native label activation, reaches the
roving item.

**Field focus requests target the Primitive's roving item.** Inspection confirmed that the
Primitive owns each item's mounted listener and derives `tabindex="0"` from the selected or most
recently focused item. Dioxus invokes only one same-name listener on an element, so capturing an
item from a second mounted listener would disable the Primitive's arrow, Home and End focus moves.
The group therefore carries a generated internal locator, and a focus request queries it directly
for the enabled descendant whose Primitive-owned tabindex is zero. The group remains a
programmatic-only `tabindex="-1"`, but is never focused as a proxy. When no item can take focus, the
request does nothing and preserves the user's current focus.

**daisyUI's size classes cannot size this element.** They are written twice:

```css
.radio-lg           { padding: 0.3125rem }
.radio-lg[type=radio] { --size: calc(var(--size-selector, 0.25rem) * 7) }
```

and `.radio` itself sets `width` and `height` from `--size`. The primitive renders a `button`
with `role="radio"`, so only the first arm can match: `radio-lg` would leave the control at the
size an unclassed radio is and pad the dot inside it *down*. A size axis whose largest value
draws the smallest dot is worse than none, so none is exposed. This is the switch's gap in a
second guise and is recorded with it as ADR-0010; the checkbox is genuinely unaffected, because
daisyUI writes its sizes as plain class selectors that set `--size` outright.

A caller who needs another size sets daisyUI's own custom property through a utility, which
concatenates onto the element the classes are already on:

```rust
RadioItem { class: "[--size:calc(var(--size-selector,0.25rem)*7)]" }
```

That is the caller's decision to take on a daisyUI internal, deliberately not one this registry
takes on their behalf.

**A named radio group submits through registry-owned hidden radio inputs.** The pinned Primitive's
`name` prop is a no-op and its `div` root cannot participate in a form, so each `RadioItem` mirrors
its value, selected state and effective disabled state into a visually hidden native radio. Their
shared name resolves explicit and Field metadata names. Unnamed groups render no native form
participants, and only a selected, enabled item contributes an entry. The hidden radios do not
carry `required`, so that prop remains the group's ARIA and Field metadata state rather than a
native submission constraint.

**Arrow keys move the selection, not just the focus.** That is the primitive's behaviour and
matches native radio groups, where focusing an option chooses it. It is worth knowing before
building a group whose options are expensive to act on.

## daisyUI classes deliberately not used

- `radio-xs`, `radio-sm`, `radio-md`, `radio-lg`, `radio-xl`: the size axis above. Not a matter
  of taste: on this element they change the padding and not the size.
- `label` and `fieldset-label`: daisyUI's wrappers for laying a control out next to its text.
  That markup is the caller's, and emitting it here would put an element between the caller and
  the item for nothing. It is also why the item takes no children.
- `join` and its `join-item`: daisyUI's way of welding controls into one block. It is a layout
  choice for the caller to make on the group, not one this component should make for them.
- `validator`: pinned daisyUI can read Field metadata's `aria-invalid`, but each item already emits
  its explicit `radio-error` modifier from the same state when colour is omitted. Adding `validator`
  would duplicate that Axis and also opt into browser-owned valid paint.
- `validator-hint`: its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- The responsive prefixes daisyUI generates for the radio classes (`sm:radio-primary` and the
  rest): a caller reaches them through `class`, which concatenates.
