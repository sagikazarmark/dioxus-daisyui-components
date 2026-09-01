# Input

A native text field styled with daisyUI's `input` classes. It renders a real `input`, accepts
native input attributes, and wraps no Primitive.

[Live examples](https://daisyui-components.dioxus.cc/components/input) ·
[their sources](docs/examples/)

`InputField` is the closed happy-path composition over the separate Field parts.

It renders `Field`, `FieldLabel`, `Input`, the optional `FieldDescription`, and an always-mounted
`FieldError`, in that order. It has no children slot. Use `Field` plus its Compound parts when
content or attributes must land between those elements. Generated part ids provide the accessible
relationships without id props on `InputField`.

Classes passed by the caller concatenate with the Component's own. Every other attribute the
caller passes overrides the Component's, so `type`, `name`, `placeholder`, `required`, `disabled`,
form attributes, and ARIA or data attributes reach the real control. `value`, `on_change`, and
`on_commit` are explicit props because they form the control's Binding contract and Dioxus'
extended attributes do not include event handlers.

`InputField` forwards the complete control surface to its `Input`: the colour, size, and appearance
Axes; binding, metadata, value, change, commit, required, and disabled props; the `prefix`,
`suffix`, and `wrapper_attributes` adornment surface; and global/native input attributes. Its
`class` and every native attribute therefore land on the input, not the surrounding Field.
`field_appearance`, `description_appearance`, and `error_appearance` forward the corresponding
Field-part Axes. The adorned wrapper is inside the control's own box rather than another part;
styling a different Field part is the signal to use the parts directly.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, the Component resolves
Field Context and then standalone state. Field metadata supplies `id`, `name`, required and
disabled state, ARIA relationships, and `data-*` state; caller attributes override the matching
metadata attributes.

## Adornments

`prefix` and `suffix` are `Element` props for content that belongs inside the control box — a
currency suffix, a unit, an icon, a `kbd` hint. When either is set, the Component emits daisyUI's
own wrapper pattern instead of the bare element: a `span` carrying the `input` class holds the
leading slot, the native `input` as a **direct child**, and the trailing slot, with focus styling
on the wrapper (`.input:focus-within`). Without either, the emitted tree is exactly today's bare
`input`. The names follow the prevailing slot convention (ADR-0029); the structural switch and the
governing decisions are recorded in ADR-0031.

**The wrapper is a `span`, not daisyUI's documented `label`.** Every wrapper-side rule in the
pinned CSS keys on the `.input` class, and the rules that do name an element (`.input input`,
`.input:has(>input[disabled])>input[disabled]`) constrain the inner element — still a real `input`,
still a direct child — so a `span` loses no styling. What it buys: a `label` wrapper would make a
standalone `Input { suffix: rsx! { "EUR" } }` announce "EUR" as the input's entire accessible
name, and could never nest inside another `label`. The cost — no implicit label association — is
paid back by the Component itself: clicking an adornment or the wrapper's padding forwards focus
to the native input (see the focus contract below).

**Attribute routing.** The wrapper owns the visible box in adorned markup (the clamp width, the
border drawn from `--input-color`, the join radii); the inner input is transparent, borderless,
and full-width. The routing rule:

- **Typed axis props** — `color`, `size`, `appearance`, and the derived `input-error` — relocate
  to the wrapper. That is what makes the axes work at all.
- **Caller `attributes`, `class` included, stay on the native input**, unchanged. Native
  attributes, the Binding contract, and Field metadata (`aria-*`, `disabled`, `required`) keep
  targeting the real control.
- **`wrapper_attributes` reaches the wrapper**, and is the surface for box-scoped utilities:
  `w-full`, responsive sizes such as `sm:input-lg`, `join-item`. On the transparent inner input
  those classes are silent no-ops, so a bare field's `class: "w-full"` **moves** to
  `wrapper_attributes` when an adornment is added; `tabular-nums` and its kind stay on `class`.
  There is one `#[props(extends)]` sink per component, so the prop is written explicitly:
  `wrapper_attributes: attributes!(span { class: "w-full" })`. A `class` inside it concatenates
  with the wrapper's own exactly as caller `class` does on the input.

**The focus scope does not move.** The complete logical focus scope stays the native input, and
adornments are non-interactive by contract: icons, text, `kbd` hints. A focusable element in a
slot would not merely degrade — every move between the input and the adornment would report Focus
Exit and run a producer's exit validation mid-interaction. A field that needs a button next to the
control stays on the Field parts. Because a plain wrapper click would blur the input and misfire
the same way, the Component forwards it: `mousedown` on a slot span is default-prevented, the
input's own `mousedown` stops propagating so the wrapper only ever sees padding clicks, and the
wrapper's handler prevents the default and requests focus on the native input. Two costs, accepted
and recorded in ADR-0031: the input's `mousedown` no longer bubbles to caller ancestors, and
adornment text ("EUR") is not selectable — the wrapper's `cursor: text` over it is a known
cosmetic dishonesty.

**Adornment presence is render-stable.** Adorned means `prefix.is_some() || suffix.is_some()`, so
an empty `Some(rsx! {})` still selects the wrapper arm. A conditional adornment toggles content
*inside* `Some` — the slot stays mounted and, being empty, is hidden (`empty:hidden` on the slot
spans), leaving no gap in the box. Toggling the option between `Some` and `None` across renders is
unsupported: it changes the tree shape and remounts the native input, losing caret, IME, and
selection state, and — because browsers fire no `focusout` when the focused node is removed — the
session's Focus Exit would never be reported.

**Floating labels do not compose with adornments.** `.floating-label>span` captures *any* direct
`span` child as the floating caption, the wrapper included, and none of `.input`'s declarations
contest the caption's `opacity: 0`. `Label { appearance: Floating }` around an adorned `Input` is
an unsupported combination; use a bare `Input` there, or the Field parts.

**Accessibility.** Under `InputField`, `aria-labelledby` wins the accessible name, so adornment
text is **not announced** — repeat a load-bearing unit in `label` or `description`. Decorative
adornments should carry `aria-hidden`, as the Examples do. A caller's `aria_describedby` overrides
the Field-merged value, so the closed path cannot wire the adornment into `describedby` yet; that
hook is a possible follow-up.

The adorned arm leans on wrapper-selector internals (`.input input`,
`.input:has(>input[disabled])`) that have shifted within daisyUI 5.x minors. The Registry pins
5.7.17 for its specs; consumers should install daisyUI 5.7 or later.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<String>` wins over Field Context;
without either, the control owns standalone state. The lower-level trio remains available:
`value: Option<ReadSignal<String>>` can override the rendered value, `on_change` observes each
value produced by native input, and `on_commit` observes the native `change` event. Writes and
commits still reach the resolved Binding. The browser fires `change` on blur after editing and when
an Enter-driven interaction commits, so that event defines the end of one interaction unit.

The complete logical focus scope is the native input itself. Commit and Focus Exit stay independent:
the native `change` event is the only thing that commits, so leaving an unchanged session reports
Focus Exit alone rather than synthesizing a Commit, which would run Commit validation over a value
the reader never edited. Typing alone only writes. Each actual exit reports once, calling the
resolved Binding's Focus Exit capability and then the optional dependency-free `on_focus_exit` prop.
Focus Exit does not itself infer touched, blurred, or validation semantics.

The resolved Field focus request calls `set_focus` on the native input handle captured at mount.
Global and native input attributes continue to spread onto that same control.

## State bridging

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `input-error`. Passing any colour value explicitly, including
`InputColor::Default`, wins over metadata. The Registry does not compute invalidity.

There is no Primitive state to bridge. As ADR-0019 records for Native controls, the browser
supplies focus, the caret, selection, editing, autofill, constraint validation, disabled behavior,
and form participation on the same real element that daisyUI styles. Disabled needs no Tier or
modifier class: the explicit or metadata-derived native attribute reaches the input, and daisyUI
reads it directly with `.input:is(:disabled,[disabled])`.

## Axes

- `color: Option<InputColor>` - the eight named daisyUI colours, plus an explicit default that emits
  nothing; omission permits invalid Field metadata to emit `input-error`.
- `size: InputSize` - `xs`, `sm`, default, `lg`, and `xl`; the default emits nothing.
- `appearance: InputAppearance` - daisyUI's default bordered input or `input-ghost`; the default
  emits nothing.

Each enum exposes `ALL`, listing every value of its Axis. The Preview iterates those lists and the
browser specs assert that every rendered value differs. The class strings are complete literals
in this Component's own file, even where the combobox and date pickers emit the same strings, so
Tailwind's scanner can see them when this Component is installed alone.

The installable Input Component revision-pins the Field Component it composes, so installing Input
also installs the matching Field parts required by `InputField`.

`floating-label` remains an appearance of `Label`, not `Input`. The class styles an ancestor label,
expects that label to contain a `span`, and reads the placeholder and focus state of the input
inside it. `Input` neither owns nor can enforce that surrounding tree, so a caller nests it inside
`Label { appearance: LabelAppearance::Floating, ... }` and supplies a placeholder.

## Deviations

**No Primitive.** This is a Native control under ADR-0019. daisyUI's rules name states and
pseudo-elements of a real `input` - including `:disabled`, `:focus-within`, `::placeholder`, and
the validation pseudo-classes used by `validator` - so replacing it with a button-based Primitive
would make those rules match nothing. The browser already supplies the behavior a text field
needs.

**The raw `oninput` passthrough was replaced by typed `on_change`.** The Registry is at `0.0.0`, so
the trio pilot takes the breaking migration rather than retaining two callbacks for the same native
event. Callers receive the resulting `String` directly; `on_commit` separately reports the native
`change` event.

**RTL url/tel/email/number inputs lose the join-radius swap when adorned.** daisyUI's rule
`:is(.input[type=url],…):dir(rtl)` requires the class and the `type` on the same element, which
adorned markup can never satisfy: the class is on the wrapper and the `type` on the inner input.

**Browser autofill paints the inner input, not the whole control.** The autofill background lands
on the transparent inner element, a fringe box inside the wrapper's border.

**Webkit pseudo-element geometry differs between the arms.** The `-10px` inner-spin-button nudge
exists only on the bare arm, and the calendar-picker indicator sits at `inset-inline-end: .75em`
bare but `-.15em` adorned for date and time types. Both are daisyUI's own per-arm rules.

## daisyUI classes deliberately not used

- `input-md` - the default size emits nothing, and daisyUI renders an unmodified input at that
  size.
- `floating-label` - it belongs to the surrounding `Label`, whose appearance Axis already emits
  it.
- `validator` - pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `input-error` modifier from the same state. Adding `validator` would duplicate
  that Axis and also opt into browser-owned valid paint.
- `validator-hint` - its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- Responsive prefixes (`sm:input-lg` and the rest) - a caller reaches them through `class`, which
  concatenates, on a bare input; on an adorned one through `wrapper_attributes`, since the wrapper
  owns the box those classes size.
