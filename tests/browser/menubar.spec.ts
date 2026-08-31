import { expect, type Locator, type Page, test } from "@playwright/test";

import {
  axisTriggers,
  computedStyle,
  example,
  expectAxisVaries,
  expectGrows,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "menubar" });
});

test.describe("styling", () => {
  // The bar is a row of buttons rather than a daisyUI menu (ADR-0018), so its
  // triggers carry the button's own axes and are asserted the way the button's
  // are: a level below the rendered set, because an axis row here is one menu
  // per value and a menu is the wrapper the primitive renders around a trigger.
  test("every trigger colour applies a colour of its own", async ({ page }) => {
    expectVaries(await computedStyle(triggers(page, "color"), "background-color"), "color");
  });

  test("every trigger size applies a size of its own", async ({ page }) => {
    expectGrows((await computedStyle(triggers(page, "size"), "height")).map(parseFloat), "size");
  });

  // The popup's axes are on an element that is in the document only while its
  // menu is open, and the primitive offers no way to pin one, so the page
  // renders a trigger per value and these open them one at a time.
  test("every menu size sizes the items inside the popup", async ({ page }) => {
    const heights = await eachMenu(page, "menu-size", (menu) =>
      menu.getByRole("menuitem").first(),
    );

    expectGrows(heights, "menu-size");
  });

  test("the popup's appearance axis decides whether it is placed and painted", async ({ page }) => {
    const drawn: string[] = [];

    const triggers = axisTriggers(page, "content");
    const values = await triggers.count();
    expect(values, "the content axis rendered nothing").toBeGreaterThan(1);

    for (let value = 0; value < values; value++) {
      const menu = await openMenu(triggers.nth(value));
      const style = await menu.evaluate((node) => {
        const computed = getComputedStyle(node);
        return [computed.position, computed.backgroundColor, computed.borderRadius].join(" ");
      });
      drawn.push(style);

      await page.keyboard.press("Escape");
    }

    expectVaries(drawn, "content");
  });

  test("the bar's appearance axis decides whether the row is laid out at all", async ({ page }) => {
    await expectAxisVaries(page, "appearance", "display");
  });

  test("an open menu's trigger is marked while it is open", async ({ page }) => {
    const trigger = example(page, "overview").getByRole("menuitem", { name: "File", exact: true });

    const shut = await computedStyle(trigger, "box-shadow");

    await trigger.click();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toBeVisible();

    // The bridged utility: the ring is written as a variant of the `data-state`
    // the primitive sets on the menu around this button, so nothing about the
    // state is recomputed here, and it has to actually apply, which is what a
    // computed style says and a class name would not.
    const open = await computedStyle(trigger, "box-shadow");
    expect(open[0], "an open menu's trigger is unmarked").not.toBe(shut[0]);
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const bar = page.locator("#caller-attributes");
    const barClasses = ((await bar.getAttribute("class")) ?? "").split(/\s+/);

    // The component's layout utilities, and then the caller's surface, on one
    // element.
    expect(barClasses).toContain("inline-flex");
    expect(barClasses).toContain("bg-base-200");
    await expect(bar).toHaveCSS("padding", "4px");

    const trigger = page.locator("#caller-trigger");
    const triggerClasses = ((await trigger.getAttribute("class")) ?? "").split(/\s+/);
    expect(triggerClasses).toContain("btn");
    expect(triggerClasses).toContain("btn-ghost");

    // The popup's width lands on the box and reaches the list inside it, which
    // is what stretching the list to the box is for.
    await trigger.click();
    const popup = page.locator("#caller-popup");
    await expect(popup).toBeVisible();
    await expect(popup).toHaveCSS("width", "224px");
    expect(await popup.locator("> ul").evaluate((node) => node.clientWidth)).toBeGreaterThan(0);
  });
});

