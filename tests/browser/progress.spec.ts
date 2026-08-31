import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "progress" });
});

test.describe("styling", () => {
  // The colour class lands on the track and sets nothing but `color`, which is
  // exactly why the fill can be the registry's own (ADR-0012): both parts read
  // that one property. Read off the fill rather than off the track, because the
  // fill is the half daisyUI does not draw here and therefore the half a
  // regression would land in.
  test("every colour paints the fill a colour of its own", async ({ page }) => {
    const tracks = axis(page, "color");
    const fills = fillsOf(page, "color");

    expect(await fills.count(), "a track rendered without a fill").toBe(await tracks.count());
    expectVaries(await computedStyle(fills, "background-color"), "color");
  });

  // The appearance axis is the fill itself: the default arm emits the
  // utilities that draw it and `None` emits nothing, which leaves the track
  // bare. What separates them is paint rather than width: an indicator with no
  // classes is still a block element filling its parent, and invisible.
  test("the appearance axis switches the fill off", async ({ page }) => {
    expectVaries(await computedStyle(fillsOf(page, "appearance"), "background-color"), "appearance");
  });

  // The escape hatch the axis exists for: with the registry's utilities off, a
  // caller's own fill draws, sized from the same custom property the primitive
  // publishes for ours.
  test("a caller draws their own fill in place of the component's", async ({ page }) => {
    const fill = page.locator("#caller-fill");

    await expect(fill).not.toHaveCSS("background-image", "none");
    expect(await widthRatio(fill), "the caller's fill did not follow the value").toBeCloseTo(
      0.6,
      1,
    );
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const track = page.locator("#caller-attributes");
    const classes = ((await track.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element, and it is
    // the track rather than the fill that the collapsed component's attributes
    // land on, which is the deviation from the dialog's convention ADR-0012
    // records.
    expect(classes).toContain("progress");
    expect(classes).toContain("progress-accent");
    expect(classes).toContain("h-4");

    // And the caller's applies, rather than merely surviving the merge, over
    // a height and a radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(track).toHaveCSS("height", "16px");
    await expect(track).toHaveCSS("border-radius", "0px");
  });
});

test.describe("behaviour", () => {
  test("the fill follows the value, whatever the maximum", async ({ page }) => {
    expect(await widthRatio(page.locator("#value-empty"))).toBeCloseTo(0, 1);
    expect(await widthRatio(page.locator("#value-part"))).toBeCloseTo(0.4, 1);
    expect(await widthRatio(page.locator("#value-full"))).toBeCloseTo(1, 1);

    // Two out of five is the same fill as forty out of a hundred: the
    // primitive works the percentage out and the fill is sized from it, so
    // there is one place the arithmetic happens.
    expect(await widthRatio(page.locator("#value-scaled"))).toBeCloseTo(0.4, 1);
  });

  test("the value is announced, and moving it moves both halves at once", async ({ page }) => {
    const driven = page.locator("#value-driven");

    await expect(driven).toHaveRole("progressbar");
    await expect(driven).toHaveAttribute("aria-valuenow", "40");
    await expect(driven).toHaveAttribute("aria-valuemax", "100");
    expect(await widthRatio(driven)).toBeCloseTo(0.4, 1);

    await page.getByRole("button", { name: "Advance" }).click();

    // What a screen reader is told and what a reader sees, from one value: a
    // fill that had been given its own copy of the state could pass one of
    // these and fail the other.
    await expect(page.getByTestId("value")).toHaveText("60");
    await expect(driven).toHaveAttribute("aria-valuenow", "60");
    expect(await widthRatio(driven)).toBeCloseTo(0.6, 1);
  });

  test("an indeterminate bar announces itself and draws no fill", async ({ page }) => {
    const indeterminate = page.locator("#value-indeterminate");

    // A bar that is running but cannot say how far along it is: the value is
    // absent from the accessibility tree rather than zero, which is what a
    // screen reader announces as indeterminate.
    await expect(indeterminate).toHaveRole("progressbar");
    await expect(indeterminate).not.toHaveAttribute("aria-valuenow");
    await expect(indeterminate).toHaveAttribute("data-state", "indeterminate");

    // daisyUI's own indeterminate styling is a native pseudo-class and cannot
    // reach this markup, so the track renders bare. What this asserts is the
    // half that would otherwise go wrong quietly: no value means no fill,
    // rather than a width that fell back to the whole track.
    expect(await widthRatio(indeterminate)).toBeCloseTo(0, 1);
  });
});

/** The fills the preview rendered for one axis, in variant-list order. */
function fillsOf(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > *`);
}

/** How much of a track its fill covers, as a fraction. */
async function widthRatio(bar: Locator): Promise<number> {
  // The fill is measured against the track it is inside, so a caller's own
  // width on the track (or a viewport of another size) cannot move the
  // number this returns. The locator is either of the two: a bar addressed by
  // id is the track, and one addressed as a fill is measured against its
  // parent.
  return bar.evaluate((node) => {
    const track = node.classList.contains("progress") ? node : (node.parentElement ?? node);
    const fill = track === node ? node.firstElementChild : node;
    if (!fill) throw new Error("the bar rendered no fill");

    return fill.getBoundingClientRect().width / track.getBoundingClientRect().width;
  });
}
