import { expect, type Locator, type Page, test } from "@playwright/test";

import { axisTriggers, computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "date_picker" });
});

test.describe("structure", () => {
  test("the field carries daisyUI's input class and lays the segments out inside it", async ({
    page,
  }) => {
    const field = input(page);
    await expect(field).toHaveClass(/(^|\s)input(\s|$)/);

    // `.input` is a flex row that lays out whatever is inside it, which is what
    // makes it usable here: the segments and the button are its children rather
    // than a form control it is.
    const [display] = await computedStyle(field, "display");
    expect(display, "the field does not lay its segments out").toBe("inline-flex");

    await expect(field.getByRole("spinbutton")).toHaveCount(3);
    await expect(field.getByRole("button")).toHaveCount(1);
  });

  test("each segment is announced as the part of the date it holds", async ({ page }) => {
    const segments = input(page).getByRole("spinbutton");

    await expect(segments.nth(0)).toHaveAccessibleName("year");
    await expect(segments.nth(1)).toHaveAccessibleName("month");
    await expect(segments.nth(2)).toHaveAccessibleName("day");
  });

  test("the calendar is in the document only while it is open", async ({ page }) => {
    await expect(calendar(page)).toHaveCount(0);

    await trigger(page).click();
    await expect(calendar(page)).toBeVisible();
    await expect(calendar(page).getByRole("grid")).toHaveCount(1);

    await page.keyboard.press("Escape");
    await expect(calendar(page)).toHaveCount(0);
  });

  test("the days in the popup are daisyUI buttons", async ({ page }) => {
    await trigger(page).click();

    const days = calendar(page).locator("tbody button");
    const count = await days.count();
    expect(count, "the popup rendered no days").toBeGreaterThanOrEqual(28);

    const classes = await days.evaluateAll((nodes) =>
      nodes.map((node) => node.classList.contains("btn")),
    );
    expect(classes.every(Boolean), "a day is not a daisyUI button").toBe(true);
  });

  test("the panel is as wide as the month in it", async ({ page }) => {
    // The panel is `w-max`, which asks the month how wide it wants to be, so
    // the grid inside it carries no width of its own. A percentage width there
    // would be a percentage of the answer to the question the panel is asking,
    // and a browser settles that cycle by handing the table every pixel going:
    // what that looked like was a panel a million pixels across with one column
    // of days in view.
    //
    // So what is asserted is where a column's width comes from: the day drawn
    // in it, and nothing else. A weekday label that out-sized a day would show
    // up here as one column wider than the other six.
    await trigger(page).click();

    const grid = calendar(page).getByRole("grid");
    const columns = await widths(grid.locator("tbody tr:first-child td"));
    const days = await widths(grid.locator("tbody tr:first-child td button"));

    expect(columns.length, "the popup rendered no columns").toBe(7);
    expect(new Set(columns).size, `the columns are not one width: ${columns}`).toBe(1);
    expect(
      columns[0],
      `a column is wider than the day in it: ${columns[0]} against ${days[0]}`,
    ).toBe(days[0]);

    // And the panel is the month plus the box drawn around it, rather than
    // whatever room the document would give an absolutely positioned element.
    const [panel] = await widths(calendar(page));
    const [table] = await widths(grid);
    expect(
      panel,
      `the panel is wider than the month in it: ${panel} against ${table}`,
    ).toBeLessThan(table * 2);
  });
});

