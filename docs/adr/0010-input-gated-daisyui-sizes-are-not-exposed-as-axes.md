# Input-gated daisyUI sizes are not exposed as axes

Neither the switch nor the radio group exposes a size axis, because daisyUI writes those size
rules against a form control the primitives do not render. Where a daisyUI class cannot do what
its name says on this markup, the registry leaves it out and records why, rather than emitting a
class that changes nothing or changes the wrong thing.

daisyUI's toggle sizes are written

```css
.toggle-sm[type=checkbox], .toggle-sm:has([type=checkbox]) { --size: … }
```

and its radio sizes twice over, once plain and once gated:

```css
.radio-lg             { padding: 0.3125rem }
.radio-lg[type=radio] { --size: … }
```

with `.toggle` and `.radio` both taking their `width` and `height` from `--size`, which each of
them also sets on itself, so an ancestor cannot supply it by inheritance.

The primitives render a `button` carrying `role="switch"` or `role="radio"`, with the hidden
input that makes the control submittable as a *sibling* rather than a descendant. So:

- **switch**: neither arm can match. `toggle-lg` would change nothing at all.
- **radio group**: only the ungated arm matches. `radio-lg` would leave the control the size an
  unclassed radio is and pad the dot inside it *down*, so the largest value would draw the
  smallest dot.

The checkbox is unaffected and keeps its size axis: daisyUI writes `.checkbox-lg` as a plain
class selector that sets `--size` outright.

## Which element carries the classes

For the switch there was a second arrangement, and it is worth recording that it was rejected.
Putting `toggle` on a wrapper around the primitive would satisfy `:has([type=checkbox])` and
bring the sizes back, and the base checked rule has a third arm, `.toggle:has(>input:checked)`,
that the mirrored input satisfies. But every colour rule and `.toggle:disabled` are written
against `[aria-checked]` and `[disabled]`, which are on the button inside. A sized switch with no
colour and no disabled state is a worse switch than an unsized one, so the classes stay on the
primitive's button.

Emitting `--size` from the registry (the one thing that would work on either element) would
mean hardcoding daisyUI's own multipliers, which is what ADR-0002 keeps out of this repository:
the value would go quietly stale on a release that repainted the scale.

## Consequences

- Two components document an axis they do **not** have, with the selector that prevents it. A
  reader who wants another size sets `--size` themselves through `class`, their decision to take
  on a daisyUI internal, rather than the registry's to take for them. The preview renders that
  escape hatch and the browser specs assert it works, so the advice cannot rot silently.
- The switch accepts no children either. daisyUI lays a toggle's children out as marks either
  side of the knob and swaps them with `.toggle:has(:checked)`: the same descendant test, and
  the same reason it cannot match. Marks written there would never swap, so the component renders
  none rather than rendering marks that lie.
- This is the first case of a daisyUI class being *reachable but wrong* rather than simply
  unreachable, and the radio group is where the distinction was found. "The class applies" is not
  the test; "the class does what its name says" is.
- A daisyUI release that adds a plain arm to either scale makes the axis available with no change
  to how the components are put together, and this record is what says why it was missing.
