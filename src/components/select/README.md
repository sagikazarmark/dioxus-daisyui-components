# Select

A select whose field is daisyUI's `select` and whose popup is daisyUI's `dropdown` and `menu`,
wrapping the `dioxus-primitives` select. Typeahead, arrow-key navigation, the `listbox` and
`option` roles, the `aria-selected` and `aria-controls` wiring and the dismissal on Escape, on a
blur and on a choice are all the primitive's rather than reimplemented here.

```rust
rsx! {
    Select::<String> {
        default_value: "lemon".to_string(),
        on_change: move |value| tracing::info!("chose {value:?}"),
        on_commit: move |()| tracing::info!("selection committed"),
        on_focus_exit: move |()| tracing::info!("focus left the select"),

        SelectTrigger { color: SelectColor::Primary,
            SelectValue { placeholder: "Pick a fruit" }
        }
        SelectList {
            SelectGroup {
                SelectGroupLabel { "Citrus" }
                SelectOption::<String> { value: "orange".to_string(), index: 0usize, "Orange" }
                SelectOption::<String> { value: "lemon".to_string(), index: 1usize, "Lemon" }
            }
            SelectGroup {
                SelectGroupLabel { "Berries" }
                SelectOption::<String> { value: "cherry".to_string(), index: 2usize, "Cherry" }
                SelectOption::<String> {
                    value: "currant".to_string(),
                    index: 3usize,
                    disabled: true,
                    "Currant"
                }
            }
        }
    }
}
```

An option's `index` is its position in the keyboard navigation order, and it is explicit because
the primitive's focus collection is ordered by it rather than by the DOM, which is what makes
the wrappers below free, and what lets the index run across groups rather than restarting inside
each one.

This is the least daisyUI-native component in the registry. daisyUI has no styled listbox at
all: its `select` is a native `<select>` element, whose popup the browser draws. So the field
here is daisyUI's `select` on a `button`, and the popup is borrowed from daisyUI's `dropdown`
and `menu`: the same borrowing, and the same two elements, as the dropdown menu component.

## Composition

The compound parts are `Select` (the dropdown), `SelectTrigger` (the field), `SelectValue` (what
the field shows), `SelectList` (the box, with the menu inside it), `SelectOption` (one row),
`SelectGroup` (a run of rows) and `SelectGroupLabel` (its title). There is no collapsed
component: the trigger and the list are two elements a caller writes between, so there is
nothing to collapse.

`Select` owns field value resolution and the explicit `meta`, `required`, `disabled`, and `name`
props. It passes that resolved metadata contract to `SelectTrigger` through private Compound
context; callers do not have to repeat metadata on both parts.

**The field is a `button` carrying `select`.** The class carries over intact: `.select` lays
out an inline flex row, pads the inline end by `1.75rem` and paints the caret into that space as
a background image, none of which asks the element to be a form control. What it loses with the
native element is recorded under Deviations.

**The popup's classes split across two elements** (ADR-0005), exactly as the dropdown menu's do.
daisyUI puts `dropdown-content` and `menu` on a single `<ul>`; here `dropdown-content` and the
box's utilities go on the primitive's list `<div>`, `menu` goes on a `<ul>` rendered inside it,
and each row is wrapped in an `<li>`:

```html
<div class="dropdown dropdown-bottom dropdown-start dropdown-open">
  <button class="select" aria-haspopup="listbox" aria-expanded="true">
    <span data-placeholder="false">Lemon</span>
  </button>
  <div class="dropdown-content bg-base-100 rounded-box shadow-sm" role="listbox">
    <ul class="menu w-full" role="none">
      <div role="group" aria-labelledby="dxc-1">
        <li role="none" class="menu-title"><div id="dxc-1">Citrus</div></li>
        <li role="none"><div role="option" aria-selected="false">Orange</div></li>
        <li role="none"><div role="option" aria-selected="true" class="menu-active">Lemon</div></li>
      </div>
    </ul>
  </div>
</div>
```

The same three constraints force it. `SelectList` renders a hardcoded `div` and has no `as`
prop. `SelectContext` is private, so a replacement list component could never reach the open
state, the option collection or the typeahead buffer. And every visual rule daisyUI's `.menu`
applies to a row is of the form `.menu :where(li:not(.menu-title)>…)`, so literal `li` elements
are required.

