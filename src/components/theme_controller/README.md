# Theme controller

A control that puts a daisyUI theme on the document, styled with daisyUI's `theme-controller`
class. It wraps no primitive and renders the `input` itself, the first component in the registry
to do so, and ADR-0019 records why.

```rust
ThemeController {
    theme: "dark",
    appearance: ThemeControllerAppearance::Toggle,
    color: ThemeControllerColor::Primary,
    aria_label: "Dark theme",
}
```

daisyUI switches the theme in CSS. Every theme it emits is written against two selectors:

```css
:root:has(input.theme-controller[value=dark]:checked), [data-theme=dark] { … }
```

So a checked input with the class and the theme's name in its `value` re-declares the whole
theme on `:root`, and everything under it repaints. Nothing is computed, stored or dispatched;
the component's whole job is to put that class, that `value` and a real `:checked` state on one
element.

The theme has to be one the app enabled in its `@plugin "daisyui"` block. daisyUI emits the rule
above only for the themes named there, and a `value` it never emitted matches nothing and leaves
the document on the theme it was already under, silently, the same way an unknown `[data-theme]`
does.

The control renders no children, because an `input` has none. A caption is the caller's (beside
it, or wrapped around it) and the control is named by `aria_label` or by `aria_labelledby`.
Under `Button`, daisyUI prints that `aria-label` as the button's own text, from
`.btn:is([type=checkbox],[type=radio])[aria-label]::after`.

## State bridging

**There is no state to bridge**, and there is no primitive to bridge it from. The checked state
is the input's own, daisyUI reads it with `:checked` on the very element its classes are on, and
this component neither keeps a copy of it nor emits a class for it. So does the theme rule, which
reads the same pseudo-class through `:has()` from `:root`.

That is what makes this component's markup non-negotiable rather than a preference. The registry
elsewhere accepts a `button` with `role="checkbox"` and bridges what daisyUI cannot match; here
the state daisyUI needs is a native pseudo-class, which only a real `input` can be in. A `button`
would leave both rules (the paint and the theme) matching nothing at all.

The disabled state is not bridged either and needs no tier: `disabled` is a native attribute on
the element, and daisyUI's rules are `.toggle:disabled`, `.checkbox:disabled`, `.radio:disabled`
and `.btn:disabled`.

## Axes

- `appearance: ThemeControllerAppearance`: `toggle`, `checkbox`, `radio`, `btn`, and a value
  that emits nothing.
- `color: ThemeControllerColor`: the eight daisyUI colours, in the scale the appearance names.
- `size: ThemeControllerSize`: `xs`, `sm`, `lg`, `xl`, in the same scale.

`theme-controller` has no paint of its own (it is a hook for the `:has()` selector and nothing
else) so daisyUI's own theme controllers borrow the look of another control, and the appearance
axis is which one this borrows. The colour and size axes follow it: their value names a colour or
a size, and the appearance decides which class carries it, because daisyUI writes a scale per
control and there is no `theme-controller-primary` for it to write one against.

`ThemeControllerColor::Default` emits nothing, which is daisyUI's uncoloured control.
`ThemeControllerSize::Default` emits nothing and renders at the same size as an explicit `-md`.
`ThemeControllerAppearance::None` emits nothing for all three, and takes no colour or size with
it: every class on those two scales only sets a custom property that the paint reads, so one
without the paint would change nothing.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

**The appearance also decides the input's `type`**, which is why it is one axis and not two. Three
of the four scales are gated on it:

```css
.toggle-lg[type=checkbox], .toggle-lg:has([type=checkbox]) { --size: … }
.radio-lg[type=radio]                                      { --size: … }
```

So a `radio`-painted checkbox would carry size classes that cannot match, which is exactly the
"reachable but wrong" trap ADR-0010 records. Each value of the axis is one of daisyUI's own
pairings instead: a toggle or a checkbox is a checkbox, a radio or a button is one of a named set.
The other side of that ADR shows up here as well: these input-gated toggle, checkbox and radio
rules match this Component because its element is a real form control of the required type.

## Deviations

