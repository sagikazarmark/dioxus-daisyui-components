import { expect, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "steps" });
});

test.describe("structure", () => {
  test("steps are adjacent list items and StepIcon is a direct child", async ({ page }) => {
    const steps = page.locator("#default-steps");

    await expect(steps).toHaveJSProperty("tagName", "UL");
    await expect(steps.locator(":scope > li.step")).toHaveCount(3);
    await expect(steps.locator(":scope > li.step + li.step")).toHaveCount(2);
    await expect(steps.locator(":scope > :not(li.step)")).toHaveCount(0);

    const icon = page.locator("#custom-icon-step > #custom-icon.step-icon");
    await expect(icon).toHaveCount(1);
    await expect(icon).toHaveJSProperty("tagName", "SPAN");
  });
});

test.describe("styling", () => {
  test("every direction lays out its steps differently", async ({ page }) => {
    expect(await computedStyle(axis(page, "direction"), "grid-auto-flow")).toEqual([
      "column",
      "row",
    ]);
  });

  test("every colour paints both a node and its connector", async ({ page }) => {
    const variants = axis(page, "color");
    await expect(variants).toHaveCount(9);

    expectVaries(
      await computedStyle(variants.locator(":scope > .step:first-child"), "background-color", "::after"),
      "step color",
    );
    expectVaries(
      await computedStyle(variants.locator(":scope > .step + .step"), "background-color", "::before"),
      "connector color",
    );
  });

  test("the colour default emits no modifier class", async ({ page }) => {
    const defaultStep = axis(page, "color").first().locator(":scope > .step:first-child");
    expect(await defaultStep.evaluate((step) => [...step.classList])).toEqual(["step"]);
  });

  test("generated counters and caller content reach the marker", async ({ page }) => {
    const generated = page.locator("#default-steps > .step");
    const content = await computedStyle(generated, "content", "::after");
    const increments = await computedStyle(generated, "counter-increment", "::after");

    expect(content).toHaveLength(3);
    expect(content.every((value) => value.includes("counter(step)"))).toBe(true);
    expect(increments.every((value) => value.includes("step"))).toBe(true);

    const custom = page.locator("#custom-content-step");
    await expect(custom).toHaveAttribute("data-content", "!");
    const customContent = (await computedStyle(custom, "content", "::after"))[0];
    expect(customContent.includes("!") || customContent.includes("attr(data-content)")).toBe(true);

    const icon = page.locator("#custom-icon");
    expect(await computedStyle(page.locator("#custom-icon-step"), "content", "::after")).toEqual([
      "none",
    ]);
    await expect(icon).toHaveCSS("display", "grid");
    await expect(icon).not.toHaveCSS("background-color", "rgba(0, 0, 0, 0)");
  });

  test("a caller's classes and attributes survive on every part", async ({ page }) => {
    const steps = page.locator("#caller-steps");
    await expect(steps).toHaveClass(/\bsteps\b/);
    await expect(steps).toHaveClass(/\bsteps-vertical\b/);
    await expect(steps).toHaveClass(/\bgap-4\b/);
    await expect(steps).toHaveAttribute("data-owner", "caller");
    await expect(steps).toHaveAttribute("aria-label", "Caller-owned process");
    await expect(steps).toHaveCSS("gap", "16px");

    const step = page.locator("#caller-step");
    await expect(step).toHaveClass(/\bstep\b/);
    await expect(step).toHaveClass(/\bstep-warning\b/);
    await expect(step).toHaveClass(/\bfont-bold\b/);
    await expect(step).toHaveAttribute("data-content", "!");
    await expect(step).toHaveAttribute("data-owner", "caller");
    await expect(step).toHaveAttribute("aria-current", "step");
    await expect(step).toHaveAttribute("title", "Waiting for approval");
    await expect(step).toHaveCSS("font-weight", "700");

    const icon = page.locator("#caller-icon");
    await expect(icon).toHaveClass(/\bstep-icon\b/);
    await expect(icon).toHaveClass(/\bsize-10\b/);
    await expect(icon).toHaveAttribute("data-owner", "caller");
    await expect(icon).toHaveAttribute("aria-hidden", "true");
    await expect(icon).toHaveCSS("width", "40px");
    await expect(icon).toHaveCSS("height", "40px");
  });
});

test.describe("semantics", () => {
  test("the caller identifies the current step and describes completion in text", async ({ page }) => {
    await expect(page.locator("#completed-step")).not.toHaveAttribute("aria-current");
    await expect(page.locator("#completed-step")).toContainText("Completed: Build");
    await expect(page.locator("#current-step")).toHaveAttribute("aria-current", "step");
    await expect(page.locator("#current-step")).toContainText("Current step: Review");
  });
});
