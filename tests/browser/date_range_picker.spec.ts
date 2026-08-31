import { expect, type Locator, type Page, test } from "@playwright/test";

import { axisTriggers, computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "date_range_picker" });
});

test.describe("structure", () => {
  test("the field carries daisyUI's input class and lays out what the caller put in it", async ({
    page,
  }) => {
    const field = input(page);
    await expect(field).toHaveClass(/(^|\s)input(\s|$)/);

    // `.input` is a flex row that lays out whatever is inside it, which is what
    // makes it usable here: the range and the button are its children rather
    // than a form control it is.
    const [display] = await computedStyle(field, "display");
    expect(display, "the field does not lay its content out").toBe("inline-flex");

    // No segments: the primitive's segmented range input is not published, for
    // the reason the component's documentation records: it never settles once a
    // range exists. What the field shows is the caller's.
    await expect(field.getByRole("spinbutton")).toHaveCount(0);
    await expect(field.getByRole("button")).toHaveCount(1);
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

  test("the trigger's look can be switched off", async ({ page }) => {
    const triggers = page.locator('[data-axis="trigger-appearance"] .input button');
    const fills = await computedStyle(triggers, "background-color");

    // Relational rather than named: what the axis decides is whether the button
    // sits in the field without filling it, and the engines spell a transparent
    // background differently.
    expect(fills.length, "the trigger appearance axis rendered nothing").toBe(2);
    expect(fills[0], "the ghost trigger paints the same fill as the plain one").not.toBe(fills[1]);
    expect(parseFloat(fills[0].split(",").at(-1) ?? "1"), "the ghost trigger is not see-through").toBe(0);
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
        // The first of the run, since a range marks every day in it.
        .locator('[data-selected="true"]')
        .first()
        .evaluate((node) => getComputedStyle(node).backgroundColor),
    );
    const plain = await eachOpened(page, "calendar-appearance", (panel) =>
      panel
        .getByRole("button", { name: /June 3, 2026$/ })
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
  test("two clicks make a range, report it and close the calendar", async ({ page }) => {
    await trigger(page).click();
    await calendar(page).getByRole("button", { name: /June 4, 2026$/ }).click();

    // One click is one end of a range rather than a range, so the calendar
    // stays open and nothing is reported.
    await expect(calendar(page)).toBeVisible();
    await expect(page.getByTestId("chosen")).toHaveText("nothing");

    await calendar(page).getByRole("button", { name: /June 12, 2026$/ }).click();

    await expect(calendar(page)).toHaveCount(0);
    await expect(page.getByTestId("chosen")).toHaveText("2026-06-04 to 2026-06-12");

    // Reopened, the run it was filled in from is painted: two ends and the days
    // between them.
    await trigger(page).click();
    await expect(calendar(page).locator('[data-selected="true"]')).toHaveCount(9);
    await expect(calendar(page).locator('[data-selection-start="true"]')).toHaveText("4");
    await expect(calendar(page).locator('[data-selection-end="true"]')).toHaveText("12");
    await expect(calendar(page).locator('[data-selection-between="true"]')).toHaveCount(7);
  });

  test("a disabled picker is marked as one, and a read-only picker still opens", async ({
    page,
  }) => {
    // With no segments in the field, a disabled picker is one the primitive
    // marks and nothing here paints: the state a caller reads from the outside.
    await expect(page.locator("#disabled")).toHaveAttribute("data-disabled", "true");

    const picker = page.locator("#read-only");
    await picker.getByRole("button").click();
    await expect(picker.locator('[role="application"]')).toBeVisible();
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
