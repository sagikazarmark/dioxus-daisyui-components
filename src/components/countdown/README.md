# Countdown

A daisyUI countdown whose CSS, visible text and accessible label follow one reactive integer.

```rust
let mut seconds = use_signal(|| 59_i32);

Countdown { class: "font-mono text-4xl",
    CountdownValue { value: seconds(), digits: CountdownDigits::Two }
}
button { onclick: move |_| seconds -= 1, "Decrease" }
```

`Countdown` renders the outer `span.countdown`. Each `CountdownValue` renders a direct child
`span`, because daisyUI's animation selector is `.countdown > *`. A root may contain several
values with caller text between them, for example hours, `:`, minutes, `:`, and seconds.

This Component does not contain a clock or timer. The caller owns intervals, pausing, reset and
what happens at zero; changing the Dioxus signal passed as `value` is enough to update the
Component.

## Values and accessibility

`value` accepts a signed integer and normalizes it by clamping to daisyUI's supported `0..=999`
range. A negative value is displayed and announced as `0`; a value above `999` is displayed and
announced as `999`. The normalized integer is computed once per render and is the source for all
three representations: the `--value` custom property, the text node, and the default `aria-label`.

Each `CountdownValue` defaults to `aria-live="polite"`, matching daisyUI's documented markup.
Its default label is only the normalized number. Callers are responsible for a more descriptive
label when the surrounding text does not provide enough meaning, for example
`aria_label: "59 seconds remaining"`. Callers may also override `aria_live`; a multi-part clock or
a value that changes every second can otherwise produce repetitive announcements. Consider one
separately labelled live region for a whole clock, or announce at a lower frequency, according to
the application's accessibility requirements.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. `value` is
a numeric Axis that writes a daisyUI custom property, not Primitive state and not state bridging.
The caller owns the changing signal, and this Presentational component wraps no Primitive.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `digits: CountdownDigits` writes no `--digits` property for `Natural`, `--digits: 2` for `Two`,
  and `--digits: 3` for `Three`.

`CountdownDigits` is non-exhaustive and exposes `ALL` in natural, two-digit and three-digit order.
Leading zeroes are daisyUI's visual output; the value's text and accessible label remain the
normalized integer rather than a separately formatted copy.

Caller classes concatenate on both parts. Caller inline style declarations are preserved, but
`--value` and `--digits` are removed from the caller's declaration block before the typed values
are appended. The props therefore stay authoritative without erasing unrelated declarations such
as colour, letter spacing or layout. HTML treats the `style` attribute name case-insensitively;
CSS custom-property names are case-sensitive, so a caller's differently cased property such as
`--VALUE` is preserved but does not affect daisyUI's Registry-owned `--value`.

## Deviations

None from daisyUI's markup or value contract. The Component adds typed, reactive ownership around
the custom properties and chooses a documented clamp for unsupported values so CSS, text and ARIA
cannot disagree.

daisyUI animates the generated digits and width whenever `--value` changes. The Registry adds no
animation and no reduced-motion behavior. Applications that require reduced motion must override
the transition for the value, for example with a caller `motion-reduce:transition-none` utility;
the generated `::before` and `::after` transitions are owned by daisyUI and may require an
application-level override. The live-region announcements are independent of that visual motion.

## daisyUI classes deliberately not used

- There are no countdown modifier classes. Digit width is expressed only through `--digits`.
- Timer utilities or JavaScript are not supplied; scheduling is caller behavior.
- Responsive-prefixed or typography utilities from daisyUI's examples are caller classes.
