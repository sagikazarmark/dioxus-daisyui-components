import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "drag_and_drop_list" });
});

test.describe("structure", () => {
  test("the list carries daisyUI's list class and every row carries its row class", async ({
    page,
  }) => {
    const list = sortable(page).getByRole("list");
    await expect(list).toHaveClass(/(^|\s)list(\s|$)/);

    const rows = list.getByRole("listitem");
    await expect(rows).toHaveCount(4);

    const classes = await rows.evaluateAll((nodes) =>
      nodes.map((node) => node.classList.contains("list-row")),
    );
    expect(classes.every(Boolean), "a row is not a daisyUI list row").toBe(true);
  });

  test("the wrapper carries nothing of its own", async ({ page }) => {
    // daisyUI's `list` belongs on the list rather than on what holds it, and
    // what holds it here is a wrapper around three things.
    const wrapper = sortable(page).locator("> div");
    expect(await wrapper.getAttribute("class"), "the wrapper emits a class").toBeNull();
  });

  test("the list is announced as sortable, with the keys written down for a screen reader", async ({
    page,
  }) => {
    const list = sortable(page).getByRole("list");

    await expect(list).toHaveAttribute("aria-roledescription", "sortable list");
    await expect(list).toHaveAccessibleName("Playlist");

    // The instructions are pointed at by the list and are what a screen reader
    // reads before the rows, so they are a part rather than something a caller
    // has to remember.
    const instructions = page.locator("#dnd-instructions").first();
    await expect(instructions).toContainText("Press Enter to start reordering");

    // And the live region each move is announced in.
    await expect(sortable(page).getByRole("status")).toBeAttached();
  });

  test("written out by hand, the parts render the same list", async ({ page }) => {
    const list = page.locator("#written-out").getByRole("list");

    await expect(list).toHaveClass(/(^|\s)list(\s|$)/);
    await expect(list.getByRole("listitem")).toHaveCount(3);
    await expect(list.getByRole("listitem").first()).toHaveClass(/(^|\s)list-row(\s|$)/);
  });
});

