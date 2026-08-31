# The slider's parts are drawn from daisyUI's variables

`SliderRoot` keeps daisyUI's `range` class and both of its axes; `SliderTrack`, `SliderRange` and
`SliderThumb` draw the groove, the fill and the handle with Tailwind utilities that read
daisyUI's own custom properties (`--range-thumb-size`, `--range-bg`, `--range-thumb`,
`--range-p` and `currentColor`) rather than with any daisyUI class.

daisyUI's range is written for `input[type=range]`, and it splits in two along that element's
seams. What `.range` puts on the element itself is a box, a radius, and a pack of custom
properties. What it puts on the *inside* is written in `::-webkit-slider-runnable-track`,
`::-webkit-slider-thumb`, `::-moz-range-track` and `::-moz-range-thumb`, which exist on a native
input and nowhere else. The primitive renders `div[role=group] > track > range + thumb`, and it
has to: a native input has no elements inside it, so it cannot carry two thumbs, cannot report a
dragging state per thumb, and cannot give the handle a `role="slider"` of its own to be named and
moved by the keyboard.

So one daisyUI class covers four elements' worth of styling and only one of the four lands. §2's
rule for that case is to drop the daisyUI class for the elements it cannot style and use
utilities there, never CSS (ADR-0002). This is ADR-0012's decision again, on a component with
three parts instead of one.

What keeps it from forking is that everything the pseudo-elements need is already published as a
custom property on the element that *does* carry the class, and custom properties inherit:

| Part   | daisyUI's declaration                        | The utility emitted here                                 |
| ------ | -------------------------------------------- | -------------------------------------------------------- |
| groove | `background-color: var(--range-bg)`, half-thumb height | `bg-[color:var(--range-bg)] h-[calc(var(--range-thumb-size)/2)]` |
| fill   | inset box shadow in `--range-progress`, which is `currentColor` | `bg-current`                            |
| handle | `--range-thumb-size` square, `--range-thumb` fill, `--range-p` border of `currentColor` | `size-[var(--range-thumb-size)] bg-[color:var(--range-thumb)] border-[length:var(--range-p)] border-current` |

The colour axis therefore needs no second mapping: `.range-primary` sets `color` and
`--range-thumb`, and every part above takes its colour from one of those two. The size axis needs
none either: `.range-lg` sets `--range-thumb-size`, and the height of the groove and the size of
the handle are both derived from it here exactly as daisyUI derives them.

## Consequences

- The utilities are defeatable (ADR-0004), one axis per part, so a caller who wants a different
  groove switches ours off rather than out-ranking it. The root's axis is defeatable too; it
  carries the positioning context the parts are laid out in.
- **The groove is inset by half a thumb at each end.** A native range insets the thumb's travel by
  that much, so its ends line up with the ends of the groove; the primitive's percentages are raw,
  so the inset is put back here instead. Without it, a thumb at either end would hang half outside
  the control, and `.range` sets `overflow: hidden`, so it would be cut in half rather than
  merely overhang.
- **The focus ring moves to the handle.** daisyUI writes `.range:focus-visible` for a control that
  focuses as a whole; here the handle is a `button` and is what takes focus, so the same ring is
  emitted there.
- **The disabled look is emitted through a data attribute.** daisyUI's is `.range:disabled`, which
  cannot fire on an element that is not a form control. The primitive reports `data-disabled` on
  every part, so the root emits daisyUI's own two declarations behind `data-[disabled=true]:`.
  The handle stays focusable while disabled, which is the primitive's decision and is documented
  rather than corrected.
- `range-vertical` is not offered. It turns a native input with `writing-mode: vertical-lr` and
  re-points the fill's box shadow, neither of which reaches an element the primitive renders. The
  primitive's own `horizontal` prop is passed through, and a vertical slider means switching the
  part axes off and writing the box, the same trade the tabs component records for its own
  orientation.
- The two-thumb `RangeSlider` gets daisyUI's look for free, on a shape daisyUI has no markup for
  at all.
