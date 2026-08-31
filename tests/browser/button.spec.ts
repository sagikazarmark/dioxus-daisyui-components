import { expect, type Page, test } from "@playwright/test";

import { example, expectAxisGrows, expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "button" });
});

test.describe("styling", () => {
  // Both of these iterate what the page rendered rather than a list of their
  // own: the page renders one button per value of the axis' variant list, so
  // an axis that gains a value is covered here without this file being
  // touched. What they assert of that set, and why it is relational rather
  // than a value per variant, is in `preview.ts`.

  test("every colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every size applies a size of its own", async ({ page }) => {
    await expectAxisGrows(page, "size", "height");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const button = page.locator("#caller-attributes");
    const classes = ((await button.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("btn");
    expect(classes).toContain("btn-primary");
    expect(classes).toContain("w-64");

    // And the caller's applies, rather than merely surviving the merge.
    await expect(button).toHaveCSS("width", "256px");
  });
});

test.describe("behaviour", () => {
  const activations = (page: Page) => page.getByTestId("activations");

  test("a button takes keyboard focus and activates from it", async ({ page }) => {
    const button = page.getByRole("button", { name: "Activate" });

    await expect(activations(page)).toHaveText("0");

    // Focus is arrived at by tabbing off the element before it, rather than
    // set on the button outright: `locator.press` would focus it first, which
    // makes any assertion about focus say nothing. The element before it is
    // the panel this example is rendered in, which is the first thing in the
    // focus order that is inside the example and not one of its buttons.
    await example(page, "interactive").focus();
    await page.keyboard.press("Tab");
    await expect(button).toBeFocused();

    await page.keyboard.press("Enter");
    await expect(activations(page)).toHaveText("1");

    await page.keyboard.press(" ");
    await expect(activations(page)).toHaveText("2");
  });

  test("a disabled button is announced disabled, takes no focus, and does not activate", async ({
    page,
  }) => {
    const disabled = page.getByRole("button", { name: "Disabled" });

    // The accessibility tree as a screen reader reads it: a button, named by
    // its own content, announced disabled. daisyUI draws the disabled look
    // from that state rather than from a class, so this is both halves of it.
    await expect(disabled).toMatchAriaSnapshot(`- button "Disabled" [disabled]`);

    // Tab from the button before it and focus lands on the one after it, so
    // the disabled button is passed over rather than merely refusing focus.
    await page.getByRole("button", { name: "Activate" }).focus();
    await page.keyboard.press("Tab");

    await expect(disabled).not.toBeFocused();
    await expect(page.getByRole("button", { name: "Reset" })).toBeFocused();

    // It carries the same handler as the button that does activate, so this
    // says the disabled attribute made it inert rather than that nothing was
    // wired to it.
    await disabled.click({ force: true });
    await expect(activations(page)).toHaveText("0");
  });
});
