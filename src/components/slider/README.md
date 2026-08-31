# Slider

A slider styled with daisyUI's `range` classes, wrapping the `dioxus-primitives` slider. Its
handle is a real control that announces its value, moves with the arrow keys (ten steps at a
time with Shift held) and can be dragged. There can be two of them, which a native range cannot
do.

[Live examples](https://daisyui-components.dioxus.cc/components/slider) ·
[their sources](docs/examples/)

`SliderField` and `RangeSliderField` are the closed happy-path compositions over the separate Field
parts.

Each renders `Field`, `FieldLabel`, its collapsed control, the optional `FieldDescription`, and an
always-mounted `FieldError`, in that order. They have no children slot. Use `Field` plus its
Compound parts when content or attributes must land between those elements. Generated control and
part ids provide the accessible relationships without id props on either wrapper.

Both wrappers forward the complete surface of their underlying control: the colour, size, and root
appearance Axes; binding, metadata, value, default value, bounds, step, required, disabled, name,
orientation, inversion, change, commit, and Focus Exit props; and global attributes. Caller classes
and attributes therefore land on the slider root, not the surrounding Field. `field_appearance`,
`description_appearance`, and `error_appearance` forward the three corresponding Field-part Axes.
Styling a different part is another signal to use the parts directly.

The visible wrapper label is passed both to `FieldLabel` and to the control's `label` prop, so the
thumb is named as well as visibly captioned. The pinned Primitive gives both `RangeSlider` thumbs
that one shared name. Callers that need distinct start and end names must use the Compound
`Field`, `FieldLabel`, `RangeSliderRoot`, `SliderTrack`, `SliderRange`, and `SliderThumb` parts and
supply the individual thumb attributes themselves.

## Composition

The compound parts are `SliderRoot` (the control, which carries `range`), `SliderTrack` (the
groove), `SliderRange` (the filled part) and `SliderThumb` (the handle). `Slider` is the collapsed
component over all four, and `RangeSliderRoot` and `RangeSlider` are the two-thumb forms, which
share every part.

The collapse is legal: daisyUI's own range is a single element whose groove, fill and handle are
pseudo-elements, so there is nothing between the parts and no caller content can go there. Caller
attributes land on the **root**, which is the element a caller sizes and positions and the one
daisyUI's class is written for.

The track must stay a direct child of the root, and the fill and the handle inside the track: the
inline percentages the primitive positions the last two with are percentages of the element they
are in.

A two-thumb slider writes two handles, `index: 0` for the start of the span and `index: 1` for its
end. Each is bounded by the other.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, `Slider` and
`RangeSlider` resolve Field Context and then standalone state. Field metadata supplies the root's
id, ARIA required and disabled state, description relationships, and `data-*` state. The root's
`role=group` omits native names and deprecated validity ARIA. Explicit props and caller attributes
win over metadata.

## Conformance

`Slider` and `SliderRoot` are **field-aware** for `f64`; `RangeSlider` and `RangeSliderRoot` are
field-aware for `Range<f64>`. An explicit Binding wins over Field Context, and either form owns
standalone state seeded by `default_value` when neither is present. The lower-level trio remains
available: a `Some` value overrides the rendered Binding value and `on_change` observes every
pointer or keyboard write. Writes and commits still reach the resolved Binding.

One pointer drag is an interaction unit: pointer movement produces changes and a root-observed
pointer release produces one commit. An arrow key changes the active thumb on keydown and commits
that interaction on keyup. The pinned Primitive exposes only `on_value_change`; it has no release
callback, so the Registry adapts release from `pointerup`/`pointercancel` on the root and arrow
`keyup` on each thumb. Global attributes continue to spread onto the root.

Focus Exit is independent of that Commit boundary. Its complete logical scope is the slider root
subtree, including both thumbs of a `RangeSlider`. Bubbling `focusout` and `focusin` on the root are
coalesced for one macrotask, so moving from the start thumb to the end thumb remains internal while
leaving the root reports once. The resolved Binding receives `focus_exit()` first, followed by the
optional direct `on_focus_exit` prop. Pointer and key release never manufacture Focus Exit, and an
unchanged departure never manufactures Commit. The Component does not infer touched, blurred, or
validation state from either event.

Field focus requests call `set_focus` on thumb `0`, which is the only thumb for `Slider` and the
start thumb for `RangeSlider`.

The installable Slider Component revision-pins the Field Component it composes, so installing
Slider also installs the matching Field parts required by `SliderField` and `RangeSliderField`.

## State bridging

There is no value state to bridge. A slider is never open, checked or selected. Its value is a
number, and daisyUI has no class that says anything about which number it is - a native range
draws its own fill from its own value, and there is no modifier to emit.

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `range-error`. Passing any colour value explicitly, including
`SliderColor::Default`, wins over metadata. The Registry does not compute invalidity.

What this component does instead is split daisyUI's own class, and ADR-0016 records the split in
full: `range` and both axes stay on the root, which is the element that is the control, while the
groove, the fill and the handle are drawn by the parts with utilities that read daisyUI's own
custom properties. `--range-thumb-size`, `--range-bg`, `--range-thumb`, `--range-p` and
`currentColor` are all published by the class on the root and inherit down, so the parts take
their colour and their size from daisyUI rather than from a second mapping of this registry's own.

The disabled state is the one thing that comes close to bridging, and it is a **Bridged utility**:
daisyUI writes the state as `.range:disabled`, which cannot fire on an element that is not a form
control, and the primitive reports it as `data-disabled` on every part. So the root emits daisyUI's
own two declarations (`cursor-not-allowed` and `opacity-30`) behind a `data-[disabled=true]:`
variant, as part of its appearance axis. Explicit props or Field metadata can supply the disabled
value, but the paint still reads the Primitive's `data-disabled` attribute rather than a modifier
class recomputed by the Registry.

## Axes

- `color: Option<SliderColor>`: `range-neutral`, `range-primary`, `range-secondary`,
  `range-accent`, `range-info`, `range-success`, `range-warning`, `range-error`; omission permits
  invalid Field metadata to emit `range-error`.
- `size: SliderSize`: `range-xs`, `range-sm`, `range-lg`, `range-xl`.
- `appearance: SliderAppearance` on the root: `relative`, plus the disabled treatment above.
- `appearance: SliderTrackAppearance`: the groove.
- `appearance: SliderRangeAppearance`: the fill.
- `appearance: SliderThumbAppearance`: the handle.

An explicit `SliderColor::Default` emits nothing, which is daisyUI's uncoloured range: a groove and
a fill in the page's own text colour. `SliderSize::Default` emits nothing and renders at the same
size as daisyUI's explicit `range-md`.

Both daisyUI axes are on the **root**, which is where daisyUI writes them and where the custom
properties they set have to be for the parts to inherit them. Neither is repeated on a part.

The four appearance axes invert the usual convention: their default value *emits* utilities and
their `None` value emits nothing. They are what ADR-0016 is about (the declarations daisyUI
writes in pseudo-elements that do not exist here) and a caller who wants a different groove
switches ours off rather than out-ranking it, since two utilities only tie and the tie is settled
by generated-stylesheet order (ADR-0004). None of them is named `style`, which collides with the
global HTML attribute.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The Primitive's callback is adapted to the Binding trio.** Its `on_value_change` callback is
exposed as `on_change`, and root/thumb release supplies `on_commit`. This is a deliberate breaking
rename at Registry version `0.0.0`; retaining both callbacks would report one write twice. A
release commits only when that interaction changed the value, so a blocked key or unrelated
pointer release is not an interaction unit of its own.

**Pointer release is adapted with pointer capture.** The pinned Primitive tracks pointer release to
end its private dragging state but exposes no callback for the Registry's commit hook. A
capture-phase listener on the root therefore calls the browser's `setPointerCapture` when a primary
pointer interaction starts. The browser retargets `pointerup` or `pointercancel` to the root even
when release happens outside it, where the Registry synchronously commits the pending change.
Starting another pointer interaction still discards any pending marker before that interaction can
write, and keyboard commits are tracked separately, so unrelated releases and blocked keys do not
commit stale changes.

**`FieldLabel` and `label` are complementary.** Field metadata carries the root id used by
`FieldLabel`, but it carries no label text or label id that the Primitive can put on its thumb
buttons. The existing `label: ReadSignal<Option<String>>` therefore remains the accessible-name
prop and is not derived from metadata. `SliderField` and `RangeSliderField` repeat their visible
label there automatically; hand-written Compound compositions must do the same. On `RangeSlider`
the Primitive applies the one label to both thumbs, so callers that need distinct start/end names
must use the Compound fallback described above.

**Field focus registration targets the first thumb.** The root shares its first mounted thumb with
the Field focus slot, because the Primitive's `div[role=group]` does not take focus. Dioxus 0.7
keeps one same-name listener per element, so the forwarded mount listeners replace the Primitive's
listeners. The wrapper repeats the Primitive's focus move after each value change, targeting the
thumb whose value changed; Field requests separately target thumb `0`.

**The control is four elements, where daisyUI's is one.** The primitive renders
`div[role=group] > track > range + thumb`, and it has to: a native input has no elements inside
it, so it could not carry two thumbs, could not report which one is being dragged, and could not
give the handle a `role="slider"` of its own to be named and moved by the keyboard. ADR-0016 is
the decision that follows.

**The groove is inset by half a handle at each end.** That is where a native range ends the
thumb's travel, and the primitive's percentages are raw, so the inset is put back here, and the
fill and the handle line up with the value the way daisyUI's do. Without it a handle at either
end would be cut in half, since `.range` sets `overflow: hidden`.

**There is no vertical slider in daisyUI's sense.** `range-vertical` turns a native input by its
writing mode and re-points the box shadow that draws the fill, and neither reaches an element the
primitive's `horizontal` prop is passed through, so the *behaviour* (which
arrow keys move the value, which axis a drag follows) is available; the look is not. A vertical
slider means switching the part axes off and writing the box. This is the same trade the tabs
component records for its own orientation.