test.describe("behaviour", () => {
  const chosen = (page: Page) => page.getByTestId("chosen");
  const trigger = (page: Page, name: string) =>
    example(page, "overview").getByRole("menuitem", { name, exact: true });

  // A roving tab stop, which the toolbar's own page shows the absence of: Tab
  // enters the bar once and leaves it once, and the arrow keys move inside it.
  test("the whole bar is one tab stop", async ({ page }) => {
    await example(page, "overview").focus();

    await page.keyboard.press("Tab");
    await expect(trigger(page, "File")).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(trigger(page, "Edit")).not.toBeFocused();
  });

  test("a click opens a menu and closes it again", async ({ page }) => {
    const file = trigger(page, "File");

    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toHaveCount(0);

    await file.click();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toBeVisible();

    await file.click();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toHaveCount(0);
  });

  test("the arrow keys move along the bar and down an open menu", async ({ page }) => {
    const file = trigger(page, "File");

    await file.click();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toBeVisible();

    // Right moves to the next menu in the bar, and the open menu follows the
    // focus rather than staying behind.
    await page.keyboard.press("ArrowRight");
    await expect(page.getByRole("menuitem", { name: "Cut", exact: true })).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toHaveCount(0);

    await page.keyboard.press("ArrowLeft");
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toBeVisible();

    // Down steps into the open menu.
    await page.keyboard.press("ArrowDown");
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(page.getByRole("menuitem", { name: "Open", exact: true })).toBeFocused();
  });

  test("Escape closes an open menu", async ({ page }) => {
    await trigger(page, "File").click();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toHaveCount(0);
  });

  test("choosing an item reports its value and closes the menu", async ({ page }) => {
    await expect(chosen(page)).toHaveText("nothing yet");

    await trigger(page, "File").click();
    await page.getByRole("menuitem", { name: "Open", exact: true }).click();

    await expect(chosen(page)).toHaveText("open");
    await expect(page.getByRole("menuitem", { name: "Open", exact: true })).toHaveCount(0);
  });

  // Tier 2 on the disabled state, observed: the class this component emits on
  // the item's wrapper is what takes the pointer events off it, which is
  // daisyUI's own way of making a menu item inert. A computed style says the
  // class reached the stylesheet and applies; the class name alone would not.
  test("a disabled item is announced disabled and takes no pointer", async ({ page }) => {
    await trigger(page, "Edit").click();

    const paste = page.getByRole("menuitem", { name: "Paste", exact: true });
    await expect(paste).toHaveAttribute("data-disabled", "true");
    await expect(paste.locator("xpath=..")).toHaveCSS("pointer-events", "none");

    // And the menu it is in is still open, which is what says the item swallowed
    // nothing rather than that the click closed everything.
    await expect(chosen(page)).toHaveText("nothing yet");
  });

  test("a disabled menu does not open", async ({ page }) => {
    const disabled = page.locator("#states").getByRole("menuitem", { name: "Disabled menu" });

    await disabled.click();
    await expect(page.getByRole("menuitem", { name: "Never reached", exact: true })).toHaveCount(0);
  });

  test("a disabled bar opens none of its menus", async ({ page }) => {
    const bar = page.locator("#disabled");

    await bar.getByRole("menuitem", { name: "File" }).click();
    await expect(page.getByRole("menuitem", { name: "New", exact: true })).toHaveCount(0);
  });
});

/** The trigger buttons of one axis' rendered set, in variant-list order. */
function triggers(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > button`);
}

/** Opens one menu from the trigger inside a marked wrapper, and returns it. */
async function openMenu(wrapper: Locator): Promise<Locator> {
  await wrapper.getByRole("menuitem").first().click();

  const menu = wrapper.locator("> [role='menu']");
  await expect(menu).toBeVisible();

  return menu;
}

/**
 * Opens every menu of one axis' rendered set in turn and measures each, which
 * is how an axis on an element that only exists while its menu is open is
 * read.
 */
async function eachMenu(
  page: Page,
  name: string,
  measured: (menu: Locator) => Locator,
): Promise<number[]> {
  const triggers = axisTriggers(page, name);
  const values = await triggers.count();
  expect(values, `the ${name} axis rendered nothing`).toBeGreaterThan(1);

  const heights: number[] = [];

  for (let value = 0; value < values; value++) {
    const menu = await openMenu(triggers.nth(value));
    heights.push(await measured(menu).evaluate((node) => node.getBoundingClientRect().height));

    await page.keyboard.press("Escape");
  }

  return heights;
}