Both wrappers carry `role="none"`, so they are out of the accessibility tree and the listbox
still owns its groups and options directly, which is what option counts and positions are
announced from.

**The list is in the document whether the popup is open or not.** This is the one place the
select's structure is not the dropdown's. The primitive mounts `SelectList`'s children even
while the popup is closed, because that is how each option registers the text the field shows
when it is the chosen one; a select that renders nothing when closed would show its placeholder
forever. So the `<ul>` cannot be rendered only while the popup is open. It is hidden instead, by
a utility that fires exactly when the list is not inside the box:

```
[:not(.dropdown-content)>&]:hidden
```

Keying on the box rather than on the open state is what keeps the closing transition intact:
daisyUI fades and scales `.dropdown-content` out over `.2s`, the primitive keeps the box mounted
until that has run, and the list is inside the box for every frame of it. A utility keyed on the
open state would empty the box before it had finished leaving.

**Where daisyUI's example utilities went** is settled the same way the dropdown menu's are:
`bg-base-100 rounded-box shadow-sm` are on the box and are what `appearance` switches off; `p-2`
is not emitted because `.menu` already pads by the same `.5rem`; `z-1` is not emitted because
daisyUI's own `.dropdown .dropdown-content` rule already sets `z-index: 999`; and `w-52` is not
emitted because a menu with no width sizes to its rows.

`w-full` **is** emitted, on the list, and it is the one utility here with no axis switching it
off. What ADR-0004 exists to prevent is a caller's utility *tying* with one of ours, and there is
no tie to lose: a caller's classes land on the box, this one is on an element no caller reaches,
and the two cooperate: the width the caller sets is the width the rows take. The same goes for
the hidden-list utility above, which fires only in a state where the element is not meant to be
seen at all.

