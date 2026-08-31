import { expect, type Locator, test } from "@playwright/test";

import { openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "radial_progress" });
});

test.describe("styling", () => {
  test("daisyUI draws the default ring and head geometry", async ({ page }) => {
    const geometry = await radialGeometry(page.locator("#default-geometry"));

    // Engines may serialize the legacy `inline-grid` computed value as `grid`.
    expect(geometry.display).toContain("grid");
    expect(geometry.width).toBeCloseTo(80, 1);
    expect(geometry.height).toBeCloseTo(80, 1);
    expect(geometry.ringPosition).toBe("absolute");
    expect(geometry.ringInset).toEqual([0, 0, 0, 0]);
    expect(geometry.ringBackground).toContain("conic-gradient");
    expect(geometry.ringMask).not.toBe("none");
    expect(geometry.headPosition).toBe("absolute");
    expect(geometry.headWidth).toBeCloseTo(8, 1);
    expect(geometry.headHeight).toBeCloseTo(8, 1);
    expect(geometry.headTransform).not.toBe("none");
    expect(geometry.headColor).toBe(geometry.color);
  });

  test("currentColor paints the ring and its head", async ({ page }) => {
    const paints = await page.locator("#color-samples > *").evaluateAll((nodes) =>
      nodes.map((node) => ({
        color: getComputedStyle(node).color,
        ringBackground: getComputedStyle(node, "::before").backgroundImage,
        ringColor: getComputedStyle(node, "::before").color,
        head: getComputedStyle(node, "::after").backgroundColor,
      })),
    );

    expect(new Set(paints.map(({ color }) => color)).size).toBe(paints.length);
    for (const { color, head, ringBackground, ringColor } of paints) {
      expect(ringBackground).toContain("conic-gradient");
      expect(ringColor).toBe(color);
      expect(head).toBe(color);
    }
  });

  test("caller size, thickness, classes, styles, and attributes compose", async ({ page }) => {
    const customSize = await radialGeometry(page.locator("#custom-size"));
    const customThickness = await radialGeometry(page.locator("#custom-thickness"));
    const caller = page.locator("#caller-attributes");
    const callerGeometry = await radialGeometry(caller);
    const classes = ((await caller.getAttribute("class")) ?? "").split(/\s+/);

    expect(customSize.width).toBeCloseTo(112, 1);
    expect(customSize.height).toBeCloseTo(112, 1);
    expect(customThickness.headWidth).toBeCloseTo(16, 1);
    expect(customThickness.headHeight).toBeCloseTo(16, 1);

    expect(classes).toContain("radial-progress");
    expect(classes).toContain("text-secondary");
    await expect(caller).toHaveAttribute("data-owner", "caller");
    await expect(caller).toContainText("70% caller");
    await expect(caller.locator(":scope > span")).toHaveText("70% caller");
    await expect(caller).toHaveCSS("outline-offset", "3px");
    expect(callerGeometry.width).toBeCloseTo(96, 1);
    expect(callerGeometry.headWidth).toBeCloseTo(8, 1);

    // The Primitive writes these before spreading attributes. The wrapper
    // removes conflicting caller values so its typed props remain authoritative.
    await expect(caller).toHaveAttribute("role", "progressbar");
    await expect(caller).toHaveAttribute("aria-valuemin", "0");
    await expect(caller).toHaveAttribute("aria-valuemax", "100");
    await expect(caller).toHaveAttribute("aria-valuenow", "70");
    await expect(caller).toHaveAttribute("data-state", "loading");
    await expect(caller).toHaveAttribute("data-max", "100");
    await expect(caller).toHaveAttribute("data-value", "70");

    const properties = await customProperties(caller);
    expect(properties.size).toBe("6rem");
    expect(properties.thickness).toBe("0.5rem");
    expect(properties.value).toBe("70");
    expect(properties.progressValue).toBe("70%");
  });
});

