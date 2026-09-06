# An example is a file that is rendered and printed

Each example on a Component page is one file under
`src/components/<component>/docs/examples/`, holding one `Example` component. The page renders
that component and prints that file's source, read with `include_str!` from the module beside
it. The preview tab and the code tab are therefore the same code, and there is no second copy
of a snippet to fall behind.

The alternatives were both worse. A snippet written out as a string beside the component it
documents drifts the moment either is edited, and nothing fails when it does. A snippet
recovered from the tokens with `stringify!` cannot drift, but it comes back as one line with
the formatting gone, which is not a snippet anybody would copy.

Reading the whole file rather than the `rsx!` block alone is deliberate: the imports are the
part a reader most needs, and an example imports from `crate::components::…`, which is the
path `dx components add` installs to. What a consumer would not write (the odd `data-axis`
or `id` the browser tests find the rendered set by) is on an element rather than in a
component's API, and stays visible rather than being stripped out of a snippet that claims to
be the code.

## Consequences

- An example that stops compiling fails the build. This is the point: the Preview is the
  acceptance test (ADR-0007), and its examples are now under the same guarantee.
- Installable Component modules do not declare their examples. Each excluded `docs/mod.rs`
  invokes the documentation macro, which uses `#[path]` to compile the colocated files. The
  generated Component page catalog exposes those modules through the Preview's logical `examples`
  facade, so Examples can share fixtures without maintaining another Component list. Component
  manifests and the root Cargo package exclude all documentation authoring files from consumer
  installs and library packaging.
- An example is a component call site written out in the page, not an entry in a table. A
  table would hand the page a function pointer, and the same component scope would be reused
  across two pages whose examples call different hooks.
- Examples that share a fixture (the select's options, the dropdown menu's items) import it
  from a file beside them, and that file is an example of its own so its source is on the page
  too.
- A documentation macro invocation's list of examples is the one place a new example is
  registered, and the browser tests reach one by the `data-example` slug it is registered under.
- The generated section names, literal-call guarantee, source inclusion, and marker protocol are
  defined by the pinned external
  [`dioxus-registry-preview` consumer contract](https://github.com/sagikazarmark/dioxus-registry-preview/blob/f11ad519a9c80f606de5c0c5ba414af8a293c61e/docs/adr/0001-versioned-consumer-contract.md).
- The code tab's highlighting is computed over that same included text when the module is
  compiled, so it can no more drift from the rendered component than the text can (ADR-0033).
