import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, example, expectAxisVaries, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "radio_group" });
});

test.describe("styling", () => {
  // The colour row is rendered with nothing chosen, which is where a whole row
  // of colours can be read at once: daisyUI puts the colour on the border
  // before anything is chosen, and draws the dot with it afterwards, and a
  // group has one chosen item by definition.
  test("every colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "border-color");
  });

  // The size gap, from the other side: this component exposes no size axis
  // because daisyUI puts a radio's size behind `[type=radio]` and the primitive
  // renders a `button`, and what it documents in their place is a caller
  // setting daisyUI's own custom property. This asserts the documented escape
  // hatch works, so that the advice cannot rot silently.
  test("a caller sizes an item through daisyUI's own custom property", async ({ page }) => {
    const [normal, sized] = (
      await computedStyle(page.locator("#caller-attributes, #caller-size"), "height")
    ).map(parseFloat);

    expect(sized, "the caller's size utility did not reach the item").toBeGreaterThan(normal);
  });

  // The group's own axis, which is layout rather than daisyUI: the value that
  // emits utilities lays the items out, and the one that emits nothing leaves
  // the element to whatever the caller put on it.
  test("the appearance axis decides whether the group lays itself out", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "appearance"), "display"), "appearance");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const item = page.locator("#caller-attributes");
    const classes = ((await item.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("radio");
    expect(classes).toContain("radio-primary");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // the fully round corner daisyUI's own class sets, which the caller wins
    // on cascade layers rather than on specificity (ADR-0004).
    await expect(item).toHaveCSS("border-radius", "0px");
  });

  // The layout follows the orientation through a variant on the attribute the
  // primitive sets, rather than through a prop this component reads, so this
  // asserts the two are wired to each other at all.
  test("the layout follows the orientation the primitive reports", async ({ page }) => {
    await expect(page.locator("#vertical")).toHaveAttribute("data-orientation", "vertical");
    await expect(page.locator("#vertical")).toHaveCSS("flex-direction", "column");

    await expect(page.locator("#horizontal")).toHaveAttribute("data-orientation", "horizontal");
    await expect(page.locator("#horizontal")).toHaveCSS("flex-direction", "row");
  });
});

test.describe("behaviour", () => {
  const value = (page: Page) => page.getByTestId("value");
  const changes = (page: Page) => page.getByTestId("changes");
  const commits = (page: Page) => page.getByTestId("radio-group-commits");
  const focusExits = (page: Page) => page.getByTestId("radio-group-focus-exits");

  // Tier 1, observed: the chosen item and an unchosen one carry the very same
  // classes and render differently, because daisyUI matches the ARIA attribute
  // the primitive already sets.
  test("choosing an item styles it without a class of its own", async ({ page }) => {
    const chosen = radio(page, "Standard");
    const unchosen = radio(page, "Priority");

    await expect(chosen).toBeChecked();
    await expect(unchosen).not.toBeChecked();

    expect(await chosen.getAttribute("class")).toBe(await unchosen.getAttribute("class"));

    // The dot, which daisyUI draws in a pseudo-element of the item itself and
    // fills only once the item is chosen.
    const [chosenDot] = await computedStyle(chosen, "background-color", "::before");
    const [unchosenDot] = await computedStyle(unchosen, "background-color", "::before");

    expect(chosenDot, "choosing the item did not fill the dot").not.toBe(unchosenDot);
  });

  test("the group is one tab stop and the arrow keys move the selection", async ({ page }) => {
    const states = example(page, "states");

    // Focus is arrived at by tabbing into the example rather than set on an
    // item outright, which is what makes the tab-stop half of this assertion
    // mean anything: the whole group takes one stop, on the chosen item.
    await states.focus();
    await page.keyboard.press("Tab");
    await expect(radio(page, "Standard")).toBeFocused();

    // Native radio-group behaviour, which the primitive reproduces: an arrow
    // key both moves the focus and chooses what it lands on.
    await page.keyboard.press("ArrowRight");
    await expect(radio(page, "Priority")).toBeFocused();
    await expect(radio(page, "Priority")).toBeChecked();
    await expect(value(page)).toHaveText("priority");
    await expect(changes(page)).toHaveText("1");
    await expect(commits(page)).toHaveText("1");
    await expect(focusExits(page)).toHaveText("0");

    // The disabled item is passed over rather than landed on and refused, and
    // the group loops around to the first item.
    await page.keyboard.press("ArrowRight");
    await expect(radio(page, "Overnight, unavailable here")).not.toBeFocused();
    await expect(radio(page, "Standard")).toBeFocused();
    await expect(value(page)).toHaveText("standard");
    await expect(commits(page)).toHaveText("2");
    await expect(focusExits(page)).toHaveText("0");

    // Tabbing again leaves the group entirely rather than walking its items,
    // which is the other half of one tab stop.
    await page.keyboard.press("Tab");
    await expect(radio(page, "Priority")).not.toBeFocused();
    await expect(radio(page, "Standard")).not.toBeFocused();
    await expect(focusExits(page)).toHaveText("1");
  });

  test("a disabled item is announced disabled and cannot be chosen", async ({ page }) => {
    const disabled = radio(page, "Overnight, unavailable here");
    const formControl = page.locator(
      'input[type="radio"][name="shipping"][value="overnight"]',
    );

    await expect(disabled).toHaveRole("radio");
    await expect(disabled).toBeDisabled();
    await expect(disabled).not.toBeChecked();
    await expect(formControl).toBeDisabled();

    await disabled.click({ force: true });
    await expect(disabled).not.toBeChecked();
    await expect(value(page)).toHaveText("standard");
  });

  test("an unnamed group renders no native form participants", async ({ page }) => {
    await expect(page.locator('#vertical input[type="radio"]')).toHaveCount(0);
  });

  test("the group is announced as a group, named, with its items inside it", async ({ page }) => {
    const group: Locator = page.getByRole("radiogroup", { name: "Shipping" });

    await expect(group).toBeAttached();
    await expect(group.getByRole("radio")).toHaveCount(3);
  });
});

