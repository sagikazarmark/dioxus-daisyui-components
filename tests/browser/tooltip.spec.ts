import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, example, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "tooltip" });
});

test.describe("styling", () => {
  // Every axis on this component is a class on the outer element whose effect
  // lands on the bubble inside it, so each of these reads a level below the
  // rendered set. The set is still one tooltip per value, in variant-list
  // order, and every one of them is held open; a closed tooltip renders no
  // bubble to measure.
  test("every colour fills the bubble differently", async ({ page }) => {
    expectVaries(await computedStyle(bubbles(page, "color"), "background-color"), "color");
  });

  // Placement is read as the whole of what daisyUI sets, rather than as one
  // property: `top` alone cannot tell a bubble on the left from one on the
  // right, since both are pinned to the middle of the trigger.
  test("every placement puts the bubble somewhere of its own", async ({ page }) => {
    expectVaries(await box(page, "side"), "side");
  });

  test("every alignment puts the bubble somewhere of its own", async ({ page }) => {
    expectVaries(await box(page, "align"), "align");
  });

  // The tail is the reason every placement emits a class, including the
  // default one: daisyUI's base rule positions the bubble and leaves the tail
  // unplaced. It is drawn in a pseudo-element of the outer element.
  test("every placement puts the tail somewhere of its own", async ({ page }) => {
    const tooltips = page.locator('[data-axis="side"] > *');

    const insets = await Promise.all(
      ["top", "left", "transform"].map((property) =>
        computedStyle(tooltips, property, "::after"),
      ),
    );

    expectVaries(zip(insets), "placement tail");
  });

  test("a caller's classes join the bubble's own", async ({ page }) => {
    const bubble = page.locator("#caller-attributes");
    const classes = ((await bubble.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("tooltip-content");
    expect(classes).toContain("w-96");

    // And the caller's applies rather than merely surviving the merge. The
    // alignment is the half cascade layers decide: daisyUI centres the text
    // from a class of the same weight, and the caller's wins on layers rather
    // than on specificity (ADR-0004).
    await expect(bubble).toHaveCSS("text-align", "start");

    // The width is the half they do not: daisyUI caps the bubble with
    // `max-width`, which no `width` utility out-ranks, so the caller lifts the
    // cap as well. Asserted so that the example cannot quietly stop showing it.
    const width = parseFloat((await computedStyle(bubble, "width"))[0]);
    expect(width, "the caller's width lost to daisyUI's cap").toBeGreaterThan(320);
  });
});

test.describe("behaviour", () => {
  const changes = (page: Page) => page.getByTestId("changes");

  test("a tooltip opens on hover and closes when the pointer leaves", async ({ page }) => {
    const overview = example(page, "overview");
    const trigger = overview.getByRole("button", { name: "Deploy" });
    const bubble = overview.getByRole("tooltip");

    await expect(bubble).toHaveCount(0);

    await trigger.hover();
    await expect(bubble).toBeVisible();

    // Away from the trigger rather than to a fixed point, so that the pointer
    // is somewhere the tooltip is not on any viewport.
    await page.mouse.move(0, 0);
    await expect(bubble).toHaveCount(0);
  });

  // The half of Tier 2 that a hover cannot prove: daisyUI reveals a bubble on
  // `:hover` by itself, so a component emitting no class at all would pass the
  // test above. Keyboard focus is the same story through `:has(:focus-visible)`
  // and it is asserted here, with the controlled test below covering the case
  // where only the emitted class can carry it.
  test("a tooltip opens on keyboard focus and dismisses on Escape", async ({ page }) => {
    const overview = example(page, "overview");
    const trigger = overview.getByRole("button", { name: "Deploy" });
    const bubble = overview.getByRole("tooltip");

    await overview.focus();
    await page.keyboard.press("Tab");
    await expect(trigger).toBeFocused();
    await expect(bubble).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(bubble).toHaveCount(0);

    // And the trigger keeps focus, so Escape dismissed the tooltip rather than
    // moving the reader somewhere else.
    await expect(trigger).toBeFocused();
  });

  test("a tooltip its caller holds open is visible with nothing hovered or focused", async ({
    page,
  }) => {
    const states = example(page, "states");
    const bubble = states.getByRole("tooltip", { name: "Held open by the page" });

    await expect(bubble).toHaveCount(0);

    // Opened from a control that is not the trigger and does not focus it,
    // which is the case neither of daisyUI's own selectors can reach: the
    // bubble is visible here only because the class was emitted.
    await states.getByRole("button", { name: "Toggle it" }).click();
    await expect(bubble).toBeVisible();
    await expect(bubble).not.toHaveCSS("opacity", "0");

    await states.getByRole("button", { name: "Toggle it" }).click();
    await expect(bubble).toHaveCount(0);
  });

  test("a disabled tooltip does not open", async ({ page }) => {
    const states = example(page, "states");
    const trigger = states.getByText("Disabled", { exact: true });

    await trigger.hover();
    await expect(states.getByRole("tooltip", { name: "Never shown" })).toHaveCount(0);
    await expect(changes(page)).toHaveText("0");
  });

  test("the trigger is described by the bubble it opens", async ({ page }) => {
    const overview = example(page, "overview");
    const trigger = overview.getByRole("button", { name: "Deploy" });

    await trigger.hover();

    // The relationship a screen reader announces the tooltip by, which is the
    // primitive's: the trigger points at the bubble's id, and the bubble is
    // the element carrying it.
    const describedby = await trigger.getAttribute("aria-describedby");
    expect(describedby, "the trigger is described by nothing").toBeTruthy();
    await expect(page.locator(`#${describedby}`)).toHaveRole("tooltip");
  });
});

/** The bubbles of one axis row, in the order its variant list is in. */
function bubbles(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > .tooltip-content`);
}

/** Where each bubble of one axis row was put, as everything daisyUI set. */
async function box(page: Page, name: string): Promise<string[]> {
  const read = await Promise.all(
    ["top", "left", "right", "transform"].map((property) =>
      computedStyle(bubbles(page, name), property),
    ),
  );

  return zip(read);
}

/** One string per element, out of the several properties read off each. */
function zip(properties: string[][]): string[] {
  return (properties[0] ?? []).map((_, index) =>
    properties.map((values) => values[index]).join(" | "),
  );
}