test.describe("behaviour", () => {
  test("the Primitive owns a fixed range and normalizes out-of-range values", async ({ page }) => {
    const expected = new Map([
      ["#value-zero", "0"],
      ["#value-part", "40"],
      ["#value-full", "100"],
      ["#value-below", "0"],
      ["#value-above", "100"],
      ["#value-nan", "0"],
      ["#value-negative-infinity", "0"],
      ["#value-positive-infinity", "100"],
    ]);

    for (const [selector, value] of expected) {
      const radial = page.locator(selector);
      await expect(radial).toHaveRole("progressbar");
      await expect(radial).toHaveAttribute("aria-valuemin", "0");
      await expect(radial).toHaveAttribute("aria-valuemax", "100");
      await expect(radial).toHaveAttribute("aria-valuenow", value);
      await expect(radial).toHaveAttribute("data-max", "100");
      await expect(radial).toHaveAttribute("data-value", value);
      await expect(radial).toHaveAttribute("data-state", "loading");

      const properties = await customProperties(radial);
      expect(properties.value).toBe(value);
      expect(properties.progressValue).toBe(`${value}%`);
    }
  });

  test("reactive updates synchronize the ring, head, ARIA, and label", async ({ page }) => {
    const radial = page.locator("#value-driven");
    const initialGeometry = await radialGeometry(radial);

    await expectSynchronized(radial, "20");
    await expect(page.getByTestId("radial-value")).toHaveText("20%");

    await page.getByRole("button", { name: "Advance radial progress" }).click();

    await expectSynchronized(radial, "65");
    await expect(page.getByTestId("radial-value")).toHaveText("65%");
    await expect
      .poll(async () => (await radialGeometry(radial)).ringBackground)
      .not.toBe(initialGeometry.ringBackground);
    await expect
      .poll(async () => (await radialGeometry(radial)).headTransform)
      .not.toBe(initialGeometry.headTransform);
  });
});

async function expectSynchronized(radial: Locator, value: string): Promise<void> {
  await expect(radial).toHaveAttribute("aria-valuemin", "0");
  await expect(radial).toHaveAttribute("aria-valuemax", "100");
  await expect(radial).toHaveAttribute("aria-valuenow", value);
  await expect(radial).toHaveAttribute("data-value", value);
  await expect.poll(async () => (await customProperties(radial)).value).toBe(value);
  await expect.poll(async () => (await customProperties(radial)).progressValue).toBe(`${value}%`);
}

function customProperties(radial: Locator): Promise<{
  progressValue: string;
  size: string;
  thickness: string;
  value: string;
}> {
  return radial.evaluate((node) => {
    const style = getComputedStyle(node);
    return {
      progressValue: style.getPropertyValue("--progress-value").trim(),
      size: style.getPropertyValue("--size").trim(),
      thickness: style.getPropertyValue("--thickness").trim(),
      value: style.getPropertyValue("--value").trim(),
    };
  });
}

function radialGeometry(radial: Locator): Promise<{
  color: string;
  display: string;
  headColor: string;
  headHeight: number;
  headPosition: string;
  headTransform: string;
  headWidth: number;
  height: number;
  ringBackground: string;
  ringInset: number[];
  ringMask: string;
  ringPosition: string;
  width: number;
}> {
  return radial.evaluate((node) => {
    const style = getComputedStyle(node);
    const ring = getComputedStyle(node, "::before");
    const head = getComputedStyle(node, "::after");
    const rect = node.getBoundingClientRect();

    return {
      color: style.color,
      display: style.display,
      headColor: head.backgroundColor,
      headHeight: parseFloat(head.height),
      headPosition: head.position,
      headTransform: head.transform,
      headWidth: parseFloat(head.width),
      height: rect.height,
      ringBackground: ring.backgroundImage,
      ringInset: [ring.top, ring.right, ring.bottom, ring.left].map(parseFloat),
      ringMask: ring.maskImage || ring.webkitMaskImage,
      ringPosition: ring.position,
      width: rect.width,
    };
  });
}