test("resolves Field Context binding, group metadata, item invalid paint, and roving focus", async ({
  page,
}) => {
  const group = page.getByRole("radiogroup", { name: "Notification cadence" });
  const weekly = radio(page, "Weekly");
  const form = page.locator("#field-aware-radio-group-form");
  const formControls = form.locator('input[type="radio"][name="cadence"]');
  const selectedFormControl = form.locator('input[type="radio"][name="cadence"]:checked');

  await expect(group).toHaveAttribute("id", "field-aware-radio-group");
  await expect(group).not.toHaveAttribute("name");
  await expect(group).not.toHaveAttribute("required");
  await expect(group).toHaveAttribute("aria-required", "true");
  await expect(group).toHaveAttribute("aria-invalid", "true");
  await expect(group).toHaveAttribute(
    "aria-describedby",
    /^\S+ \S+$/,
  );
  await expect(group).toHaveAttribute("aria-errormessage", /^\S+$/);
  await expect(weekly).toHaveAttribute("id", "field-aware-radio-weekly");
  await expect(weekly).toHaveAttribute("tabindex", "0");
  await expect(weekly).toHaveClass(/\bradio-error\b/);
  await expect(formControls).toHaveCount(3);
  await expect(selectedFormControl).toHaveValue("weekly");
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ cadence: "weekly" });

  await page.locator("#focus-field-aware-radio-group").click();
  await expect(weekly).toBeFocused();

  await radio(page, "Daily").click();
  await expect(page.getByTestId("field-aware-radio-group-value")).toHaveText(
    "Current value: daily",
  );
  await expect(selectedFormControl).toHaveValue("daily");
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ cadence: "daily" });
});

test("a focus request preserves focus when every item is disabled", async ({ page }) => {
  const current = page.locator("#focus-field-aware-radio-group");
  const group = page.locator("#all-items-disabled-radio-group");

  await enableFocusFixtures(page);
  await observeNextFocusLookup(page);
  await expect(group).not.toHaveAttribute("data-disabled", "true");
  await expect(group.locator('[role="radio"]')).toHaveCount(2);
  await expect(group.locator('[role="radio"]:not(:disabled)')).toHaveCount(0);
  await current.focus();
  await page.locator("#focus-all-items-disabled-radio-group").evaluate((button) => {
    (button as HTMLButtonElement).click();
  });

  await expect(page.locator("html")).toHaveAttribute("data-radio-group-focus-settled", "true");
  await expect(current).toBeFocused();
});

test("a focus request preserves focus when the group is disabled", async ({ page }) => {
  const current = page.locator("#focus-field-aware-radio-group");
  const group = page.locator("#disabled-field-radio-group");

  await enableFocusFixtures(page);
  await expect(group).toHaveAttribute("data-disabled", "true");
  await current.focus();
  await page.locator("#focus-disabled-field-radio-group").evaluate((button) => {
    (button as HTMLButtonElement).click();
  });

  await expect(current).toBeFocused();
});

test("the generated focus locator cannot be replaced by caller attributes", async ({ page }) => {
  const group = page.locator("#reserved-locator-radio-group");
  const item = page.locator("#reserved-locator-radio-item");

  await enableFocusFixtures(page);
  await expect(group).not.toHaveAttribute("data-field-group", "caller-locator");
  await page.locator("#focus-reserved-locator-radio-group").evaluate((button) => {
    (button as HTMLButtonElement).click();
  });

  await expect(item).toBeFocused();
});

/** One of the radio items the page renders, by the name it is announced under. */
function radio(page: Page, name: string) {
  return page.getByRole("radio", { name, exact: true });
}

/** Makes Preview-only focus fixtures interactive for the test that requested them. */
async function enableFocusFixtures(page: Page) {
  await page.locator("#radio-group-focus-fixtures").evaluate((fixtures) => {
    fixtures.removeAttribute("inert");
    fixtures.removeAttribute("aria-hidden");
  });
}

/** Marks completion after the next direct group lookup returns to the eval callback. */
async function observeNextFocusLookup(page: Page) {
  await page.evaluate(() => {
    const querySelector = document.querySelector.bind(document);
    document.querySelector = ((selector: string) => {
      const result = querySelector(selector);
      if (selector.startsWith('[data-field-group="')) {
        queueMicrotask(() => {
          document.documentElement.setAttribute("data-radio-group-focus-settled", "true");
        });
      }
      return result;
    }) as typeof document.querySelector;
  });
}
