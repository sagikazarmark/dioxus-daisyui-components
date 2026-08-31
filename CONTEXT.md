# dioxus-daisyui-components

A `dx components` registry of Dioxus components that apply daisyUI class names to
`dioxus-primitives` behaviour, native browser behaviour, or presentational markup. The registry
owns the component API and any mapping between state and daisyUI selectors; it owns no design
decisions.

## Language

### The registry

**Registry**:
The set of components this repository publishes, consumed by `dx components add`.
_Avoid_: library, crate, package

**Component**:
One installable unit: a manifest plus the Rust module that `dx` copies into the consuming
app's source tree.

**Preview**:
The Dioxus documentation app that exercises every Component. It is the acceptance test.
_Avoid_: demo, example app, playground

**Component page**:
One Component's page in the Preview: a list of Examples that between them render every value
of every Axis that Component exposes. It is addressed by URL, together with the theme it is
rendered under, so that the browser tests reach it deterministically.
_Avoid_: story, demo page

**Example**:
One file under a Component's excluded `docs/examples/` directory, holding one component that a
Component page both renders and prints the source of: the same code seen two ways. It is the unit
a page is built from and the unit a reader copies. See ADR-0009.
_Avoid_: snippet, sample, story

**Documentation tooling**:
The external `dioxus-registry-preview` package, validation library, DOM protocol, isolated default
chrome, and generic Playwright helpers. The tooling owns discovery, generic validation,
generated APIs, and protocol definitions; this Registry owns all authoring inputs, Preview routing
and presentation, themes, and daisyUI acceptance policy. See ADR-0027.

**Documentation chrome**:
Consumer-owned Preview source implementing the Example and README adapters and the branded shell.
The external tooling offers isolated defaults, but this Registry replaces them so its Tailwind and
daisyUI classes remain visible to the Registry's scan and match its branded Preview.

**Component group**:
The section of the Preview's navigation a Component page is listed under, written as a heading
in the sidebar and on the component index. The same grouping in both means a component is found
in the same place either way. It cuts by what a component is *for* rather than by what it is
made of, which is why the menu bar is navigation though it opens menus and the toast is an
overlay though it is feedback. It is the Preview's presentation and nothing more: no Address,
manifest or test names one.
_Avoid_: category, section, tag

**Address**:
One Component page under one theme: the pair the Preview's URL names, and the unit the
browser tests navigate to. The page is the path and the theme is a query parameter, which is how
the two are reached: a page is navigated to, and a theme restyles whatever page is already open.
A screenshot is taken at one Address per page per *baseline* theme rather than at all of them,
since the Preview offers every theme daisyUI ships.

**Primitive**:
An unstyled behaviour component from `dioxus-primitives`: focus management, keyboard
navigation, ARIA wiring, dismissal. The drawer has one narrow adapter after Primitive focus
restoration: it repeats focus on the trigger after close because a browser's later pointer default
can otherwise undo the Primitive's move. The Primitive still owns the trap and its cleanup; see
ADR-0024.
_Avoid_: headless component, base component

### Component structure

**Compound parts**:
The set of components that together render one daisyUI component. They mirror the Primitive's
composition when there is one, and daisyUI's documented markup for a Presentational component.
_Avoid_: subcomponents, slots

**Collapsed component**:
A single component that renders more than one compound part, offered alongside the parts
rather than instead of them.
_Avoid_: flat component, convenience wrapper

**Composition sugar**:
A closed per-control component that renders a common arrangement of existing Components and
Compound parts. It adds no new state or Field mechanism, has no children slot, and exists only for
the happy path; callers drop to the composed parts as soon as they need content or attributes the
sugar does not expose. Today: input, textarea, OTP, checkbox, switch, slider, and range-slider
Fields.

**Legal collapse**:
The condition under which a collapsed component may be offered: daisyUI's markup has no
element between the parts, and no caller content can go there. Checkable against daisyUI's
CSS, not a matter of taste.

**Omitted part**:
A primitive part the registry deliberately does not expose, because it carries no behaviour
and its element stands between daisyUI's selectors and the elements they target.

**Axis**:
An independent dimension of daisyUI styling exposed as its own prop. daisyUI's axes are
orthogonal, so colour, size, and component-specific axes are separate props rather than one
enum.
_Avoid_: variant (reserved for the colour axis specifically)

**Derived axis**:
An Axis nobody passes: its value follows from something the Primitive reports rather than from
a prop, because the element it styles is rendered by the Registry rather than by the caller.
Still an Axis in every other way: a class per value, and a variant list the Preview renders
every one of. Today: the toast's colour, which follows from the kind it was dispatched as.

**Provider**:
A Component a caller mounts once, above everything that uses it, rather than writing where it
appears. Its markup is reached through a portal, so where it lands on screen is its own classes
rather than its place in the tree. Today: the toast.
_Avoid_: root, context

