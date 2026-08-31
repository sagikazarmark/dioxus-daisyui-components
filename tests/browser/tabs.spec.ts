import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectAxisGrows, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "tabs" });
});

test.describe("structure", () => {
  // These are the specs for ADR-0003 itself: the rearrangement is the whole
  // component, and every styling rule daisyUI has for tabs is written against
  // the shape asserted here.

  test("the tabs element is the tablist, and no part inside it is", async ({ page }) => {
    const tabs = controlled(page);

    await expect(tabs).toHaveClass(/(^|\s)tabs(\s|$)/);
    await expect(tabs).toHaveAttribute("role", "tablist");

    // The primitive's tab list part, omitted. A second tablist in here would
    // be that element back, and every daisyUI rule below would stop reaching.
    await expect(tabs.locator("[role=tablist]")).toHaveCount(0);
  });

  test("triggers and panels interleave as direct children", async ({ page }) => {
    const children = await controlled(page).evaluate((element) =>
      Array.from(element.children).map(
        (child) => `${child.tagName.toLowerCase()}:${child.getAttribute("role")}`,
      ),
    );

    // Trigger, its panel, the next trigger, its panel: daisyUI's own tabs
    // markup, and the only shape in which its adjacent-sibling rule reaches a
    // panel at all.
    expect(children).toEqual([
      "button:tab",
      "div:tabpanel",
      "button:tab",
      "div:tabpanel",
      "button:tab",
      "div:tabpanel",
      "button:tab",
      "div:tabpanel",
    ]);
  });

  test("a trigger controls the panel written after it", async ({ page }) => {
    const trigger = tab(page, "Usage");
    const controls = await trigger.getAttribute("aria-controls");

    expect(controls, "the trigger controls no panel").toBeTruthy();

    // The panel the trigger names is the element straight after it, which is
    // what makes the two an adjacent pair to daisyUI as well as a labelled
    // pair to a screen reader.
    const named = await trigger.evaluate(
      (element, id) => element.nextElementSibling?.id === id,
      controls,
    );

    expect(named, `the panel after the trigger is not the one it controls: ${controls}`).toBe(true);
    await expect(page.locator(`[id="${controls}"]`)).toHaveAttribute("role", "tabpanel");
  });
});

