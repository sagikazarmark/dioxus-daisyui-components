import { expect, type Locator, type Page, test } from "@playwright/test";

import {
  axisTriggers,
  computedStyle,
  example,
  expectGrows,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "navbar" });
});

test.describe("structure", () => {
  test("uses daisyUI's navbar regions and dropdown split", async ({ page }) => {
    const bar = overview(page)
      .getByRole("navigation", { name: "Primary" })
      .getByRole("menubar");

    await expect(bar).toHaveClass(/\bnavbar\b/);
    await expect(bar.locator(".navbar-start")).toHaveCount(1);
    await expect(bar.locator(".navbar-center")).toHaveCount(1);
    await expect(bar.locator(".navbar-end")).toHaveCount(1);

    const inputs = trigger(page, "Inputs");
    const wrapper = nav(page, "Inputs");
    await expect(wrapper).toHaveClass(/\bdropdown\b/);

    await inputs.press("Enter");
    const content = popup(page, "Inputs");
    await expect(content).toBeVisible();
    await expect(content).toHaveClass(/\bdropdown-content\b/);
    await expect(content).not.toHaveClass(/(^|\s)menu(\s|$)/);
    await expect(content.locator(":scope > ul")).toHaveClass(/\bmenu\b/);

    const items = content.getByRole("menuitem");
    await expect(items).toHaveCount(3);
    for (let index = 0; index < (await items.count()); index++) {
      await expect(items.nth(index).locator("xpath=..")).toHaveAttribute(
        "role",
        "none",
      );
    }

    await expect(content.getByRole("list")).toHaveCount(0);
    await expect(content.getByRole("listitem")).toHaveCount(0);
    expect(
      await trigger(page, "Home")
        .locator("xpath=..")
        .evaluate((node) => node.tagName),
    ).not.toBe("LI");
  });
});

test.describe("styling", () => {
  test("every trigger colour applies a colour of its own", async ({ page }) => {
    expectVaries(
      await computedStyle(triggers(page, "color"), "background-color"),
      "color",
    );
  });

  test("every trigger size applies a size of its own", async ({ page }) => {
    expectGrows(
      (await computedStyle(triggers(page, "size"), "height")).map(parseFloat),
      "size",
    );
  });

  test("every menu size sizes the popup items", async ({ page }) => {
    const heights = await eachMenu(page, "menu-size", (menu) =>
      menu.getByRole("menuitem").first(),
    );
    expectGrows(heights, "menu-size");
  });

  test("the popup appearance axis switches its box utilities off", async ({
    page,
  }) => {
    const styles: string[] = [];
    const wrappers = axisTriggers(page, "content");
    const values = await wrappers.count();
    expect(values, "the content axis rendered nothing").toBeGreaterThan(1);

    for (let value = 0; value < values; value++) {
      const menu = await openMenu(wrappers.nth(value));
      styles.push(
        await menu.evaluate((node) => {
          const style = getComputedStyle(node);
          return [
            style.backgroundColor,
            style.borderRadius,
            style.boxShadow,
          ].join(" ");
        }),
      );
      await page.keyboard.press("Escape");
    }

    expectVaries(styles, "content");
  });

  test("marks an open trigger without taking ownership of open state", async ({
    page,
  }) => {
    const inputs = trigger(page, "Inputs");
    const shut = await computedStyle(inputs, "box-shadow");

    await inputs.press("Enter");
    await expect(popup(page, "Inputs")).toBeVisible();

    const open = await computedStyle(inputs, "box-shadow");
    expect(open[0], "an open navbar trigger is unmarked").not.toBe(shut[0]);
  });

  test("the open-trigger appearance can be switched off", async ({ page }) => {
    const shadows: string[] = [];
    const wrappers = axisTriggers(page, "trigger-open");
    const values = await wrappers.count();
    expect(values, "the trigger-open axis rendered nothing").toBeGreaterThan(1);

    for (let value = 0; value < values; value++) {
      const trigger = wrappers.nth(value).getByRole("menuitem").first();
      await trigger.press("Enter");
      await expect(
        wrappers.nth(value).locator(":scope > [role='menu']"),
      ).toBeVisible();
      shadows.push((await computedStyle(trigger, "box-shadow"))[0]);
      await page.keyboard.press("Escape");
    }

    expectVaries(shadows, "trigger-open");
  });

  test("places a popup below its trigger", async ({ page }) => {
    const inputs = trigger(page, "Inputs");
    await inputs.press("Enter");

    const triggerBox = await inputs.boundingBox();
    const popupBox = await popup(page, "Inputs").boundingBox();
    expect(triggerBox).not.toBeNull();
    expect(popupBox).not.toBeNull();
    expect(popupBox!.y).toBeGreaterThanOrEqual(
      triggerBox!.y + triggerBox!.height - 1,
    );
  });

  test("a caller's classes join every compound part's own", async ({
    page,
  }) => {
    const bar = page.locator("#caller-attributes");
    expect(classes(await bar.getAttribute("class"))).toEqual(
      expect.arrayContaining(["navbar", "bg-base-200", "rounded-box", "p-1"]),
    );

    const start = page.locator("#caller-start");
    expect(classes(await start.getAttribute("class"))).toEqual(
      expect.arrayContaining(["navbar-start", "gap-2"]),
    );

    const callerTrigger = page.locator("#caller-trigger");
    expect(classes(await callerTrigger.getAttribute("class"))).toEqual(
      expect.arrayContaining(["btn", "btn-ghost"]),
    );

    await callerTrigger.press("Enter");
    const callerPopup = page.locator("#caller-popup");
    await expect(callerPopup).toBeVisible();
    expect(classes(await callerPopup.getAttribute("class"))).toEqual(
      expect.arrayContaining(["dropdown-content", "w-56"]),
    );
    await expect(callerPopup).toHaveCSS("width", "224px");
    expect(
      await callerPopup
        .locator(":scope > ul")
        .evaluate((node) => node.clientWidth),
    ).toBeGreaterThan(0);
    await expect(
      callerPopup.getByRole("menuitem", { name: "Documentation" }),
    ).toHaveClass(/\bfont-semibold\b/);
  });
});