**Home and End do not move the value.** The primitive listens for the four arrow keys and for
Shift with them, and nothing else; a native range jumps to its ends on Home and End. It is the
primitive's gap rather than a daisyUI one, and it is recorded here rather than papered over with a
key handler of this component's own: the registry maps classes, and behaviour belongs upstream.

**A disabled slider's handle leaves the tab order.** The primitive gives the handle
`tabindex="0"` and only `data-disabled` whatever the state, then refuses its keys. The Registry
overrides those semantics with `tabindex="-1"` and `aria-disabled="true"`, so the inert handle is
announced disabled and sequential keyboard navigation passes it over like a native disabled range.

**The fill does not animate.** daisyUI's native fill does not either (it is a box shadow that
follows the thumb) and the registry adds no animation of its own. A caller who wants one adds a
transition utility.

**A slider is not a form control.** There is no hidden input, and the `div[role=group]` root omits
the Field name because a native name does not apply there. Its value reaches an app through
`on_change`. That is the primitive's shape rather than this component's.

## daisyUI classes deliberately not used

- `range-vertical`: the deviation above.
- `range-md`: the size value the default renders at, which is what emitting nothing already does.
- The `:disabled` and `:focus-visible` rules `.range` carries: neither can fire on the elements
  the primitive renders, and both are re-emitted where they can: the muting through a data
  attribute on the root, the focus ring on the handle that actually takes focus (ADR-0016).
- The responsive prefixes daisyUI generates for the colour and size classes (`sm:range-lg` and the
  rest): a caller reaches those through `class`, which concatenates.
- `validator`: `aria-invalid` is deprecated on the root's `role=group`, while this Component
  already emits its explicit `range-error` modifier from Field metadata. Adding `validator` would
  duplicate that Axis and also opt into browser-owned valid paint.
- `validator-hint`: its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead renders a polite live region and joins the root's `aria-describedby`, without
  that adjacency and class coupling.
