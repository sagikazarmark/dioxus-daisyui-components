import { expect, type Page, test } from "@playwright/test";

import {
  axis,
  computedStyle,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
  themes,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "theme_controller" });
});

/** The theme on `:root`, read off one of the custom properties it declares. */
function rootTheme(page: Page): Promise<string> {
  return page.evaluate(() =>
    getComputedStyle(document.documentElement).getPropertyValue("--color-base-100"),
  );
}

test.describe("styling", () => {
  // daisyUI draws each of these as a different control, so this reads the set
  // for itself across the properties that separate them: a toggle is a wide
  // pill, a checkbox a rounded square, a radio a circle, a button a padded box,
  // and the value that emits nothing is whatever the browser draws.
  test("every appearance draws a control of its own", async ({ page }) => {
    const rendered = await axis(page, "appearance").evaluateAll((nodes) =>
      nodes.map((node) => {
        const style = getComputedStyle(node);
        return [style.width, style.height, style.borderRadius].join(" ");
      }),
    );

    expectVaries(rendered, "appearance");
  });

  // The other half of the appearance axis, which is not paint at all: the value
  // also decides the input's type, because three of daisyUI's four size scales
  // are gated on it. The pairing is the axis' own promise, so it is asserted
  // outright rather than through a computed style.
  test("every appearance carries the input type daisyUI writes its scale against", async ({
    page,
  }) => {
    const controls = axis(page, "appearance");

    const attributes = await controls.evaluateAll((nodes) =>
      nodes.map((node) => ({
        type: node.getAttribute("type"),
        value: node.getAttribute("value"),
        controller: node.classList.contains("theme-controller"),
      })),
    );

    expect(attributes.map((attribute) => attribute.type)).toEqual([
      "checkbox",
      "checkbox",
      "radio",
      "radio",
      "checkbox",
    ]);

    // And every one of them is a theme controller carrying the theme, which is
    // the whole of what this component puts on the element.
    expect(attributes.every((attribute) => attribute.controller)).toBe(true);
    expect(attributes.every((attribute) => attribute.value === "parchment")).toBe(true);
  });

  // One colour axis, four scales: the value names a colour and the appearance
  // decides which class carries it. Where daisyUI lands that colour differs
  // with the control (a toggle and a radio take it as their own `color`, a
  // checked checkbox and a button fill with it) so each row is read where its
  // own scale paints.
  test("every colour applies a colour of its own, on each scale", async ({ page }) => {
    await expectAxisVaries(page, "color-toggle", "color");
    await expectAxisVaries(page, "color-checkbox", "background-color");
    await expectAxisVaries(page, "color-radio", "color");
    await expectAxisVaries(page, "color-button", "background-color");
  });

  // The scales ADR-0010 records as unreachable elsewhere: `.toggle-lg` and
  // `.radio-lg` are gated on the input's type, and this is the one component
  // whose styled element is a real input, so here they apply, and what is
  // asserted is the ordinary thing about a size axis.
  test("every size grows, on each scale", async ({ page }) => {
    await expectAxisGrows(page, "size-toggle", "height");
    await expectAxisGrows(page, "size-checkbox", "height");
    await expectAxisGrows(page, "size-radio", "height");
    await expectAxisGrows(page, "size-button", "height");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const control = page.locator("#caller-attributes");
    const classes = ((await control.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("toggle");
    expect(classes).toContain("theme-controller");
    expect(classes).toContain("toggle-primary");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(control).toHaveCSS("border-radius", "0px");
  });

  // The axis value that emits nothing, which is what a caller switches to when
  // they want the input with a control of their own around it, or, as here,
  // one of the pairings the axis does not offer: a button that is a checkbox.
  test("the appearance axis can emit no class at all", async ({ page }) => {
    const control = page.locator("#caller-button-checkbox");
    const classes = ((await control.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).not.toContain("toggle");
    expect(classes).not.toContain("checkbox");
    expect(classes).not.toContain("radio");
    expect(classes).toContain("theme-controller");
    expect(classes).toContain("btn");

    await expect(control).toHaveAttribute("type", "checkbox");
  });
});

test.describe("behaviour", () => {
  // What the mechanism does when the theme is not one the app enabled, which is
  // every control on this page: daisyUI emits its `:has()` rule only for the
  // themes named in the app's own `@plugin` block, so a value it never emitted
  // matches nothing. That is what keeps a reader clicking through the examples
  // from re-theming the page the examples are documented on. The rule working is
  // asserted where it is put to work: the preview's own switcher, in
  // shell.spec.ts.
  test("a controller naming a theme the app has not enabled themes nothing", async ({ page }) => {
    const before = await rootTheme(page);
    const control = page.locator("#overview-seafoam");

    await control.click();

    await expect(control, "the control did not take the click").toBeChecked();
    expect(await rootTheme(page), "an example re-themed the page it documents").toBe(before);
  });

  // The guard behind that, over the whole page rather than one control: no
  // example may name a theme the preview enables. One that did would repaint the
  // site from inside a documentation page and, since the screenshots are taken
  // per address per theme, would do it in a baseline.
  test("no control on the page names a theme the preview enables", async ({ page }) => {
    const enabled = new Set(await themes(page));
    const named = await page
      .locator("[data-page] input.theme-controller[value]")
      .evaluateAll((nodes) => nodes.map((node) => node.getAttribute("value") ?? ""));

    expect(named.length, "the page renders no controller at all").toBeGreaterThan(0);

    const clashing = [...new Set(named.filter((value) => enabled.has(value)))];
    expect(clashing, `an example would theme the page it documents: ${clashing}`).toEqual([]);
  });

  // The browser is the behaviour here, which is what having no primitive costs
  // and buys: one name over a set is the whole of what keeps a single theme
  // checked.
  test("a set sharing a name keeps one theme checked", async ({ page }) => {
    const parchment = page.locator("#overview-parchment");
    const seafoam = page.locator("#overview-seafoam");

    await expect(parchment).toBeChecked();

    await seafoam.click();
    await expect(seafoam).toBeChecked();
    await expect(parchment).not.toBeChecked();
  });

  test("the arrow keys walk the set", async ({ page }) => {
    const midnight = page.locator("#overview-midnight");

    await page.locator("#overview-parchment").focus();

    await page.keyboard.press("ArrowRight");

    await expect(midnight).toBeFocused();
    await expect(midnight).toBeChecked();
  });

  // No class is emitted for the checked state, because daisyUI reads it off the
  // input itself: the two controls carry the very same class attribute and
  // render differently anyway.
  test("checking a controller styles it without a class of its own", async ({ page }) => {
    const off = page.locator("#states-off");
    const on = page.locator("#states-on");

    await expect(off).not.toBeChecked();
    await expect(on).toBeChecked();

    expect(await on.getAttribute("class")).toBe(await off.getAttribute("class"));

    const [offPaint, onPaint] = await computedStyle(
      page.locator("#states-off, #states-on"),
      "background-color",
    );
    expect(onPaint, "checking the controller repainted nothing").not.toBe(offPaint);
  });

  test("a disabled controller is announced disabled and takes no click", async ({ page }) => {
    const disabled = page.getByRole("checkbox", { name: "Disabled", exact: true });

    await expect(disabled).toBeDisabled();
    await expect(disabled).not.toBeChecked();

    await disabled.click({ force: true });

    await expect(disabled).not.toBeChecked();
  });

  // The one thing the CSS cannot do is remember the choice, so the component
  // offers the change to the page. The handler runs after the browser has moved
  // the control, which is also after daisyUI would have applied the theme in an
  // app that enabled it; what it gets is the name to write down.
  test("the change reports the theme the browser applied", async ({ page }) => {
    const remembered = page.getByTestId("remembered");
    const control = page.locator("#states-remembered");

    await expect(remembered).toHaveText("none");

    await control.click();
    await expect(remembered).toHaveText("midnight");

    await control.click();
    await expect(remembered).toHaveText("none");
  });
});