test.describe("behaviour", () => {
  test("the whole bar is one tab stop", async ({ page }) => {
    await overview(page).focus();
    await page.keyboard.press("Tab");
    await expect(trigger(page, "Inputs")).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(trigger(page, "Home")).not.toBeFocused();
    await expect(trigger(page, "Information")).not.toBeFocused();
  });

  test("the arrow keys walk links and triggers across all three regions", async ({
    page,
  }) => {
    await bar(page).focus();
    await expect(trigger(page, "Inputs")).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(trigger(page, "Home")).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(trigger(page, "Information")).toBeFocused();

    await page.keyboard.press("ArrowLeft");
    await expect(trigger(page, "Home")).toBeFocused();
  });

  test("a trigger opens and closes its popup", async ({ page }) => {
    const inputs = trigger(page, "Inputs");
    await expect(popup(page, "Inputs")).toHaveCount(0);

    await inputs.press("Enter");
    await expect(popup(page, "Inputs")).toBeVisible();

    await inputs.press("Enter");
    await expect(popup(page, "Inputs")).toHaveCount(0);
  });

  test("ArrowDown opens a popup and skips a disabled item", async ({
    page,
  }) => {
    await bar(page).focus();
    await expect(trigger(page, "Inputs")).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(trigger(page, "Calendar")).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(trigger(page, "Slider")).not.toBeFocused();
    await expect(trigger(page, "Checkbox")).toBeFocused();
  });

  test("Escape dismisses an open popup", async ({ page }) => {
    await trigger(page, "Inputs").press("Enter");
    await expect(popup(page, "Inputs")).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(popup(page, "Inputs")).toHaveCount(0);
  });

  test("a disabled item is announced and pointer-inert", async ({ page }) => {
    await trigger(page, "Inputs").press("Enter");
    const slider = trigger(page, "Slider");

    await expect(slider).toHaveAttribute("data-disabled", "true");
    await expect(slider.locator("xpath=..")).toHaveCSS(
      "pointer-events",
      "none",
    );
  });

  test("a disabled dropdown does not open", async ({ page }) => {
    const disabled = page
      .locator("#states")
      .getByRole("menuitem", { name: "Disabled menu" });
    await disabled.click();
    await expect(trigger(page, "Never reached")).toHaveCount(0);
  });

  test("a disabled navbar opens none of its dropdowns", async ({ page }) => {
    const disabled = page.locator("#disabled");
    await disabled.getByRole("menuitem", { name: "File" }).click();
    await expect(disabled.getByRole("menuitem", { name: "New" })).toHaveCount(
      0,
    );
  });

  test("a disabled direct link does not navigate", async ({ page }) => {
    const link = page
      .locator("#states")
      .getByRole("menuitem", { name: "Disabled link" });
    const before = page.url();

    await expect(link).toHaveAttribute("data-disabled", "true");
    await link.click();
    await expect(page).toHaveURL(before);
  });

  test("a disabled navbar prevents its direct links from navigating", async ({
    page,
  }) => {
    const link = page
      .locator("#disabled")
      .getByRole("menuitem", { name: "Disabled navbar link" });
    const before = page.url();

    await expect(link).toHaveAttribute("data-disabled", "true");
    await link.click();
    await expect(page).toHaveURL(before);
  });
});

function overview(page: Page): Locator {
  return example(page, "overview");
}

function bar(page: Page): Locator {
  return overview(page).getByRole("menubar");
}

function trigger(page: Page, name: string): Locator {
  return overview(page).getByRole("menuitem", { name, exact: true });
}

function nav(page: Page, name: string): Locator {
  return trigger(page, name).locator("xpath=..");
}

function popup(page: Page, name: string): Locator {
  return nav(page, name).locator(":scope > [role='menu']");
}

function triggers(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > button[role="menuitem"]`);
}

async function openMenu(wrapper: Locator): Promise<Locator> {
  await wrapper.getByRole("menuitem").first().press("Enter");
  const menu = wrapper.locator(":scope > [role='menu']");
  await expect(menu).toBeVisible();
  return menu;
}

async function eachMenu(
  page: Page,
  name: string,
  measured: (menu: Locator) => Locator,
): Promise<number[]> {
  const wrappers = axisTriggers(page, name);
  const values = await wrappers.count();
  expect(values, `the ${name} axis rendered nothing`).toBeGreaterThan(1);

  const heights: number[] = [];
  for (let value = 0; value < values; value++) {
    const menu = await openMenu(wrappers.nth(value));
    heights.push(
      await measured(menu).evaluate(
        (node) => node.getBoundingClientRect().height,
      ),
    );
    await page.keyboard.press("Escape");
  }
  return heights;
}

function classes(value: string | null): string[] {
  return (value ?? "").split(/\s+/);
}
