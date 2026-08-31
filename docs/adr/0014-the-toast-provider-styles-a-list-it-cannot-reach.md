# The toast provider styles a list it cannot reach

daisyUI's `toast` class goes on the primitive's region element, and the column and the spacing it
would have given the toasts are re-emitted as utilities that reach through the list between them:
`[&>ol]:flex [&>ol]:flex-col [&>ol]:gap-2`.

daisyUI's toast container is a fixed box whose **own children** are the toasts. It is a flex
column with a gap, and it animates each child in. The primitive renders four levels instead of
two (`div[role=region] > ol > li > div[role=alertdialog]`) because a stack of notifications is
a list, and because the region is what `F6` focuses and what announces how many notifications
there are.

Neither of the two elements in between takes an attribute. `ToastList` and `ToastListItem` are
public components with attribute props, but the provider renders them itself and passes nothing
through, and the provider is not replaceable: `render_toast` reaches the toast inside the `li`,
not the `li`. So there is no element to put a class on between the container and the toast.

Putting `toast` on the region is still right: it is the element that has to be fixed to a corner
of the viewport, and it is the one whose position the placement and alignment axes describe. What
is lost is what daisyUI's rules would have done to that container's children, and the loss is
one level deep, so a descendant utility reaches it exactly.

## Consequences

- The utilities are defeatable (ADR-0004): a caller who wants another arrangement switches ours
  off rather than out-ranking them.
- **The per-toast animation is lost.** daisyUI animates `.toast > *`, which here is the list
  rather than each toast, so a toast that arrives while another is up slides nothing. The
  registry adds no animation of its own (§2), so this is documented rather than replaced. A
  caller who wants one writes it in their own stylesheet against the `li`, which is theirs to
  target even though it is not theirs to attribute.
- The list keeps its semantics. `display: contents` on the `ol` would have made the `li`s the
  flex children of `.toast` directly and given daisyUI's own rules something to work on, at the
  price of a list whose box is gone, which is exactly the shape assistive technology has
  historically dropped list semantics for. A gap and a column are not worth that.
- Anything that dispatches a toast must be a component *under* the provider, because `use_toast`
  reads the provider's context. That is the primitive's design rather than this decision, but it
  is the first thing anyone meets when they write one, so the component's documentation says it
  outright.
