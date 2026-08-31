import { expect, type Locator, type Page, test } from "@playwright/test";

import { example, expectAxisGrows, expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "toggle_group" });
});

test.describe("styling", () => {
  test("every colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every size applies a size of its own", async ({ page }) => {
    await expectAxisGrows(page, "size", "height");
  });

  // What `join` buys, and the reason the items have to be its direct children:
  // daisyUI rounds the ends of the row and squares everything between them,
  // through `:scope > :first-child` and `:scope > :last-child`. An item inside
  // a wrapper would leave every corner rounded, which is what this would catch.
  test("a joined row is rounded at its ends and square between them", async ({ page }) => {
    const items = page.locator("#group-horizontal > *");
    await expect(items).toHaveCount(3);

    const [first, middle, last] = await items.evaluateAll((nodes) =>
      nodes.map((node) => {
        const style = getComputedStyle(node);
        return {
          start: parseFloat(style.borderStartStartRadius),
          end: parseFloat(style.borderEndEndRadius),
        };
      }),
    );

    expect(first.start, "the first item is not rounded at the start of the row").toBeGreaterThan(0);
    expect(last.end, "the last item is not rounded at the end of the row").toBeGreaterThan(0);
    expect(middle.start, "an item in the middle of the row is rounded").toBe(0);
    expect(middle.end, "an item in the middle of the row is rounded").toBe(0);
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const item = page.locator("#caller-attributes");
    const classes = ((await item.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, including the class it emits for the pressed state,
    // and then the caller's, on one element.
    expect(classes).toContain("join-item");
    expect(classes).toContain("btn");
    expect(classes).toContain("btn-primary");
    expect(classes).toContain("btn-active");
    expect(classes).toContain("rounded-s-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // the radius daisyUI's own join put on the first item of the row, which the
    // caller wins on cascade layers rather than on specificity (ADR-0004).
    await expect(item).toHaveCSS("border-start-start-radius", "0px");

    // The group takes a caller's classes the same way.
    expect(((await page.locator("#caller-group").getAttribute("class")) ?? "").split(/\s+/))
      .toEqual(expect.arrayContaining(["join", "join-horizontal", "w-64"]));
  });
});

test.describe("behaviour", () => {
  test("pressing an item announces it pressed and repaints it", async ({ page }) => {
    const off = item(page, "Off");
    const on = item(page, "On");

    // Tier 2, observed from both ends: the state is in the accessibility tree
    // because the primitive puts it there, and it is on screen because this
    // component emits daisyUI's own class for it. daisyUI matches
    // `aria-pressed` nowhere, so a component that had left the class off would
    // pass the first assertion and fail the third.
    await expect(off).toHaveAttribute("aria-pressed", "false");
    await expect(on).toHaveAttribute("aria-pressed", "true");

    const [offPaint, onPaint] = await page
      .locator("#group-states > *")
      .evaluateAll((nodes) => nodes.map((node) => getComputedStyle(node).backgroundColor));
    expect(onPaint, "a pressed item paints the same as an unpressed one").not.toBe(offPaint);

    await off.click();
    await expect(off).toHaveAttribute("aria-pressed", "true");
    expect(((await off.getAttribute("class")) ?? "").split(/\s+/)).toContain("btn-active");
  });

  test("the group is one tab stop and the arrow keys move inside it", async ({ page }) => {
    const group = page.locator("#group-horizontal");
    const items = group.locator("> *");

    // Focus is arrived at by tabbing off the panel this example is rendered
    // in, rather than set outright: a `focus()` on the item would make any
    // assertion about the focus order say nothing.
    await example(page, "orientation").focus();
    await page.keyboard.press("Tab");
    await expect(items.nth(0)).toBeFocused();

    await page.keyboard.press("ArrowRight");
    await expect(items.nth(1)).toBeFocused();

    // Right and left move through a row; up and down are the column's, and the
    // group is horizontal here, so they are left alone.
    await page.keyboard.press("ArrowDown");
    await expect(items.nth(1)).toBeFocused();

    await page.keyboard.press("End");
    await expect(items.nth(2)).toBeFocused();

    // One tab stop: tabbing again leaves the group rather than walking through
    // the rest of it.
    await page.keyboard.press("Tab");
    await expect(items.nth(0)).not.toBeFocused();
    await expect(items.nth(1)).not.toBeFocused();
    await expect(items.nth(2)).not.toBeFocused();
  });

  test("a column moves on the up and down arrows instead", async ({ page }) => {
    const items = page.locator("#group-vertical > *");

    await items.nth(0).focus();
    await page.keyboard.press("ArrowDown");
    await expect(items.nth(1)).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(items.nth(0)).toBeFocused();

    // And it is laid out the way it is navigated, which is what the emitted
    // orientation class is for: an unclassed `.join` is a row whatever the
    // keyboard does.
    const [first, second] = await items.evaluateAll((nodes) =>
      nodes.map((node) => node.getBoundingClientRect()),
    );
    expect(second.top, "a vertical group laid itself out as a row").toBeGreaterThanOrEqual(
      first.bottom - 1,
    );
  });

  test("a group that allows one at a time releases the last", async ({ page }) => {
    const pressed = page.getByTestId("pressed");
    const grid = item(page, "Grid");
    const list = item(page, "List");

    await expect(pressed).toHaveText("nothing");

    await grid.click();
    await expect(pressed).toHaveText("0");
    await expect(grid).toHaveAttribute("aria-pressed", "true");

    // The state travelled out through the change callback and back in through
    // the pressed prop, so this says the group is genuinely controlled rather
    // than keeping a copy of its own, and that the released item lost the
    // class along with the state.
    await list.click();
    await expect(pressed).toHaveText("1");
    await expect(list).toHaveAttribute("aria-pressed", "true");
    await expect(grid).toHaveAttribute("aria-pressed", "false");
    expect(((await grid.getAttribute("class")) ?? "").split(/\s+/)).not.toContain("btn-active");
  });

  test("a disabled item is announced disabled, takes no focus, and does not press", async ({
    page,
  }) => {
    const disabled = item(page, "Disabled");

    await expect(disabled).toBeDisabled();
    await expect(disabled).toHaveAttribute("aria-pressed", "false");

    // The roving focus passes over it rather than landing on something inert.
    await item(page, "Off").focus();
    await page.keyboard.press("ArrowRight");
    await expect(item(page, "On")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(disabled).not.toBeFocused();

    await disabled.click({ force: true });
    await expect(disabled).toHaveAttribute("aria-pressed", "false");
  });
});

/** One of the items the page renders, by the name it is announced under. */
function item(page: Page, name: string): Locator {
  return page.getByRole("button", { name, exact: true });
}
