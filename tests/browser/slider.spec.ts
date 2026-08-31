import { expect, type Locator, type Page, test } from "@playwright/test";

import { computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "slider" });
});

test.describe("styling", () => {
  // daisyUI's colour classes set `color` and `--range-thumb`, and every part
  // this component draws takes its colour from one of the two (ADR-0016). The
  // fill is `bg-current`, so it is the one that reads the first of them back.
  test("every colour fills the slider differently", async ({ page }) => {
    expectVaries(await computedStyle(fills(page, "color"), "background-color"), "color");
  });

  test("every colour gives the handle a fill of its own", async ({ page }) => {
    expectVaries(await computedStyle(thumbs(page, "color"), "background-color"), "color");
  });

  // The size axis is one custom property, and daisyUI derives the control's
  // height from it, so the root and the handle grow together, which is the
  // whole of ADR-0016's claim that no second mapping was needed.
  test("every size sizes the handle and the control together", async ({ page }) => {
    expectGrows((await computedStyle(thumbs(page, "size"), "width")).map(parseFloat), "size");
    expectGrows((await computedStyle(roots(page, "size"), "height")).map(parseFloat), "size");
  });

  // Each slider in this row sits in a positioned box of its own, so what the
  // axis decides is read off the roots rather than off the row's children.
  test("the root's appearance axis decides whether it lays the parts out", async ({ page }) => {
    expectVaries(await computedStyle(roots(page, "root"), "position"), "root");
  });

  test("the track's appearance axis decides whether the groove is drawn", async ({ page }) => {
    expectVaries(await computedStyle(tracks(page, "track"), "background-color"), "track");
  });

  test("the range's appearance axis decides whether the fill is drawn", async ({ page }) => {
    expectVaries(await computedStyle(fills(page, "range"), "background-color"), "range");
  });

  test("the thumb's appearance axis decides whether the handle is drawn", async ({ page }) => {
    expectVaries(await computedStyle(thumbs(page, "thumb"), "position"), "thumb");
  });

  test("a caller's classes join the root's own", async ({ page }) => {
    const root = page.locator("#caller-attributes");
    const classes = ((await root.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("range");
    expect(classes).toContain("range-secondary");
    expect(classes).toContain("w-32");

    // And the caller's applies rather than merely surviving the merge: daisyUI's
    // own `.range` sets a width, and a utility beats a component class on
    // cascade layers.
    const width = (await root.boundingBox())?.width ?? 0;
    expect(width).toBeCloseTo(128, 0);
  });

  test("a caller redraws the handle by switching this component's utilities off", async ({
    page,
  }) => {
    const thumb = page.locator("#caller-thumb").getByRole("slider");

    // Two utilities only tie, and a tie is settled by generated-stylesheet order
    // rather than by the class attribute (ADR-0004).
    await expect(thumb).toHaveCSS("border-radius", "0px");
  });

  // The one thing the utilities have to get right that daisyUI would otherwise
  // have done: a native range ends the thumb's travel half a thumb in from each
  // end, and `.range` clips anything that hangs over.
  test("the groove is inset by half a handle, so a handle at the end is whole", async ({
    page,
  }) => {
    const root = page.locator("#stepped");
    const rootBox = await root.boundingBox();
    const track = await root.locator("> *").boundingBox();
    const thumb = await root.getByRole("slider").boundingBox();

    expect(Math.abs(track!.x - rootBox!.x - thumb!.width / 2)).toBeLessThan(1);
    expect(
      Math.abs(rootBox!.x + rootBox!.width - (track!.x + track!.width) - thumb!.width / 2),
    ).toBeLessThan(1);
  });
});

test.describe("behaviour", () => {
  const thumbOf = (page: Page, id: string) => page.locator(id).getByRole("slider");

  test("the handle announces its value against its bounds, and is named", async ({ page }) => {
    const thumb = thumbOf(page, "#controlled");

    await expect(thumb).toHaveAttribute("aria-valuenow", "40");
    await expect(thumb).toHaveAttribute("aria-valuemin", "0");
    await expect(thumb).toHaveAttribute("aria-valuemax", "100");
    await expect(thumb).toHaveAccessibleName("Volume");
  });

  test("the arrow keys move the value, and Shift moves it ten steps", async ({ page }) => {
    const value = page.getByTestId("volume");
    const thumb = thumbOf(page, "#controlled");

    await thumb.focus();

    await page.keyboard.press("ArrowRight");
    await expect(value).toHaveText("41");

    await page.keyboard.press("ArrowLeft");
    await expect(value).toHaveText("40");

    await page.keyboard.press("Shift+ArrowRight");
    await expect(value).toHaveText("50");

    // Controlled: the number the caller holds and the number the handle reports
    // are the same one, because the caller wrote it back.
    await expect(thumb).toHaveAttribute("aria-valuenow", "50");
  });

  test("a key commits on keyup without Focus Exit, then departure exits", async ({ page }) => {
    const thumb = thumbOf(page, "#controlled");
    const changes = page.getByTestId("slider-changes");
    const commits = page.getByTestId("slider-commits");
    const focusExits = page.getByTestId("slider-focus-exits");

    await thumb.focus();
    await page.keyboard.down("ArrowRight");
    await expect(changes).toHaveText("1");
    await expect(commits).toHaveText("0");

    await page.keyboard.up("ArrowRight");
    await expect(commits).toHaveText("1");
    await expect(focusExits).toHaveText("0");

    await thumbOf(page, "#stepped").focus();
    await expect(focusExits).toHaveText("1");
  });

  test("pointer release without a change does not commit", async ({ page }) => {
    await page.locator("#controlled").dispatchEvent("pointerup", { pointerType: "mouse" });
    await page.locator("#span").dispatchEvent("pointerup", { pointerType: "mouse" });

    await expect(page.getByTestId("slider-changes")).toHaveText("0");
    await expect(page.getByTestId("slider-commits")).toHaveText("0");
    await expect(page.getByTestId("range-slider-changes")).toHaveText("0");
    await expect(page.getByTestId("range-slider-commits")).toHaveText("0");
  });

  test("a drag writes while moving and commits on pointer release", async ({ page }) => {
    const thumb = thumbOf(page, "#controlled");
    const focusExits = page.getByTestId("slider-focus-exits");
    const box = await thumb.boundingBox();

    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.move(box!.x + box!.width / 2 + 30, box!.y + box!.height / 2, {
      steps: 5,
    });

    await expect
      .poll(async () => Number(await page.getByTestId("slider-changes").textContent()))
      .toBeGreaterThan(0);
    await expect(page.getByTestId("slider-commits")).toHaveText("0");
    await expect(focusExits).toHaveText("0");

    await page.mouse.up();
    await expect(page.getByTestId("slider-commits")).toHaveText("1");
    await expect(focusExits).toHaveText("0");

    await thumbOf(page, "#stepped").focus();
    await expect(focusExits).toHaveText("1");
  });

  test("an outside release commits exactly once", async ({ page }) => {
    const root = page.locator("#controlled");
    const thumb = thumbOf(page, "#controlled");
    const rootBox = await root.boundingBox();
    const thumbBox = await thumb.boundingBox();

    await page.mouse.move(
      thumbBox!.x + thumbBox!.width / 2,
      thumbBox!.y + thumbBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(
      thumbBox!.x + thumbBox!.width / 2 + 30,
      thumbBox!.y + thumbBox!.height / 2,
      { steps: 5 },
    );
    await expect
      .poll(async () => Number(await page.getByTestId("slider-changes").textContent()))
      .toBeGreaterThan(0);

    await page.mouse.move(rootBox!.x + rootBox!.width + 30, rootBox!.y + rootBox!.height + 30);
    await page.mouse.up();
    await expect(page.getByTestId("slider-commits")).toHaveText("1");

    const currentThumbBox = await thumb.boundingBox();
    await page.mouse.move(
      currentThumbBox!.x + currentThumbBox!.width / 2,
      currentThumbBox!.y + currentThumbBox!.height / 2,
    );
    const changesBeforeNextInteraction = await page.getByTestId("slider-changes").textContent();
    await page.mouse.down({ button: "right" });
    await expect(page.getByTestId("slider-changes")).toHaveText(changesBeforeNextInteraction!);
    await page.mouse.up({ button: "right" });
    await expect(page.getByTestId("slider-commits")).toHaveText("1");
  });

  test("a blocked key does not recommit an outside pointer change", async ({ page }) => {
    const root = page.locator("#controlled");
    const thumb = thumbOf(page, "#controlled");
    const rootBox = await root.boundingBox();
    const thumbBox = await thumb.boundingBox();

    await page.mouse.move(
      thumbBox!.x + thumbBox!.width / 2,
      thumbBox!.y + thumbBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(rootBox!.x + rootBox!.width, rootBox!.y + rootBox!.height / 2, {
      steps: 10,
    });
    await expect(page.getByTestId("volume")).toHaveText("100");
    await page.mouse.move(rootBox!.x + rootBox!.width + 30, rootBox!.y + rootBox!.height + 30);
    await page.mouse.up();
    await expect(page.getByTestId("slider-commits")).toHaveText("1");

    await thumb.focus();
    await page.keyboard.press("ArrowRight");
    await expect(page.getByTestId("slider-commits")).toHaveText("1");
  });

  test("the value moves in whole steps", async ({ page }) => {
    const thumb = thumbOf(page, "#stepped");

    await expect(thumb).toHaveAttribute("aria-valuenow", "30");

    await thumb.focus();
    await page.keyboard.press("ArrowRight");
    await expect(thumb).toHaveAttribute("aria-valuenow", "40");
  });

  test("a press on the control moves the handle to it", async ({ page }) => {
    const root = page.locator("#stepped");
    const thumb = thumbOf(page, "#stepped");
    const box = await root.boundingBox();

    await root.click({ position: { x: (box?.width ?? 0) * 0.8, y: (box?.height ?? 0) / 2 } });

    // The whole control is the pointer surface, which is what a native range
    // does too. The value is stepped by ten, so what is asserted is the region
    // rather than the number.
    const now = Number(await thumb.getAttribute("aria-valuenow"));
    expect(now).toBeGreaterThan(60);
    expect(now).toBeLessThanOrEqual(100);
  });

  test("a disabled slider does not move", async ({ page }) => {
    const root = page.locator("#disabled");
    const thumb = thumbOf(page, "#disabled");

    await expect(root).toHaveAttribute("data-disabled", "true");

    await thumb.focus();
    await page.keyboard.press("ArrowRight");
    await expect(thumb).toHaveAttribute("aria-valuenow", "50");

    await root.click({ position: { x: 10, y: 8 }, force: true });
    await expect(thumb).toHaveAttribute("aria-valuenow", "50");
  });

  test("disabled state updates the announcement and Tab order", async ({ page }) => {
    const thumb = thumbOf(page, "#disabled");
    const toggle = page.locator("#toggle-disabled-slider");
    const next = page.locator('[data-example="field"]').getByRole("tab", { name: "Preview" });

    await expect(thumb).toHaveAttribute("aria-disabled", "true");

    await toggle.focus();
    await page.keyboard.press("Tab");

    await expect(thumb).not.toBeFocused();
    await expect(next).toBeFocused();

    await toggle.click();
    await expect(thumb).not.toHaveAttribute("aria-disabled");

    await toggle.focus();
    await page.keyboard.press("Tab");
    await expect(thumb).toBeFocused();
  });

  test("both handles of a range slider move, and each is bounded by the other", async ({
    page,
  }) => {
    const start = page.locator("#span").getByRole("slider").first();
    const end = page.locator("#span").getByRole("slider").last();

    await expect(page.getByTestId("span-start")).toHaveText("20");
    await expect(page.getByTestId("span-end")).toHaveText("70");

    await start.focus();
    await page.keyboard.press("ArrowRight");
    await expect(page.getByTestId("span-start")).toHaveText("21");

    // The start cannot walk past the end: ten shift-steps from 21 would be 121,
    // and what it reports is the end it ran into.
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Shift+ArrowRight");
    await expect(page.getByTestId("span-start")).toHaveText("70");

    await end.focus();
    await page.keyboard.press("ArrowRight");
    await expect(page.getByTestId("span-end")).toHaveText("71");
  });

  test("a range slider commits when its active key is released", async ({ page }) => {
    const start = page.locator("#span").getByRole("slider").first();

    await start.focus();
    await page.keyboard.down("ArrowRight");
    await expect(page.getByTestId("range-slider-changes")).toHaveText("1");
    await expect(page.getByTestId("range-slider-commits")).toHaveText("0");

    await page.keyboard.up("ArrowRight");
    await expect(page.getByTestId("range-slider-commits")).toHaveText("1");
    await expect(page.getByTestId("range-slider-focus-exits")).toHaveText("0");
  });

  test("range thumb-to-thumb Tab stays inside before leaving once", async ({ page }) => {
    const start = page.locator("#span").getByRole("slider").first();
    const end = page.locator("#span").getByRole("slider").last();
    const focusExits = page.getByTestId("range-slider-focus-exits");

    await start.focus();
    await page.keyboard.press("Tab");

    await expect(end).toBeFocused();
    await expect(focusExits).toHaveText("0");

    await page.keyboard.press("Tab");
    await expect(page.locator("#toggle-disabled-slider")).toBeFocused();
    await expect(focusExits).toHaveText("1");
  });

  test("a blocked range key neither writes, commits, nor moves focus", async ({ page }) => {
    const start = page.locator("#span").getByRole("slider").first();
    const changes = page.getByTestId("range-slider-changes");
    const commits = page.getByTestId("range-slider-commits");

    await start.focus();
    await page.keyboard.press("Shift+ArrowLeft");
    await page.keyboard.press("Shift+ArrowLeft");
    await expect(changes).toHaveText("2");
    await expect(commits).toHaveText("2");
    await expect(start).toBeFocused();

    await page.keyboard.press("Shift+ArrowLeft");
    await expect(changes).toHaveText("2");
    await expect(commits).toHaveText("2");
    await expect(start).toBeFocused();
  });

  test("a range pointer press writes and commits on release", async ({ page }) => {
    const root = page.locator("#span");
    const box = await root.boundingBox();

    await root.click({
      position: { x: box!.width * 0.9, y: box!.height / 2 },
    });

    await expect(page.getByTestId("range-slider-changes")).toHaveText("1");
    await expect(page.getByTestId("range-slider-commits")).toHaveText("1");
  });
});

test("SliderField composes generated relationships and forwards appearances", async ({ page }) => {
  const sliderRoot = page.getByTestId("field-aware-slider");
  const sliderThumb = sliderRoot.getByRole("slider");
  const parts = await fieldParts(page, sliderRoot);

  await expect(parts.label).toHaveText("Volume");
  await expect(parts.label).toHaveAttribute("for", parts.controlId);
  await expect(parts.description).toHaveText("Set the playback volume.");
  await expect(parts.error).toHaveAttribute("aria-live", "polite");
  await expect(parts.error).toHaveText("Choose a supported volume");
  await expect(sliderThumb).toHaveAccessibleName("Volume");

  await expect(sliderRoot).toHaveClass(/\brange-sm\b/);
  await expect(sliderRoot).toHaveClass(/\brange-error\b/);
  await expect(sliderRoot).toHaveClass(/\brelative\b/);
  await expect(sliderRoot).not.toHaveClass(/data-\[disabled=true\]:cursor-not-allowed/);
  await expect(sliderRoot.locator("xpath=..")).toHaveCSS("display", "block");
  await expect(parts.description).toHaveCSS("white-space", "nowrap");
  await expect(parts.error).not.toHaveClass(/\btext-error\b/);
  const [fieldColor, errorColor] = await sliderRoot.locator("xpath=..").evaluate((field) => {
    const error = field.querySelector('[aria-live="polite"]');
    if (!(error instanceof HTMLElement)) {
      throw new Error("SliderField has no error region");
    }
    return [getComputedStyle(field).color, getComputedStyle(error).color];
  });
  expect(errorColor).toBe(fieldColor);

  await expect(sliderRoot).not.toHaveAttribute("name");
  await expect(sliderRoot).not.toHaveAttribute("required");
  await expect(sliderRoot).toHaveAttribute("aria-required", "true");
  await expect(sliderRoot).not.toHaveAttribute("aria-invalid");
  await expect(sliderRoot).not.toHaveAttribute("aria-errormessage");
  await expect(sliderRoot).toHaveAttribute("data-invalid", "true");

  await sliderThumb.focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.getByTestId("field-aware-slider-value")).toHaveText(
    "Current value: 36",
  );

  await page.locator("#focus-field-aware-slider").click();
  await expect(sliderThumb).toBeFocused();
});

test("RangeSliderField labels both thumbs and preserves Field behaviour", async ({ page }) => {
  const rangeRoot = page.getByTestId("field-aware-range-slider");
  const rangeStart = rangeRoot.getByRole("slider").first();
  const rangeEnd = rangeRoot.getByRole("slider").last();
  const parts = await fieldParts(page, rangeRoot);

  await expect(parts.label).toHaveText("Price range");
  await expect(parts.label).toHaveAttribute("for", parts.controlId);
  await expect(parts.description).toHaveText("Set the acceptable price span.");
  await expect(parts.error).toHaveAttribute("aria-live", "polite");
  await expect(parts.error).toHaveText("Choose a supported price range");
  await expect(rangeStart).toHaveAccessibleName("Price range");
  await expect(rangeEnd).toHaveAccessibleName("Price range");

  await expect(rangeRoot).toHaveClass(/\brange-lg\b/);
  await expect(rangeRoot).toHaveClass(/\brange-error\b/);
  await expect(rangeRoot.locator("xpath=..")).toHaveCSS("display", "grid");
  await expect(parts.description).toHaveCSS("white-space", "normal");
  await expect(parts.error).toHaveClass(/\btext-error\b/);

  await expect(rangeRoot).not.toHaveAttribute("name");
  await expect(rangeRoot).not.toHaveAttribute("required");
  await expect(rangeRoot).toHaveAttribute("aria-required", "true");
  await expect(rangeRoot).not.toHaveAttribute("aria-invalid");
  await expect(rangeRoot).not.toHaveAttribute("aria-errormessage");
  await expect(rangeRoot).toHaveAttribute("data-invalid", "true");

  await rangeStart.focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.getByTestId("field-aware-range-slider-value")).toHaveText(
    "Current range: 21 to 70",
  );

  await page.locator("#focus-field-aware-range-slider").click();
  await expect(rangeStart).toBeFocused();
});

async function fieldParts(page: Page, control: Locator) {
  await expect(control).toHaveAttribute("id", /^\S+$/);
  await expect(control).toHaveAttribute("aria-labelledby", /^\S+$/);
  await expect(control).toHaveAttribute("aria-describedby", /^\S+ \S+$/);

  const controlId = (await control.getAttribute("id"))!;
  const labelId = (await control.getAttribute("aria-labelledby"))!;
  const [descriptionId, errorId] = (await control.getAttribute("aria-describedby"))!.split(/\s+/);

  expect(new Set([controlId, labelId, descriptionId, errorId]).size).toBe(4);

  return {
    controlId,
    label: page.locator(`#${labelId}`),
    description: page.locator(`#${descriptionId}`),
    error: page.locator(`#${errorId}`),
  };
}

/**
 * The roots one axis row rendered, in the order its variant list is in.
 *
 * Reached by descent rather than as direct children, because the row that
 * varies the root's own axis wraps each slider in a positioned box, which is
 * what a caller who switches those utilities off has to supply.
 */
function roots(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] [role="group"]`);
}

/** The grooves inside those roots. */
function tracks(page: Page, name: string): Locator {
  return roots(page, name).locator("> *");
}

/** The fills inside those grooves, which are the elements that are not handles. */
function fills(page: Page, name: string): Locator {
  return roots(page, name).locator('> * > *:not([role="slider"])');
}

/** The handles inside those grooves. */
function thumbs(page: Page, name: string): Locator {
  return roots(page, name).locator('> * > [role="slider"]');
}
