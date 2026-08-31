import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "dropdown_menu" });
});

test.describe("structure", () => {
  // These are the specs for ADR-0005 itself: daisyUI puts its content and menu
  // classes on one list element, this component cannot, and every rule below
  // is written against the shape that split leaves behind.

  test("the box carries the content class and the list inside it carries the menu class", async ({
    page,
  }) => {
    const box = openBox(page);

    await expect(box).toHaveClass(/(^|\s)dropdown-content(\s|$)/);
    expect(
      await box.getAttribute("class"),
      "the menu class is on the box rather than on the list",
    ).not.toMatch(/(^|\s)menu(\s|$)/);

    await expect(box.locator("> ul")).toHaveClass(/(^|\s)menu(\s|$)/);

    // Every item is the child of a list item, which is the only shape daisyUI
    // reaches an item in: each of its menu rules is written against a literal
    // `li`, and the primitive's item is a `div`.
    const items = box.getByRole("option");
    await expect(items).toHaveCount(3);

    const wrappers = await items.evaluateAll((nodes) =>
      nodes.map((node) => `${node.parentElement?.tagName.toLowerCase()}`),
    );
    expect(wrappers).toEqual(["li", "li", "li"]);
  });

  test("the wrappers are presentational, so the listbox still owns its options", async ({
    page,
  }) => {
    const box = openBox(page);

    await expect(box).toHaveRole("listbox");

    // The list and the list items are out of the accessibility tree
    // altogether, so what the listbox owns is its options, which is the
    // ownership a screen reader announces option counts and positions from,
    // and the whole reason both wrappers are marked presentational.
    await expect(box.getByRole("list")).toHaveCount(0);
    await expect(box.getByRole("listitem")).toHaveCount(0);
    await expect(box.getByRole("option")).toHaveCount(3);
  });
});