test.describe("styling", () => {
  test("every appearance renders differently", async ({ page }) => {
    // Not one property but several, the way the dialog's position axis is
    // read: daisyUI expresses an appearance across the tabs element, the
    // active tab and the panel at once, and no single property tells all four
    // values apart. Every value differing from every other in the composite is
    // the same relational assertion the shared axis helpers make, and for the
    // same reason: daisyUI owns the numbers.
    const sets = axis(page, "appearance");
    const count = await sets.count();

    expect(count, "the appearance axis rendered nothing").toBeGreaterThan(1);

    const looks: string[] = [];
    for (let index = 0; index < count; index++) {
      looks.push(await look(sets.nth(index)));
    }

    expect(new Set(looks).size, `two appearances render alike: ${looks}`).toBe(looks.length);
  });

  test("every size applies a size of its own", async ({ page }) => {
    // The size row holds tab bars with no panels under them, so a set's height
    // is the tab's own height, which is what a daisyUI size actually sets.
    await expectAxisGrows(page, "size", "height");
  });

  test("the panel appearance can be switched off", async ({ page }) => {
    // Read off the visible panel of each set rather than off the row's own
    // children, because a panel is only ever inside a set of tabs: the axis is
    // rendered one set per value, and the element the axis styles is the one
    // the active tab reveals.
    const panels = page.locator('[data-axis="panel"] [aria-selected="true"] + .tab-content');

    const padding = await computedStyle(panels, "padding-top");
    expect(padding.length, "the panel axis rendered nothing").toBe(2);
    expect(padding[0], "the default panel emits no padding").not.toBe("0px");
    expect(padding[1], "the switched-off panel still emits padding").toBe("0px");

    // And the border the joined appearances need, which daisyUI draws at a
    // width of its own but leaves transparent, so the colour is ours to emit
    // and ours to switch off (ADR-0004).
    const border = await computedStyle(panels, "border-top-color");
    expect(border[0], "the default panel emits no border colour").not.toBe("rgba(0, 0, 0, 0)");
    expect(border[1], "the switched-off panel still emits a border colour").toBe("rgba(0, 0, 0, 0)");
  });

  test("the lifted appearance joins the active tab to its panel", async ({ page }) => {
    const active = tab(page, "Overview");
    const inactive = tab(page, "Usage");
    const panel = activePanel(page);

    // The active tab loses the rule under it and gains one around its other
    // three sides, and the panel is pulled up over the gap, which is the
    // joined look, and it only exists because the panel is the tab's sibling.
    expect(parseFloat(await length(active, "border-bottom-width"))).toBe(0);
    expect(parseFloat(await length(inactive, "border-bottom-width"))).toBeGreaterThan(0);
    expect(parseFloat(await length(panel, "margin-top"))).toBeLessThan(0);

    const [tabBorder] = await computedStyle(active, "border-top-color");
    const [panelBorder] = await computedStyle(panel, "border-top-color");

    expect(tabBorder, "the active tab is drawing no border").not.toBe("rgba(0, 0, 0, 0)");
    expect(panelBorder, "the panel does not meet the tab's border").toBe(tabBorder);
  });

  test("the boxed appearance paints the active tab like its panel", async ({ page }) => {
    // The boxed set from the appearance row rather than the one the caller
    // restyles, so that what is measured is daisyUI's appearance and nothing
    // else.
    const boxed = page.locator('[data-axis="appearance"] > .tabs-box');
    const active = boxed.locator("[aria-selected=true]");
    const panel = boxed.locator("[aria-selected=true] + .tab-content");

    const [set] = await computedStyle(boxed, "background-color");
    const [tab] = await computedStyle(active, "background-color");
    const [surface] = await computedStyle(panel, "background-color");

    // Where the lifted appearance joins tab to panel with a border, the boxed
    // one joins them by paint: both are lifted off the filled box in the same
    // colour, and the panel is inset from it rather than pulled onto it.
    expect(tab, "the active tab is not painted like its panel").toBe(surface);
    expect(tab, "the active tab is painted like the box it sits in").not.toBe(set);
    expect(set, "the box is painting no fill of its own").not.toBe("rgba(0, 0, 0, 0)");
    expect(parseFloat(await length(panel, "margin-top"))).toBeGreaterThan(0);
  });

  test("a caller's classes join the set's own", async ({ page }) => {
    const boxed = page.locator("#caller-attributes");
    const classes = ((await boxed.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("tabs");
    expect(classes).toContain("tabs-box");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(boxed).toHaveCSS("border-radius", "0px");
  });

  // The whole of tier 1, observed twice: the active tab and the inactive ones
  // carry the very same classes and still render differently, and the panel
  // daisyUI reveals carries the same classes as the ones it hides. A component
  // that had reached for `tab-active` or hidden the panels itself would pass
  // neither half.
  test("the active tab styles without a class of its own", async ({ page }) => {
    const active = tab(page, "Overview");
    const inactive = tab(page, "Usage");

    await expect(active).toHaveAttribute("aria-selected", "true");
    await expect(inactive).toHaveAttribute("aria-selected", "false");

    expect(await active.getAttribute("class")).toBe(await inactive.getAttribute("class"));

    const [activePaint] = await computedStyle(active, "background-color");
    const [inactivePaint] = await computedStyle(inactive, "background-color");
    expect(activePaint, "activating the tab painted nothing").not.toBe(inactivePaint);
  });

  test("panels show and hide through daisyUI's own rule", async ({ page }) => {
    const panels = controlled(page).locator(".tab-content");
    const display = await computedStyle(panels, "display");

    // One shown, the rest hidden, from the `aria-selected` attribute on the
    // sibling before each, since nothing here emits a class for it.
    expect(display.filter((value) => value !== "none")).toEqual(["block"]);

    const classes = await panels.evaluateAll((nodes) => nodes.map((node) => node.className));
    expect(new Set(classes).size, `the panels do not all carry the same classes: ${classes}`).toBe(
      1,
    );
  });
});

test.describe("behaviour", () => {
  const changes = (page: Page) => page.getByTestId("changes");

  test("arrow keys move focus along the row, and activation is a second press", async ({
    page,
  }) => {
    await tab(page, "Overview").focus();
    await expect(tab(page, "Overview")).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(tab(page, "Usage")).toBeFocused();

    // Moved but not activated. Manual activation is the primitive's, and it is
    // what makes arrow keys usable in a set whose panels are expensive.
    await expect(tab(page, "Usage")).toHaveAttribute("aria-selected", "false");
    await expect(changes(page)).toHaveText("0");

    await page.keyboard.press("Enter");
    await expect(tab(page, "Usage")).toHaveAttribute("aria-selected", "true");
    await expect(tab(page, "Overview")).toHaveAttribute("aria-selected", "false");

    // The set is controlled by the page, so the value travelled out through
    // the change callback and back in through the value prop rather than the
    // primitive keeping a copy of its own.
    await expect(changes(page)).toHaveText("1");
    await expect(activePanel(page)).toHaveText("Usage panel");

    await page.keyboard.press("ArrowLeft");
    await expect(tab(page, "Overview")).toBeFocused();
  });

  test("Home and End reach the ends of the row", async ({ page }) => {
    await tab(page, "Usage").focus();

    await page.keyboard.press("End");
    // The last tab that can be reached rather than the last one rendered: the
    // fourth is disabled.
    await expect(tab(page, "Cost")).toBeFocused();

    await page.keyboard.press("Home");
    await expect(tab(page, "Overview")).toBeFocused();
  });

  test("a disabled tab is skipped, and does not activate", async ({ page }) => {
    const archived = tab(page, "Archived");
    await expect(archived).toBeDisabled();

    await tab(page, "Cost").focus();
    await page.keyboard.press("ArrowRight");

    // Past the disabled tab and around to the first one, which is both the
    // skip and the loop.
    await expect(archived).not.toBeFocused();
    await expect(tab(page, "Overview")).toBeFocused();

    await archived.click({ force: true });
    await expect(archived).toHaveAttribute("aria-selected", "false");
    await expect(changes(page)).toHaveText("0");
  });

  test("the row is one tab stop, and its panel follows it", async ({ page }) => {
    await tab(page, "Overview").focus();

    // Roving focus: the whole row is one stop in the document's tab order, so
    // tabbing off the active tab lands on the panel rather than on the next
    // tab in the row.
    await page.keyboard.press("Tab");
    await expect(activePanel(page)).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(page.locator("#after")).toBeFocused();

    // And back the same way, which says the panel sits in the order between
    // the row and what follows it: the announcement order a tabbed interface
    // is expected to have.
    await page.keyboard.press("Shift+Tab");
    await expect(activePanel(page)).toBeFocused();

    await page.keyboard.press("Shift+Tab");
    await expect(tab(page, "Overview")).toBeFocused();
  });

  test("a set of tabs is announced as tabs, and only the active panel is in the tree", async ({
    page,
  }) => {
    // What a screen reader is given: a tablist, four tabs in it, one of them
    // selected and one disabled, and exactly one panel; the rest are hidden
    // outright, which is what keeps the ARIA cost of ADR-0003 to a single
    // stray child of the tablist.
    await expect(controlled(page).getByRole("tab")).toHaveCount(4);
    await expect(controlled(page).getByRole("tab", { selected: true })).toHaveCount(1);
    await expect(controlled(page).getByRole("tabpanel")).toHaveCount(1);

    await expect(tab(page, "Overview")).toHaveAccessibleName("Overview");
    await expect(activePanel(page)).toHaveText("Overview panel");
  });
});

/** The set of tabs the behavioural specs drive, which the page controls. */
function controlled(page: Page): Locator {
  return page.locator("#controlled");
}

/** One tab of that set, by the name it is announced under. */
function tab(page: Page, name: string): Locator {
  return controlled(page).getByRole("tab", { name, exact: true });
}

/** The one panel of that set daisyUI is showing. */
function activePanel(page: Page): Locator {
  return controlled(page).locator("[aria-selected=true] + .tab-content");
}

/** One length off an element, as the string the computed style gives. */
async function length(element: Locator, property: string): Promise<string> {
  const [value] = await computedStyle(element, property);
  return value;
}

/**
 * How one set of tabs is drawn, as the handful of properties an appearance is
 * expressed across taken together.
 *
 * daisyUI puts an appearance in three places at once (the tabs element is
 * painted or not, the active tab is bordered, rounded and filled or not, and
 * the panel is pulled onto it or off it) and no one of them separates all
 * four values.
 */
function look(tabs: Locator): Promise<string> {
  return tabs.evaluate((element) => {
    const active = element.querySelector("[aria-selected=true]");
    if (!active || !active.nextElementSibling) {
      return "no active tab with a panel after it";
    }

    const set = getComputedStyle(element);
    const tab = getComputedStyle(active);
    const panel = getComputedStyle(active.nextElementSibling);

    return [
      set.backgroundColor,
      set.paddingTop,
      tab.backgroundColor,
      tab.borderTopWidth,
      tab.borderBottomWidth,
      tab.borderTopLeftRadius,
      panel.marginTop,
    ].join(" ");
  });
}
