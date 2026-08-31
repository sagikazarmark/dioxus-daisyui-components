# A README links to its examples rather than restating them

A Component's `README.md` opens on its heading, its introduction, and then the two links that
lead to code: its page in the deployed Preview, and the `docs/examples/` directory that page is
built from. It does not open on a synopsis.

Every README used to carry one: a complete call, written out immediately under the introduction,
showing the component's axes and required props. That snippet was a second copy of what the
`overview` Example already renders. ADR-0009 put the code in the Example precisely so that a
snippet could not drift from what it documents and a snippet that stopped compiling would fail
the build. A synopsis in the README is outside that guarantee — nothing compiles it, nothing
renders it, and nothing notices when the API it shows moves.

Nothing had drifted when this was decided. Each of the fifty-seven synopses still named props
and types that existed. The decision is therefore about where the guarantee reaches rather than
about a defect: a second copy is worth removing while it is still correct, because the moment it
is wrong is the moment a reader has already trusted it.

## What stays

Prose may still quote code. A fragment written into an argument — the elided
`DropdownMenu { id: "actions", … }` that shows which element a caller should address, the
`DialogCtx` close button, the `FieldRow` carrying `justify-between` — illustrates the sentence
around it and is not a call anybody copies whole. Each such fragment is already covered by an
Example, and lifting it into one would mean inventing the context it elides and leaving the
paragraph pointing at nothing. daisyUI's own CSS and markup, quoted to show what a selector
matches, was never the Registry's code to compile.

The line between the two is what the fragment is *for*: a synopsis stands in for an Example, and
a fragment stands in for a sentence.

## Consequences

- The Registry policy test checks that a README's body begins with its own component's links, so
  a synopsis cannot come back unnoticed and a copied README cannot point at the wrong page.
- A reader on GitHub no longer sees working code without leaving the page. They get the Preview,
  where the code is rendered beside the source, and `docs/examples/`, which GitHub lists in
  place. This is the accepted cost of single-sourcing.
- The API a synopsis used to summarise is now read from the Axes section, the props' own doc
  comments (ADR-0029), and the Examples.
- A README is narrative documentation and developer notes: composition, state bridging,
  deviations, and the daisyUI classes deliberately left alone. Nothing in it duplicates the
  Preview.
- No Component renders its README into its own Preview page. `countdown` and `radial_progress`
  were the two that did, and their links line would have pointed at the page the reader was
  already standing on. Dropping `render_readme` from both makes every Component page
  examples-only, which is what the rest of the Preview already was, and leaves each document
  read where it belongs: the README on GitHub, the Examples in the Preview.
- `crate::example::ReadmeSection` stays. It is the Registry's README adapter, replacing the
  tooling's isolated default so that a rendered README's Tailwind and daisyUI classes stay
  visible to the Registry's scan (ADR-0027). Nothing invokes it today; a documentation module
  that opts back in with `render_readme: true` picks it up again by convention.
