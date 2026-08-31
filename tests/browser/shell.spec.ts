import { expect, type Locator, type Page, test } from "@playwright/test";

import { addresses, openPreview, themes } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "button", theme: "light" });
});

test("browser coverage is independent of visible navigation", async ({ page }) => {
  const entries = page.locator('[data-page-catalog] [data-catalog-page]');
  const catalog = await entries.evaluateAll((pages) =>
    pages.map((catalogPage) => ({
      id: catalogPage.getAttribute("data-catalog-page"),
      path: catalogPage.getAttribute("data-path"),
    })),
  );
  const combobox = page.locator('[data-page-catalog] [data-catalog-page="combobox"]');

  expect(new Set(catalog.map(({ id }) => id)).size).toBe(catalog.length);
  expect(new Set(catalog.map(({ path }) => path)).size).toBe(catalog.length);
  await expect(combobox).toHaveAttribute("data-listing", "unlisted");
  await expect(combobox).toHaveAttribute("data-browser-test", "enabled");
  await expect(
    page.locator('[data-switcher="component"] [data-value="combobox"]'),
  ).toHaveCount(0);

  expect(await addresses(page)).toContainEqual({
    component: "combobox",
    path: "/components/combobox",
    theme: "light",
  });
});

/** One theme in the switcher's menu, which is the control that themes the page. */
function choice(page: Page, theme: string): Locator {
  return page.locator(`[data-part="themes"] input[value="${theme}"]`);
}

/** Opens the theme menu. */
async function openMenu(page: Page): Promise<void> {
  await page.locator('[data-switcher="theme"] > button').click();
}

/**
 * The theme on the document root, read off one of the custom properties it
 * declares. This is the preview's theme: daisyUI puts it there from whichever
 * control is checked, and nothing else in the app has a say.
 */
function rootTheme(page: Page): Promise<string> {
  return page.evaluate(() =>
    getComputedStyle(document.documentElement).getPropertyValue("--color-base-100"),
  );
}

