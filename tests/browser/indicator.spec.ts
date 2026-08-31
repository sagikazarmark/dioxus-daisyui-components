import { expect, type Locator, type Page, test } from "@playwright/test";

import { openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "indicator" });
});

test.describe("styling", () => {
  test("all nine axis combinations occupy distinct positions", async ({
    page,
  }) => {
    const indicator = page.locator("#position-grid");
    const items = indicator.locator(":scope > .indicator-item");
    await expect(items).toHaveCount(9);

    const inline = ["Start", "Center", "End"] as const;
    const block = ["Top", "Middle", "Bottom"] as const;
    const positions = new Set<string>();

    for (const [blockIndex, blockValue] of block.entries()) {
      for (const [inlineIndex, inlineValue] of inline.entries()) {
        const item = indicator.locator(
          `:scope > [data-inline="${inlineValue}"][data-block="${blockValue}"]`,
        );
        await expect(item).toHaveCount(1);

        const classes = splitClasses(await item.getAttribute("class"));
        expect(classes).toEqual(
          expect.arrayContaining([
            "indicator-item",
            `indicator-${inlineValue.toLowerCase()}`,
            `indicator-${blockValue.toLowerCase()}`,
          ]),
        );

        const [x, y] = await relativeCenter(item, indicator);
        expect(
          Math.abs(x - inlineIndex / 2),
          `${inlineValue} was at ${x}`,
        ).toBeLessThan(0.02);
        expect(
          Math.abs(y - blockIndex / 2),
          `${blockValue} was at ${y}`,
        ).toBeLessThan(0.02);
        positions.add(`${x.toFixed(2)}:${y.toFixed(2)}`);
      }
    }

    expect(positions.size).toBe(9);
  });

  test("logical start and end mirror under RTL", async ({ page }) => {
    const ltr = page.locator("#ltr-indicator");
    const rtl = page.locator("#rtl-indicator");

    await expect(ltr).toHaveAttribute("dir", "ltr");
    await expect(rtl).toHaveAttribute("dir", "rtl");

    const ltrEnd = await relativeHorizontalCenter(
      page.locator("#ltr-end"),
      ltr,
    );
    const ltrStart = await relativeHorizontalCenter(
      page.locator("#ltr-start"),
      ltr,
    );
    const rtlEnd = await relativeHorizontalCenter(
      page.locator("#rtl-end"),
      rtl,
    );
    const rtlStart = await relativeHorizontalCenter(
      page.locator("#rtl-start"),
      rtl,
    );

    expect(Math.abs(ltrEnd - 1)).toBeLessThan(0.02);
    expect(Math.abs(ltrStart)).toBeLessThan(0.02);
    expect(Math.abs(rtlEnd)).toBeLessThan(0.02);
    expect(Math.abs(rtlStart - 1)).toBeLessThan(0.02);
  });

  test("defaults and caller attributes merge on both parts", async ({
    page,
  }) => {
    await expect(page.locator("#default-indicator")).toHaveClass("indicator");

    const defaultItemClasses = splitClasses(
      await page.locator("#default-item").getAttribute("class"),
    ).filter((className) => className.startsWith("indicator"));
    expect(defaultItemClasses).toEqual([
      "indicator-item",
      "indicator-end",
      "indicator-top",
    ]);

    const indicator = page.locator("#caller-indicator");
    expect(splitClasses(await indicator.getAttribute("class"))).toEqual(
      expect.arrayContaining([
        "indicator",
        "rounded-none",
        "outline-2",
        "outline-primary",
      ]),
    );
    await expect(indicator).toHaveAttribute("data-owner", "caller-root");
    await expect(indicator).toHaveCSS("outline-style", "solid");

    const item = page.locator("#caller-item");
    expect(splitClasses(await item.getAttribute("class"))).toEqual(
      expect.arrayContaining([
        "indicator-item",
        "indicator-start",
        "indicator-bottom",
        "badge",
        "badge-accent",
        "italic",
      ]),
    );
    await expect(item).toHaveAttribute("data-owner", "caller-item");
    await expect(item).toHaveAttribute("aria-label", "Caller item");
    await expect(item).toHaveCSS("font-style", "italic");
    await expect(indicator.locator(":scope > .indicator-item")).toHaveCount(2);
  });
});

async function relativeCenter(
  item: Locator,
  container: Locator,
): Promise<[number, number]> {
  const itemBox = await item.boundingBox();
  const containerBox = await container.boundingBox();
  expect(itemBox, "the indicator item rendered no box").not.toBeNull();
  expect(containerBox, "the indicator rendered no box").not.toBeNull();

  return [
    (itemBox!.x + itemBox!.width / 2 - containerBox!.x) / containerBox!.width,
    (itemBox!.y + itemBox!.height / 2 - containerBox!.y) / containerBox!.height,
  ];
}

async function relativeHorizontalCenter(
  item: Locator,
  container: Locator,
): Promise<number> {
  return (await relativeCenter(item, container))[0];
}

function splitClasses(value: string | null): string[] {
  return (value ?? "").split(/\s+/).filter(Boolean);
}