**No primitive.** Recorded in full in ADR-0019: `dioxus-primitives` renders every control as a
`button` with, at most, an `aria-hidden` input beside it that it does not let a caller class, so
no primitive can carry `theme-controller` where daisyUI reads it. The behaviour a primitive would
have added is the browser's here (focus, the space bar, arrow keys within a named radio set, and
form participation) and the component still depends on the crate, for the `merge_attributes`
that gives every component in the registry the same rule about a caller's classes.

**The state is the browser's, not the caller's.** There is no `checked` prop to control, only
`default_checked` to seed with, and it is passed as `initial_checked` (the input's
`defaultChecked`) so a re-render cannot put the control back where the caller started it. A
controlled theme is a different thing from this component: it is `[data-theme]` written by the
app, on the element it wants themed. `onchange` is offered for the one thing the CSS cannot do,
which is remember the choice across a reload.

What that event carries is the input's, not this component's, and it differs with the appearance
the way it differs between any checkbox and any radio: a checkbox reports its state, so
`value()` is `"true"` or `"false"` and `checked()` is what to read; the theme is the one the
caller named. A radio reports its `value`, which here *is* the theme, and is how a set says which
of them was picked.

**Several controllers on a page all feed the same selector.** They are independent inputs, and
`:root` matches every theme whose controller is checked, so the theme that wins is the one daisyUI
emitted last: the order of the app's own `themes` list, not the order the reader clicked in. A
set of themes to choose between is therefore radios sharing a `name`, where the browser keeps one
checked, rather than a row of checkboxes.

**A theme belongs to one control.** The rule above settles what looks like a layout question: a
quick light–dark switch beside a full theme list means two controls offering the same theme, and
there is no honest way to render that. One radio group leaves the browser able to check only one
of the two, so the control the reader did not use shows nothing, or is painted as though it were
checked, which lies to everything reading the page rather than looking at it. A group each leaves
two themes checked at once, which is the paragraph above. So the two controls are a partition:
each theme is offered exactly once, and the browser's single checked input is the answer for the
paint, for the theme and for the accessibility tree at the same time. The preview's own header is
one such menu, and the only theme control on the page; a quick light–dark switch beside it would
be a second control over two themes the menu already offers, which is the case above.

The same rule is what a `swap`-shaped controller runs into. It is a checkbox, so it cannot join a
radio group, and it can therefore only be the *only* theme control on the page.

**Under `Button`, a checked control is primary unless it is coloured.** daisyUI paints any checked
button `:where(:checked)` with the primary colour, which is how its own documentation shows the
selected theme. `ThemeControllerColor::Default` is that colour when the control is checked and the
uncoloured button when it is not: the one place on these axes where the default arm is not a
fixed look.

## daisyUI classes deliberately not used

- `swap`, `swap-on`, `swap-off`, `swap-rotate`, `swap-flip`, `swap-active`: daisyUI's answer for
  a control that swaps one child for another, and the shape its own icon theme controller uses.
  It is markup rather than paint: the classes go on a `label` the caller writes and on the two
  children they put beside the input, and daisyUI reaches them with `.swap input:checked ~
  .swap-on`, a sibling of the element this component renders. `ThemeControllerAppearance::None` is
  what that arrangement wants (the bare input, with the paint left to the caller) and the
  preview renders it.
- `join`, `join-item`: how daisyUI's own button row is welded into one shape. It is layout across
  several controls, which is the caller's markup, not one control's class.
- `dropdown`, `dropdown-content`, `menu`: the wrapper daisyUI's theme *menu* is built from. Same
  reason: it is the container around a set, and a caller who wants it has the registry's dropdown
  menu or daisyUI's own classes.
- `toggle-md`, `checkbox-md`, `radio-md`, `btn-md`: the default size emits nothing, and daisyUI
  renders an unclassed control at exactly that size.
- `btn-outline`, `btn-soft`, `btn-ghost`, `btn-dash`, `btn-link`, `btn-wide`, `btn-block`,
  `btn-square`, `btn-circle`, `btn-active`: the button component's other looks, not exposed here
  for the same reason they are not exposed on the toggle: a caller reaches them through `class`,
  which concatenates. `btn-active` in particular would say a button is pressed while `:checked`
  said otherwise.
- The responsive prefixes daisyUI generates for every class on these scales (`sm:toggle-lg` and
  the rest): a caller reaches those through `class`, which concatenates.
