import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "combobox" });
});

test.describe("structure", () => {
  // The popup is the dropdown's structure borrowed whole (ADR-0005) for the
  // fourth time, so most of what is below is the select's rules read against a
  // field that is typed into rather than pressed.

  test("the field is a real input carrying daisyUI's input class", async ({ page }) => {
    const field = input(page);

    expect(await field.evaluate((node) => node.tagName.toLowerCase())).toBe("input");
    await expect(field).toHaveClass(/(^|\s)input(\s|$)/);

    // Which is the whole of what makes this field different from the select's:
    // daisyUI's own rules for a field that is typed into apply as written. The
    // caret cursor is the one of them nothing else would produce; a browser
    // computes `auto` on an input it is not asked about.
    const [cursor] = await computedStyle(field, "cursor");
    expect(cursor, "the field is not drawn as something to type into").toBe("text");
  });

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

    // Every option is the child of a list item, which is the only shape daisyUI
    // reaches a row in: each of its menu rules is written against a literal
    // `li`, and the primitive's option is a `div`.
    const options = box.getByRole("option");
    await expect(options).toHaveCount(4);

    const wrappers = await options.evaluateAll((nodes) =>
      nodes.map((node) => `${node.parentElement?.tagName.toLowerCase()}`),
    );
    expect(wrappers).toEqual(["li", "li", "li", "li"]);
  });

  test("the wrappers are presentational, so the listbox still owns its options", async ({
    page,
  }) => {
    const box = openBox(page);

    await expect(box).toHaveRole("listbox");
    await expect(box.getByRole("list")).toHaveCount(0);
    await expect(box.getByRole("listitem")).toHaveCount(0);
    await expect(box.getByRole("option")).toHaveCount(4);
  });

  test("the list is in the document while the popup is closed, and hidden", async ({ page }) => {
    // The primitive renders this part's children where they stand while the
    // popup is closed, so that every option registers the text the field shows
    // when it is the chosen one. The list therefore cannot be rendered only
    // while the popup is; it is hidden instead, by a utility that fires exactly
    // when the list is not inside the box.
    const combobox = controlled(page);
    await expect(combobox.locator(".dropdown-content")).toHaveCount(0);

    const list = combobox.locator("ul");
    await expect(list).toHaveCount(1);
    const [closed] = await computedStyle(list, "display");
    expect(closed, "the list shows itself while the popup is closed").toBe("none");

    await input(page).click();
    await expect(menu(page)).toBeVisible();
    const [open] = await computedStyle(menu(page).locator("> ul"), "display");
    expect(open, "the list is still hidden inside an open popup").not.toBe("none");
  });

  test("the closed combobox is one tab stop, list and all", async ({ page }) => {
    await input(page).focus();
    await page.keyboard.press("Tab");
    await expect(page.locator("#after")).toBeFocused();
  });
});

