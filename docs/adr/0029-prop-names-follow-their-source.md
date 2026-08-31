# Prop names follow their source

A Component's public surface is frozen by `dx components add`: a consumer copies the module, and
a later rename is a change to their source tree rather than to a dependency. Before the first
release the surface was swept for names that differ between Components for no reason, and most of
what looked like drift turned out to be one rule nobody had written down. This decision writes it
down, and records the renames made where the rule was actually broken.

## A prop is named after what it forwards to

- **A part that wraps a Primitive takes the Primitive's prop names and types**, so a reader of
  the Primitive's documentation finds the same surface here: `on_click: Callback<()>` on the
  toolbar button, `on_trigger_click` on the accordion item, `default_query: ReadSignal<String>` on
  the combobox, `side` and `align` on floating content. Where the Primitive lacks a prop, as the
  accordion item lacks a controlled `open` (ADR-0011), the Registry does not invent one; it is
  added when the Primitive gains it.
- **A Native control or a presentational part uses the Dioxus element attribute**: `onclick`,
  `onchange`, `onmounted`. They forward the element's own event, and `extends` cannot reach
  handlers, so each is a prop spelled the way the element spells it.
- **A form value follows the `dioxus-field` trio** (ADR-0028): `value` / `default_value` /
  `on_change` / `on_commit`. The trio wins over the Primitive for the value pair, which is why
  the checkbox and switch say `default_value` where their Primitives say `default_checked`. The
  theme controller keeps `default_checked`: it is not trio-conformant, and the name is the
  input's `defaultChecked` it seeds.
- **A Native control with no Primitive takes its prop types from its Native siblings**:
  `Option<bool>` and `Option<String>` for `disabled`, `required`, `name`, as `Input` and
  `Textarea` have them, rather than the `ReadSignal` forms a Primitive would use.

## Axis vocabulary

daisyUI's placement-shaped classes fall into four different shapes, and each has one name:

- **`side` / `align`**: where anchored floating content opens against its trigger and where it
  sits along that side. Mirrors the Primitive's `ContentSide` / `ContentAlign`. Popover, tooltip,
  select, combobox, dropdown menu, hover card, both date pickers.
- **`placement`**: daisyUI's single axis for where a fixed element sits, `chat-start`,
  `drawer-end`, `modal-top` … `modal-end`. Chat, drawer, separator, dialog, alert dialog.
- **`inline` / `block`**: daisyUI's two-axis corner placement, `toast-start` + `toast-top`,
  `indicator-start` + `indicator-top`, named by the CSS logical axes they move along. Toast,
  indicator.
- **`positioning`**: the Defeatable utility (ADR-0004) that takes a panel out of the flow,
  `Default | None`. Hover card, both date pickers.

`Placement` no longer means three things, and `Position` is not used.

## Names around the surface

- **Composition sugar is `<Control>Field`**: `InputField`, `TextareaField`, `CheckboxField`,
  `SwitchField`, `OtpField`, `SliderField`, `RangeSliderField`.
- **A module is the snake_case of daisyUI's component name**: `file_input`, `radial_progress`,
  `theme_controller`. The module name is the `dx components add` identifier and the consumer's
  import path, which is why it is settled before release.
- **A Primitive type that appears in a public prop is re-exported** from the Component that uses
  it (`CheckboxState`, `DialogCtx`, `AvatarState`, `DateRange`), because the Primitive is a Git
  revision pin whose module paths may move. **Published crates are not re-exported**: `time` and
  `dioxus-field` have stable paths, every manifest that needs one declares it, and the Examples
  import them directly.
- **A Field-aware control that supports more than one Binding type panics** when the Field
  Context carries a third: it is a producer programming error visible at first render, and a
  silent fallback would render a control the form never reaches. The checkbox README states it.
- **Every prop except `attributes` and `children` carries a `///` comment.**
  `tests/registry_policy.rs` reads each `#[component]` signature and fails on one that does not.

## Consequences

- `fileinput` and `radialprogress` became `file_input` and `radial_progress`; `TextField` became
  `InputField`; `default_checked` became `default_value` on Checkbox, CheckboxField, Switch and
  SwitchField; `placement` / `alignment` became `side` / `align` on popover, tooltip, select,
  combobox and dropdown menu; `placement` became `positioning` on hover card and both date
  pickers; `placement` / `alignment` became `inline` / `block` on the toast provider; `position`
  became `placement` on dialog and alert dialog; the file input and theme controller take
  `Option` props. Enum names followed their props.
- A reviewer can now tell a deviation from a mirror: a name that differs between Components is
  wrong only if the two forward to the same kind of source.
- Adding a controlled `open` to the accordion item, or `on_open_change` beside its `on_change`,
  waits for the Primitive rather than being papered over here.