**Native control**:
A Component that renders a real form control itself and wraps no Primitive, because daisyUI's
rule for it names native states or pseudo-elements on that control (`:checked`, `:disabled`,
`::placeholder`), which no button-based Primitive can stand in for. The browser is then the
behaviour: focus, editing, the keyboard, validation, grouping and form participation are the
element's own. Today: the theme controller, file input, input, OTP field and textarea. The OTP
field is visually several boxes but behaviourally one input; daisyUI draws the boxes around it.
See ADR-0019.
_Avoid_: raw component, unwrapped component

**Presentational component**:
A Component that wraps no Primitive because it has no behaviour at all: no state, focus,
keyboard interaction or ARIA wiring. Its Compound parts follow daisyUI's documented markup rather
than a Primitive's composition. This is distinct from a Native control, where the browser supplies
behaviour. See ADR-0023.
_Avoid_: primitive-free component, CSS-only component

### Fields

**Field**:
A producer-defined scope connecting one form control with its Binding, Field metadata, focus
behaviour, label, description and errors.
_Avoid_: form item, field wrapper

**Binding**:
The reactive two-way contract for one Field's value: a readable value, origin-preserving writes,
notification that one interaction unit committed, and a separate report that focus left the
control's complete logical scope. Commit and Focus Exit are independent.
_Avoid_: signal, controlled state

**Focus Exit**:
Notification that focus left a control's complete logical scope, including its child controls and
owned popup content. Internal focus movement does not count, and the Registry does not interpret
Focus Exit as blurred, touched, validation, or any other form state.
_Avoid_: blur event, commit

**Field metadata**:
Producer-defined presentation and accessibility facts for one Field: its declared id and name,
state flags, and pre-rendered errors. They report validity to the Registry; the Registry does not
compute it.
_Avoid_: validation logic, form state

**Conformance level**:
The declared extent to which a form control follows the Field convention. Trio-conformant controls
honour the `value` / `on_change` / `on_commit` prop trio and attribute spread; Field-aware controls
additionally resolve the Field Context. See ADR-0028.
_Avoid_: support level, compatibility tier

### Styling

**State bridging**:
Connecting state a Primitive exposes or a producer supplies through Field metadata to the selectors
daisyUI actually matches on.

**Tier 1**:
State bridging that needs no action because daisyUI already matches the ARIA attribute the
primitive sets.

**Tier 2**:
State bridging performed by emitting daisyUI's own modifier class from Rust, keeping the class
name visible to Tailwind's scanner.

**Lifted state**:
State a registry component owns on the primitive's behalf (seeded from the primitive's
`default_*` prop, passed back down as controlled), so that daisyUI's otherwise unreachable
selector state can be expressed. Usually that means emitting a Tier 2 modifier class; the drawer
instead projects the state into an inert native checkbox because daisyUI exposes no transient-open
class. The only state the registry takes responsibility for. See ADR-0006 and ADR-0024.

**Mirrored state**:
State a registry component reads off the primitive rather than owning (seeded from the same
`default_*` prop and updated from the primitive's change callback, with nothing passed back
down), so that a Tier 2 modifier class can be emitted where the primitive offers no controlled
prop to lift. The primitive stays the only writer. See ADR-0011.

**Defeatable utility**:
A Tailwind utility the registry emits where daisyUI has no component class, paired with an axis
value that emits nothing, so a caller overrides by switching it off rather than by out-ranking
it.

**Bridged utility**:
A Defeatable utility written as a Tailwind variant of the `data-*` attribute a Primitive already
sets (`data-[selected=true]:ring-2`), used where daisyUI has no class for that state at all. It
is neither Tier: Tier 1 needs a daisyUI rule that matches the attribute, and Tier 2 needs a
daisyUI class to emit. Nothing is recomputed in Rust and the Primitive stays the only owner of
the state, which is what tells it apart from Lifted and Mirrored state. It is the registry's
usual answer wherever daisyUI has no component at all: every state a calendar day, a sortable
row or a date segment is painted from is one of these. Two shapes are worth naming apart: the
menubar trigger's open ring, written as a variant of an attribute the Primitive sets on the
element *above* it, since that is where the state is reported; and the combobox option's
highlight, whose variant emits a daisyUI *class* rather than a Tailwind utility, because daisyUI
has the class for that state and simply no selector that reaches it (ADR-0021).

**Recomposed default**:
A Component part that renders what the Primitive's own collapsed part renders, put together
again out of the Registry's parts. It exists where the Primitive's collapsed part takes
attributes for its outer element alone, so no class could reach what it renders inside: a
weekday heading, a cell, a day. The composition is the Primitive's and the classes are the
Registry's; nothing about the behaviour moves. Today: the calendar's grid, the drag and drop
list's rows, and both date pickers' fields and months. See ADR-0022.
