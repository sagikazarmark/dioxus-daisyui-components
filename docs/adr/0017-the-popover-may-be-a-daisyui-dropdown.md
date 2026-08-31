# The popover may be a daisyUI dropdown, where the hover card may not

`Popover` positions its panel with daisyUI's `dropdown`, `dropdown-content` and the placement
family, the way the dropdown menu and the select do. ADR-0015 records that the hover card
cannot, and the two decisions have to be read together or the registry looks inconsistent.

The rule ADR-0015 turns on is still there:

```css
:is(.dropdown.dropdown-open, .dropdown:focus, .dropdown:focus-within) > [tabindex]:first-child {
  pointer-events: none;
}
```

It does not reach this component, for two independent reasons.

**The trigger is not a `[tabindex]` element.** `PopoverTrigger` renders a `button`, focusable
because of what it is rather than because of an attribute, and the primitive offers no way to
render it as anything else: there is no `as` prop on it, unlike the dropdown menu's trigger. The
selector needs a `tabindex` attribute and there is none, so the rule matches nothing here. This is
also why the component does not expose a way to swap the element out: doing so would hand a caller
the one shape that breaks it.

**The interaction the rule was written for is the one a popover wants.** A hover card opens on
`mouseenter` and closes on `mouseleave`, so a trigger that stops answering the pointer never
receives the second event, and the card flickers open and shut. A popover toggles on click. Even
if a trigger did carry `tabindex`, the click that the rule lets fall through to the page is
dismissed by the primitive's own outside-click listener, which closes the popover, which is what
the second click was for.

## Consequences

- The open state is lifted (ADR-0006) so `dropdown-open` can be emitted, as it is for the dropdown
  menu; without the class daisyUI hides the panel outright.
- The placement and alignment axes are daisyUI's classes rather than utilities of the registry's,
  and every value emits one (ADR-0008). The hover card's equivalents are utilities, which is not a
  difference in taste but the same difference recorded here.
- The same axes are handed to the primitive, so `data-side` and `data-align` report what the
  classes did, and a caller who switches the classes off still has them to position from.
- The panel does not collide-detect, as daisyUI's dropdown does not: the anchor-positioning path
  leaves it to the browser, and the fallback does nothing.
- If upstream ever gives `PopoverTrigger` an element-override prop, this ADR is what says the
  registry should not pass it through without dealing with the rule first.
