import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "calendar" });
});

test.describe("structure", () => {
  // daisyUI has no calendar this registry can use (ADR-0022), so what is
  // asserted here is the shape the resolution took: days that are daisyUI
  // buttons, in a table the component writes out itself.

  test("every day is a daisyUI button", async ({ page }) => {
    const days = calendar(page).getByRole("button").filter({ hasNotText: /[‹›]/ });

    // Five or six weeks of seven, and each one carrying the class daisyUI puts
    // on a day cell wherever it draws one itself.
    const count = await days.count();
    expect(count, "the grid rendered no days").toBeGreaterThanOrEqual(28);

    const classes = await days.evaluateAll((nodes) =>
      nodes.map((node) => node.classList.contains("btn")),
    );
    expect(classes.every(Boolean), "a day is not a daisyUI button").toBe(true);
  });

  test("the grid is a table with a heading per weekday", async ({ page }) => {
    // The grid is composed here rather than taken whole from the primitive,
    // which is what lets a class reach the headings and the cells at all.
    const grid = calendar(page).getByRole("grid");
    await expect(grid).toHaveCount(1);
    await expect(grid.locator("th")).toHaveCount(7);

    // The headings are out of the accessibility tree (they are the primitive's
    // `aria-hidden` head) so what a screen reader hears is the days.
    await expect(grid.locator("thead")).toHaveAttribute("aria-hidden", "true");
  });

  test("a day is announced by its date rather than by its number", async ({ page }) => {
    const tenth = day(page, "2026-06-10");
    await expect(tenth).toHaveAccessibleName("Wednesday, June 10, 2026");
    await expect(tenth).toHaveText("10");
  });
});

