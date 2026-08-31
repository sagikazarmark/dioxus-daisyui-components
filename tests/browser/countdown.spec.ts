import { expect, type Locator, test } from "@playwright/test";

import { axis, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "countdown" });
});

test("renders a span.countdown with direct value children", async ({ page }) => {
  const root = page.locator("#fixed-countdown");
  const value = page.locator("#fixed-value");

  await expect(root).toHaveJSProperty("tagName", "SPAN");
  await expect(root).toHaveClass(/\bcountdown\b/);
  await expect(root).toHaveClass(/\bfont-mono\b/);
  await expect(root.locator(":scope > span")).toHaveCount(1);
  expect(await value.evaluate((node) => node.parentElement?.id)).toBe("fixed-countdown");
});

test("every digits Axis value writes its custom property and changes width", async ({ page }) => {
  const values = axis(page, "digits").locator(":scope > span");

  expect(await values.count()).toBe(3);
  expect(
    await values.evaluateAll((nodes) =>
      nodes.map((node) => (node as HTMLElement).style.getPropertyValue("--digits")),
    ),
  ).toEqual(["", "2", "3"]);

  const widths = await values.evaluateAll((nodes) =>
    nodes.map((node) => node.getBoundingClientRect().width),
  );
  expect(widths).toEqual([...widths].sort((left, right) => left - right));
  expect(new Set(widths).size).toBe(3);
});

test("reactive updates keep CSS, text, and default ARIA synchronized", async ({ page }) => {
  const value = page.locator("#dynamic-value");

  await expectSynchronizedValue(value, "8");
  await expect(value).toHaveAttribute("aria-live", "polite");

  await page.getByRole("button", { name: "Advance" }).click();

  await expectSynchronizedValue(value, "9");
});

test("out-of-range values clamp before CSS, text, and ARIA are produced", async ({ page }) => {
  const below = page.locator("#below-range");

  await expectSynchronizedValue(below, "0");
  expect(await inlineProperty(below, "--digits")).toBe("");
  await expectSynchronizedValue(page.locator("#above-range"), "999");
});

test("caller styles compose while typed custom properties remain authoritative", async ({ page }) => {
  const value = page.locator("#caller-styled");

  await expect(value).toHaveCSS("color", "rgb(12, 34, 56)");
  await expect(value).toHaveCSS("letter-spacing", "2px");
  expect(await inlineProperty(value, "--caller-payload")).toBe('"left;right"');
  expect(await inlineProperty(value, "--VALUE")).toBe("888");
  expect(await inlineProperty(value, "--value")).toBe("7");
  expect(await inlineProperty(value, "--digits")).toBe("2");
  await expect(value).toHaveAttribute("aria-label", "7 seconds remaining");
  await expect(value).toHaveAttribute("aria-live", "off");
});

test("multiple values and caller separators share one root", async ({ page }) => {
  const root = page.locator("#clock-countdown");
  const values = root.locator(":scope > span");

  await expect(values).toHaveCount(3);
  expect(await values.evaluateAll((nodes) => nodes.map((node) => node.parentElement?.id))).toEqual([
    "clock-countdown",
    "clock-countdown",
    "clock-countdown",
  ]);
  expect(await values.evaluateAll((nodes) => nodes.map((node) => node.textContent))).toEqual([
    "1",
    "4",
    "9",
  ]);
  await expect(root).toContainText(" : ");
});

async function expectSynchronizedValue(
  value: Locator,
  expected: string,
): Promise<void> {
  await expect(value).toHaveText(expected);
  await expect(value).toHaveAttribute("aria-label", expected);
  expect(await inlineProperty(value, "--value")).toBe(expected);
}

async function inlineProperty(
  value: Locator,
  property: string,
): Promise<string> {
  return value.evaluate((node, name) => (node as HTMLElement).style.getPropertyValue(name), property);
}
