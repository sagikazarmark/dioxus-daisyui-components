import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "avatar" });
});

test.describe("styling", () => {
  // daisyUI draws the status dot as a `::before` on the avatar itself, so that
  // is where it is read. The value that draws none is part of the set rather
  // than excluded from it: a dot that stopped being drawn would otherwise look
  // like a pass.
  test("every status draws a dot of its own", async ({ page }) => {
    expectVaries(
      await computedStyle(axis(page, "status"), "background-color", "::before"),
      "status",
    );
  });

  // The frame's appearance axis is the size and the shape, which daisyUI writes
  // as utilities rather than as classes of its own. The frames the preview
  // renders for it differ in both, and width is the one that reads as a number.
  test("the frame's appearance axis sizes it", async ({ page }) => {
    expectVaries(await computedStyle(framesOf(page, "frame"), "width"), "frame");
  });

  test("the fallback's appearance axis paints the placeholder", async ({ page }) => {
    expectVaries(
      await computedStyle(page.locator('[data-axis="fallback"] > * > * > *'), "background-color"),
      "fallback",
    );
  });

  test("a caller's classes join the component's own, on the frame", async ({ page }) => {
    const frame = page.locator("#caller-attributes");
    const classes = ((await frame.getAttribute("class")) ?? "").split(/\s+/);

    // The collapsed component's attributes land on the frame, which is the
    // element daisyUI's avatar is drawn on (ADR-0013), so the component's own
    // size and shape and the caller's ring are on one element.
    expect(classes).toContain("w-16");
    expect(classes).toContain("rounded-full");
    expect(classes).toContain("ring-2");

    // And the caller's applies, rather than merely surviving the merge: a ring
    // is a box shadow, and the frame has none of its own.
    await expect(frame).not.toHaveCSS("box-shadow", "none");

    // And the root above it is still the daisyUI avatar, with nothing of the
    // caller's on it.
    const root = frame.locator("..");
    expect(((await root.getAttribute("class")) ?? "").split(/\s+/)).toContain("avatar");
  });

  // The case the appearance axis exists for: a caller who wants a *smaller*
  // avatar cannot win a tie against the width already on the element, because
  // a tie between two utilities is settled by stylesheet order (ADR-0004).
  // Switching the component's off is what wins it.
  test("a caller sizes the avatar down by switching the component's utilities off", async ({
    page,
  }) => {
    const [normal, sized] = (
      await computedStyle(page.locator("#caller-attributes, #caller-size"), "width")
    ).map(parseFloat);

    expect(sized, "the caller's width did not reach the frame").toBeLessThan(normal);
  });
});

test.describe("behaviour", () => {
  test("an avatar with an image shows it and carries no placeholder class", async ({ page }) => {
    const frame = page.locator("#avatar-loaded");

    await expect(frame.locator("img")).toBeVisible();
    expect(await classesOfRoot(frame)).not.toContain("avatar-placeholder");

    // Named by its label rather than by what is inside it: the primitive gives
    // the root `role="img"`, and an image takes its name from the author.
    await expect(page.getByRole("img", { name: "Loaded" })).toBeVisible();
  });

  // Tier 2, mirrored (ADR-0011): the primitive owns the state (the browser
  // decides it) and the class follows the state out through the change
  // callback. Both ways of ending up without an image are asserted, because
  // they arrive at the same class by different routes: one never had a source,
  // the other had one that failed.
  test("an avatar with no image shows the fallback and carries the placeholder class", async ({
    page,
  }) => {
    const frame = page.locator("#avatar-empty");

    await expect(frame.getByText("GH")).toBeVisible();
    await expect(frame.locator("img")).toHaveCount(0);
    expect(await classesOfRoot(frame)).toContain("avatar-placeholder");
  });

  test("an avatar whose image fails ends up as a placeholder", async ({ page }) => {
    const frame = page.locator("#avatar-broken");

    // The image is unmounted rather than left broken, which is the primitive's
    // doing; the class arriving on the root is this component's.
    await expect(frame.getByText("KJ")).toBeVisible();
    await expect(frame.locator("img")).toHaveCount(0);
    expect(await classesOfRoot(frame)).toContain("avatar-placeholder");
  });

  test("the state a caller is told about is the state the class is emitted from", async ({
    page,
  }) => {
    // The watched avatar loads, so it ends up Loaded, with no placeholder. What
    // this asserts is that the callback a caller passes still fires (the
    // component intercepts it rather than replacing it) and that it reports
    // the same state the class was taken off for.
    await expect(page.getByTestId("state")).toHaveText("Loaded");
    expect(await classesOfRoot(page.locator("#avatar-watched"))).not.toContain(
      "avatar-placeholder",
    );
  });
});

/** The frames the preview rendered for one axis, in variant-list order. */
function framesOf(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > *`);
}

/** The classes on the root above a frame, which is where the state lands. */
async function classesOfRoot(frame: Locator): Promise<string[]> {
  const root = frame.locator("..");
  return ((await root.getAttribute("class")) ?? "").split(/\s+/);
}