test.describe("styling", () => {
  test("every colour paints the field differently", async ({ page }) => {
    expectVaries(await computedStyle(fields(page, "color"), "border-top-color"), "color");
  });

  test("every size sizes the field", async ({ page }) => {
    expectGrows((await computedStyle(fields(page, "size"), "height")).map(parseFloat), "size");
  });

  test("the segments' paint can be switched off", async ({ page }) => {
    // Read off the segment nothing has been typed into, which is the state the
    // axis carries a variant for, and which is a variant of an attribute the
    // primitive sets rather than anything recomputed here.
    const painted = page.locator('[data-axis="segment-appearance"] [role="spinbutton"]');
    const padding = await computedStyle(painted, "padding-left");

    expect(padding.length, "the segment appearance axis rendered nothing").toBe(6);
    expect(parseFloat(padding[0]), "the painted segment has no padding").toBeGreaterThan(0);
    expect(parseFloat(padding.at(-1)!), "the switched-off segment still has padding").toBe(0);
  });

  test("the trigger's look can be switched off", async ({ page }) => {
    const triggers = page.locator('[data-axis="trigger-appearance"] button');
    const fills = await computedStyle(triggers, "background-color");

    expect(fills.length, "the trigger appearance axis rendered nothing").toBe(2);
    expect(fills[0], "the ghost trigger paints a fill").toBe("rgba(0, 0, 0, 0)");
    expect(fills[1], "the switched-off trigger is still a ghost").not.toBe("rgba(0, 0, 0, 0)");
  });

  test("the popover's positioning box can be switched off", async ({ page }) => {
    const popovers = page.locator('[data-axis="popover-appearance"] > * > div');
    expectVaries(await computedStyle(popovers, "position"), "popover-appearance");
  });

  test("every side opens the calendar somewhere of its own", async ({ page }) => {
    expectVaries(await offsets(page, "side"), "side");
  });

  test("every alignment sits somewhere of its own along that side", async ({ page }) => {
    expectVaries(await offsets(page, "align"), "align");
  });

  test("the positioning can be switched off", async ({ page }) => {
    const placed = await eachOpened(page, "positioning", (panel) =>
      panel.evaluate((node) => getComputedStyle(node).position),
    );
    expect(placed[0], "the placed calendar is not taken out of the flow").toBe("absolute");
    expect(placed[1], "the switched-off calendar is still taken out of the flow").toBe("static");
  });

  test("the box around the calendar can be switched off", async ({ page }) => {
    const fills = await eachOpened(page, "content-appearance", (panel) =>
      panel.evaluate((node) => getComputedStyle(node).backgroundColor),
    );
    expect(fills[0], "the default box paints no fill").not.toBe("rgba(0, 0, 0, 0)");
    expect(fills[1], "the switched-off box still paints a fill").toBe("rgba(0, 0, 0, 0)");
  });

  test("the month's paint can be switched off", async ({ page }) => {
    // Read off the chosen day, which is the state worth seeing: painted, it is
    // told apart from every other day; switched off, the month is a table of
    // plain buttons.
    const chosen = await eachOpened(page, "calendar-appearance", (panel) =>
      panel
        .locator('[data-selected="true"]')
        .evaluate((node) => getComputedStyle(node).backgroundColor),
    );
    const plain = await eachOpened(page, "calendar-appearance", (panel) =>
      panel
        .getByRole("button", { name: /June 11, 2026$/ })
        .evaluate((node) => getComputedStyle(node).backgroundColor),
    );

    expect(chosen[0], "the chosen day is painted like the one beside it").not.toBe(plain[0]);
    expect(chosen[1], "the switched-off chosen day is still painted apart").toBe(plain[1]);
  });

  test("a caller's classes join the field's own", async ({ page }) => {
    const field = input(page);
    const classes = ((await field.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("input");
    expect(classes.filter((name) => name.startsWith("input-"))).toEqual([]);
  });
});

test.describe("behaviour", () => {
  test("typing into the segments makes a date and reports it", async ({ page }) => {
    const segments = input(page).getByRole("spinbutton");

    await segments.nth(0).focus();
    await page.keyboard.type("2026");

    // The year fills up and the keyboard moves on by itself, which is the
    // primitive's: four digits is a year.
    await expect(segments.nth(1)).toBeFocused();
    await page.keyboard.type("06");
    await expect(segments.nth(2)).toBeFocused();
    await page.keyboard.type("04");

    await expect(page.getByTestId("chosen")).toHaveText("2026-06-04");
    await expect(segments.nth(0)).toHaveText("2026");
    await expect(segments.nth(1)).toHaveText("06");
    await expect(segments.nth(2)).toHaveText("04");
  });

  test("the arrow keys step a segment and move between them", async ({ page }) => {
    const segments = input(page).getByRole("spinbutton");

    await segments.nth(2).focus();
    await page.keyboard.press("ArrowUp");
    await expect(segments.nth(2)).not.toHaveText("DD");

    await page.keyboard.press("ArrowLeft");
    await expect(segments.nth(1)).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(segments.nth(2)).toBeFocused();
  });

  test("choosing a day fills the field in and closes the calendar", async ({ page }) => {
    await trigger(page).click();
    await calendar(page).getByRole("button", { name: /June 4, 2026$/ }).click();

    await expect(calendar(page)).toHaveCount(0);
    await expect(page.getByTestId("chosen")).toHaveText("2026-06-04");

    const segments = input(page).getByRole("spinbutton");
    await expect(segments.nth(0)).toHaveText("2026");
    await expect(segments.nth(1)).toHaveText("06");
    await expect(segments.nth(2)).toHaveText("04");

    // Reopened, the day it was filled in from is the one that is marked.
    await trigger(page).click();
    await expect(calendar(page).locator('[data-selected="true"]')).toHaveText("4");
  });

  test("a disabled picker is faded, and still takes typing, which is the gap", async ({
    page,
  }) => {
    const picker = page.locator("#disabled");
    const segments = picker.getByRole("spinbutton");

    // Faded, through a variant of the attribute the primitive sets, read
    // against a segment of the picker beside it, which holds a date: an empty
    // segment is faded too, by the variant of the attribute for one nothing has
    // been typed into.
    await expect(segments.first()).toHaveAttribute("data-disabled", "true");
    const [muted] = await computedStyle(segments.first(), "opacity");
    const [plain] = await computedStyle(
      page.locator("#read-only").getByRole("spinbutton").first(),
      "opacity",
    );
    expect(parseFloat(muted), "a disabled segment is not faded").toBeLessThan(parseFloat(plain));

    // And still editable, which is the primitive's: `disabled` marks the
    // segments and takes them out of its own focus collection, but the elements
    // stay `contenteditable` and their key handler still writes. Nothing here
    // can fix that without reimplementing the behaviour this component keeps a
    // primitive for, so it is asserted as it is, and the day upstream closes it
    // is the day this spec fails and the documentation is corrected.
    await expect(segments.nth(2)).toHaveAttribute("contenteditable", "true");
    await segments.nth(2).focus();
    await page.keyboard.type("07");
    await expect(segments.nth(2)).not.toHaveText("15");
  });

  test("a read-only picker is filled in from the calendar and not from the keyboard", async ({
    page,
  }) => {
    const picker = page.locator("#read-only");
    const segments = picker.getByRole("spinbutton");

    // Not editable, which is what read-only means here: the primitive drops
    // `contenteditable` from every segment.
    await expect(segments.first()).not.toHaveAttribute("contenteditable", "true");

    await picker.getByRole("button").click();
    const popup = picker.locator('[role="dialog"], [data-side]').first();
    await expect(popup).toBeVisible();
  });

  test("a picker with a range only offers the dates inside it", async ({ page }) => {
    const picker = page.locator("#limited");
    await picker.getByRole("button", { name: "Open the calendar" }).click();

    const popup = page.locator("#limited-calendar");
    await expect(popup).toBeVisible();

    // The three days it was told to refuse are drawn as unavailable, and the
    // month buttons are inert at both ends of the fortnight it accepts.
    await expect(popup.locator('[data-unavailable="true"]')).toHaveCount(3);
    await expect(popup.getByRole("button", { name: "Previous month" })).toBeDisabled();
    await expect(popup.getByRole("button", { name: "Next month" })).toBeDisabled();
  });
});

/** The picker the behavioural specs drive. */
function picker(page: Page): Locator {
  return page.locator("#picker");
}

/** Its field. */
function input(page: Page): Locator {
  return picker(page).locator(".input");
}

/** The button that opens its calendar. */
function trigger(page: Page): Locator {
  return page.locator("#picker-trigger");
}

/** Its calendar, which is only in the document while it is open. */
function calendar(page: Page): Locator {
  return page.locator("#picker-calendar");
}

/** What a set of elements measure across, rounded to whole pixels. */
function widths(elements: Locator): Promise<number[]> {
  return elements.evaluateAll((nodes) =>
    nodes.map((node) => Math.round(node.getBoundingClientRect().width)),
  );
}

/** The fields one axis row renders, in the order its variant list is in. */
function fields(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] .input`);
}

/**
 * Opens each picker of an axis row in turn and reads something off the calendar
 * it opened, in the order the variant list is in.
 *
 * A date picker's open state is the primitive's alone (it lives in a context
 * that component keeps to itself) so a row of these cannot be held open the
 * way the select's popups are. They are opened one at a time instead, which is
 * what `data-axis-triggers` marks a row as needing.
 */
async function eachOpened<T>(
  page: Page,
  name: string,
  read: (panel: Locator) => Promise<T>,
): Promise<T[]> {
  const pickers = axisTriggers(page, name);
  const count = await pickers.count();
  expect(count, `the ${name} axis rendered nothing`).toBeGreaterThan(1);

  const values: T[] = [];
  for (let index = 0; index < count; index += 1) {
    const one = pickers.nth(index);
    await one.getByRole("button", { name: "Open the calendar" }).click();

    const panel = one.locator('[role="application"]').locator("xpath=..");
    await expect(panel).toBeVisible();
    values.push(await read(panel));

    await page.keyboard.press("Escape");
    await expect(one.locator('[role="application"]')).toHaveCount(0);
  }

  return values;
}

/**
 * Where each of a row's calendars sits, measured from the field it belongs to.
 *
 * Geometry rather than computed styles, because a side and an alignment are
 * `inset` properties that resolve to `auto` on the edges they leave alone, so
 * two values that put the panel in quite different places can compute alike.
 */
function offsets(page: Page, name: string): Promise<string[]> {
  return eachOpened(page, name, async (panel) => {
    const box = await panel.boundingBox();
    const field = await panel
      .locator("xpath=ancestor::*[contains(@class, 'relative')]")
      .first()
      .boundingBox();

    return `${Math.round((box?.x ?? 0) - (field?.x ?? 0))} ${Math.round(
      (box?.y ?? 0) - (field?.y ?? 0),
    )}`;
  });
}
