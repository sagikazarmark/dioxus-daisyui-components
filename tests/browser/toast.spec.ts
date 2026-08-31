import { expect, type Locator, type Page, test } from "@playwright/test";

import { axisTriggers, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "toast" });
});

test.describe("styling", () => {
  // The kinds example sends one permanent toast per colour when the page
  // arrives, so the rendered set is on screen without a click. They are read
  // through the list the primitive puts between the region and each toast,
  // which is the shape ADR-0014 is about.
  test("every kind is drawn in a colour of its own", async ({ page }) => {
    const toasts = toastsIn(page, "types");
    await expect(toasts).toHaveCount(4);

    expectVaries(await computedStyle(toasts, "background-color"), "color");
  });

  // Both position axes, one value at a time: every value moves the region, so
  // what is compared is where the region ended up. A row of nine would be nine
  // stacks overlapping in the corners of the viewport.
  test("every inline position pins the region to a different edge", async ({ page }) => {
    await expectPositionsVary(page, "inline");
  });

  test("every block position pins the region to a different edge", async ({ page }) => {
    await expectPositionsVary(page, "block");
  });

  // The layout daisyUI would have applied to its own children, put back with
  // utilities that reach through the list, and the same utilities switched
  // off, with the caller's own in their place (ADR-0014, ADR-0004).
  test("the layout axis lays the list out, and switches off", async ({ page }) => {
    await show(page, "The component's own");
    await show(page, "Parts of your own");

    const emitted = listIn(page, "default-parts");
    const caller = listIn(page, "custom-parts");

    await expect(emitted).toHaveCSS("display", "flex");
    await expect(emitted).toHaveCSS("row-gap", "8px");

    // Switched off, the list is laid out by the caller instead: a different
    // display and a different gap, from classes on the same element.
    await expect(caller).toHaveCSS("display", "grid");
    await expect(caller).toHaveCSS("row-gap", "16px");
  });

  // The parts' own axes, rendered across the toasts of the caller-styled
  // provider: a title has one appearance at a time, so the values are spread
  // over the stack rather than over one toast.
  test("the title and description axes render every value", async ({ page }) => {
    await show(page, "Parts of your own");

    const toasts = toastsIn(page, "custom-parts");
    await expect(toasts).toHaveCount(2);

    expectVaries(await computedStyle(toasts.locator("> * > *:first-child"), "font-weight"), "title");
    expectVaries(
      await computedStyle(toasts.locator("> * > *:last-child"), "font-size"),
      "description",
    );
  });
});

test.describe("behaviour", () => {
  test("a dispatched toast is announced, and carries the colour its kind implies", async ({
    page,
  }) => {
    await show(page, "Success");

    const toast = page.getByRole("alertdialog", { name: "Deployed" });
    await expect(toast).toBeVisible();

    // Announced as a live region holding both halves of what was dispatched,
    // which is the primitive's doing, and painted by the class this component
    // emits from the kind, which daisyUI matches no attribute for.
    await expect(toast).toContainText("Deployed");
    await expect(toast).toContainText("Live in eu-west-1");
    await expect(toast.getByRole("alert")).toBeAttached();

    const classes = ((await toast.getAttribute("class")) ?? "").split(/\s+/);
    expect(classes).toContain("alert");
    expect(classes).toContain("alert-success");
    expect(await toast.getAttribute("data-type")).toBe("success");
  });

  test("the close button dismisses the toast it is in", async ({ page }) => {
    await show(page, "Warning, permanent");

    const toast = page.getByRole("alertdialog", { name: "Certificate expires tomorrow" });
    await expect(toast).toBeVisible();

    await toast.getByRole("button", { name: "close" }).click();
    await expect(toast).toHaveCount(0);
  });

  test("a timed toast goes on its own and a permanent one stays", async ({ page }) => {
    // Motion is quiesced by the harness, but a duration is a timer rather than
    // an animation, so this is the one spec that waits for real time to pass.
    await show(page, "Success");
    await show(page, "Warning, permanent");

    const timed = page.getByRole("alertdialog", { name: "Deployed" });
    const permanent = page.getByRole("alertdialog", { name: "Certificate expires tomorrow" });

    await expect(timed).toBeVisible();
    await expect(permanent).toBeVisible();

    // The default duration is five seconds, so this outlives the timed toast
    // and says nothing about the permanent one having a longer one: it has
    // none at all.
    await expect(timed).toHaveCount(0, { timeout: 15_000 });
    await expect(permanent).toBeVisible();
  });

  test("F6 moves focus to a toast region", async ({ page }) => {
    // The keyboard shortcut for reaching notifications, which the primitive
    // registers globally: the region takes focus itself, and tabbing on from
    // it walks the toasts.
    //
    // Which region is not asserted, deliberately: this page mounts a provider
    // per example and every one of them registers the shortcut, so the last
    // one to handle it wins. An app has one provider and no such question.
    await page.locator("body").click({ position: { x: 5, y: 5 } });
    await page.keyboard.press("F6");

    const focused = await page.evaluate(() => {
      const element = document.activeElement;
      return {
        role: element?.getAttribute("role") ?? "",
        classes: (element?.getAttribute("class") ?? "").split(/\s+/),
      };
    });

    expect(focused.role, "F6 focused something that is not a region").toBe("region");
    expect(focused.classes, "F6 focused a region that is not a toast region").toContain("toast");
  });
});

/** The toasts one region is showing, through the list between them. */
function toastsIn(page: Page, region: string): Locator {
  return page.locator(`[data-region="${region}"] li > *`);
}

/** The list one region keeps its toasts in, which is the element ADR-0014 is about. */
function listIn(page: Page, region: string): Locator {
  return page.locator(`[data-region="${region}"] > ol`);
}

/** Sends one of the page's toasts, by the name of the button that sends it. */
async function show(page: Page, trigger: string): Promise<void> {
  await page.getByRole("button", { name: trigger, exact: true }).click();
}

/**
 * Asserts that every value of a position axis puts the region somewhere else.
 *
 * The triggers move one provider rather than standing up one each, so this
 * clicks through them and reads where the region landed: the same shape the
 * dialog's position axis is asserted in, and for the same reason.
 */
async function expectPositionsVary(page: Page, name: string): Promise<void> {
  const triggers = axisTriggers(page, name);
  const values = await triggers.count();
  expect(values, `the ${name} axis rendered nothing`).toBeGreaterThan(1);

  const region = page.locator("#positioned");
  const boxes: string[] = [];

  for (let value = 0; value < values; value++) {
    await triggers.nth(value).click();
    await expect(region).toBeVisible();

    const box = await region.boundingBox();
    expect(box, "the region rendered no box at all").not.toBeNull();
    boxes.push(`${Math.round(box!.x)}x${Math.round(box!.y)}`);
  }

  expectVaries(boxes, name);
}
