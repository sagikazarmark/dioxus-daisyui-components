import { expect, type Locator, type Page, test } from "@playwright/test";

import {
  computedStyle,
  expectGrows,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "select" });
});

test.describe("structure", () => {
  // The popup is the dropdown's structure borrowed whole (ADR-0005), so most of
  // what is below is the dropdown's rules read against a listbox, plus the two
  // things that are this component's own: a trigger that looks like a daisyUI
  // field, and a list that is in the document whether the popup is open or not.

  test("the trigger takes daisyUI's select class, caret and all", async ({
    page,
  }) => {
    const field = trigger(page);

    await expect(field).toHaveClass(/(^|\s)select(\s|$)/);

    // daisyUI paints the caret as a background image and pads the inline end to
    // leave room for it, so a trigger that carries the class carries both.
    const [caret] = await computedStyle(field, "background-image");
    expect(caret, "the trigger draws no caret").not.toBe("none");

    const [start] = await computedStyle(field, "padding-inline-start");
    const [end] = await computedStyle(field, "padding-inline-end");
    expect(
      parseFloat(end),
      `the caret has no room beside the value: ${start} then ${end}`,
    ).toBeGreaterThan(parseFloat(start));
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

    // The list and the list items are out of the accessibility tree altogether,
    // so what the listbox owns is its groups and its options, which is the
    // ownership a screen reader announces option counts and positions from.
    await expect(box.getByRole("list")).toHaveCount(0);
    await expect(box.getByRole("listitem")).toHaveCount(0);
    await expect(box.getByRole("group")).toHaveCount(2);
    await expect(box.getByRole("option")).toHaveCount(4);
  });

  test("a group label takes daisyUI's menu title class, on the list item", async ({
    page,
  }) => {
    const box = openBox(page);
    const wrapper = box.getByText("Citrus", { exact: true }).locator("..");

    // On the wrapper rather than on the element the caller writes into, because
    // that is the element daisyUI wrote the class for, and because `.menu`'s
    // row rules exclude a list item carrying it, which is what keeps a label
    // from being padded and highlighted like an option.
    expect(await wrapper.evaluate((node) => node.tagName.toLowerCase())).toBe(
      "li",
    );
    await expect(wrapper).toHaveClass(/(^|\s)menu-title(\s|$)/);

    const [title] = await computedStyle(wrapper, "color");
    const [option] = await computedStyle(
      box.getByRole("option").first(),
      "color",
    );
    expect(title, "a group label is painted like an option").not.toBe(option);
  });

  test("the list is in the document while the popup is closed, and hidden", async ({
    page,
  }) => {
    // The primitive keeps this part's children mounted while the popup is
    // closed, so that every option registers the text the trigger shows when it
    // is the chosen one. The list therefore cannot be rendered only while the
    // popup is; it is hidden instead, by a utility that fires exactly when the
    // list is not inside the box.
    const select = controlled(page);
    await expect(select.locator(".dropdown-content")).toHaveCount(0);

    const list = select.locator("ul");
    await expect(list).toHaveCount(1);
    const [closed] = await computedStyle(list, "display");
    expect(closed, "the list shows itself while the popup is closed").toBe(
      "none",
    );

    // And it is back the moment the list is inside the box, which is the only
    // thing that utility keys on.
    await trigger(page).click();
    await expect(menu(page)).toBeVisible();
    const [open] = await computedStyle(menu(page).locator("> ul"), "display");
    expect(open, "the list is still hidden inside an open popup").not.toBe(
      "none",
    );
  });

  test("the closed select is one tab stop, list and all", async ({ page }) => {
    // The cost of leaving the list in the document is that it could bring tab
    // stops with it: a list element the primitive gives `tabindex="0"` while it
    // is open, and an option per row. Hidden, none of them is focusable, so a
    // closed select is the trigger and nothing else.
    await trigger(page).focus();
    await page.keyboard.press("Tab");
    await expect(page.locator("#after")).toBeFocused();
  });
});

