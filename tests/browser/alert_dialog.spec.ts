import { expect, type Locator, type Page, test } from "@playwright/test";

import { axisTriggers, computedStyle, example, expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "alert_dialog" });
});

test.describe("styling", () => {
  // Nothing on this page is open on arrival, so every one of these opens what
  // it measures: a closed alert dialog renders no elements at all, not even
  // the modal, which the dialog's primitive does leave mounted.

  test("every button colour renders differently", async ({ page }) => {
    await openAppearance(page);

    await expectAxisVaries(page, "button", "background-color");
  });

  test("every title appearance renders differently", async ({ page }) => {
    expectVariesOverTriggers(await measureAppearance(page, "title", "font-size"), "title");
  });

  test("every description appearance renders differently", async ({ page }) => {
    expectVariesOverTriggers(
      await measureAppearance(page, "description", "padding-top"),
      "description",
    );
  });

  // Every value of this axis is a full-viewport modal, so the page renders one
  // trigger per value and this opens them one at a time.
  //
  // What it compares is a handful of properties together rather than one,
  // because no single property tells all six apart: daisyUI expresses a
  // position as where the modal places the box *and* as which of the box's
  // corners it rounds and how far it lets it grow. Every value differing from
  // every other in the composite is the same relational assertion the shared
  // axis helpers make, and for the same reason: daisyUI owns the numbers.
  test("every placement places the box differently", async ({ page }) => {
    const triggers = axisTriggers(page, "placement");
    const positions = await triggers.count();
    expect(positions, "the placement axis rendered nothing").toBeGreaterThan(1);

    const placements: string[] = [];

    for (let position = 0; position < positions; position++) {
      const box = await openPositioned(page, position);
      placements.push(await placement(box));

      await page.keyboard.press("Escape");
      await expect(box).toHaveCount(0);
    }

    expect(new Set(placements).size, `two positions render alike: ${placements}`).toBe(
      placements.length,
    );
  });

  test("a caller's classes reach the parts they were written on", async ({ page }) => {
    const customization = example(page, "customization");
    await customization.getByRole("button", { name: "Revoke the token" }).click();

    const box = page.locator("#caller-attributes");
    await expect(box).toBeVisible();

    const classes = ((await box.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("modal-box");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(box).toHaveCSS("border-radius", "0px");

    // The outer element is the one the parts reach and the collapsed component
    // does not: the placement axis was written there.
    await expect(modal(box)).toHaveClass(/(^|\s)modal-bottom(\s|$)/);
  });
});

test.describe("state", () => {
  // The lifted state observed twice over: the class is on the modal, and the
  // dialog is visible because of it. daisyUI hides a modal until its modifier
  // class is there, and matches nothing the primitive sets.
  test("an open dialog carries the modifier class that reveals it", async ({ page }) => {
    const box = await openConfirm(page);

    await expect(box).toBeVisible();
    await expect(modal(box)).toHaveClass(/modal-open/);
  });

  test("the action and the cancel each close the dialog and report", async ({ page }) => {
    const outcome = page.getByTestId("outcome");
    await expect(outcome).toHaveText("nothing yet");

    let box = await openConfirm(page);
    await page.getByRole("button", { name: "Keep it" }).click();
    await expect(box).toHaveCount(0);
    await expect(outcome).toHaveText("cancel");

    // Both parts close the dialog themselves before the caller's handler runs,
    // which is what this component has instead of a close part.
    box = await openConfirm(page);
    await page.getByRole("button", { name: "Delete it" }).click();
    await expect(box).toHaveCount(0);
    await expect(outcome).toHaveText("delete");
  });

  test("a controlled dialog dismisses through its caller", async ({ page }) => {
    const changes = page.getByTestId("changes");
    await expect(changes).toHaveText("0");

    // Opening is the page setting its own state, so nothing has travelled out
    // of the dialog yet.
    const box = await openPositioned(page);
    await expect(changes).toHaveText("0");

    await page.keyboard.press("Escape");
    await expect(box).toHaveCount(0);

    // Escape is the primitive's. It closed a dialog the page controls, so the
    // dismissal travelled out through the change callback and back in through
    // the open prop rather than the primitive keeping a copy of the state.
    await expect(changes).toHaveText("1");
  });
});

test.describe("behaviour", () => {
  test("a dialog is announced as an alert dialog, named and described", async ({ page }) => {
    const box = await openConfirm(page);

    // `alertdialog` rather than `dialog`, which is the whole difference a
    // screen reader hears: it says this one interrupts.
    await expect(box).toHaveRole("alertdialog");
    await expect(box).toHaveAttribute("aria-modal", "true");
    await expect(box).toHaveAccessibleName("Delete the project?");
    await expect(box).toHaveAccessibleDescription(
      "Every deployment goes with it, and the name is released. This cannot be undone.",
    );
  });

  // The difference from the dialog, and the reason this component exists: the
  // decision has to be taken rather than dismissed by a stray click.
  test("a click outside the box does not dismiss the dialog", async ({ page }) => {
    const box = await openConfirm(page);

    // The top-left corner of the viewport, which is inside the modal and
    // outside the box on every position this dialog can take.
    await page.mouse.click(5, 5);

    await expect(box).toBeVisible();
  });

  test("Escape still dismisses it, so a keyboard user is never trapped", async ({ page }) => {
    const box = await openConfirm(page);

    await page.keyboard.press("Escape");
    await expect(box).toHaveCount(0);
  });

  test("focus is trapped while a dialog is open and returned to the trigger", async ({ page }) => {
    const overview = example(page, "overview");

    // Focus is arrived at with the keyboard rather than by clicking: not every
    // engine focuses a button that is clicked, and the trigger is the element
    // focus has to come back to.
    const trigger = overview.getByRole("button", { name: "Delete the project" });
    await trigger.focus();
    await page.keyboard.press("Enter");

    const box = page.locator("#confirm");
    await expect(box).toBeVisible();

    const cancel = page.getByRole("button", { name: "Keep it" });
    const remove = page.getByRole("button", { name: "Delete it" });

    // Opening moved focus into the dialog, onto the first control in it.
    await expect(cancel).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(remove).toBeFocused();

    // Tabbing off the last control comes back to the first rather than walking
    // behind the dialog.
    await page.keyboard.press("Tab");
    await expect(cancel).toBeFocused();

    await page.keyboard.press("Escape");
    await expect(box).toHaveCount(0);
    await expect(trigger).toBeFocused();
  });
});

/** The box of the confirmation the overview opens, built as one component. */
async function openConfirm(page: Page): Promise<Locator> {
  const overview = example(page, "overview");
  await overview.getByRole("button", { name: "Delete the project" }).click();

  const box = page.locator("#confirm");
  await expect(box).toBeVisible();

  return box;
}

/** Opens the positioned dialog from one of the position triggers. */
async function openPositioned(page: Page, position = 0): Promise<Locator> {
  await axisTriggers(page, "placement").nth(position).click();

  const box = page.locator("#positioned-box");
  await expect(box).toBeVisible();

  return box;
}

/** Opens the dialog the title, description and button axes are rendered in. */
async function openAppearance(page: Page): Promise<Locator> {
  await axisTriggers(page, "title").first().click();

  const box = page.locator("#showcase");
  await expect(box).toBeVisible();

  return box;
}

/**
 * One computed style per value of an axis that cannot be rendered as a row.
 *
 * The primitive names and describes the dialog by two ids it owns and puts them
 * on whatever title and description are rendered, so a second of either would
 * repeat an id. The page renders one trigger per value instead, and this opens
 * them one at a time, the same shape the placement axis is read in.
 */
async function measureAppearance(page: Page, name: string, property: string): Promise<string[]> {
  const triggers = axisTriggers(page, name);
  const values = await triggers.count();
  expect(values, `the ${name} axis rendered nothing`).toBeGreaterThan(1);

  const measured: string[] = [];

  for (let value = 0; value < values; value++) {
    await triggers.nth(value).click();

    const box = page.locator("#showcase");
    await expect(box).toBeVisible();

    const part = name === "title" ? box.getByRole("heading") : box.locator("p");
    measured.push((await computedStyle(part, property))[0] ?? "");

    await page.keyboard.press("Escape");
    await expect(box).toHaveCount(0);
  }

  return measured;
}

/** The same assertion the shared axis helpers make, over values read one at a time. */
function expectVariesOverTriggers(values: string[], name: string): void {
  expect(new Set(values).size, `two ${name} values render alike: ${values}`).toBe(values.length);
}

/** The modal a box is in, which is the outer of the two elements. */
function modal(box: Locator): Locator {
  return box.locator("..");
}

/** Where a position put the box, and what it did to its corners and its size. */
function placement(box: Locator): Promise<string> {
  return box.evaluate((element) => {
    // Placement is the modal's (daisyUI puts the box somewhere by laying out
    // the grid around it) and the rest is the box's own.
    const modal = getComputedStyle(element.parentElement as Element);
    const style = getComputedStyle(element);

    return [
      modal.alignItems,
      modal.justifyItems,
      style.width,
      style.maxHeight,
      style.borderTopLeftRadius,
      style.borderBottomLeftRadius,
    ].join(" ");
  });
}
