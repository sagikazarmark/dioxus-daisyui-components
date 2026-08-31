import { expect, type Page, test } from "@playwright/test";

import { axis, computedStyle, example, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "accordion" });
});

test.describe("styling", () => {
  // The marker is the one axis whose class is on the item while its effect is
  // on the title inside it (daisyUI draws the sign in a pseudo-element of
  // `.collapse-title`) so this reads a level below the rendered set rather
  // than off it. The set is still one item per value, in variant-list order.
  test("every marker draws a mark of its own", async ({ page }) => {
    const titles = page.locator('[data-axis="marker"] > * > .collapse-title');

    expectVaries(await computedStyle(titles, "content", "::after"), "marker");
  });

  test("the set's appearance axis decides whether it spaces its items", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "set"), "row-gap"), "set");
  });

  test("the item's appearance axis decides whether it paints a surface", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "item"), "background-color"), "item");
  });

  test("the trigger's appearance axis decides how the title reads", async ({ page }) => {
    const titles = page.locator('[data-axis="trigger"] > * > .collapse-title');

    expectVaries(await computedStyle(titles, "text-align"), "trigger");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const item = page.locator("#caller-attributes");
    const classes = ((await item.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("collapse");
    expect(classes).toContain("collapse-arrow");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(item).toHaveCSS("border-radius", "0px");
  });
});

test.describe("behaviour", () => {
  const open = (page: Page) => page.getByTestId("open");

  // Tier 2, observed from the outside: what a reader can see is that a panel
  // is readable while its item is open and gone once it is not. Which class
  // does it is deliberately not asserted; that is composition, and ADR-0007
  // keeps these specs off it.
  test("a panel is revealed only while its item is open", async ({ page }) => {
    const overview = example(page, "overview");
    const first = overview.getByRole("button", { name: "What does the registry install?" });
    const second = overview.getByRole("button", { name: "Does it ship any CSS?" });
    const firstPanel = overview.getByText("One directory per component");
    const secondPanel = overview.getByText("Components emit daisyUI class names");

    await expect(first).toHaveAttribute("aria-expanded", "true");
    await expect(firstPanel).toBeVisible();
    await expect(secondPanel).toHaveCount(0);

    // One item at a time is the primitive's default: opening the second closes
    // the first, and the first panel leaves the document entirely.
    await second.click();
    await expect(second).toHaveAttribute("aria-expanded", "true");
    await expect(secondPanel).toBeVisible();
    await expect(first).toHaveAttribute("aria-expanded", "false");
    await expect(firstPanel).toHaveCount(0);
  });

  test("every item can be open at once when the set allows it", async ({ page }) => {
    const states = example(page, "states");
    const build = states.getByRole("button", { name: "Build" });
    const deploy = states.getByRole("button", { name: "Deploy" });
    const buildPanel = states.getByText("Runs on every push.");
    const deployPanel = states.getByText("Runs on a tag.");

    await expect(open(page)).toHaveText("0");

    await build.click();
    await expect(buildPanel).toBeVisible();
    await expect(open(page)).toHaveText("1");

    // The second opening leaves the first alone, which is the whole of
    // `allow_multiple_open`, and the count says both items reported it.
    await deploy.click();
    await expect(buildPanel).toBeVisible();
    await expect(deployPanel).toBeVisible();
    await expect(open(page)).toHaveText("2");

    // And closing one reports again, which is what keeps daisyUI's class in
    // step with the primitive rather than only on the way in.
    await build.click();
    await expect(buildPanel).toHaveCount(0);
    await expect(deployPanel).toBeVisible();
    await expect(open(page)).toHaveText("1");
  });

  test("the arrow keys move between triggers and Home and End reach the ends", async ({ page }) => {
    const states = example(page, "states");
    const build = states.getByRole("button", { name: "Build" });
    const deploy = states.getByRole("button", { name: "Deploy" });

    await build.focus();

    await page.keyboard.press("ArrowDown");
    await expect(deploy).toBeFocused();

    // Moving the focus is not opening anything: unlike a radio group, an
    // accordion opens on its trigger being pressed.
    await expect(deploy).toHaveAttribute("aria-expanded", "false");
    await expect(open(page)).toHaveText("0");

    await page.keyboard.press("ArrowUp");
    await expect(build).toBeFocused();

    await page.keyboard.press("End");
    await expect(deploy).toBeFocused();

    await page.keyboard.press("Home");
    await expect(build).toBeFocused();

    // Space works on a real button, which is what the trigger is: this is the
    // half of the swap from daisyUI's `div` that matters.
    await page.keyboard.press(" ");
    await expect(build).toHaveAttribute("aria-expanded", "true");
    await expect(open(page)).toHaveText("1");
  });

  test("a disabled item takes no focus and does not open", async ({ page }) => {
    const states = example(page, "states");
    const rollback = states.getByRole("button", { name: "Rollback" });

    await expect(rollback).toBeDisabled();
    await expect(rollback).toHaveAttribute("aria-expanded", "false");

    // The end of the keyboard order is the item before it, so the disabled one
    // is passed over rather than landed on and refused.
    await states.getByRole("button", { name: "Build" }).focus();
    await page.keyboard.press("End");
    await expect(rollback).not.toBeFocused();

    await rollback.click({ force: true });
    await expect(rollback).toHaveAttribute("aria-expanded", "false");
    await expect(open(page)).toHaveText("0");
  });

  test("a trigger names the panel it opens", async ({ page }) => {
    const overview = example(page, "overview");
    const first = overview.getByRole("button", { name: "What does the registry install?" });

    // The relationship a screen reader announces the panel by, which is the
    // primitive's and which the class this component emits has to leave
    // intact: the trigger points at the panel's id, and the panel is there.
    const controls = await first.getAttribute("aria-controls");
    expect(controls, "the trigger points at no panel").toBeTruthy();
    await expect(page.locator(`#${controls}`)).toBeVisible();
  });
});
