# Desktop/WebView smoke checklist

The Registry supports browser CSR and desktop/WebView through the same Dioxus DOM APIs.
Browser evidence is automated (Playwright, axe, and Chromium screenshot comparisons).
Desktop evidence is **manual**, per OS and WebView: there is no automated desktop run or
desktop screenshot diff. A native build or a Playwright WebKit pass is not a WebView smoke pass.

## Run it

Install the [Dioxus desktop prerequisites](https://dioxuslabs.com/learn/0.7/getting_started/)
for your OS. Linux needs GTK 3, WebKitGTK 4.1, app indicator development libraries, and a
graphical session. From the repository root:

```shell
npm install
npm run build
dx serve --package dioxus-daisyui-components --bin preview --features preview --platform desktop
```

Record the date, commit, OS, WebView version, and tester. Open each Component page from the
sidebar, use both pointer and keyboard, and let asynchronous focus settle before judging it.
Use the WebView inspector when needed to inspect `document.activeElement`, `inert`, and native
input properties. Start each independent scenario from a freshly opened page. Open an Example's
**Code** tab: the source must be highlighted, readable, and match the rendered Example.
Navigate between pages and change the header theme; styles and highlighted code must remain usable.

Enter `PASS`, `FAIL` with a symptom/issue link, or `BLOCKED` with a reason in **every** row.
Copy the table for each additional OS/WebView run; do not infer a Windows or macOS result from
Linux. Record failures even when the browser suite passes.

## First run — 2026-09-16

- Revision: based on `e22e5c8`, issue #23 working tree.
- Environment: Linux x86_64, Dioxus CLI 0.7.10; agent shell without `DISPLAY` or
  `WAYLAND_DISPLAY` or an interactive WebView. GTK 3.24.52 and WebKitGTK 2.52.5 were
  supplied through a temporary devenv package override for the native build.
- Tester: coding agent (build checks only; no human interaction run).
- Linux WebKitGTK: **BLOCKED** for interactive smoke; first manual run still required.
- Windows WebView2 and macOS WKWebView: **BLOCKED**, hosts unavailable.

The dated results below record the attempted run, not a successful desktop qualification.

| Page | Scenario and expected result | Dated result (Linux WebKitGTK) |
| --- | --- | --- |
| `slider` | Tab to the single thumb and both range thumbs; arrows and Shift+arrows update the value, Home/End reach bounds. Drag and click the track: the appropriate thumb gains focus, updates value, and remains keyboard-operable. Field focus targets the intended thumb. | 2026-09-16 BLOCKED — no interactive WebView |
| `drawer` | Open by pointer and keyboard: focus enters the drawer. Tab/Shift+Tab stay inside. Escape, close button, and backdrop dismiss; focus returns to the trigger, including backdrop pointer dismissal. Reopen and repeat. | 2026-09-16 BLOCKED — no interactive WebView |
| `select` | ArrowDown opens on the first enabled option, ArrowUp on the last; Enter/Space opens on the list. Arrows, Home/End, and typeahead navigate; Enter selects, disabled options are skipped. Escape closes without selection. Record close focus: the current Primitive does not restore the trigger (the documented browser baseline is `body`); verify the next Tab remains usable. Field focus targets the trigger. | 2026-09-16 BLOCKED — no interactive WebView |
| `combobox` | Focus the input, type to filter, and open the list. Arrows navigate enabled matches and Enter selects; Escape dismisses and editing focus remains usable. Pointer selection and outside dismissal work; moving within the input/list does not report Field focus exit. | 2026-09-16 BLOCKED — no interactive WebView |
| `checkbox` | Tab and Space toggle, mixed state is visible, disabled controls do not toggle. Field focus reaches the visible control; native checked/indeterminate state and reset agree with the displayed value. | 2026-09-16 BLOCKED — no interactive WebView |
| `switch` | Tab and Space toggle; Field focus reaches the visible switch. The hidden native form participant is not a tab stop, checked state agrees with the switch, and reset restores the value. | 2026-09-16 BLOCKED — no interactive WebView |
| `radio_group` | Tab enters on the selected/first enabled item; arrows/Home/End move focus and selection, skipping disabled items. Field focus reaches that same item. Movement within the group does not report focus exit; leaving does. | 2026-09-16 BLOCKED — no interactive WebView |
| `textarea` | Label and Field focus reach the textarea; multiline editing, selection, and blur work. Disabled/read-only behavior is preserved. | 2026-09-16 BLOCKED — no interactive WebView |
| `input` | Label and Field focus reach the native input; type, select, edit, and blur. Prefix/suffix adornments do not steal editing focus; disabled/read-only behavior is preserved. | 2026-09-16 BLOCKED — no interactive WebView |
| `native_select` | Label and Field focus reach the native select. Open with the platform's keyboard shortcut, navigate options, accept and cancel. Value and focus agree after the native popup closes; disabled options cannot be selected. | 2026-09-16 BLOCKED — no interactive WebView |
| `otp` | Clicking any visual slot and Field focus reach the single native input. Type, paste, select, backspace, and use arrows: displayed slots/caret follow the value and selection without introducing extra tab stops. | 2026-09-16 BLOCKED — no interactive WebView |
| `dialog` | Pointer and keyboard open move focus inside. Tab and Shift+Tab wrap at both ends; background content is inert and cannot be focused or activated. Escape, close button, and backdrop dismiss and return focus to the trigger; background becomes usable again. | 2026-09-16 BLOCKED — no interactive WebView |
| `alert_dialog` | Open by pointer and keyboard; focus enters and Tab/Shift+Tab remain trapped while the background is inert. Cancel, confirm, and Escape close and restore trigger focus; backdrop clicks do not dismiss. With stacked dialogs, closing the top restores focus to the lower dialog and keeps the background inert until both close. | 2026-09-16 BLOCKED — no interactive WebView |
| `popover` | Open and verify initial focus in the panel, then close with Escape/outside click and verify appropriate focus return. Modal example traps Tab/Shift+Tab; non-modal example allows focus to leave. Reopen after both dismissal paths. | 2026-09-16 BLOCKED — no interactive WebView |
| `toast` | Dispatch each kind: the provider places/stylizes the live toast list. Use F6 to reach the toast region and Tab to reach its close control. Dismiss by keyboard and verify focus remains usable. Timed toasts expire (hover/focus do not pause the current Primitive's timer); permanent toasts remain until closed. | 2026-09-16 BLOCKED — no interactive WebView |
| `theme_controller` | Header theme selection restyles the whole Preview and preserves the Component page. Radio/checkbox Examples are keyboard-operable, checked state and reset agree; example-only theme names do not replace the header's real theme. | 2026-09-16 BLOCKED — no interactive WebView |
| `schemaform_daisyui` — resync | In Controls, choose MFA and Recovery replacements: each native select returns to its placeholder. Nickname starts null and cannot accept text until replaced: in the inspector, set that input's native value to `rejected` and dispatch a bubbling `input` event. It must return to its canonical empty value and the null presence state must remain. A Quantity edit `abc` is a retained parse draft, **not** a rejected-write resync: it stays visible with a linked error. | 2026-09-16 BLOCKED — no interactive WebView |
| `schemaform_daisyui` — submission | In Overview, set Display name to `A` and submit. Submission is blocked, findings appear, and focus reaches the findings summary. Tab to its summary finding and activate it: focus reaches the invalid control. Correct the value and submit again. | 2026-09-16 BLOCKED — no interactive WebView |
| `schemaform_daisyui` — arrays | In Arrays, append and insert Tags and Team items: focus reaches the new item's first editable control. Move an item up and down: focus stays with its action in the moved row, or the opposite move action when the invoked action disappears at a boundary. Remove middle/last items: focus moves to a surviving item; remove all Tags: focus falls back to append. Exercise container remove/materialize and verify focus is usable afterward. Check values, row identity, and the live announcement after **each** mutation. | 2026-09-16 BLOCKED — no interactive WebView |

## Build evidence

Record build/check outcomes here separately from the interaction table. The schemaform and
schemaform-dioxus 0.4.1 requirements already include
[schemaform#29](https://github.com/sagikazarmark/schemaform/issues/29), which removes the
desktop focus/resync no-op implementations.

- 2026-09-16: before this change, `cargo check --features preview,desktop --bin preview`
  failed because the package had no `desktop` feature.
- 2026-09-16: after selecting the desktop renderer, the host check reached `glib-sys` and was
  blocked by missing system development libraries. This is not an interactive pass.
- 2026-09-16: with the temporary Linux prerequisites, `cargo check --locked --features desktop
  --bin preview` passed, including compile-time Example highlighting.
- 2026-09-16: `dx build --package dioxus-daisyui-components --bin preview --features preview
  --platform desktop` passed and bundled `target/dx/preview/debug/linux/app`. The temporary
  devenv packages were `pkg-config gtk3 webkitgtk_4_1 libayatana-appindicator openssl xdotool`;
  `CARGO_PROFILE_DESKTOP_DEV_DEBUG=0 CARGO_INCREMENTAL=0` kept the build within disk capacity.
- 2026-09-16: the requested `dx serve` command built and attempted to launch the Linux app,
  which exited with `Failed to initialize GTK` in this display-less session. Window opening
  and interaction remain unverified.
- 2026-09-16: `dagger check preview:install:closure preview:install:registry` passed; both
  checks compiled installed source for the host and `wasm32-unknown-unknown`.
- 2026-09-16: `cargo test --locked` passed (126 conformance tests, 5 policy tests), as did
  `cargo fmt --check`. Playwright attempts were blocked by insufficient disk space during
  container-image extraction and workspace snapshotting; no browser or screenshot result is
  claimed for this change.
