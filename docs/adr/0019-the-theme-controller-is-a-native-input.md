# The theme controller is a native input

`ThemeController` renders an `input` of its own and wraps no primitive. It was the first component
in the registry to do so, and the shape is daisyUI's rather than a preference.

daisyUI switches the theme in CSS. Every theme it emits carries a second selector beside
`[data-theme]`:

```css
:root:has(input.theme-controller[value=dark]:checked), [data-theme=dark] { … }
```

That selector names the element type, the class, the attribute and the pseudo-class. Only a real
form control can be `:checked`: it is a state the browser puts an element in, not one anything
can write, so the styled element has to be an `input`.

No primitive can be that element. `dioxus-primitives` renders every control as a `button` with a
`role`, which is exactly what makes its checkbox and its switch work without a form control at
all. Two of them keep an `input` in the DOM so a form still submits, but it is `aria-hidden`,
visually hidden, and takes no attributes from outside: `Checkbox` passes its bubble input the
checked state, the name and the value, and nothing else; there is no way to put a class on it.
A theme controller built on either would carry `theme-controller` on a `button`, where daisyUI's
theme rule and its paint both match nothing.

## What the browser supplies instead

Everything the primitive would have. A native checkbox and a native radio have focus, the space
bar, the arrow keys within a named group, the `disabled` attribute, form participation and their
own ARIA roles: the list this registry keeps primitives for, arrived at from the other side. A
theme controller needs no state machine, no dismissal and no portal, which is why this component
can afford to have none.

The component still depends on `dioxus-primitives`, for the `merge_attributes` every component
uses to give a caller the same rule about classes and attributes. So the manifest declares the
primitive exactly as the other twenty-six do, and the Registry policy test is unchanged.

## Consequences

- The registry's first component with no primitive part, and the first whose element is a form
  control. The **Native control** entry in `CONTEXT.md` names the case; another daisyUI component
  written against a real input state follows the same reasoning.
- **No state to bridge, and none to lift.** The checked state is the input's own, daisyUI reads it
  where it is, and this component emits no class for it. ADR-0006's lifting exists so a modifier
  class can be emitted from Rust; here there is no modifier class and nothing to emit it from.
- The state is the browser's rather than the caller's: `default_checked` seeds the input through
  `initial_checked` and the component never writes it again. A theme the *app* owns is
  `[data-theme]`, which is a different thing from this component and is what the preview itself
  does.
- The input-gated size scales ADR-0010 records as unreachable are reachable here, and this is the
  only place they are. That ADR's `.toggle-lg[type=checkbox]` and `.radio-lg[type=radio]` match
  when the element is an input of the right type, which is why the appearance axis owns the type
  as well as the paint.
- The component takes no `extends = input` attribute list. It renders the element, and `type` and
  `value` are what make it a theme controller; a caller who could override them could produce a
  control that themes nothing while looking exactly like one that does.
- The preview is themed by this component, through the switcher in its header, so the mechanism
  is on screen on every page rather than described. ADR-0020 records what that took: no controller
  on a component page may name a theme the preview enables, or a documentation page would re-theme
  the site it is documented on.