test.describe("styling", () => {
  test("the box can be switched off", async ({ page }) => {
    const lists = page.locator('[data-axis="appearance"] ul');

    const fill = await computedStyle(lists, "background-color");
    expect(fill.length, "the appearance axis rendered nothing").toBe(2);
    expect(fill[0], "the default list paints no fill").not.toBe("rgba(0, 0, 0, 0)");
    expect(fill[1], "the switched-off list still paints a fill").toBe("rgba(0, 0, 0, 0)");

    const radius = await computedStyle(lists, "border-top-left-radius");
    expect(parseFloat(radius[0]), "the default list rounds no corner").toBeGreaterThan(0);
    expect(parseFloat(radius[1]), "the switched-off list still rounds a corner").toBe(0);
  });

  test("the row's paint can be switched off", async ({ page }) => {
    // Read off the cursor, which is the part of it that shows without a drag in
    // flight: a row that can be dragged looks like one.
    const rows = page.locator('[data-axis="item-appearance"] li');
    const cursors = await rows.evaluateAll((nodes) =>
      nodes.map((node) => getComputedStyle(node).cursor),
    );

    expect(cursors[0], "a draggable row does not look draggable").toBe("grab");
    expect(cursors.at(-1), "the switched-off row still looks draggable").not.toBe("grab");
  });

  test("the drop line can be switched off", async ({ page }) => {
    // The line is only in the document while a move is in flight, so the axis
    // is read with one under way: Enter picks a row up and an arrow key points
    // the drop at the next slot.
    const drawn = await indicatorHeight(page, 0);
    const switchedOff = await indicatorHeight(page, 1);

    expect(drawn, "the drop line is not drawn").toBeGreaterThan(0);
    expect(switchedOff, "the switched-off drop line is still drawn").toBe(0);
  });

  test("a caller's classes join the list's own", async ({ page }) => {
    const list = sortable(page).getByRole("list");
    const classes = ((await list.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("list");
    expect(classes).toContain("bg-base-100");
  });

  test("every value of every axis renders differently", async ({ page }) => {
    expectVaries(
      await computedStyle(page.locator('[data-axis="appearance"] ul'), "background-color"),
      "appearance",
    );
    expectVaries(
      await page
        .locator('[data-axis="item-appearance"] ul')
        .evaluateAll((lists) =>
          lists.map((list) => getComputedStyle(list.querySelector("li") as Element).cursor),
        ),
      "item-appearance",
    );
  });
});

test.describe("behaviour", () => {
  test("a row is picked up with the keyboard and moved with the arrow keys", async ({ page }) => {
    await expect(rowTexts(page)).resolves.toEqual(["Rise", "Ember", "Lantern", "Tidewater"]);

    const first = sortable(page).getByRole("listitem").first();
    await first.focus();
    await expect(first).toBeFocused();

    // Enter picks the row up, which the primitive says out loud and marks on
    // the row, and this component paints from that mark.
    await page.keyboard.press("Enter");
    await expect(sortable(page).locator('[data-is-grabbing="true"]')).toHaveCount(1);

    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");

    // The two rows have swapped. What travelled is the whole row the caller
    // passed, its position number included, which is why the tracks are read
    // off their own line rather than off the row's text.
    await expect(rowTexts(page)).resolves.toEqual(["Ember", "Rise", "Lantern", "Tidewater"]);
    await expect(sortable(page).locator('[data-is-grabbing="true"]')).toHaveCount(0);
  });

  test("Escape puts a row back where it started", async ({ page }) => {
    const before = await rowTexts(page);

    const first = sortable(page).getByRole("listitem").first();
    await first.focus();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");

    await expect(rowTexts(page)).resolves.toEqual(before);

    // And says so, in the region the moves are announced in.
    await expect(sortable(page).getByRole("status")).toContainText("Movement cancelled");
  });

  test("the arrow keys move focus through the rows while nothing is being carried", async ({
    page,
  }) => {
    const rows = sortable(page).getByRole("listitem");

    await rows.first().focus();
    await page.keyboard.press("ArrowDown");
    await expect(rows.nth(1)).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(rows.first()).toBeFocused();

    // And the order is untouched, because nothing was picked up.
    await expect(rowTexts(page)).resolves.toEqual(["Rise", "Ember", "Lantern", "Tidewater"]);
  });

  test("the drop line appears beside the row a move is aimed at", async ({ page }) => {
    const rows = sortable(page).getByRole("listitem");
    await rows.first().focus();
    await page.keyboard.press("Enter");

    // Nothing is drawn while the drop would leave the row where it is; the row
    // says that itself instead, which is the one state the line cannot show.
    await expect(sortable(page).locator('[data-drop-at-origin="true"]')).toHaveCount(1);
    await expect(sortable(page).locator("ul > div")).toHaveCount(0);

    await page.keyboard.press("ArrowDown");
    await expect(sortable(page).locator("ul > div")).toHaveCount(1);
  });

  test("a row is grabbable by the pointer", async ({ page }) => {
    // The attribute rather than a drag: a headless browser's drag events do not
    // carry a data transfer, so what is asserted is that the row offers itself
    // to the pointer at all.
    const rows = sortable(page).getByRole("listitem");
    await expect(rows.first()).toHaveAttribute("draggable", "true");
    await expect(rows.first()).toHaveAttribute("aria-roledescription", "sortable item");
  });
});

/** The list the behavioural specs drive. */
function sortable(page: Page): Locator {
  return page.locator("#sortable");
}

/**
 * What its rows are, in the order they are rendered in.
 *
 * Read off the track's own line rather than off the whole row, because a row
 * carries a position number that travels with it: the number is part of what
 * the caller passed as that row, so moving the row moves the number too.
 */
function rowTexts(page: Page): Promise<string[]> {
  return sortable(page)
    .getByRole("listitem")
    .evaluateAll((nodes) =>
      nodes.map((node) => (node.querySelector(".list-col-grow div")?.textContent ?? "").trim()),
    );
}

/**
 * How tall the drop line of one of the axis row's lists is while a move is in
 * flight, or zero if there is none.
 */
async function indicatorHeight(page: Page, index: number): Promise<number> {
  const list = page.locator('[data-axis="indicator-appearance"] ul').nth(index);
  const row = list.getByRole("listitem").first();

  await row.focus();
  await page.keyboard.press("Enter");
  await page.keyboard.press("ArrowDown");

  const line = list.locator("> div");
  await expect(line).toHaveCount(1);

  const height = await line.evaluate((node) => node.getBoundingClientRect().height);
  await page.keyboard.press("Escape");

  return height;
}
