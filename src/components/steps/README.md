# Steps

A daisyUI-styled indicator for the steps in a process.

[Live examples](https://daisyui-components.dioxus.cc/components/steps) ·
[their sources](docs/examples/)

This is a Presentational component and wraps no Primitive. `Steps` renders `ul.steps`, each
`Step` renders a direct `li.step`, and `StepIcon` renders `span.step-icon`. Keep `Step` parts as
adjacent children of `Steps`: daisyUI colours a connector with selectors such as
`.step-primary + .step-primary::before`, so a wrapper between two steps breaks that connector.
Likewise, `StepIcon` must be a direct child of `Step` because daisyUI styles it through
`> .step-icon`. The parts add no wrappers of their own.

Without a `StepIcon`, daisyUI writes a generated counter in `Step::after`. A caller can replace
that counter by passing `"data-content": "!"` to `Step`, or replace the entire generated circle
with a direct `StepIcon`. Generated CSS content, including `data-content`, is not exposed
consistently by browsers and assistive technologies. Every step therefore needs a meaningful
text label in its children; do not rely on its counter, symbol, colour, or decorative icon as the
only accessible indication of order or status. Hide a purely decorative `StepIcon` with
`aria-hidden="true"`.

Colour does not mean completed to the Component, and `aria-current` does not choose a colour.
The caller decides which steps are complete, current, or pending, applies colour accordingly, and
adds `aria-current="step"` to the one current item when that describes the process. There is no
generic ARIA "completed" state; when completion must be announced, include it in the step's text
or accessible name. `Steps` is a visual process indicator, not a wizard controller: it performs no
navigation, validation, progression, or state management.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. Direction
and colour are caller-selected Axes, while completion and current-step semantics remain entirely
caller-owned. There is no state to lift, mirror, derive, or pass to a Primitive.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `direction: StepsDirection` on `Steps`: `steps-horizontal` or `steps-vertical`. Both values emit
  an explicit complete class literal.
- `color: StepColor` on `Step`: `step-neutral`, `step-primary`, `step-secondary`, `step-accent`,
  `step-info`, `step-success`, `step-warning`, `step-error`, or nothing.

The Axes are separate because direction lays out the list while colour paints a step and the
connector between adjacent steps of that colour. Both enums are non-exhaustive and expose `ALL`
for the Preview. `StepColor::Default` emits nothing and keeps daisyUI's base step colour rather
than aliasing `Neutral`.

## Deviations

None from daisyUI. The three Compound parts reproduce its documented `ul > li` structure and its
direct custom-icon child. The Component wraps no Primitive because an indicator has no behaviour
for one to provide.

## daisyUI classes deliberately not used

- None of the Steps classes are omitted: both directions, every colour, the base `steps` and
  `step` classes, and `step-icon` are exposed.
- Responsive-prefixed direction classes such as `lg:steps-horizontal` remain caller classes. They
  concatenate through `class` without becoming another Axis value.
- Completion is not a daisyUI class. Callers choose a colour and supply any completion semantics
  their process needs.
