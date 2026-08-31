# The registry is a root library with a gated preview

The single-package workspace and authoring layout in this decision are superseded by ADR-0026.

The repository root is one unpublished Cargo package named `dioxus-daisyui-components`. Its library
exports the Components authored under `src/components/<name>/`, and its explicit `preview`
binary lives under `src/preview/` and requires the `preview` feature. There is no Cargo
workspace and no separate Preview package.

The Preview re-exports the library's `components` module at its crate root. Examples can keep
using `crate::components::...`, the path a `dx components add` consumer uses, while the Preview
and the library compile one authored component module rather than parallel copies.

Examples are colocated at `src/components/<name>/examples/`. The Preview preserves one logical
`examples` facade with `#[path]` modules that point to those directories, so each example's
`include_str!` still reads the source file compiled beside it. Component `mod.rs` files do not
declare examples. Every component manifest excludes `component.json`, `docs.md`, and
`examples`, preventing authoring material from being copied into a consumer.

The Cargo package includes `src/lib.rs` and Rust files recursively below `src/components/`,
then explicitly excludes every `examples` subtree. It also includes the README and both
licenses. Preview sources, examples, registry metadata, and component documentation are not
package contents. Recursive library inclusion is deliberate: adding a nested implementation
module later must not require another packaging rule.

ADR-0001 put the Registry inside a separate Preview app to keep components at the same
crate-root-relative location a consumer receives. The root library keeps that property while
making the repository's primary Rust target the Components themselves. The Preview is now a
development consumer of the library, with launch and web-renderer support paid for only when
its feature is enabled. Router support remains in the library graph because Navbar requires it.

## Consequences

- A default `cargo check` checks the Component library without enabling web launch support or
  compiling Preview examples.
- Preview commands name the package, binary, and feature explicitly:
  `--package dioxus-daisyui-components --bin preview --features preview`.
- The binary name remains `preview`, so Dioxus keeps writing its built site under
  `target/dx/preview/`.
- `tailwind.css`, npm metadata, and generated `assets/tailwind.css` live at the package root,
  where the Dioxus CLI expects the package's styling inputs and output.
- The Registry and Cargo package have different delivery boundaries: `dx components add`
  copies one Component, while the unpublished library exists to compile all Components
  together and provide the Preview's dependency boundary. Applications may also consume that
  library from a path or Git dependency, but their Tailwind build must scan `src/components` at a
  stable application-controlled path. Cargo's generated Git checkout path is not a portable
  Tailwind source, so this mode requires a path, submodule, or vendored dependency.