test.describe("the theme switcher", () => {
  // The preview is themed by the component it documents (ADR-0020), so there is
  // no themed wrapper left anywhere; a `data-theme` reappearing would be an
  // element quietly taking the theme back off the root.
  test("nothing in the page carries data-theme", async ({ page }) => {
    await expect(page.locator("[data-theme]")).toHaveCount(0);
  });

  // The menu offers every theme daisyUI ships, under one radio name, so the
  // browser's single checked control is the theme the page is under.
  test("the menu offers every theme once, as one radio group", async ({ page }) => {
    const offered = await themes(page);

    expect(offered.length, "the switcher offers no theme").toBeGreaterThan(1);
    expect(new Set(offered).size, `a theme is offered twice: ${offered}`).toBe(offered.length);

    const names = await page
      .locator('[data-part="themes"] input[value]')
      .evaluateAll((nodes) => nodes.map((node) => node.getAttribute("name")));
    expect(new Set(names).size, `the menu spans more than one group: ${names}`).toBe(1);
  });

  test("exactly one control is checked, and it is the theme of the address", async ({ page }) => {
    const checked = page.locator('[data-part="themes"] input:checked');

    await expect(checked).toHaveCount(1);
    await expect(checked).toHaveAttribute("value", "light");

    await openMenu(page);
    await choice(page, "dracula").click();

    await expect(checked).toHaveCount(1);
    await expect(checked).toHaveAttribute("value", "dracula");
  });

  // The component doing its one job, in the app that documents it: the control
  // is checked and daisyUI re-declares the theme on the root. No state in the
  // preview stands between the two.
  test("a pick themes the document, in CSS", async ({ page }) => {
    const light = await rootTheme(page);

    await openMenu(page);
    await choice(page, "dracula").click();
    const dracula = await rootTheme(page);
    expect(dracula, "checking the control did not theme the root").not.toBe(light);

    await openMenu(page);
    await choice(page, "cupcake").click();
    expect(await rootTheme(page), "the third theme did not replace the second").not.toBe(dracula);
  });

  // Why the control the reader is using is never rebuilt: a radio group is
  // walked with the arrow keys, and every step of that walk changes the theme. A
  // switcher that reseeded the control under their hands would take focus away
  // at the first press.
  test("the arrow keys walk the menu without losing it", async ({ page }) => {
    const light = await rootTheme(page);

    await openMenu(page);
    await choice(page, "light").focus();
    await page.keyboard.press("ArrowDown");

    // The menu stays open under a walk, which is why it closes on a click
    // rather than on a change: every arrow press changes the theme, and a menu
    // that closed on that would end the walk at its first step.
    await expect(page.locator('[data-part="themes"]')).toBeVisible();
    await expect(choice(page, "dark")).toBeFocused();
    await expect(choice(page, "dark")).toBeChecked();
    expect(await rootTheme(page), "walking the group did not take the theme along").not.toBe(light);
  });

  // What having the theme on the root rather than on an element inside it buys,
  // beyond the mechanism working: daisyUI declares `color-scheme` alongside a
  // theme's colours, and the root is what draws the scrollbar and the overscroll
  // area from it.
  test("the document root takes the theme's colour scheme", async ({ page }) => {
    const scheme = () =>
      page.evaluate(() => getComputedStyle(document.documentElement).colorScheme);

    expect(await scheme()).toBe("light");

    await openMenu(page);
    await choice(page, "dracula").click();
    expect(await scheme(), "the root kept the old colour scheme").toBe("dark");

    await openMenu(page);
    await choice(page, "cupcake").click();
    expect(await scheme()).toBe("light");
  });

  test("picking a theme keeps the reader on the page, under that theme", async ({ page }) => {
    await openMenu(page);
    await choice(page, "cupcake").click();

    // The paint has already happened; what the navigation buys is an address
    // that still names what is on screen, the page it names included, which is
    // to say unchanged.
    await expect(page).toHaveURL(/theme=cupcake/);
    await expect(page.locator('[data-page="button"]')).toBeAttached();
    await expect(choice(page, "cupcake")).toBeChecked();
  });

  // A theme that arrives from anywhere but the menu has to be pushed back into
  // it, and here that is the same thing as repainting: `defaultChecked` no
  // longer moves an input the reader has touched, so the switcher rebuilds the
  // menu from the address.
  test("the back button puts the theme back", async ({ page }) => {
    const light = await rootTheme(page);

    await openMenu(page);
    await choice(page, "dracula").click();
    expect(await rootTheme(page)).not.toBe(light);

    await page.goBack();

    await expect(page).toHaveURL(/theme=light/);
    await expect(choice(page, "light")).toBeChecked();
    await expect(choice(page, "dracula")).not.toBeChecked();
    expect(await rootTheme(page), "the back button left the page on the old theme").toBe(light);
  });
});

test.describe("the theme menu", () => {
  test("it opens and closes, and closes on a pointer pick", async ({ page }) => {
    const trigger = page.locator('[data-switcher="theme"] > button');
    const menu = page.locator('[data-part="themes"]');

    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await expect(menu).toBeHidden();

    await trigger.click();
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(menu).toBeVisible();

    await choice(page, "cupcake").click();
    await expect(menu).toBeHidden();
  });

  // What the keyboard has instead, since walking the list is what the arrow
  // keys are for and the walk is not a decision to close on.
  test("escape puts an open menu away", async ({ page }) => {
    const menu = page.locator('[data-part="themes"]');

    await openMenu(page);
    await choice(page, "light").focus();
    await expect(menu).toBeVisible();

    await page.keyboard.press("Escape");

    await expect(menu).toBeHidden();
  });

  // A screenshot is taken per page per baseline theme rather than per theme, so
  // the marker has to be on the switcher for the harness to read, and it has to
  // be a subset of what the menu offers, or the sweep would ask for an address
  // the preview does not have.
  test("the baseline themes are marked, and are themes the menu offers", async ({ page }) => {
    const offered = await themes(page);
    const baseline = await page
      .locator('[data-part="themes"] [data-baseline]')
      .evaluateAll((nodes) => nodes.map((node) => node.getAttribute("value") ?? ""));

    expect(baseline.length, "no theme is marked as a baseline").toBeGreaterThan(0);
    expect(baseline.length, "every theme is marked as a baseline").toBeLessThan(offered.length);
    expect(offered).toEqual(expect.arrayContaining(baseline));
  });
});