test.describe("styling", () => {
  // The rows whose axis styles the popup are held open by the page rather than
  // opened here: the primitive closes a popup that nothing in it is focused, so
  // a row of popups standing open side by side is a row whose caller pins them.

  test("every colour paints the trigger differently", async ({ page }) => {
    expectVaries(
      await computedStyle(fields(page, "color"), "border-top-color"),
      "color",
    );
  });

  test("every size sizes the trigger", async ({ page }) => {
    expectGrows(
      (await computedStyle(fields(page, "size"), "height")).map(parseFloat),
      "size",
    );
  });

  test("the placeholder is told apart from a chosen value", async ({
    page,
  }) => {
    // daisyUI fades a native select's own placeholder and has no class for one
    // standing in for it, so the utility is this component's, keyed on the
    // attribute the primitive already sets, and switchable off (ADR-0004).
    const values = fields(page, "placeholder").locator("> span");
    const faded = await computedStyle(values, "opacity");

    expect(faded.length, "the placeholder axis rendered nothing").toBe(2);
    expect(parseFloat(faded[0]), "the placeholder is not faded").toBeLessThan(
      1,
    );
    expect(
      parseFloat(faded[1]),
      "the switched-off placeholder is still faded",
    ).toBe(1);

    // And a value that was chosen is not faded, which is the state the axis
    // exists to tell the placeholder apart from.
    const chosen = page.locator("#selected .select > span");
    await expect(chosen).toHaveAttribute("data-placeholder", "false");
    const [solid] = await computedStyle(chosen, "opacity");
    expect(
      parseFloat(solid),
      "a chosen value is faded like a placeholder",
    ).toBe(1);
  });

  test("every placement opens the popup on a side of its own", async ({
    page,
  }) => {
    expectVaries(await offsets(boxes(page, "side")), "side");
  });

  test("every alignment sits somewhere of its own along that side", async ({
    page,
  }) => {
    expectVaries(await offsets(boxes(page, "align")), "align");
  });

  test("every list size sizes the options rather than the box", async ({
    page,
  }) => {
    // Read off the options, because that is what daisyUI's menu sizes: the box
    // around them has no size of its own, it is whatever the options need. It
    // is an axis apart from the trigger's, because daisyUI's are apart.
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
    expect(fill[0], "the default box paints no fill").not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    expect(fill[1], "the switched-off box still paints a fill").toBe(
      "rgba(0, 0, 0, 0)",
    );

    const radius = await computedStyle(painted, "border-top-left-radius");
    expect(
      parseFloat(radius[0]),
      "the default box rounds no corner",
    ).toBeGreaterThan(0);
    expect(
      parseFloat(radius[1]),
      "the switched-off box still rounds a corner",
    ).toBe(0);

    const shadow = await computedStyle(painted, "box-shadow");
    expect(shadow[0], "the default box casts no shadow").not.toBe("none");
    expect(shadow[1], "the switched-off box still casts a shadow").toBe("none");
  });

  test("the box pads once and stacks without utilities of its own", async ({
    page,
  }) => {
    // The two utilities daisyUI's own dropdown example carries that this
    // component leaves out, asserted as what makes leaving them out right. The
    // dropdown menu pins the same claim about its own box; this one is a box of
    // its own, so the claim is its own too.
    const box = openBox(page);

    // Its `p-2`: the padding is the list's, from inside the box, so the rows are
    // inset once rather than twice.
    const [outer] = await computedStyle(box, "padding-top");
    const [inner] = await computedStyle(box.locator("> ul"), "padding-top");
    expect(outer, "the box pads as well as the list inside it").toBe("0px");
    expect(
      parseFloat(inner),
      "the list pads the rows by nothing",
    ).toBeGreaterThan(0);

    // And its `z-1`: daisyUI's own rule already lifts the box off the page, so a
    // stacking utility here would say what it has said.
    const [stacking] = await computedStyle(box, "z-index");
    expect(stacking, "the box takes no stacking order from daisyUI").not.toBe(
      "auto",
    );
  });

  test("a caller's classes join the box's own, and reach the options", async ({
    page,
  }) => {
    const box = page.locator("#caller-attributes");
    const classes = ((await box.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("dropdown-content");
    expect(classes).toContain("w-52");

    // And the caller's applies. A width is the class worth following through
    // the split: daisyUI carries one on the single element that is both the box
    // and the menu, so a width that stopped at the box here would leave the
    // options shrink-wrapped inside a wider one.
    const [width] = await computedStyle(box, "width");
    const [list] = await computedStyle(box.locator("> ul"), "width");
    expect(
      list,
      `the list did not take the box's width: ${list} of ${width}`,
    ).toBe(width);
  });

  test("the chosen option is marked, and it is the only one that is", async ({
    page,
  }) => {
    // The other half of the lift (ADR-0006). `.menu` matches no ARIA attribute
    // at all: not the `aria-selected` the primitive sets on the option, not
    // anything else, so without the class the chosen row would be identical to
    // every other one.
    const box = page.locator("#selected .dropdown-content");
    const options = box.getByRole("option");

    const marked = await options.evaluateAll((nodes) =>
      nodes
        .filter((node) => node.classList.contains("menu-active"))
        .map((node) => node.textContent),
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

  test("an option highlights on hover and on keyboard focus, with no class for either", async ({
    page,
  }) => {
    // Opened from the keyboard so that the highlight below is the one a keyboard
    // user gets: daisyUI matches `:focus-visible`, and an engine only matches
    // that on a programmatic focus if the interaction that led to it was a
    // keyboard one.
    await trigger(page).focus();
    await page.keyboard.press("ArrowDown");
    await expect(menu(page)).toBeVisible();

    const orange = option(page, "Orange");
    const lemon = option(page, "Lemon");

    await expect(orange).toBeFocused();
    const [focused] = await computedStyle(orange, "background-color");
    const [idle] = await computedStyle(lemon, "background-color");
    expect(idle, "an option is painted before anything reaches it").toBe(
      "rgba(0, 0, 0, 0)",
    );
    expect(focused, "the focused option is not highlighted").not.toBe(idle);

    await lemon.hover();
    const [hovered] = await computedStyle(lemon, "background-color");
    expect(hovered, "the hovered option is not highlighted").not.toBe(idle);

    // Both of those without a class for either state: the highlighted option
    // and the one beside it carry the very same classes.
    expect(await orange.getAttribute("class")).toBe(
      await lemon.getAttribute("class"),
    );
  });

  test("a disabled option is drawn as disabled and answers no pointer", async ({
    page,
  }) => {
    const box = openBox(page);
    const options = box.getByRole("option");

    const colours = await computedStyle(options, "color");
    expect(
      colours.at(-1),
      "the disabled option is painted like the rest",
    ).not.toBe(colours[0]);

    // daisyUI mutes a disabled row from its list item, and the primitive reports
    // the state as ARIA and data attributes daisyUI matches nowhere, so the
    // class is emitted onto the wrapper from Rust.
    await expect(options.last().locator("..")).toHaveClass(
      /(^|\s)menu-disabled(\s|$)/,
    );
    const [pointer] = await computedStyle(
      options.last().locator(".."),
      "pointer-events",
    );
    expect(pointer, "the disabled option still answers the pointer").toBe(
      "none",
    );
  });

  test("a disabled select is drawn as disabled with no class of its own", async ({
    page,
  }) => {
    // Tier 1: the primitive puts the `disabled` attribute on the trigger, and
    // daisyUI's `.select` matches it, so nothing is emitted for this state.
    const field = page.locator("#disabled .select");
    await expect(field).toBeDisabled();

    const enabled = fields(page, "color").first();
    expect(
      await field.getAttribute("class"),
      "the disabled trigger carries a class the enabled one does not",
    ).toBe(await enabled.getAttribute("class"));

    const [muted] = await computedStyle(field, "background-color");
    const [normal] = await computedStyle(enabled, "background-color");
    expect(
      muted,
      "the disabled trigger is painted like an enabled one",
    ).not.toBe(normal);
  });
});

test.describe("state", () => {
  test("the popup is in the document only while it is open", async ({
    page,
  }) => {
    const select = controlled(page);

    // A closed popup renders no box at all: the primitive mounts it when the
    // select opens and unmounts it once the exit transition has run.
    await expect(select.locator(".dropdown-content")).toHaveCount(0);

    await trigger(page).click();
    await expect(select).toHaveClass(/dropdown-open/);
    await expect(menu(page)).toBeVisible();
  });

  test("the modifier class is what shows the popup", async ({ page }) => {
    // Read off a popup the page holds open with nothing in it focused, which is
    // the state the class is load-bearing in: daisyUI hides a dropdown's content
    // outright unless the modifier is on the element above it, and matches no
    // attribute the primitive sets.
    const box = openBox(page);
    await expect(box).toBeVisible();

    // Taken off by hand, the popup goes with it, which is the whole of why this
    // component owns the open state rather than reading it.
    await box.evaluate((element) =>
      element.closest(".dropdown")?.classList.remove("dropdown-open"),
    );
    await expect(box).toBeHidden();
  });

  test("a controlled select opens and dismisses through its caller", async ({
    page,
  }) => {
    const changes = page.getByTestId("changes");
    await expect(changes).toHaveText("0");

    await trigger(page).click();
    await expect(menu(page)).toBeVisible();

    // The select is controlled by the page, so opening travelled out through the
    // change callback and back in through the open prop rather than the
    // primitive keeping a copy of the state.
    await expect(changes).toHaveText("1");

    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(changes).toHaveText("2");
  });
});

test.describe("behaviour", () => {
  test("the trigger, listbox, and options form one repeatable focus scope", async ({
    page,
  }) => {
    const focusExits = page.getByTestId("focus-exits");
    await expect(focusExits).toHaveText("0");

    await trigger(page).focus();
    await trigger(page).click();
    await expect(menu(page)).toBeFocused();
    await expect(focusExits).toHaveText("0");

    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Orange")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Lemon")).toBeFocused();
    await expect(focusExits).toHaveText("0");

    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(focusExits).toHaveText("1");

    await trigger(page).focus();
    await page.keyboard.press("Tab");
    await expect(page.locator("#after")).toBeFocused();
    await expect(focusExits).toHaveText("2");

    await trigger(page).focus();
    await page.keyboard.press("Tab");
    await expect(page.locator("#after")).toBeFocused();
    await expect(focusExits).toHaveText("3");
  });

  test("caller-controlled closure reports focus removed with the popup", async ({
    page,
  }) => {
    const focusExits = page.getByTestId("focus-exits");
    await trigger(page).click();
    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Orange")).toBeFocused();

    await page
      .locator("#close-controlled")
      .evaluate((element: HTMLButtonElement) => element.click());

    await expect(menu(page)).toBeHidden();
    await expect(page.locator("body")).toBeFocused();
    await expect(focusExits).toHaveText("1");
  });

  test("arrow keys move focus through the options, skipping the disabled one", async ({
    page,
  }) => {
    // The trigger's own arrow keys open the popup onto an end of the list, which
    // is the primitive's and is what makes the presentational wrappers free:
    // options register by the index they are given rather than by where they sit
    // in the DOM, and here they sit two elements deep, inside a group.
    await trigger(page).focus();
    await page.keyboard.press("ArrowDown");
    await expect(menu(page)).toBeVisible();
    await expect(option(page, "Orange")).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Lemon")).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Cherry")).toBeFocused();

    // Past the disabled option and around to the first, which is both the skip
    // and the loop.
    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Currant")).not.toBeFocused();
    await expect(option(page, "Orange")).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(option(page, "Cherry")).toBeFocused();
  });

  test("typing jumps to the option that matches, and never to the disabled one", async ({
    page,
  }) => {
    await trigger(page).focus();
    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Orange")).toBeFocused();

    await page.keyboard.press("l");
    await expect(option(page, "Lemon")).toBeFocused();

    // An arrow key clears the buffer, so what is typed next is a search of its
    // own rather than a longer one.
    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Cherry")).toBeFocused();

    // `cu` is the disabled `Currant` and nothing else, and the typeahead only
    // ever considers options that can be chosen, so it lands elsewhere rather
    // than on the one option that spells it.
    await page.keyboard.type("cu");
    await expect(option(page, "Currant")).not.toBeFocused();

    const reachable = await page.evaluate(() =>
      document.activeElement?.getAttribute("aria-disabled"),
    );
    expect(
      reachable,
      "the typeahead landed on something that cannot be chosen",
    ).toBe("false");
  });

  test("choosing an option reports it, shows it on the trigger and closes the popup", async ({
    page,
  }) => {
    const commits = page.getByTestId("commits");
    const focusExits = page.getByTestId("focus-exits");
    await trigger(page).click();
    await option(page, "Cherry").click();

    await expect(menu(page)).toBeHidden();
    await expect(page.getByTestId("selected")).toHaveText("Cherry");
    await expect(commits).toHaveText("1");
    await expect(focusExits).toHaveText("1");

    // The trigger shows the chosen option's text with the popup closed, which is
    // what the list staying in the document is for: the text is registered by
    // the option, and the option is only there because the list is.
    await expect(trigger(page)).toHaveText("Cherry");
    await expect(trigger(page)).toHaveAccessibleName("Cherry");

    // Reopened, the chosen option is the marked one: the class travels from the
    // value this component lifted, through the change callback and back down.
    await trigger(page).click();
    await expect(option(page, "Cherry")).toHaveClass(/(^|\s)menu-active(\s|$)/);
    await expect(option(page, "Orange")).not.toHaveClass(
      /(^|\s)menu-active(\s|$)/,
    );

    // The disabled option chooses nothing, and the popup stays where it was.
    await option(page, "Currant").click({ force: true });
    await expect(page.getByTestId("selected")).toHaveText("Cherry");
  });

  test("the popup is announced as a listbox the trigger expands, with its options grouped", async ({
    page,
  }) => {
    await expect(trigger(page)).toHaveAttribute("aria-haspopup", "listbox");
    await expect(trigger(page)).toHaveAttribute("aria-expanded", "false");
    await expect(trigger(page)).toHaveAttribute(
      "aria-controls",
      "controlled-list",
    );

    await trigger(page).click();
    await expect(trigger(page)).toHaveAttribute("aria-expanded", "true");

    await expect(menu(page)).toHaveRole("listbox");
    await expect(menu(page).getByRole("group").first()).toHaveAccessibleName(
      "Citrus",
    );
    await expect(menu(page).getByRole("option")).toHaveCount(4);
    await expect(option(page, "Currant")).toHaveAttribute(
      "aria-disabled",
      "true",
    );

    await option(page, "Lemon").click();
    await trigger(page).click();
    await expect(option(page, "Lemon")).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(option(page, "Orange")).toHaveAttribute(
      "aria-selected",
      "false",
    );
  });

  test("the popup takes focus when it opens, and gives it to the document when it closes", async ({
    page,
  }) => {
    // Both halves of this are the primitive's, and the second is a gap: the
    // popup goes out of the document with the focus still inside it, so focus
    // lands on the body rather than returning to the trigger. Nothing here
    // restores it, because telling a dismissal apart from a click on something
    // else needs state the primitive holds and this component cannot see.
    //
    // Asserted rather than left to the documentation so that the day upstream
    // restores focus is a day this suite fails, and this spec is the one to
    // rewrite.
    await trigger(page).click();
    await expect(menu(page)).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(option(page, "Orange")).toBeFocused();

    await page.keyboard.press("Escape");
    await expect(menu(page)).toBeHidden();
    await expect(trigger(page)).not.toBeFocused();
    await expect(page.locator("body")).toBeFocused();
    await expect(page.getByTestId("focus-exits")).toHaveText("1");
  });
});

test("resolves Field Context on the trigger with binding, invalid paint, parts, and focus", async ({
  page,
}) => {
  const fieldSelect = page.locator("#field-aware-select");
  const fieldRoot = fieldSelect.locator("..");
  const focusExits = page.getByTestId("field-aware-select-focus-exits");

  await expect(fieldSelect).toHaveAttribute("name", "fruit");
  await expect(fieldSelect).not.toHaveAttribute("required");
  await expect(fieldSelect).not.toHaveAttribute("aria-required");
  await expect(fieldSelect).not.toHaveAttribute("aria-invalid");
  await expect(fieldSelect).toHaveAttribute("aria-describedby", /^\S+ \S+$/);
  await expect(fieldSelect).not.toHaveAttribute("aria-errormessage");
  await expect(fieldSelect).toHaveClass(/\bselect-error\b/);
  await expect(fieldRoot).not.toHaveAttribute("aria-invalid");
  await expect(fieldRoot).not.toHaveAttribute("name");

  await page.locator("#focus-field-aware-select").click();
  await expect(fieldSelect).toBeFocused();
  await expect(focusExits).toHaveText("0");
  await trigger(page).click();
  await expect(focusExits).toHaveText("1");

  await fieldSelect.click();
  await fieldRoot.getByRole("option", { name: "Cherry", exact: true }).click();
  await expect(page.getByTestId("field-aware-select-value")).toHaveText(
    "Current value: Cherry",
  );

  await page.locator("#focus-field-aware-select").click();
  await expect(fieldSelect).toBeFocused();
});

test.fail(
  "field-labelled select exposes its name and its value",
  async ({ page }) => {
    const fieldSelect = page.locator("#field-aware-select");

    await fieldSelect.click();
    await fieldSelect
      .locator("..")
      .getByRole("option", { name: "Cherry", exact: true })
      .click();

    await expect(fieldSelect).toHaveAccessibleName("Fruit");
    await expect(fieldSelect).toMatchAriaSnapshot(`- combobox "Fruit": Cherry`);
  },
);

/** The select the behavioural specs drive, which the page controls. */
function controlled(page: Page): Locator {
  return page.locator("#controlled");
}

/** Its trigger. */
function trigger(page: Page): Locator {
  return page.locator("#controlled-trigger");
}

/** Its popup, which is only in the document while it is open. */
function menu(page: Page): Locator {
  return page.locator("#controlled-list");
}

/** One option of that popup, by the name it is announced under. */
function option(page: Page, name: string): Locator {
  return menu(page).getByRole("option", { name, exact: true });
}

/** The triggers one axis row renders, in the order its variant list is in. */
function fields(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] .select`);
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
 * Where each of a row's popups sits, measured from the select it belongs to.
 *
 * Relative rather than absolute, because the row lays the selects out side by
 * side: what a placement or an alignment decides is where the popup falls
 * against its own trigger, and the absolute position would be the row's doing
 * as much as the axis'.
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
      const select = anchor.getBoundingClientRect();

      return `${Math.round(popup.left - select.left)} ${Math.round(popup.top - select.top)}`;
    }),
  );
}