test.describe("styling", () => {
  test("the chosen day is painted, and it is the only one that is", async ({ page }) => {
    // A Bridged utility over `data-selected`: nothing is recomputed in Rust and
    // the primitive stays the only owner of which day is chosen.
    const chosen = shared(page).locator('[data-selected="true"]');
    await expect(chosen).toHaveCount(1);
    await expect(chosen).toHaveText("10");

    const [painted] = await computedStyle(chosen, "background-color");
    const [plain] = await computedStyle(
      shared(page).getByRole("button", { name: /June 11, 2026$/ }),
      "background-color",
    );
    expect(painted, "the chosen day is painted like the rest").not.toBe(plain);
  });

  test("today is ringed, and stays ringed when it is not the chosen day", async ({ page }) => {
    const today = shared(page).locator('[data-today="true"]');
    await expect(today).toHaveCount(1);
    await expect(today).toHaveText("15");

    // A ring is a shadow in Tailwind rather than an outline, so that is where
    // it is read from, and against the day beside it, which has none.
    const [ring] = await computedStyle(today, "box-shadow");
    const [none] = await computedStyle(
      shared(page).getByRole("button", { name: /June 16, 2026$/ }),
      "box-shadow",
    );
    expect(ring, `today is not ringed: ${ring} against ${none}`).not.toBe(none);
  });

  test("the days of the months either side are faded", async ({ page }) => {
    const outside = shared(page).locator('[data-month="last"]').first();
    const inside = shared(page).locator('[data-month="current"]').first();

    const [faded] = await computedStyle(outside, "opacity");
    const [solid] = await computedStyle(inside, "opacity");
    expect(parseFloat(faded), "a day outside the month is not faded").toBeLessThan(parseFloat(solid));
  });

  test("every day size sizes the days", async ({ page }) => {
    const heights = await page
      .locator('[data-axis="day-size"] [role="grid"] button')
      .evaluateAll((nodes) => nodes.map((node) => node.getBoundingClientRect().height));

    // One calendar per value, read off the first day of each: a month is a
    // month, so the first day of each grid is the same day of the week.
    const perCalendar = await page.locator('[data-axis="day-size"] [role="grid"]').count();
    expect(perCalendar, "the day size axis rendered nothing").toBe(5);

    const first = await page
      .locator('[data-axis="day-size"] [role="grid"]')
      .evaluateAll((grids) =>
        grids.map((grid) => (grid.querySelector("tbody button") as Element).getBoundingClientRect().height),
      );
    expectGrows(first, "day-size");
    expect(heights.length, "no days were rendered").toBeGreaterThan(0);
  });

  test("the day's paint can be switched off", async ({ page }) => {
    // Read as the difference the axis is for: whether the chosen day is painted
    // apart from the day beside it. Switched off, a day is `btn` and nothing
    // else, so the two are the same button.
    //
    // Both are read in one evaluation per calendar rather than in two passes
    // over the page, because daisyUI transitions a button's background: two
    // round trips can sample the same colour on either side of a transition and
    // get two spellings of it.
    const painted = await page
      .locator('[data-axis="day-appearance"] [role="application"]')
      .evaluateAll((calendars) =>
        calendars.map((calendar) => {
          const chosen = calendar.querySelector('[data-selected="true"]') as Element;
          const plain = calendar.querySelector(
            '[data-month="current"][data-selected="false"]',
          ) as Element;

          return [
            getComputedStyle(chosen).backgroundColor,
            getComputedStyle(plain).backgroundColor,
          ];
        }),
      );

    expect(painted.length, "the day appearance axis rendered nothing").toBe(2);
    expect(painted[0][0], "the chosen day is painted like the one beside it").not.toBe(
      painted[0][1],
    );
    expect(painted[1][0], "the switched-off chosen day is still painted apart").toBe(painted[1][1]);
  });

  test("every month-button colour, size and look renders its own", async ({ page }) => {
    expectVaries(
      await computedStyle(navigationButtons(page, "button-color"), "background-color"),
      "button-color",
    );
    expectGrows(
      (await computedStyle(navigationButtons(page, "button-size"), "height")).map(parseFloat),
      "button-size",
    );
    expectVaries(
      await computedStyle(navigationButtons(page, "button-appearance"), "background-color"),
      "button-appearance",
    );
  });

  test("the box can be switched off", async ({ page }) => {
    // daisyUI has no calendar class, so the box is utilities this component
    // emits, and a utility it emits is one a caller can only tie with, so
    // switching it off has to be an axis of its own (ADR-0004).
    const boxes = page.locator('[data-axis="appearance"] > *');

    const fill = await computedStyle(boxes, "background-color");
    expect(fill.length, "the appearance axis rendered nothing").toBe(2);
    expect(fill[0], "the default box paints no fill").not.toBe("rgba(0, 0, 0, 0)");
    expect(fill[1], "the switched-off box still paints a fill").toBe("rgba(0, 0, 0, 0)");

    const border = await computedStyle(boxes, "border-top-width");
    expect(parseFloat(border[0]), "the default box draws no border").toBeGreaterThan(0);
    expect(parseFloat(border[1]), "the switched-off box still draws a border").toBe(0);
  });

  test("the month is as wide as its days rather than as wide as the room it is in", async ({
    page,
  }) => {
    // The grid carries no width, and that is deliberate: the box around it
    // shrink-wraps a month, so a percentage width on the table would be a
    // percentage of a width still being worked out from that table, a cycle a
    // browser settles by handing the table every pixel going. What that looked
    // like was a calendar stretched across its column, and a date picker's
    // `w-max` panel blown up to the widest box the engine would draw.
    //
    // So what is asserted is where a column's width comes from: the day drawn
    // in it, and nothing else. A stretched table shows up here as columns far
    // wider than the buttons standing in them, and a weekday label that
    // out-sized a day would show up as one column wider than the other six.
    const grid = shared(page).getByRole("grid");

    const columns = await widths(grid.locator("tbody tr:first-child td"));
    const days = await widths(grid.locator("tbody tr:first-child td button"));

    expect(columns.length, "the grid rendered no columns").toBe(7);
    expect(new Set(columns).size, `the columns are not one width: ${columns}`).toBe(1);
    expect(
      columns[0],
      `a column is wider than the day in it: ${columns[0]} against ${days[0]}`,
    ).toBe(days[0]);
  });

  test("each part that lays a month out can be switched off", async ({ page }) => {
    // Four axes, each over utilities rather than a daisyUI class, and each read
    // in the property it drives.
    expectVaries(
      await computedStyle(page.locator('[data-axis="view-appearance"] [role="application"] > div'), "display"),
      "view-appearance",
    );
    expectVaries(
      await computedStyle(page.locator('[data-axis="navigation-appearance"] [role="heading"] > div'), "display"),
      "navigation-appearance",
    );
    expectVaries(
      await computedStyle(page.locator('[data-axis="title-appearance"] [role="heading"] div > div'), "font-weight"),
      "title-appearance",
    );
    expectVaries(
      await computedStyle(page.locator('[data-axis="grid-appearance"] [role="grid"]'), "table-layout"),
      "grid-appearance",
    );
    // The same axis carries the weekday headings' utilities, which is the half
    // of it no class could otherwise reach: the primitive's own grid takes
    // attributes for the table and nothing inside it.
    const grids = page.locator('[data-axis="grid-appearance"] [role="grid"]');
    const headings = await grids.evaluateAll((nodes) =>
      nodes.map((node) => getComputedStyle(node.querySelector("th") as Element).fontSize),
    );
    expectVaries(headings, "grid-appearance headings");
  });

  test("a caller's classes join the calendar's own", async ({ page }) => {
    const title = page.locator("#month-title");
    const classes = ((await title.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("font-medium");
    expect(classes).toContain("text-sm");
  });
});

test.describe("a range", () => {
  test("the two ends and the middle are painted apart", async ({ page }) => {
    const fixed = page.locator("#range-fixed");

    // Every day of the range reports itself as chosen, and the ones between the
    // ends report that too, which is what the middle is painted from.
    await expect(fixed.locator('[data-selected="true"]')).toHaveCount(7);
    await expect(fixed.locator('[data-selection-start="true"]')).toHaveCount(1);
    await expect(fixed.locator('[data-selection-end="true"]')).toHaveCount(1);
    await expect(fixed.locator('[data-selection-between="true"]')).toHaveCount(5);

    const [end] = await computedStyle(fixed.locator('[data-selection-end="true"]'), "background-color");
    const [middle] = await computedStyle(
      fixed.locator('[data-selection-between="true"]').first(),
      "background-color",
    );
    expect(middle, "the middle of a range is painted like its ends").not.toBe(end);

    // And squared off, so that the run reads as one band rather than as seven
    // marks.
    const [corner] = await computedStyle(
      fixed.locator('[data-selection-between="true"]').first(),
      "border-top-left-radius",
    );
    expect(parseFloat(corner), "the middle of a range is still rounded").toBe(0);
  });

  test("two clicks make a range and report it", async ({ page }) => {
    const live = page.locator("#range-live");

    await live.getByRole("button", { name: /June 3, 2026$/ }).click();
    await live.getByRole("button", { name: /June 6, 2026$/ }).click();

    await expect(page.getByTestId("range")).toHaveText("2026-06-03 to 2026-06-06");
    await expect(live.locator('[data-selected="true"]')).toHaveCount(4);
  });
});

test.describe("behaviour", () => {
  test("clicking a day chooses it and reports it", async ({ page }) => {
    await day(page, "2026-06-04").click();

    await expect(page.getByTestId("selected")).toHaveText("2026-06-04");
    await expect(day(page, "2026-06-04")).toHaveAttribute("data-selected", "true");

    // A second click on the same day clears the choice, which is the
    // primitive's and is reported as nothing chosen.
    await day(page, "2026-06-04").click();
    await expect(page.getByTestId("selected")).toHaveText("nothing");
  });

  test("the arrow keys walk the grid a day at a time", async ({ page }) => {
    await day(page, "2026-06-04").click();
    await expect(day(page, "2026-06-04")).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(day(page, "2026-06-05")).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(day(page, "2026-06-12")).toBeFocused();

    await page.keyboard.press("ArrowLeft");
    await expect(day(page, "2026-06-11")).toBeFocused();

    await page.keyboard.press("Enter");
    await expect(page.getByTestId("selected")).toHaveText("2026-06-11");
  });

  test("the month buttons change the month on show", async ({ page }) => {
    await expect(page.locator("#month-title")).toHaveText("June 2026");

    await page.locator("#next-month").click();
    await expect(page.locator("#month-title")).toHaveText("July 2026");
    await expect(page.getByTestId("view")).toHaveText("2026-07-01");

    await page.locator("#previous-month").click();
    await page.locator("#previous-month").click();
    await expect(page.locator("#month-title")).toHaveText("May 2026");
  });

  test("a second view shows the month after the first", async ({ page }) => {
    const months = await page
      .locator("#months [role='grid']")
      .evaluateAll((grids) => grids.length);
    expect(months, "the second view rendered no month of its own").toBe(2);

    // The two grids are a month apart: the first holds the tenth of June and
    // the second does not.
    await expect(page.locator("#months").getByRole("button", { name: /June 10, 2026$/ })).toHaveCount(1);
    await expect(page.locator("#months").getByRole("button", { name: /July 10, 2026$/ })).toHaveCount(1);
  });

  test("an unavailable day cannot be chosen, and is drawn as much", async ({ page }) => {
    const unavailable = page.locator("#unavailable").locator('[data-unavailable="true"]');
    await expect(unavailable).toHaveCount(5);

    const [muted] = await computedStyle(unavailable.first(), "opacity");
    const [plain] = await computedStyle(
      page.locator("#unavailable").getByRole("button", { name: /June 23, 2026$/ }),
      "opacity",
    );
    expect(parseFloat(muted), "an unavailable day is not muted").toBeLessThan(parseFloat(plain));

    // Chosen it is not: the calendar keeps the day it started on.
    await unavailable.first().click();
    await expect(page.locator("#unavailable").locator('[data-selected="true"]')).toHaveText("10");
  });

  test("a disabled calendar disables its days and its month buttons", async ({ page }) => {
    const calendar = page.locator("#disabled");

    // The primitive puts the native attribute on the two month buttons, which
    // is what daisyUI's own `.btn:disabled` rule matches, so nothing is
    // emitted for that state here.
    const buttons = calendar.getByRole("button", { name: /month/ });
    await expect(buttons).toHaveCount(2);
    await expect(buttons.first()).toBeDisabled();

    // The days stay clickable to the DOM but choose nothing, which is the
    // primitive's own reading of a disabled calendar.
    await calendar.getByRole("button", { name: /June 4, 2026$/ }).click();
    await expect(calendar.locator('[data-selected="true"]')).toHaveText("10");
  });

  test("the calendar is announced as one thing with a heading for the month", async ({ page }) => {
    await expect(calendar(page)).toHaveRole("application");
    await expect(calendar(page)).toHaveAccessibleName("Calendar");
    await expect(calendar(page).getByRole("heading")).toHaveCount(1);
  });
});

/** The calendar the behavioural specs drive, which the page controls. */
function calendar(page: Page): Locator {
  return page.locator("#controlled");
}

/** One of its days, by the date it is announced under. */
function day(page: Page, date: string): Locator {
  const [year, month, dayOfMonth] = date.split("-");
  const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  const name = `${months[Number(month) - 1]} ${Number(dayOfMonth)}, ${year}`;

  return calendar(page).getByRole("button", { name: new RegExp(`${name}$`) });
}

/** The calendar the page holds on a fixed month, chosen date and today. */
function shared(page: Page): Locator {
  return page.locator('[data-example="dates"] [role="application"]');
}

/** What a set of elements measure across, rounded to whole pixels. */
function widths(elements: Locator): Promise<number[]> {
  return elements.evaluateAll((nodes) =>
    nodes.map((node) => Math.round(node.getBoundingClientRect().width)),
  );
}

/** The buttons one axis row renders, in the order its variant list is in. */
function navigationButtons(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] button[aria-label="Previous month"]`);
}
