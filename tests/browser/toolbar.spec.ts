import { expect, type Page, test } from "@playwright/test";

import {
  axis,
  computedStyle,
  example,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "toolbar" });
});

test.describe("styling", () => {
  test("every button colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every button size applies a size of its own", async ({ page }) => {
    await expectAxisGrows(page, "size", "height");
  });

  // The colour is read off the rule rather than off the element, as it is on
  // the separator's own page: daisyUI draws both halves of the line in
  // `::before` and `::after`, and the element itself is transparent under every
  // value.
  test("every separator colour paints the rule a colour of its own", async ({ page }) => {
    expectVaries(
      await computedStyle(axis(page, "separator"), "background-color", "::before"),
      "separator",
    );
  });

  test("the appearance axis decides whether the row is laid out at all", async ({ page }) => {
    await expectAxisVaries(page, "appearance", "display");
  });

  // The direction is emitted from the same prop the arrow keys go by, so that
  // the two cannot disagree: this is the looking half, and the keyboard half is
  // below.
  test("a vertical toolbar runs down rather than across", async ({ page }) => {
    await expect(page.locator("#overview")).toHaveCSS("flex-direction", "row");
    await expect(page.locator("#vertical")).toHaveCSS("flex-direction", "column");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const toolbar = page.locator("#caller-attributes");
    const classes = ((await toolbar.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own (the layout utilities and the direction) and then
    // the caller's, on one element.
    expect(classes).toContain("inline-flex");
    expect(classes).toContain("flex-row");
    expect(classes).toContain("bg-base-200");
    expect(classes).toContain("p-1");

    // And the caller's applies, rather than merely surviving the merge.
    await expect(toolbar).toHaveCSS("padding", "4px");

    // The escape hatch out of the layout utilities, which is what a caller
    // needs to write daisyUI's own joined row in their place (ADR-0004).
    const joined = page.locator("#caller-join");
    expect(((await joined.getAttribute("class")) ?? "").split(/\s+/)).not.toContain("inline-flex");
    await expect(joined.locator("> *").first()).toHaveCSS("border-end-end-radius", "0px");
  });
});

test.describe("behaviour", () => {
  const action = (page: Page) => page.getByTestId("action");

  /**
   * One control of one toolbar, by the name it is announced under.
   *
   * Scoped to a toolbar rather than to the page, because the page renders
   * several and a control's name is only unique within the row it is in.
   */
  const control = (page: Page, toolbar: string, name: string) =>
    page.locator(toolbar).getByRole("button", { name, exact: true });

  // Every control is its own tab stop rather than the row being one, which is
  // the primitive's and is documented as a gap rather than worked around: a
  // handler or a tabindex written here would be the registry taking on
  // behaviour that belongs upstream. Asserted so that the documentation cannot
  // rot silently: if upstream gives the toolbar a roving tab stop, this is what
  // says so.
  test("every control is its own tab stop", async ({ page }) => {
    await example(page, "overview").focus();
    await page.keyboard.press("Tab");
    await expect(control(page, "#overview", "Cut")).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(control(page, "#overview", "Copy")).toBeFocused();
  });

  test("the arrow keys move between controls, and Home returns to the first", async ({ page }) => {
    await control(page, "#overview", "Cut").focus();

    await page.keyboard.press("ArrowRight");
    await expect(control(page, "#overview", "Copy")).toBeFocused();

    // Across the separator, which is passed over rather than landed on: it
    // takes no index, and nothing about it is focusable.
    await control(page, "#overview", "Paste").focus();
    await page.keyboard.press("ArrowRight");
    await expect(control(page, "#overview", "Undo")).toBeFocused();
    await expect(page.locator("#overview [role='separator']")).toBeAttached();

    await page.keyboard.press("ArrowLeft");
    await expect(control(page, "#overview", "Paste")).toBeFocused();

    await page.keyboard.press("Home");
    await expect(control(page, "#overview", "Cut")).toBeFocused();
  });

  // The other half of the direction decision: a vertical toolbar is moved
  // through with up and down rather than left and right, from the same prop
  // that turned the row.
  test("a vertical toolbar moves on the up and down keys", async ({ page }) => {
    await control(page, "#vertical", "Top").focus();

    await page.keyboard.press("ArrowDown");
    await expect(control(page, "#vertical", "Middle")).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(control(page, "#vertical", "Middle")).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(control(page, "#vertical", "Top")).toBeFocused();
  });

  test("a control reports what it is", async ({ page }) => {
    await expect(action(page)).toHaveText("nothing yet");

    await control(page, "#overview", "Copy").click();
    await expect(action(page)).toHaveText("Copy");

    await control(page, "#overview", "Undo").click();
    await expect(action(page)).toHaveText("Undo");
  });

  // The other documented gap: the arrow keys stop at a disabled control rather
  // than stepping over it, because the primitive asks for the neighbouring
  // index and a disabled index resolves to no control. What a reader can see is
  // that focus never lands on the disabled button and never gets past it.
  test("a disabled control takes no focus and stops the arrow keys", async ({ page }) => {
    const disabled = control(page, "#states", "Justify");

    await expect(disabled).toBeDisabled();

    await control(page, "#states", "Align left").focus();
    await page.keyboard.press("ArrowRight");
    await expect(disabled).not.toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(control(page, "#states", "Align right")).not.toBeFocused();
    await expect(control(page, "#states", "Align left")).toBeFocused();
  });

  test("a disabled toolbar disables every control in it", async ({ page }) => {
    const controls = page.locator("#disabled button");

    await expect(controls).toHaveCount(2);
    for (const button of await controls.all()) {
      await expect(button).toBeDisabled();
    }
  });
});
