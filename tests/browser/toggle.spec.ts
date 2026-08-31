import { expect, type Page, test } from "@playwright/test";

import { computedStyle, expectAxisGrows, expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "toggle" });
});

test.describe("styling", () => {
  // Both rows are rendered pressed, which is the state this component emits a
  // class for: an unpressed row would assert the button component's axes over
  // again rather than this one's.
  test("every colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every size applies a size of its own", async ({ page }) => {
    await expectAxisGrows(page, "size", "height");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const toggle = page.locator("#caller-attributes");
    const classes = ((await toggle.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, including the class it emits for the pressed state,
    // and then the caller's, on one element.
    expect(classes).toContain("btn");
    expect(classes).toContain("btn-primary");
    expect(classes).toContain("btn-active");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over a
    // corner radius daisyUI's own class sets, which the caller wins on cascade
    // layers rather than on specificity (ADR-0004).
    await expect(toggle).toHaveCSS("border-radius", "0px");
  });
});

test.describe("behaviour", () => {
  const changes = (page: Page) => page.getByTestId("changes");
  const state = (page: Page) => page.getByTestId("state");

  // Tier 2, observed from the outside: a pressed toggle and a released one
  // report the same state to a screen reader through `aria-pressed`, and paint
  // differently only because a class was emitted. Which class does it is
  // deliberately not asserted here; that is composition, and ADR-0007 keeps
  // these specs off it.
  test("a pressed toggle is painted as well as announced", async ({ page }) => {
    const released = page.locator("#released");
    const pressed = page.locator("#pressed");

    await expect(released).toHaveAttribute("aria-pressed", "false");
    await expect(pressed).toHaveAttribute("aria-pressed", "true");

    const [releasedPaint, pressedPaint] = await computedStyle(
      page.locator("#released, #pressed"),
      "background-color",
    );
    expect(pressedPaint, "a pressed toggle paints like a released one").not.toBe(releasedPaint);
  });

  test("a click and a keyboard press both flip the state", async ({ page }) => {
    const toggle = page.getByRole("button", { name: "Bold" });

    await expect(toggle).toHaveAttribute("aria-pressed", "false");

    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");

    // The primitive focuses the button on click (Safari and Firefox on macOS
    // do not) so the keyboard is already on it by here.
    await expect(toggle).toBeFocused();

    await page.keyboard.press(" ");
    await expect(toggle).toHaveAttribute("aria-pressed", "false");

    await page.keyboard.press("Enter");
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
  });

  test("a disabled toggle is announced disabled, takes no focus, and does not move", async ({
    page,
  }) => {
    const disabled = page.locator("#disabled");

    await expect(disabled).toBeDisabled();
    await expect(disabled).toHaveAttribute("aria-pressed", "false");

    // Tab from the toggle before it and focus lands past both disabled ones, so
    // they are passed over rather than merely refusing focus.
    await page.locator("#pressed").focus();
    await page.keyboard.press("Tab");
    await expect(disabled).not.toBeFocused();
    await expect(page.locator("#disabled-pressed")).not.toBeFocused();

    await disabled.click({ force: true });
    await expect(disabled).toHaveAttribute("aria-pressed", "false");
  });

  test("a controlled toggle answers to its caller rather than to itself", async ({ page }) => {
    const toggle = page.locator("#controlled");
    const outside = page.locator("#release");

    await expect(state(page)).toHaveText("released");

    // The toggle reports through the callback and the caller writes the state
    // back, which is the lift ADR-0006 describes seen from the outside.
    await toggle.click();
    await expect(state(page)).toHaveText("pressed");
    await expect(changes(page)).toHaveText("1");
    await expect(toggle).toHaveAttribute("aria-pressed", "true");

    // And a caller who never touched the toggle can release it, which is the
    // half an uncontrolled toggle cannot do.
    await outside.click();
    await expect(state(page)).toHaveText("released");
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
  });
});