test.describe("styling", () => {
  // The axis rows are held open by the page rather than opened here: the
  // primitive closes a menu that nothing in it is focused, so a row of menus
  // standing open side by side is a row whose caller pins them.

  test("every placement opens the menu on a side of its own", async ({ page }) => {
    expectVaries(await offsets(boxes(page, "side")), "side");
  });

  test("every alignment sits somewhere of its own along that side", async ({ page }) => {
    expectVaries(await offsets(boxes(page, "align")), "align");
  });

  test("every size sizes the items rather than the box", async ({ page }) => {
    // Read off the items, because that is what daisyUI's menu sizes: the box
    // around them has no size of its own, it is whatever the items need.
    const items = page.locator('[data-axis="size"] li:first-child > [role="option"]');
    expectGrows((await computedStyle(items, "font-size")).map(parseFloat), "size");
  });

  test("the box appearance can be switched off", async ({ page }) => {
    // daisyUI's `dropdown-content` only positions the element; the fill, the
    // corners and the shadow are utilities this component emits, and a
    // utility it emits is one a caller can only tie with, so switching them
    // off has to be an axis of its own (ADR-0004).
    const painted = boxes(page, "appearance");

    const fill = await computedStyle(painted, "background-color");
    expect(fill.length, "the appearance axis rendered nothing").toBe(2);
    expect(fill[0], "the default box paints no fill").not.toBe("rgba(0, 0, 0, 0)");
    expect(fill[1], "the switched-off box still paints a fill").toBe("rgba(0, 0, 0, 0)");

    const radius = await computedStyle(painted, "border-top-left-radius");
    expect(parseFloat(radius[0]), "the default box rounds no corner").toBeGreaterThan(0);
    expect(parseFloat(radius[1]), "the switched-off box still rounds a corner").toBe(0);

    const shadow = await computedStyle(painted, "box-shadow");
    expect(shadow[0], "the default box casts no shadow").not.toBe("none");
    expect(shadow[1], "the switched-off box still casts a shadow").toBe("none");
  });

  test("a caller's classes join the box's own, and reach the items", async ({ page }) => {
    const box = page.locator("#caller-attributes");
    const classes = ((await box.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("dropdown-content");
    expect(classes).toContain("w-52");

    // And the caller's applies. A width is the class worth following through
    // the split: daisyUI carries one on the single element that is both the
    // box and the menu, so a width that stopped at the box here would leave
    // the items shrink-wrapped inside a wider one.
    const [width] = await computedStyle(box, "width");
    const [list] = await computedStyle(box.locator("> ul"), "width");
    expect(list, `the menu did not take the box's width: ${list} of ${width}`).toBe(width);
  });

  test("the box pads once and stacks without utilities of its own", async ({ page }) => {
    // The two utilities daisyUI's own dropdown example carries that this
    // component leaves out, asserted as what makes leaving them out right.
    const box = openBox(page);

    // Its `p-2`: the padding is the menu's, from inside the box, so the items
    // are inset once rather than twice. Emitting it here as well would put one
    // padding inside the other.
    const [outer] = await computedStyle(box, "padding-top");
    const [inner] = await computedStyle(box.locator("> ul"), "padding-top");
    expect(outer, "the box pads as well as the menu inside it").toBe("0px");
    expect(parseFloat(inner), "the menu pads the items by nothing").toBeGreaterThan(0);

    // And its `z-1`: daisyUI's own rule already lifts the box off the page, so
    // a stacking utility here would say what it has said.
    const [stacking] = await computedStyle(box, "z-index");
    expect(stacking, "the box takes no stacking order from daisyUI").not.toBe("auto");
  });

  test("the trigger takes daisyUI's button class, and so does one the caller renders", async ({
    page,
  }) => {
    await expect(trigger(page)).toHaveClass(/(^|\s)btn(\s|$)/);

    const [fill] = await computedStyle(trigger(page), "background-color");
    expect(fill, "the trigger is not painted like a button").not.toBe("rgba(0, 0, 0, 0)");

    // The primitive's element override, passed through: the element is the
    // caller's and the label is the caller's markup, because the part's own
    // children never reach it, and daisyUI's class arrives on it anyway,
    // along with everything the trigger needs to work.
    const rendered = page.locator("#as-trigger").getByRole("button");
    await expect(rendered).toHaveClass(/(^|\s)btn(\s|$)/);
    await expect(rendered.locator("span")).toHaveText("Rendered by the caller");

    await rendered.click();
    await expect(page.locator("#as-trigger")).toHaveClass(/dropdown-open/);
  });

  test("an item highlights on hover and on keyboard focus, with no class for either", async ({
    page,
  }) => {
    // Opened from the keyboard so that the highlight below is the one a
    // keyboard user gets: daisyUI matches `:focus-visible`, and an engine only
    // matches that on a programmatic focus if the interaction that led to it
    // was a keyboard one.
    await trigger(page).focus();
    await page.keyboard.press("Enter");

    const edit = item(page, "Edit");
    const duplicate = item(page, "Duplicate");

    const [idle] = await computedStyle(edit, "background-color");
    expect(idle, "an item is painted before anything reaches it").toBe("rgba(0, 0, 0, 0)");

    await page.keyboard.press("ArrowDown");
    await expect(edit).toBeFocused();

    const [focused] = await computedStyle(edit, "background-color");
    expect(focused, "the focused item is not highlighted").not.toBe(idle);

    await duplicate.hover();
    const [hovered] = await computedStyle(duplicate, "background-color");
    expect(hovered, "the hovered item is not highlighted").not.toBe(idle);

    // Both of those without a class for either state: the highlighted item and
    // the one beside it carry the very same classes.
    expect(await edit.getAttribute("class")).toBe(await duplicate.getAttribute("class"));
  });

  test("a disabled item is drawn as disabled and answers no pointer", async ({ page }) => {
    const box = openBox(page);
    const items = box.getByRole("option");

    const colours = await computedStyle(items, "color");
    expect(colours.at(-1), "the disabled item is painted like the rest").not.toBe(colours[0]);

    // daisyUI mutes a disabled item from its list item, and the primitive
    // reports the state as a data attribute daisyUI matches nowhere, so the
    // class is emitted onto the wrapper from Rust, which is the second half of
    // this component's tier 2.
    const [pointer] = await computedStyle(items.last().locator(".."), "pointer-events");
    expect(pointer, "the disabled item still answers the pointer").toBe("none");
  });
});

test.describe("state", () => {
  test("the menu is in the document only while it is open", async ({ page }) => {
    const dropdown = controlled(page);

    // A closed menu renders nothing at all: the primitive mounts the content
    // when it opens and unmounts it once the exit animation has run.
    await expect(dropdown.locator(".dropdown-content")).toHaveCount(0);

    await trigger(page).click();
    await expect(dropdown).toHaveClass(/dropdown-open/);
    await expect(menu(page)).toBeVisible();
  });

  test("the modifier class is what shows the menu", async ({ page }) => {
    // Read off a menu the page holds open with nothing in it focused, which is
    // the state the class is load-bearing in: daisyUI hides a dropdown's
    // content outright unless the modifier is on the element above it, and
    // matches no attribute the primitive sets. Its other route in is
    // `:focus-within`, which covers a menu being operated and nothing else.
    const box = openBox(page);
    await expect(box).toBeVisible();

    // Taken off by hand, the menu goes with it, which is the whole of why
    // this component owns the open state rather than reading it.
    await box.evaluate((element) =>
      element.closest(".dropdown")?.classList.remove("dropdown-open"),
    );
    await expect(box).toBeHidden();
  });

  test("a controlled menu opens and dismisses through its caller", async ({ page }) => {
    const changes = page.getByTestId("changes");
    await expect(changes).toHaveText("0");

    await trigger(page).click();
    await expect(menu(page)).toBeVisible();

    // The menu is controlled by the page, so opening travelled out through the
    // change callback and back in through the open prop rather than the
    // primitive keeping a copy of the state.
    await expect(changes).toHaveText("1");

    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(changes).toHaveText("2");
  });
});

test.describe("behaviour", () => {
  test("arrow keys move focus through the menu, skipping the disabled item", async ({ page }) => {
    await trigger(page).focus();
    await page.keyboard.press("Enter");
    await expect(menu(page)).toBeVisible();

    await page.keyboard.press("ArrowDown");
    await expect(item(page, "Edit")).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(item(page, "Duplicate")).toBeFocused();

    // Past the disabled item and around to the first, which is both the skip
    // and the loop, and which says the presentational wrappers cost the
    // primitive nothing, since items register by the index they are given
    // rather than by where they sit in the DOM.
    await page.keyboard.press("ArrowDown");
    await expect(item(page, "Archive")).not.toBeFocused();
    await expect(item(page, "Edit")).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(item(page, "Duplicate")).toBeFocused();
  });

  test("Escape dismisses the menu, and the trigger keeps the focus it had", async ({ page }) => {
    await trigger(page).focus();
    await page.keyboard.press("Enter");
    await expect(menu(page)).toBeVisible();

    // Dismissed from the trigger, which is where the focus is in the common
    // case (the trigger is what opened it) and where it stays.
    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(trigger(page)).toBeFocused();
  });

  test("focus is not returned to the trigger when the menu is dismissed from inside it", async ({
    page,
  }) => {
    // The gap this records is the primitive's, recorded in the component's
    // documentation as well: the focused item goes out of the document with
    // the menu, and focus goes to the body with it. Nothing here restores it,
    // because telling a dismissal apart from a click on something else needs
    // state the primitive holds and this component cannot see.
    //
    // Asserted rather than left to the documentation so that the day upstream
    // fixes it is a day this suite fails: when it does, this spec is the one
    // to delete, and the one above gains the case it is missing.
    await trigger(page).focus();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await expect(item(page, "Edit")).toBeFocused();

    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(trigger(page)).not.toBeFocused();
    await expect(page.locator("body")).toBeFocused();
  });

  test("selecting an item reports it and closes the menu", async ({ page }) => {
    await trigger(page).click();
    await item(page, "Duplicate").click();

    await expect(menu(page)).toBeHidden();
    await expect(page.getByTestId("selected")).toHaveText("Duplicate");

    // The disabled item selects nothing, and the menu stays where it was.
    await trigger(page).click();
    await item(page, "Archive").click({ force: true });
    await expect(page.getByTestId("selected")).toHaveText("Duplicate");
  });

  test("the menu is announced as a listbox the trigger expands", async ({ page }) => {
    await expect(trigger(page)).toHaveAttribute("aria-haspopup", "listbox");
    await expect(trigger(page)).toHaveAttribute("aria-expanded", "false");

    await trigger(page).click();
    await expect(trigger(page)).toHaveAttribute("aria-expanded", "true");

    // Named by the trigger, holding its options directly, and reporting the
    // one that cannot be chosen. The roles are the primitive's and are left
    // alone despite this component's name (ADR-0005).
    await expect(menu(page)).toHaveRole("listbox");
    await expect(menu(page)).toHaveAccessibleName("Actions");
    await expect(menu(page).getByRole("option")).toHaveCount(3);
    await expect(item(page, "Archive")).toHaveAttribute("data-disabled", "true");
  });
});

/** The dropdown the behavioural specs drive, which the page controls. */
function controlled(page: Page): Locator {
  return page.locator("#controlled");
}

/** Its trigger. */
function trigger(page: Page): Locator {
  return controlled(page).getByRole("button", { name: "Actions" });
}

/** Its menu, which is only in the document while it is open. */
function menu(page: Page): Locator {
  return page.locator("#controlled-menu");
}

/** One item of that menu, by the name it is announced under. */
function item(page: Page, name: string): Locator {
  return menu(page).getByRole("option", { name, exact: true });
}

/** The boxes one axis row holds open, in the order its variant list is in. */
function boxes(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] .dropdown-content`);
}

/**
 * One of the menus the page holds open, any of them, for the specs that need
 * a menu standing still rather than a particular value of an axis.
 */
function openBox(page: Page): Locator {
  return boxes(page, "appearance").first();
}

/**
 * Where each of a row's menus sits, measured from the dropdown it belongs to.
 *
 * Relative rather than absolute, because the row lays the dropdowns out side
 * by side: what a placement or an alignment decides is where the menu falls
 * against its own trigger, and the absolute position would be the row's doing
 * as much as the axis'.
 *
 * Geometry rather than the computed styles every other axis is read through,
 * because daisyUI expresses a placement as `inset` properties that resolve to
 * `auto` on the sides it leaves alone, so two values that put the menu in
 * quite different places can compute alike. Where the menu ended up is the
 * stronger evidence, and it is still evidence that the class applied.
 */
function offsets(menus: Locator): Promise<string[]> {
  return menus.evaluateAll((nodes) =>
    nodes.map((node) => {
      const anchor = node.closest(".dropdown") as Element;
      const menu = node.getBoundingClientRect();
      const dropdown = anchor.getBoundingClientRect();

      return `${Math.round(menu.left - dropdown.left)} ${Math.round(menu.top - dropdown.top)}`;
    }),
  );
}