**A group's element sits between the list and its rows.** `SelectGroup` is the primitive's
`<div role="group">`, and it is where the group's `aria-labelledby` and `aria-disabled` live, so
it cannot be dropped. Nothing is emitted on it: `.menu` reaches a row through a descendant
selector rather than a child one, and a plain block between the two is laid out by `.menu`'s
column exactly as `display: contents` would be. The cost is that the `<ul>` has a child that is
not an `<li>`, which no HTML validator will like; see Deviations.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<Option<T>>` wins over Field Context;
without either, the control owns standalone state seeded by `default_value`. The lower-level trio
remains available: `value: Option<ReadSignal<Option<T>>>` can override the rendered Binding value,
`on_change` observes each successful option selection, and `on_commit` observes that selection as
a completed interaction unit. Writes and commits still reach the resolved Binding. A successful
option selection is the exact Commit boundary, so change and Commit have the same cadence.

Focus Exit is independent. The complete logical focus scope is the outer `Select`, including its
trigger, popup listbox, groups, and every option. Movement from trigger to listbox, listbox to an
option, or option to option stays inside that scope and reports nothing. Selection may Commit while
focus remains inside. One confirmed departure calls `Binding::focus_exit()` and then the optional
dependency-free `on_focus_exit` prop. Popup open changes, closing, selection, and an individual
child blur do not themselves imply Focus Exit; when the pinned Primitive closes a focused popup and
focus consequently lands on the document body, that actual departure does. Neither Commit nor
Focus Exit infers form touched, blurred, validation, or other form-library semantics.

`Select` resolves metadata once and carries it to `SelectTrigger` through private Compound context.
`meta.attributes_for(...)` lands on the trigger button because that is the interactive control and
the element Field focus requests reach. Its surface emits native `disabled` and `name`, but omits
required and validity attributes unsupported on `role=button`; the corresponding `data-*` state
still paints and reports metadata. No metadata attributes land on the outer `dropdown` div. There
is no second form-participation element: the primitive renders no hidden input, so this Select still
does not submit a native form value. Explicit root `required`, `disabled`, and `name` props override
metadata, and caller attributes on `SelectTrigger` override matching metadata attributes while
classes concatenate.

The resolved Field focus request calls `set_focus` on the trigger handle captured at mount.
Attributes passed to `Select` continue to spread onto the outer dropdown, while attributes passed
to `SelectTrigger` spread onto the control. `data-select-focus-scope` is reserved for the generated
locator used to verify focus after the Primitive removes popup content.

## State bridging

**Tier 2** on the open state, with **lifted state** (ADR-0006), and here the lift is mandatory
rather than cosmetic: daisyUI hides `.dropdown-content` outright (`display: none`, no opacity,
scaled down) unless the element above it carries `dropdown-open`, and that class matches no ARIA
or `data-*` attribute the primitive sets.

**Tier 2** on the value as well, with the same lift, and this half is the select's own. daisyUI
marks the chosen row of a menu with `menu-active`, and `.menu` matches **no ARIA attribute at
all**: not the `aria-selected` the primitive puts on the option, not anything else. Without the
class the chosen row would render identically to every other one.

`Select` therefore owns both: it seeds open state from `default_open`, resolves value state from a
Binding seeded by `default_value`, always hands the primitive controlled state, and intercepts both
change callbacks before calling the caller's. A controlled caller and an uncontrolled one both
work. The value travels down to the options through a context of this component's own, because
`SelectContext` is private and the class belongs on the option rather than on the element that
knows the value.

**Tier 2** on an option's disabled state, emitted onto that option's wrapper. daisyUI mutes a
disabled row through `.menu-disabled` on the list item or a `disabled` attribute on the row
itself; the primitive reports the state as `aria-disabled` and `data-disabled`, which daisyUI
matches nowhere.

**Tier 2** on producer-defined invalidity: when the trigger's colour Axis is omitted and Field
metadata is invalid, `SelectTrigger` emits `select-error`. Passing any colour value explicitly,
including `SelectColor::Default`, wins over metadata. The Registry does not compute invalidity.

**Tier 1** everywhere else, which is most of what a select does:

- **The field's focus ring**, which `.select:focus, .select:focus-within` draws. The primitive
  focuses the trigger the way any button is focused, and daisyUI matches it.
- **The disabled select**, which `.select:is(:disabled, [disabled])` mutes. The primitive puts
  the attribute on the trigger from `Select`'s resolved disabled state.
- **The keyboard focus highlight on a row**, which daisyUI's own rule matches through
  `:focus-visible` as well as through its `.menu-focus` class, and the primitive moves real DOM
  focus onto the option.
- **The hover highlight on a row**, which daisyUI draws from `li > *:hover` inside a `.menu`.

One state is bridged by neither tier, because daisyUI has no class for it. **The placeholder**
is a `<span>` here, where daisyUI fades a native select's `::placeholder`; the primitive marks
it with `data-placeholder`, and `SelectValue` emits a Tailwind utility keyed on that attribute:
`data-[placeholder=true]:opacity-50`, at the opacity daisyUI fades a native placeholder by. It
is a **defeatable utility**, so it comes with an axis that emits nothing (ADR-0004).

The trigger's `aria-haspopup`, `aria-expanded` and `aria-controls`, the list's `listbox` role and
`aria-multiselectable`, the group's `role` and `aria-labelledby` and the option's `aria-selected`
and `aria-disabled` are all the primitive's, and daisyUI styles none of them.

## Axes

- `color: Option<SelectColor>` on `SelectTrigger`: `select-neutral`, `select-primary`,
  `select-secondary`, `select-accent`, `select-info`, `select-success`, `select-warning`,
  `select-error`; omission permits invalid Field metadata to emit `select-error`.
- `size: SelectSize` on `SelectTrigger`: `select-xs`, `select-sm`, `select-lg`, `select-xl`.
- `side: SelectSide` on `Select`: `dropdown-top`, `dropdown-bottom`, `dropdown-left`,
  `dropdown-right`.
- `align: SelectAlign` on `Select`: `dropdown-start`, `dropdown-center`, `dropdown-end`.
- `size: SelectListSize` on `SelectList`: `menu-xs`, `menu-sm`, `menu-lg`, `menu-xl`, on the
  list. It sizes the rows rather than the box, which has no size of its own.
- `appearance: SelectListAppearance` on `SelectList`: `bg-base-100 rounded-box shadow-sm`.
- `appearance: SelectValueAppearance` on `SelectValue`: `data-[placeholder=true]:opacity-50`.

The two size axes are separate because daisyUI's are separate: `select-lg` sizes a field and
`menu-lg` sizes a list, and nothing in daisyUI ties one to the other. A select whose field and
popup should grow together sets both.

`SelectListSize` is an axis rather than something a caller passes through `class` because the
split leaves it unreachable otherwise. `menu-lg` only works on the element carrying `menu`, and
that element is the list this component renders inside the box; a caller's classes land on the
box, one element above it. This is the axis the split owes the caller.

`side` and `align` **emit a class for every value, the default one included** (ADR-0008)
which is the inverse of the convention the colour and size axes follow. They are the dropdown's axes,
borrowed with its structure and with its reasoning: daisyUI's unclassed dropdown places its
content wherever it would have fallen in flow, which is under the trigger only for as long as
the trigger is the one thing written before it.

The two `appearance` axes invert the convention the other way, as the dialog's title and
description do: their default value *emits* utilities and their `None` value emits nothing.
daisyUI has no class for either element, so what a caller would otherwise have to override is a
Tailwind utility this component emitted, and two utilities only tie, with the tie settled by
generated-stylesheet order rather than by the class attribute. Switching ours off is how a caller
wins it (ADR-0004).

`SelectSize::Default` and `SelectListSize::Default` emit nothing, which renders at the same size
as daisyUI's explicit `select-md` and `menu-md`.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it and the browser
specs read the same rendered set.

## Deviations

**Multi-select is not supported.** The primitive has a `SelectMulti` component, and nothing here
wraps it. daisyUI's own multiple select is a native `<select multiple>`, whose rule
(`.select[multiple]`) drops the caret and turns the field into a scrolling list of native
`option` elements, a shape this component cannot produce, since it has no native select in it.
Marking several rows of a borrowed `menu` is possible, and choosing what a field showing four
values looks like is a design decision this registry does not own (ADR-0002). A caller who needs
one uses `dioxus_primitives::select::SelectMulti` directly and styles it themselves.

**The caret does not flip when the popup is open.** daisyUI redraws it upwards from
`.select:open`, and `:open` matches a native `<select>` with its picker showing, never a
`button`, whatever else is on the page. There is no daisyUI class for the state and no attribute
of the primitive's that reaches it, so the field's caret points down throughout. The popup is
still visible, and `aria-expanded` still says which way it goes.

**A select cannot start open on its own.** The primitive keeps the open state in step with its
focus collection, and closes a popup that nothing in it is focused, so `default_open: true` is
undone as soon as the page settles. A select that is to be open without being operated is one
its caller controls, holding `open: Some(true)`, which is how the preview's axis rows stand open.

**Focus is not returned to the trigger when the popup closes.** The popup takes focus when it
opens (the primitive focuses the list itself, and the arrow keys move focus onto a row) and
when it closes, the focused element goes out of the document with it and focus lands on the
document body. The gap is the primitive's; nothing here restores focus on its behalf, because
doing so means deciding whether the popup closed because the user dismissed it or because they
clicked something else, which is state the primitive holds and this component does not. A
browser spec pins the gap down rather than leaving it to this paragraph, so the day upstream
restores focus is the day that spec fails and this component inherits the fix.

**The list's `<ul>` has a child that is not an `<li>`.** A `SelectGroup` renders the primitive's
`<div role="group">`, and the rows it holds are inside it. The nesting is what an HTML validator
will object to; nothing observable rests on it, because both the `<ul>` and the `<li>`s are
`role="none"` scaffolding for daisyUI's selectors rather than a list anyone is told about, and
the group is what a screen reader announces instead. The alternative, dropping the group, would
cost the grouping itself, and there is no arrangement of the primitive's elements that is both
valid and styled, for the reasons ADR-0005 records.

**The primitive's `SelectItemIndicator` part is not exposed.** It renders its children only when
its option is the chosen one, which is a check mark beside the row in the primitive's own
examples. daisyUI marks the chosen row by repainting it (that is what `menu-active` is) so a
mark as well would say the same thing twice. A caller who wants one writes
`dioxus_primitives::select::SelectItemIndicator` inside a `SelectOption`; it takes no styling and
needs no wrapper.

**`SelectOption` declares no `aria_label` or `aria_roledescription`**, where the primitive takes
both as props of its own. They are ordinary global attributes, so they arrive through the
attribute list and land on the same element; declaring them here would offer a second way to
set one attribute, and the primitive spreads the attribute list last, so the two would not even
agree on a winner. Every other part in the registry treats an ARIA attribute the same way; these
two are called out only because the primitive names them.

**Field metadata does not add native form participation.** `Select` accepts `name` and `required`
so explicit values can override Field metadata. The trigger button can carry `name`, but its
`role=button` supports neither native nor ARIA required state, so required is exposed only through
`data-required`. The primitive still renders no hidden input. A form that needs the selected value
carries it in its own state.

**The primitive's `on_value_change` is exposed as `on_change`.** This is a deliberate breaking
rename for the Registry's Binding trio at version `0.0.0`. The wrapper calls `on_commit` in the same
selection callback because choosing one option completes one interaction unit.

**Field focus registration lands on the trigger.** A forwarded `onmounted` listener captures the
actual button and `use_focus_registration` calls `set_focus` on that handle. There is no focus
registration on the outer dropdown or the popup parts.

**No `extends = button` on the trigger**, unlike `DropdownMenuTrigger`. The primitive renders the
element and puts `type` on it, and `disabled` on it from `Select`'s own prop, so extending the
button attribute list here would offer a second and conflicting way to set the two attributes
worth reaching. Disable the select rather than its trigger.

**A closed popup renders no box.** The primitive mounts the box when the select opens and
unmounts it once the exit transition has finished, where daisyUI's own dropdown is always in the
document and merely hidden. The list inside it is the exception described under Composition.

## daisyUI classes deliberately not used

- `select-ghost`: daisyUI's style axis for a select, alongside its colours. It is not exposed as
  an axis because the registry's colour axes are the semantic colours, and it needs no axis to be
  reachable: `SelectColor::Default` emits nothing, so a caller's `select-ghost` through `class`
  arrives as the only one of its kind.
- `select-md`: the explicit middle of the size axis, which renders identically to the unclassed
  field `SelectSize::Default` emits.
- `menu-md`: the same, for `SelectListSize::Default`.
- daisyUI's rules for a native select's innards: `.select select`, `optgroup`, `option`,
  `selectedcontent`, `::picker(select)` and `.select[multiple]`. There is no native select here
  for any of them to reach.
- `dropdown-hover`: opens the popup on hover, with no state behind it. The open state here is
  the primitive's, and a popup that opened on hover would leave the class and the state
  disagreeing.
- `dropdown-close`: forces a dropdown closed whatever else is on it. It exists for markup that
  has no state to close, which is the opposite of this component's problem.
- `menu-horizontal` and `menu-vertical`: the primitive navigates the list with the up and down
  arrows, so a horizontal menu would announce and navigate one way while rendering another. A
  vertical menu is what `.menu` already is.
- `menu-focus`: daisyUI's own class for the row the keyboard is on. The same rule matches
  `:focus-visible`, and the primitive moves real focus, so emitting it would be a second way of
  saying what the DOM already says.
- `menu-dropdown`, `menu-dropdown-toggle` and `menu-paged`: nested submenus and paged
  navigation. The primitive's select has no part for either, and a listbox whose options open
  further listboxes is a different control.
- `validator`: the trigger's `role=button` does not support `aria-invalid`, while this Component
  already emits its explicit `select-error` modifier from Field metadata. Adding `validator` would
  duplicate that Axis and also opt into browser-owned valid paint where the trigger is not a native
  select.
- `validator-hint`: its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead renders a polite live region and joins the trigger's `aria-describedby`,
  following producer metadata without that adjacency and class coupling.
- `disabled` on a list item: daisyUI's hover rules exclude it but nothing defines it;
  `menu-disabled` is the class that does the work.
- `dropdown-open`, `menu-active`, `menu-title` and `menu-disabled` are **not** in this list: they
  are emitted, and they are the whole of this component's Tier 2.
- The responsive prefixes daisyUI generates for all of the above (`sm:select`, `md:menu-lg`, and
  so on): a caller reaches them through `class`, which concatenates.
