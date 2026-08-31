import { expect, test } from "@playwright/test";

import { axis, computedStyle, example, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "collapsible" });
});

test.describe("styling", () => {
  // The marker's class is on the root while its effect is on the title inside
  // it (daisyUI draws the sign in a pseudo-element of `.collapse-title`) so
  // this reads a level below the rendered set rather than off it. The set is
  // still one disclosure per value, in variant-list order.
  test("every marker draws a mark of its own", async ({ page }) => {
    const titles = page.locator('[data-axis="marker"] > * > .collapse-title');

    expectVaries(await computedStyle(titles, "content", "::after"), "marker");
  });

  test("the root's appearance axis decides whether it paints a surface", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "root"), "background-color"), "root");
  });

  test("the trigger's appearance axis decides how the title reads", async ({ page }) => {
    const titles = page.locator('[data-axis="trigger"] > * > .collapse-title');

    expectVaries(await computedStyle(titles, "text-align"), "trigger");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const root = page.locator("#caller-attributes");
    const classes = ((await root.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("collapse");
    expect(classes).toContain("collapse-arrow");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over a
    // corner radius daisyUI's own class sets, which the caller wins on cascade
    // layers rather than on specificity (ADR-0004).
    await expect(root).toHaveCSS("border-radius", "0px");
  });
});

test.describe("behaviour", () => {
  // Tier 2, observed from the outside: what a reader can see is that the panel
  // is readable while the disclosure is open and not while it is closed. Which
  // class does it is deliberately not asserted; that is composition, and
  // ADR-0007 keeps these specs off it.
  test("the panel is revealed only while the disclosure is open", async ({ page }) => {
    const overview = example(page, "overview");
    const trigger = overview.getByRole("button", { name: "What ships when I install" });
    const panel = overview.getByText("One directory: the component's module");

    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(panel).toBeVisible();

    await trigger.click();
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await expect(panel).toHaveCount(0);

    await trigger.click();
    await expect(panel).toBeVisible();
  });

  test("the trigger is a button, so Enter and Space work on it", async ({ page }) => {
    const overview = example(page, "overview");
    const trigger = overview.getByRole("button", { name: "Does the panel exist" });

    await trigger.focus();
    await expect(trigger).toHaveAttribute("aria-expanded", "false");

    await page.keyboard.press("Enter");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");

    await page.keyboard.press(" ");
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
  });

  test("a trigger names the panel it opens", async ({ page }) => {
    const overview = example(page, "overview");
    const trigger = overview.getByRole("button", { name: "What ships when I install" });

    // The relationship a screen reader announces the panel by, which is the
    // primitive's and which the class this component emits has to leave intact:
    // the trigger points at the panel's id, and the panel is there.
    const controls = await trigger.getAttribute("aria-controls");
    expect(controls, "the trigger points at no panel").toBeTruthy();
    await expect(page.locator(`#${controls}`)).toBeVisible();
  });

  test("a controlled disclosure answers to its caller rather than to itself", async ({ page }) => {
    const changes = page.getByTestId("changes");
    const state = page.getByTestId("state");
    const trigger = page.locator("#controlled-trigger");
    const outside = page.locator("#toggle");

    await expect(state).toHaveText("closed");

    // The title reports through the callback and the caller writes the state
    // back, which is the lift ADR-0006 describes seen from the outside.
    await trigger.click();
    await expect(state).toHaveText("open");
    await expect(changes).toHaveText("1");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");

    // And a caller who never touched the title can close it, which is the half
    // an uncontrolled disclosure cannot do.
    await outside.click();
    await expect(state).toHaveText("closed");
    await expect(changes).toHaveText("2");
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
  });

  test("a disabled disclosure takes no focus and does not open", async ({ page }) => {
    const trigger = page.locator("#disabled").getByRole("button");

    await expect(trigger).toBeDisabled();
    await expect(trigger).toHaveAttribute("aria-expanded", "false");

    await trigger.click({ force: true });
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
  });

  test("a kept-mounted panel stays in the document while it is closed", async ({ page }) => {
    const kept = page.locator("#kept");
    const trigger = kept.getByRole("button");
    const content = page.locator("#kept-content");

    // In the document while the disclosure is shut, which is the whole of
    // `keep_mounted`: everything else on this page mounts its panel only while
    // it is open. Whether a shut panel is *readable* is daisyUI's business and
    // not the same in every engine (see the component's documentation) so
    // what is asserted here is that the element is there and that opening the
    // disclosure shows it.
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await expect(content).toBeAttached();

    await trigger.click();
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(content).toBeVisible();
  });
});
