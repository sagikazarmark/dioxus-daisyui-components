import { expect, type Page, test } from "@playwright/test";

import { axis, computedStyle, example, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "tag_group" });
});

test.describe("styling", () => {
  test("every colour fills a tag differently", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "color"), "background-color"), "color");
  });

  test("every size renders a tag at a size of its own", async ({ page }) => {
    expectGrows((await computedStyle(axis(page, "size"), "height")).map(parseFloat), "size");
  });

  // daisyUI has no selected badge, so what marks one is a utility written as a
  // variant of the attribute the primitive sets. Both tags in this row are
  // selected, so what varies is the axis rather than the state.
  test("the tag's appearance axis decides whether a selected tag is ringed", async ({ page }) => {
    expectVaries(
      await computedStyle(page.locator('[data-axis="tag"] [role="row"]'), "box-shadow"),
      "tag",
    );
  });

  test("the group's appearance axis decides whether it stacks", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "group"), "display"), "group");
  });

  test("the list's appearance axis decides whether it lays the tags out", async ({ page }) => {
    expectVaries(
      await computedStyle(page.locator('[data-axis="list"] [role="grid"]'), "display"),
      "list",
    );
  });

  test("the empty state's appearance axis decides whether it is muted", async ({ page }) => {
    expectVaries(
      await computedStyle(page.locator('[data-axis="empty"] [role="row"]'), "opacity"),
      "empty",
    );
  });

  test("a caller's classes join the tag's own", async ({ page }) => {
    const tag = page.locator("#caller-attributes");
    const classes = ((await tag.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("badge");
    expect(classes).toContain("badge-primary");
    expect(classes).toContain("rounded-none");

    // And the caller's applies rather than merely surviving the merge, over a
    // corner radius daisyUI's own class sets, which the caller wins on cascade
    // layers rather than on specificity.
    await expect(tag).toHaveCSS("border-radius", "0px");
  });

  test("a caller shows selection as a fill by switching this component's ring off", async ({
    page,
  }) => {
    const tag = page.locator("#caller-selection");

    await expect(tag).toHaveAttribute("data-selected", "true");
    await expect(tag).toHaveCSS("box-shadow", "none");
    const selected = (await computedStyle(tag, "background-color"))[0];

    // Deselecting is what proves the caller's rule is the one doing the work.
    await tag.click();
    await expect(tag).toHaveAttribute("data-selected", "false");
    expect((await computedStyle(tag, "background-color"))[0]).not.toBe(selected);
  });
});

test.describe("behaviour", () => {
  const tag = (page: Page, slug: string, name: string) =>
    example(page, slug).getByRole("row", { name, exact: false });

  test("the group is announced as a grid of rows", async ({ page }) => {
    const grid = example(page, "overview").getByRole("grid");

    await expect(grid).toHaveAttribute("aria-multiselectable", "true");
    await expect(grid.getByRole("row")).toHaveCount(3);

    // Named by the label, which the primitive points `aria-labelledby` at: the
    // reason the label is a part rather than any old text above the tags.
    await expect(grid).toHaveAccessibleName("Labels");
  });

  test("the arrow keys move between tags and Home and End reach the ends", async ({ page }) => {
    const bug = tag(page, "overview", "bug");
    const docs = tag(page, "overview", "docs");
    const first = tag(page, "overview", "good first issue");

    await bug.focus();

    await page.keyboard.press("ArrowRight");
    await expect(docs).toBeFocused();

    await page.keyboard.press("End");
    await expect(first).toBeFocused();

    await page.keyboard.press("Home");
    await expect(bug).toBeFocused();
  });

  test("Enter and Space select, and Escape clears the selection", async ({ page }) => {
    const selected = page.getByTestId("selected");
    const docs = tag(page, "overview", "docs");

    await docs.focus();
    await page.keyboard.press("Enter");
    await expect(docs).toHaveAttribute("aria-selected", "true");
    await expect(selected).toContainText("docs");

    await page.keyboard.press("Escape");
    await expect(docs).toHaveAttribute("aria-selected", "false");
    await expect(selected).toHaveText("nothing");
  });

  test("Delete removes a tag, and the remove button removes the one it is in", async ({ page }) => {
    const overview = example(page, "overview");
    const bug = tag(page, "overview", "bug");

    await bug.focus();
    await page.keyboard.press("Delete");
    await expect(bug).toHaveCount(0);

    // The button is named after the tag it belongs to, so a row of them is not
    // a row of controls all called "remove".
    await overview.getByRole("button", { name: /docs/ }).click();
    await expect(tag(page, "overview", "docs")).toHaveCount(0);
  });

  test("the empty state appears once the last tag is gone", async ({ page }) => {
    const overview = example(page, "overview");
    const empty = overview.getByText("Every label has been removed.");

    await expect(empty).toHaveCount(0);

    for (const name of ["bug", "docs", "good first issue"]) {
      await tag(page, "overview", name).focus();
      await page.keyboard.press("Delete");
    }

    await expect(empty).toBeVisible();
  });

  test("one tag at a time in a single-selection group", async ({ page }) => {
    const priority = page.getByTestId("priority");
    const low = tag(page, "selection", "Low");
    const medium = tag(page, "selection", "Medium");

    await expect(medium).toHaveAttribute("aria-selected", "true");

    await low.click();
    await expect(low).toHaveAttribute("aria-selected", "true");
    await expect(medium).toHaveAttribute("aria-selected", "false");
    await expect(priority).toHaveText("Low");

    // Selecting the selected one again clears it, which is
    // `allow_empty_selection`.
    await low.click();
    await expect(low).toHaveAttribute("aria-selected", "false");
    await expect(priority).toHaveText("none");
  });

  test("a disabled tag is skipped and cannot be selected", async ({ page }) => {
    const medium = tag(page, "selection", "Medium");
    const high = tag(page, "selection", "High");

    await expect(high).toHaveAttribute("aria-disabled", "true");

    // The end of the keyboard order is the tag before it, so the disabled one is
    // passed over rather than landed on and refused.
    await medium.focus();
    await page.keyboard.press("End");
    await expect(high).not.toBeFocused();

    await high.click({ force: true });
    await expect(high).toHaveAttribute("aria-selected", "false");
    await expect(page.getByTestId("priority")).toHaveText("Medium");
  });
});
