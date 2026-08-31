import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "card" });
});

test.describe("styling", () => {
  // daisyUI's card sizes the body's padding rather than the card itself.
  test("every size pads the body differently", async ({ page }) => {
    expectGrows(
      (await computedStyle(bodies(page, "size"), "padding-top")).map(parseFloat),
      "size",
    );
  });

  // Style and width together, because Tailwind's reset gives an unbordered
  // element a solid border style at zero width.
  test("every border draws an edge of its own", async ({ page }) => {
    const styles = await computedStyle(axis(page, "border"), "border-top-style");
    const widths = await computedStyle(axis(page, "border"), "border-top-width");

    expectVaries(
      styles.map((style, index) => `${style} ${widths[index]}`),
      "border",
    );
  });

  test("every layout arranges the card differently", async ({ page }) => {
    const rendered = await axis(page, "layout").evaluateAll((cards) =>
      cards.map((card) => {
        const style = getComputedStyle(card);
        return `${style.display} ${style.flexDirection}`;
      }),
    );

    expectVaries(rendered, "layout");
  });

  test("a caller's classes join every part's own", async ({ page }) => {
    await expectPartClasses(page, "#caller-card", "card", "rounded-none");
    await expectPartClasses(page, "#caller-body", "card-body", "p-2");
    await expectPartClasses(page, "#caller-title", "card-title", "italic");
    await expectPartClasses(page, "#caller-actions", "card-actions", "justify-center");
    await expect(page.locator("#caller-card")).toHaveCSS("border-radius", "0px");
  });

  test("defaults emit no modifier classes", async ({ page }) => {
    const classes = ((await page.locator("#default-card").getAttribute("class")) ?? "")
      .split(/\s+/)
      .filter(Boolean);

    expect(classes).toEqual(["card"]);
  });
});

/** The bodies inside those cards, which is where a card's size lands. */
function bodies(page: Page, name: string): Locator {
  return axis(page, name).locator(":scope > .card-body");
}

async function expectPartClasses(
  page: Page,
  selector: string,
  componentClass: string,
  callerClass: string,
): Promise<void> {
  const classes = ((await page.locator(selector).getAttribute("class")) ?? "").split(/\s+/);
  expect(classes).toContain(componentClass);
  expect(classes).toContain(callerClass);
}
