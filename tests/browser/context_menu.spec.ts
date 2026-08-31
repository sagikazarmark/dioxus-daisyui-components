import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, example, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "context_menu" });
});

test.describe("styling", () => {
  // daisyUI's menu sizes the items rather than the box they are in, so the axis
  // is read off the items: one menu per value, in variant-list order, and the
  // first item of each stands for the menu it is in.
  test("every size renders the items at a size of its own", async ({ page }) => {
    expectGrows(
      (await computedStyle(firstItems(page, "size"), "font-size")).map(parseFloat),
      "size",
    );
  });

  test("the appearance axis decides whether the box is painted", async ({ page }) => {
    expectVaries(await computedStyle(boxes(page, "appearance"), "background-color"), "appearance");
  });

  test("a caller's classes join the box's own, and reach the items", async ({ page }) => {
    const box = page.locator("#caller-attributes");
    const classes = ((await box.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("bg-base-100");
    expect(classes).toContain("rounded-box");
    expect(classes).toContain("w-56");

    // The list is stretched to the box, so a width on the box is a width on the
    // menu inside it rather than a box with a shrink-wrapped list in it.
    const boxWidth = (await box.boundingBox())?.width ?? 0;
    const menuWidth = (await box.locator(".menu").boundingBox())?.width ?? 0;
    expect(boxWidth).toBeGreaterThan(0);
    expect(menuWidth).toBeCloseTo(boxWidth, 0);
  });

  test("a caller repaints the box by switching this component's utilities off", async ({
    page,
  }) => {
    const painted = page.locator("#caller-box");

    // Two utilities only tie, and a tie is settled by generated-stylesheet
    // order rather than by the class attribute (ADR-0004), so the corner
    // radius the caller squared off is only square because ours is switched
    // off.
    await expect(painted).toHaveCSS("border-radius", "0px");
  });

  test("a disabled item carries daisyUI's own muting class", async ({ page }) => {
    const items = example(page, "items");
    const disabled = items.getByRole("menuitem", { name: "Archive" });

    // Tier 2: the primitive reports the state as `data-disabled`, which daisyUI
    // matches nowhere, so the class is emitted on the wrapper daisyUI wrote it
    // for.
    await expect(disabled).toHaveAttribute("data-disabled", "true");
    await expect(disabled.locator("..")).toHaveClass(/menu-disabled/);
  });
});

test.describe("behaviour", () => {
  const surface = (page: Page) => page.locator("#overview-surface");
  const menu = (page: Page) => page.locator("#overview-menu");

  test("a right click opens the menu where the pointer was", async ({ page }) => {
    await expect(menu(page)).toHaveCount(0);

    await surface(page).click({ button: "right", position: { x: 30, y: 20 } });
    await expect(menu(page)).toBeVisible();
    await expect(surface(page)).toHaveAttribute("aria-expanded", "true");
    const near = await menu(page).boundingBox();

    await page.keyboard.press("Escape");
    await expect(menu(page)).toHaveCount(0);

    await surface(page).click({ button: "right", position: { x: 200, y: 90 } });
    await expect(menu(page)).toBeVisible();
    const far = await menu(page).boundingBox();

    // Pinned to the pointer rather than to the surface, which is the whole
    // difference from a dropdown: no daisyUI placement class could describe it.
    // The two openings are compared with each other rather than with the click
    // points, because where a fixed element lands is measured in a coordinate
    // space the engines do not all report the same way.
    expect(far!.x - near!.x).toBeGreaterThan(100);
    expect(far!.y - near!.y).toBeGreaterThan(40);
  });

  test("the menu is announced as a menu of menu items", async ({ page }) => {
    await surface(page).click({ button: "right" });

    // The list and the item wrappers this component renders are marked
    // presentational, so the menu still owns its items (ADR-0005). Scoped to
    // this menu, because every other example on the page holds one open.
    const owned = menu(page).getByRole("menuitem");
    await expect(owned).toHaveCount(3);
    await expect(owned.first()).toHaveText("Edit");
  });

  test("Escape dismisses the menu and an outside click does too", async ({ page }) => {
    await surface(page).click({ button: "right" });
    await expect(menu(page)).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(menu(page)).toHaveCount(0);

    await surface(page).click({ button: "right" });
    await expect(menu(page)).toBeVisible();

    // Somewhere outside both the menu and the surface it belongs to.
    await page.getByRole("heading", { level: 1 }).first().click({ force: true });
    await expect(menu(page)).toHaveCount(0);
  });

  test("the arrow keys move through the items and skip the disabled one", async ({ page }) => {
    await surface(page).click({ button: "right" });

    const edit = menu(page).getByRole("menuitem", { name: "Edit" });
    const duplicate = menu(page).getByRole("menuitem", { name: "Duplicate" });
    const archive = menu(page).getByRole("menuitem", { name: "Archive" });

    await page.keyboard.press("ArrowDown");
    await expect(edit).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(duplicate).toBeFocused();

    // The disabled item is passed over rather than landed on and refused, and
    // with the loop on, the pass lands back at the first.
    await page.keyboard.press("ArrowDown");
    await expect(archive).not.toBeFocused();
    await expect(edit).toBeFocused();
  });

  test("selecting an item reports its value and closes the menu", async ({ page }) => {
    const selected = page.getByTestId("selected");
    await expect(selected).toHaveText("nothing");

    await surface(page).click({ button: "right" });
    await menu(page).getByRole("menuitem", { name: "Duplicate" }).click();

    await expect(selected).toHaveText("Duplicate");
    await expect(menu(page)).toHaveCount(0);
  });

  test("a disabled item reports nothing and does not close the menu", async ({ page }) => {
    const selected = page.getByTestId("selected");

    await surface(page).click({ button: "right" });
    await menu(page).getByRole("menuitem", { name: "Archive" }).click({ force: true });

    await expect(selected).toHaveText("nothing");
    await expect(menu(page)).toBeVisible();
  });
});

/** The boxes one axis row rendered, in the order its variant list is in. */
function boxes(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > [role="menu"]`);
}

/** The first item of each of those menus, which is what a size axis is read off. */
function firstItems(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > [role="menu"] > .menu > li:first-child > *`);
}
