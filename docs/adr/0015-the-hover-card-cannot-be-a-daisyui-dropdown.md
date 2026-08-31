# The hover card cannot be a daisyUI dropdown

`HoverCard` positions its panel with Tailwind utilities (`absolute z-10` plus one utility per
side and one per alignment) rather than with daisyUI's `dropdown` and `dropdown-content`.

daisyUI has exactly one floating box: the dropdown. `.dropdown` is `position: relative;
display: inline-block`, `.dropdown-content` is `position: absolute`, and the placement family
resolves the side and the alignment between them. On paper that is the hover card's positioning
already written, with two of its own axes mapping onto daisyUI's, which is what the dropdown
menu and the select both do.

It cannot be used here, because of one rule that comes with it:

```css
:is(.dropdown.dropdown-open, .dropdown:focus, .dropdown:focus-within) > [tabindex]:first-child {
  pointer-events: none;
}
```

That rule is how daisyUI's own CSS-only dropdown closes on a second click: the trigger stops
answering the pointer while the menu is open, so the click lands on the page and blurs it. The
primitive's hover card trigger is a `tabindex="0"` first child, and it is what the card opens and
closes on: `mouseenter` opens it, `mouseleave` closes it. A trigger that stops receiving pointer
events the moment the card opens never receives the `mouseleave` either, and the browser
synthesises one as the element leaves the hit-testing tree: the card closes, the pointer is over
the trigger again, and it reopens. That is a flicker loop rather than a hover card.

Neither half of the rule can be avoided. `:focus-within` matches whenever the trigger is focused,
so it applies to a keyboard user even if the open class were never emitted; `dropdown-open` is
what would have to be emitted for `dropdown-content` to be visible at all. And the registry ships
no CSS (ADR-0002), so the rule cannot be turned off where it is written.

## Consequences

- The positioning utilities are defeatable (ADR-0004), through a placement axis that switches all
  of them off at once; a caller using anchor positioning or a floating-element library needs the
  panel *not* placed, rather than placed differently.
- The alignment utility depends on the side as well as on the alignment, because `left-0` aligns
  the start edge of a panel above or below the trigger and `top-0` aligns the start edge of one
  beside it. The two stay separate axes in the API, since that is how a caller thinks about them,
  and become one class in the emitting.
- **No offset is emitted between the trigger and the panel.** daisyUI's dropdown has none either,
  and here it would be actively wrong: a gap is a gap the pointer crosses, and a pointer that has
  left the trigger without arriving on the panel has closed the card.
- The side and the alignment are still handed to the primitive, so `data-side` and `data-align`
  report what the utilities did. A caller who switches the placement off keeps those attributes
  and can position from them.
- The panel does not collide-detect. daisyUI's dropdown does not either (its anchor-positioning
  path leaves it to the browser and its fallback does nothing), so what is lost against daisyUI
  is nothing, and what is lost against a floating-element library is what the placement axis
  exists to hand over.