test.describe("styling", () => {
  test("every colour paints the field differently", async ({ page }) => {
    expectVaries(await computedStyle(fields(page, "color"), "border-top-color"), "color");
  });

  test("every size sizes the field", async ({ page }) => {
    expectGrows((await computedStyle(fields(page, "size"), "height")).map(parseFloat), "size");
  });

  test("every placement opens the popup on a side of its own", async ({ page }) => {
    expectVaries(await offsets(boxes(page, "side")), "side");
  });

  test("every alignment sits somewhere of its own along that side", async ({ page }) => {
    expectVaries(await offsets(boxes(page, "align")), "align");
  });

  test("every list size sizes the options rather than the box", async ({ page }) => {
    const sizes = await boxes(page, "list-size").evaluateAll((nodes) =>
      nodes.map((node) => {
        const option = node.querySelector('[role="option"]') as Element;
        return parseFloat(getComputedStyle(option).fontSize);
      }),
    );
    expectGrows(sizes, "list-size");
  });

  test("the box appearance can be switched off", async ({ page }) => {
    // daisyUI's `dropdown-content` only positions the element; the fill, the
    // corners and the shadow are utilities this component emits, and a utility
    // it emits is one a caller can only tie with, so switching them off has to
    // be an axis of its own (ADR-0004).
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

  test("a caller's classes join the box's own, and reach the options", async ({ page }) => {
    const box = page.locator("#caller-attributes");
    const classes = ((await box.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("dropdown-content");
    expect(classes).toContain("w-52");

    const [width] = await computedStyle(box, "width");
    const [list] = await computedStyle(box.locator("> ul"), "width");
    expect(list, `the list did not take the box's width: ${list} of ${width}`).toBe(width);
  });

  test("the chosen option is marked, and it is the only one that is", async ({ page }) => {
    // The other half of the lift (ADR-0006). `.menu` matches no ARIA attribute
    // at all, so without the class the chosen row would be identical to every
    // other one.
    const box = page.locator("#selected .dropdown-content");
    const options = box.getByRole("option");

    const marked = await options.evaluateAll((nodes) =>
      nodes.filter((node) => node.classList.contains("menu-active")).map((node) => node.textContent),
    );
    expect(marked).toEqual(["Lemon"]);

    const chosen = box.getByRole("option", { name: "Lemon", exact: true });
    await expect(chosen).toHaveAttribute("aria-selected", "true");

    const [active] = await computedStyle(chosen, "background-color");
    const [idle] = await computedStyle(
      box.getByRole("option", { name: "Orange" }),
      "background-color",
    );
    expect(active, "the chosen option is painted like the rest").not.toBe(idle);
  });

  test("a disabled option is drawn as disabled and answers no pointer", async ({ page }) => {
    const box = openBox(page);
    const options = box.getByRole("option");

    const colours = await computedStyle(options, "color");
    expect(colours.at(-1), "the disabled option is painted like the rest").not.toBe(colours[0]);

    await expect(options.last().locator("..")).toHaveClass(/(^|\s)menu-disabled(\s|$)/);
    const [pointer] = await computedStyle(options.last().locator(".."), "pointer-events");
    expect(pointer, "the disabled option still answers the pointer").toBe("none");
  });

  test("a disabled combobox is drawn as disabled with no class of its own", async ({ page }) => {
    // The primitive puts the native attribute on the field, which is an `input`
    // so daisyUI's own disabled rule matches and nothing is emitted here.
    const field = page.locator("#disabled .input");
    await expect(field).toBeDisabled();

    // Against the enabled field beside it, which the page wrote the same
    // caller classes onto, so what is left to differ is what this component
    // emitted.
    const enabled = page.locator("#selected .input");
    expect(
      await field.getAttribute("class"),
      "the disabled field carries a class the enabled one does not",
    ).toBe(await enabled.getAttribute("class"));

    const [muted] = await computedStyle(field, "background-color");
    const [normal] = await computedStyle(enabled, "background-color");
    expect(muted, "the disabled field is painted like an enabled one").not.toBe(normal);
  });
});

test.describe("the highlight", () => {
  // ADR-0021: focus stays in the field while the list is walked, so daisyUI's
  // own `:focus-visible` rule (the one that highlights a select's options for
  // free) never matches. The paint is a Bridged utility over the attribute the
  // primitive reports the highlight as.

  test("the option the keyboard is on is painted while focus is still in the field", async ({
    page,
  }) => {
    const combobox = highlighted(page, 0);
    const field = combobox.locator(".input");

    await field.click();
    await page.keyboard.press("ArrowDown");

    const first = combobox.getByRole("option", { name: "Orange", exact: true });
    const second = combobox.getByRole("option", { name: "Lemon", exact: true });

    // The keyboard is on the first option and the focus is not: that is the
    // whole of what this component has to paint around.
    await expect(field).toBeFocused();
    await expect(first).toHaveAttribute("data-highlighted", "true");
    await expect(field).toHaveAttribute("aria-activedescendant", (await first.getAttribute("id"))!);

    const [painted] = await computedStyle(first, "background-color");
    const [idle] = await computedStyle(second, "background-color");
    expect(idle, "an option is painted before anything reaches it").toBe("rgba(0, 0, 0, 0)");
    expect(painted, "the highlighted option is not painted").not.toBe(idle);

    // And it follows the keyboard rather than being stuck on the first row.
    await page.keyboard.press("ArrowDown");
    await expect(second).toHaveAttribute("data-highlighted", "true");
    const [moved] = await computedStyle(second, "background-color");
    expect(moved, "the highlight did not follow the keyboard").toBe(painted);
  });

  test("the highlight can be switched off, and hover still works without it", async ({ page }) => {
    const combobox = highlighted(page, 1);
    const field = combobox.locator(".input");
    const first = combobox.getByRole("option", { name: "Orange", exact: true });

    await field.click();
    await page.keyboard.press("ArrowDown");
    await expect(first).toHaveAttribute("data-highlighted", "true");

    const [unpainted] = await computedStyle(first, "background-color");
    expect(unpainted, "the switched-off highlight still paints").toBe("rgba(0, 0, 0, 0)");

    // The hover highlight is daisyUI's own plain `:hover` rule, which this axis
    // has nothing to do with.
    await first.hover();
    const [hovered] = await computedStyle(first, "background-color");
    expect(hovered, "a hovered option is not highlighted").not.toBe(unpainted);
  });
});

test.describe("filtering", () => {
  test("a query keeps the options that match and drops the rest", async ({ page }) => {
    const box = page.locator("#matching .dropdown-content");
    await expect(box.getByRole("option")).toHaveCount(1);
    await expect(box.getByRole("option")).toHaveText("Lemon");
  });

  test("a query that keeps nothing shows the empty line and no rules", async ({ page }) => {
    const box = page.locator("#unmatched .dropdown-content");
    await expect(box.getByRole("option")).toHaveCount(0);
    await expect(box.getByText("No fruit by that name")).toBeVisible();

    // The list items of the dropped options are still in the document and empty,
    // which daisyUI would otherwise draw as a divider rule apiece.
    const drawn = await box.locator("li").evaluateAll((nodes) =>
      nodes.map((node) => getComputedStyle(node).display),
    );
    expect(drawn.filter((display) => display !== "none")).toHaveLength(1);
  });

  test("typing filters the list and reports the query to the caller", async ({ page }) => {
    await input(page).click();
    await expect(menu(page)).toBeVisible();

    await page.keyboard.type("che");
    await expect(page.getByTestId("query")).toHaveText("che");
    await expect(menu(page).getByRole("option")).toHaveCount(1);
    await expect(menu(page).getByRole("option")).toHaveText("Cherry");

    // A query nothing matches leaves the empty line and nothing else.
    await page.keyboard.type("ss");
    await expect(menu(page).getByRole("option")).toHaveCount(0);
    await expect(menu(page).getByText("No fruit by that name")).toBeVisible();
  });
});

test.describe("behaviour", () => {
  test("keyboard selection and Escape stay inside the scope until Tab leaves", async ({ page }) => {
    const commits = page.getByTestId("commits");
    const focusExits = page.getByTestId("focus-exits");

    await input(page).click();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(input(page)).toBeFocused();
    await expect(commits).toHaveText("1");
    await expect(focusExits).toHaveText("0");

    await input(page).click();
    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(input(page)).toBeFocused();
    await expect(commits).toHaveText("1");
    await expect(focusExits).toHaveText("0");

    await page.keyboard.press("Tab");
    await expect(page.locator("#after")).toBeFocused();
    await expect(focusExits).toHaveText("1");
  });

  test("pointer selection stays inside the scope until a click leaves", async ({ page }) => {
    const commits = page.getByTestId("commits");
    const focusExits = page.getByTestId("focus-exits");

    await input(page).click();
    await option(page, "Cherry").click();
    await expect(input(page)).toBeFocused();
    await expect(commits).toHaveText("1");
    await expect(focusExits).toHaveText("0");

    await page.locator("#after").click();
    await expect(page.locator("#after")).toBeFocused();
    await expect(focusExits).toHaveText("1");
  });

  test("arrow keys walk the list without moving focus, skipping the disabled option", async ({
    page,
  }) => {
    await input(page).click();
    await expect(menu(page)).toBeVisible();

    await page.keyboard.press("ArrowDown");
    await expectHighlighted(page, "Orange");

    await page.keyboard.press("ArrowDown");
    await expectHighlighted(page, "Lemon");

    await page.keyboard.press("ArrowDown");
    await expectHighlighted(page, "Cherry");

    // Past the disabled option and around to the first, which is both the skip
    // and the loop, and the field has had focus the whole way.
    await page.keyboard.press("ArrowDown");
    await expectHighlighted(page, "Orange");
    await expect(input(page)).toBeFocused();
  });

  test("Enter chooses the highlighted option, shows it in the field and closes the popup", async ({
    page,
  }) => {
    await input(page).click();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");

    await expect(menu(page)).toBeHidden();
    await expect(page.getByTestId("selected")).toHaveText("Lemon");
    await expect(input(page)).toHaveValue("Lemon");

    // Reopened, the chosen option is the marked one: the class travels from the
    // value this component lifted, through the change callback and back down.
    await input(page).click();
    await expect(option(page, "Lemon")).toHaveClass(/(^|\s)menu-active(\s|$)/);
    await expect(option(page, "Orange")).not.toHaveClass(/(^|\s)menu-active(\s|$)/);
  });

  test("a click chooses an option, and a disabled one chooses nothing", async ({ page }) => {
    await input(page).click();
    await option(page, "Cherry").click();

    await expect(menu(page)).toBeHidden();
    await expect(page.getByTestId("selected")).toHaveText("Cherry");

    await input(page).click();
    await option(page, "Currant").click({ force: true });
    await expect(page.getByTestId("selected")).toHaveText("Cherry");
  });

  test("Escape closes the popup, and the caller is told each time", async ({ page }) => {
    const changes = page.getByTestId("changes");
    await expect(changes).toHaveText("0");

    await input(page).click();
    await expect(menu(page)).toBeVisible();
    await expect(changes).toHaveText("1");

    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(changes).toHaveText("2");
  });

  test("the popup is announced as a listbox the field expands", async ({ page }) => {
    await expect(input(page)).toHaveRole("combobox");
    await expect(input(page)).toHaveAttribute("aria-haspopup", "listbox");
    await expect(input(page)).toHaveAttribute("aria-expanded", "false");
    await expect(input(page)).toHaveAttribute("aria-controls", "controlled-list");

    await input(page).click();
    await expect(input(page)).toHaveAttribute("aria-expanded", "true");

    await expect(menu(page)).toHaveRole("listbox");
    await expect(menu(page).getByRole("option")).toHaveCount(4);
    await expect(option(page, "Currant")).toHaveAttribute("aria-disabled", "true");

    await option(page, "Lemon").click();
    await input(page).click();
    await expect(option(page, "Lemon")).toHaveAttribute("aria-selected", "true");
    await expect(option(page, "Orange")).toHaveAttribute("aria-selected", "false");
  });
});

test("resolves Field Context binding, metadata, invalid paint, and focus", async ({ page }) => {
  const field = page.locator("#field-aware-combobox");
  const focusExits = page.getByTestId("field-aware-combobox-focus-exits");

  await expect(field).toHaveAccessibleName("Fruit");
  await expect(field).toHaveAttribute("name", "fruit");
  await expect(field).toHaveAttribute("required", "true");
  await expect(field).toHaveAttribute("aria-invalid", "true");
  await expect(field).toHaveAttribute(
    "aria-describedby",
    /^\S+ \S+$/,
  );
  await expect(field).toHaveAttribute("aria-errormessage", /^\S+$/);
  await expect(field).toHaveClass(/\binput-error\b/);

  await page.locator("#focus-field-aware-combobox").click();
  await expect(field).toBeFocused();
  await expect(focusExits).toHaveText("0");
  await input(page).click();
  await expect(focusExits).toHaveText("1");

  await field.click();
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  await expect(page.getByTestId("field-aware-combobox-value")).toHaveText(
    'Current value: Some("Orange")',
  );

  await page.locator("#focus-field-aware-combobox").click();
  await expect(field).toBeFocused();
});

/** The combobox the behavioural specs drive, which the page controls. */
function controlled(page: Page): Locator {
  return page.locator("#controlled");
}

/** Its field. */
function input(page: Page): Locator {
  return page.locator("#controlled-input");
}

/** Its popup, which is only in the document while it is open. */
function menu(page: Page): Locator {
  return page.locator("#controlled-list");
}

/** One option of that popup, by the name it is announced under. */
function option(page: Page, name: string): Locator {
  return menu(page).getByRole("option", { name, exact: true });
}

/** One of the two comboboxes the highlight axis renders, by variant-list order. */
function highlighted(page: Page, index: number): Locator {
  return page.locator('[data-axis="option-appearance"] > *').nth(index);
}

/** The fields one axis row renders, in the order its variant list is in. */
function fields(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] .input`);
}

/** The boxes one axis row holds open, in the order its variant list is in. */
function boxes(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] .dropdown-content`);
}

/**
 * One of the popups the page holds open, any of them, for the specs that need
 * a popup standing still rather than a particular value of an axis.
 */
function openBox(page: Page): Locator {
  return boxes(page, "appearance").first();
}

/**
 * Asserts which option the keyboard is on, which is what a combobox has instead
 * of a focused option: the field keeps focus and names the option through
 * `aria-activedescendant`.
 */
async function expectHighlighted(page: Page, name: string): Promise<void> {
  const target = option(page, name);
  await expect(target).toHaveAttribute("data-highlighted", "true");
  await expect(input(page)).toHaveAttribute("aria-activedescendant", (await target.getAttribute("id"))!);
}

/**
 * Where each of a row's popups sits, measured from the combobox it belongs to.
 *
 * Geometry rather than the computed styles every other axis is read through,
 * because daisyUI expresses a placement as `inset` properties that resolve to
 * `auto` on the sides it leaves alone, so two values that put the popup in
 * quite different places can compute alike.
 */
function offsets(popups: Locator): Promise<string[]> {
  return popups.evaluateAll((nodes) =>
    nodes.map((node) => {
      const anchor = node.closest(".dropdown") as Element;
      const popup = node.getBoundingClientRect();
      const field = anchor.getBoundingClientRect();

      return `${Math.round(popup.left - field.left)} ${Math.round(popup.top - field.top)}`;
    }),
  );
}
