# Schemaform DaisyUI widgets a form may not reach are add-ons

`schemaform_daisyui` used to install everything it could ever render. Its manifest pulled the
compound `select`, `radio_group` and `checkbox` Components and shipped the homogeneous-array
collection renderer, whether a host's schemas reached them or not. The ACP Inspector's elicitation
schemas reach none of them: it names no widget symbol, has no nullable booleans and no arrays of
free strings. About 2.1k of the 5.1k lines it installed were unreachable at runtime, and a binary
host warns on every one (#33). `dx components add` has no features to turn parts of a Component
off, so the only unit of opt-in is a Component.

## The heavy widgets become add-on Components

Three Components now extend `schemaform_daisyui` rather than living inside it:

- `schemaform_daisyui_radio` registers the `daisyui:radio` widget symbol and depends on
  `radio_group`.
- `schemaform_daisyui_select` registers `daisyui:select` and depends on `select`.
- `schemaform_daisyui_collection` fills the homogeneous-array slot of the structure bundle.

Radio and select are separate rather than one "choice widgets" add-on because each maps to
exactly one heavy dependency. A host that wants radios should not install the listbox, which is
the same reasoning ADR-0032 gave for the native select's own manifest.

The base keeps what every form reaches: text, number and integer inputs, the native select for
choices, booleans, multiple choice, constants, presence operations, the shell, and the finding
presenter.

## The nullable checkbox is replaced, not made optional

The registry `Checkbox` was there for one reason: a nullable boolean shows JSON null as the
indeterminate state, and a native checkbox has no declarative way to. Once the base set the
native `indeterminate` property for a boolean holding no value (#36), the native checkbox could
show null the same way, reach it through the same set-null affordance, and leave the click
behaviour unchanged. Every host loses the dependency and nothing has to be installed back.

## The seam is a trait the base declares

`schemaform_daisyui` declares `Addon`, with `controls` and `structure` methods that default to
leaving the control registry and the structure bundle alone. Both receive the form's `Options`,
so an add-on's renderers share its appearance, density and button classes. `SchemaformDaisyui`
takes `addons: Addons`, read once at mount like every other renderer input, and
`configuration_with_addons` composes the same configuration for a host binding `SchemaForm`
itself. A host composing seams by hand calls an add-on's methods directly.

A widget add-on's renderer is the base `DaisyuiControlRenderer` with its choice widget swapped.
An exact widget request reaches its renderer whatever the control's kind, so the add-on delegates
every other kind to the base, and the base's scope for density and button classes wraps the
add-on's control too.

The parts an add-on builds on — the field frame, the mapping helpers, the presence buttons, the
icon — are `pub(crate)` re-exports of the base. Every installed Component lives in the host's own
crate, so they reach the add-ons without joining the base's public API. The add-ons'
`componentDependencies` pin `schemaform_daisyui` to a revision of this Registry, like every other
dependency on it, so an add-on and its base always share one contract.

## Consequences

- A UI schema naming a widget symbol whose add-on is not installed fails to bind with the
  adapter's `MissingWidget` finding, which `SchemaformDaisyui` raises as a render error. A
  homogeneous array without the collection add-on keeps the adapter's built-in collection, as
  every unfilled structure slot does.
- Adding a widget add-on twice registers its symbol twice; the adapter reports that as an
  ambiguous widget at bind.
- The split removes unreachable source from a host's tree. It does not shrink the resolved crate
  graph: the base's own registry dependencies still depend on `dioxus-primitives`. A binary host
  still warns on whatever part of an installed Component's public API it leaves unused.
- `dx components add --force` does not remove Components an install no longer needs, so a host
  upgrading from before the split deletes `checkbox`, `radio_group` and `select` itself.
- The documentation tooling requires a public `#[component]` in every member's `component.rs`.
  The add-ons expose their controls (`RadioGroupControl`, `SelectControl`) and the collection's
  two renderings (`DaisyuiCollectionFieldset`, `DaisyuiCollectionItem`) as components, which a
  host with its own renderer can reuse.
