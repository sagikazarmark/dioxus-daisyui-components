import { expect, type Locator, type Page, test } from "@playwright/test";

import { axisTriggers, computedStyle, expectAxisVaries, openPreview } from "./preview";

declare global {
  interface Window {
    /** What the motion specs record; see `recordMotion`. */
    motion?: string[];
  }
}

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "dialog" });
});

test.describe("styling", () => {
  // The page arrives with the showcase dialog open, which is what these read:
  // a closed dialog renders no elements at all, so there is nothing to take a
  // computed style off until one is open.

  test("every title appearance renders differently", async ({ page }) => {
    await expectAxisVaries(page, "title", "font-size");
  });

  test("every description appearance renders differently", async ({ page }) => {
    await expectAxisVaries(page, "description", "padding-top");
  });

  // Unlike the other axes, this one is not read off a row of rendered
  // elements: every value is a full-viewport modal, so the page renders one
  // trigger per value and this opens them one at a time.
  //
  // What it compares is a handful of properties together rather than one,
  // because no single property tells all six apart: daisyUI expresses a
  // position as where the modal places the box *and* as which of the box's
  // corners it rounds and how far it lets it grow, and the values overlap in
  // each of those taken alone. Every value differing from every other in the
  // composite is the same relational assertion the shared axis helpers make,
  // and for the same reason: daisyUI owns the numbers.
  test("every placement places the box differently", async ({ page }) => {
    await dismissShowcase(page);

    const triggers = axisTriggers(page, "placement");
    const positions = await triggers.count();
    expect(positions, "the placement axis rendered nothing").toBeGreaterThan(1);

    const placements: string[] = [];

    for (let position = 0; position < positions; position++) {
      const box = await openPositioned(page, position);
      placements.push(await placement(box));

      await page.keyboard.press("Escape");
      await expect(box).toBeHidden();
    }

    expect(new Set(placements).size, `two positions render alike: ${placements}`).toBe(
      placements.length,
    );
  });

  test("the parts reach the modal that the collapsed component does not", async ({ page }) => {
    await dismissShowcase(page);

    const box = await openPositioned(page);

    // The positioned dialog is built from the parts, and the page put an id on
    // each of them. Both arrived where they were written, which is the escape
    // hatch that makes the collapsed component's box-only attributes worth
    // offering: an attribute for the modal itself goes on `DialogRoot`.
    await expect(box).toHaveClass(/(^|\s)modal-box(\s|$)/);
    await expect(page.locator("#positioned")).toHaveClass(/(^|\s)modal(\s|$)/);
  });

  test("a caller's attributes land on the box rather than on the modal", async ({ page }) => {
    // The collapsed component renders both elements, and this is the one the
    // caller's id reached: the box, which is also the element the primitive
    // gives the dialog role to.
    const box = showcase(page);
    await expect(box).toHaveRole("dialog");

    const classes = ((await box.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("modal-box");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(box).toHaveCSS("border-radius", "0px");

    // The outer element is the one the caller did not reach, and the one the
    // position axis is for.
    const outer = modal(box);
    await expect(outer).toHaveClass(/(^|\s)modal(\s|$)/);
    expect(((await outer.getAttribute("class")) ?? "").split(/\s+/)).not.toContain("rounded-none");
  });

  test("the modal draws its own dim, with no backdrop element", async ({ page }) => {
    const outer = modal(showcase(page));

    const [dim] = await computedStyle(outer, "background-color");
    expect(dim, "the modal element is painting no dim of its own").not.toBe("rgba(0, 0, 0, 0)");

    // daisyUI has a backdrop element for the CSS-only modal, and this
    // component adds none: the box is the modal's only child, and the dim
    // above is on the modal itself.
    await expect(page.locator(".modal-backdrop")).toHaveCount(0);
    await expect(outer.locator("> *")).toHaveCount(1);
  });
});

test.describe("state", () => {
  test("an uncontrolled dialog opens from its default and closes from the inside", async ({
    page,
  }) => {
    const box = showcase(page);
    const outer = modal(box);

    // Open on arrival, from `default_open` alone. daisyUI hides a modal until
    // its modifier class is on the element and matches nothing the primitive
    // sets, so this is the lifted state observed twice over: the class is
    // there, and the dialog is visible because of it.
    await expect(box).toBeVisible();
    await expect(outer).toHaveClass(/modal-open/);

    // Closing from a control inside the dialog, which is a call into the
    // primitive's context: it reaches the primitive, which is controlled by
    // the styled wrapper, which is what actually takes the class off again.
    await page.getByRole("button", { name: "Close" }).click();
    await expect(box).toBeHidden();
  });

  test("a controlled dialog dismisses through its caller", async ({ page }) => {
    await dismissShowcase(page);

    const changes = page.getByTestId("changes");
    await expect(changes).toHaveText("0");

    // Opening is the page setting its own state, so nothing has travelled out
    // of the dialog yet.
    const box = await openPositioned(page);
    await expect(changes).toHaveText("0");

    await page.keyboard.press("Escape");
    await expect(box).toBeHidden();

    // Escape is the primitive's. It closed a dialog the page controls, so the
    // dismissal travelled out through the change callback and back in through
    // the open prop rather than the primitive keeping a copy of the state.
    await expect(changes).toHaveText("1");
  });
});

test.describe("behaviour", () => {
  test("a dialog is named by its title and described by its description", async ({ page }) => {
    await dismissShowcase(page);

    const box = await openPositioned(page);

    await expect(box).toHaveRole("dialog");
    await expect(box).toHaveAttribute("aria-modal", "true");
    await expect(box).toHaveAccessibleName("Delete the project?");
    await expect(box).toHaveAccessibleDescription(
      "Every deployment goes with it. This cannot be undone.",
    );
  });

  test("focus is trapped while a dialog is open and returned to the trigger", async ({ page }) => {
    await dismissShowcase(page);

    // Focus is arrived at with the keyboard rather than by clicking: not every
    // engine focuses a button that is clicked, and the trigger is the element
    // focus has to come back to.
    const trigger = axisTriggers(page, "placement").first();
    await trigger.focus();
    await page.keyboard.press("Enter");

    const box = positioned(page);
    await expect(box).toBeVisible();

    const cancel = page.getByRole("button", { name: "Cancel" });
    const remove = page.getByRole("button", { name: "Delete" });

    // Opening moved focus into the dialog, onto the first control in it.
    await expect(cancel).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(remove).toBeFocused();

    // Tabbing off the last control comes back to the first rather than walking
    // behind the dialog.
    await page.keyboard.press("Tab");
    await expect(cancel).toBeFocused();

    await page.keyboard.press("Escape");
    await expect(box).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("content behind a modal dialog is inert while it is open", async ({ page }) => {
    await dismissShowcase(page);

    // The trigger sits outside the dialog it opens, which is what makes it
    // the probe: the walk from the dialog to the root marked it, or marked an
    // ancestor of it, and either way it is unreachable through it.
    const trigger = axisTriggers(page, "placement").first();
    const box = await openPositioned(page);

    await expect
      .poll(() => trigger.evaluate((element) => element.closest("[inert]") !== null), {
        message: "nothing between the trigger and the root went inert",
      })
      .toBe(true);

    await page.keyboard.press("Escape");
    await expect(box).toBeHidden();

    // Closing unwound the marks, and through the dialog's own tags rather
    // than wholesale: none are left behind for the next open to trip on.
    await expect
      .poll(() => trigger.evaluate((element) => element.closest("[inert]") !== null), {
        message: "the trigger is still inert after the dialog closed",
      })
      .toBe(false);
    await expect(page.locator("[data-inert-by]")).toHaveCount(0);
  });
});

test.describe("motion", () => {
  // The one place the preview is driven with its animations left running: a
  // transition that has been quiesced to nothing still runs, but there is no
  // longer a window in which it is observably running.
  test.beforeEach(async ({ page }) => {
    await openPreview(page, { component: "dialog" }, "running");
  });

  test("a dialog animates in, and out while it is still mounted", async ({ page }) => {
    await dismissShowcase(page);
    await recordMotion(page);

    const box = await openPositioned(page);

    // Entering. daisyUI transitions the modal's opacity up from a starting
    // style, which needs the modifier class to be on the element as it is
    // inserted, and it is, because the class is emitted from the same render
    // that mounts it.
    //
    // Polled rather than read once: the box is on the page as soon as it is
    // inserted, but a transition out of a starting style does not begin until
    // the style resolution after that, so a single read here catches an engine
    // mid-insertion.
    await expect
      .poll(() => recordedMotion(page), { message: "nothing transitioned on the way in" })
      .toContain("opacity");

    await forgetMotion(page);

    await page.keyboard.press("Escape");
    await expect(box).toBeHidden();

    // Leaving, by the same opacity daisyUI transitions on the way in. A
    // transition only runs on an element that is in the document, so motion on
    // the way out says the modifier class came off while the element was still
    // mounted, which is what the lifted state buys, and what an
    // unmount-on-close would have lost.
    await expect
      .poll(() => recordedMotion(page), { message: "nothing transitioned on the way out" })
      .toContain("opacity");
  });
});

/** The box of the collapsed dialog the page opens with. */
function showcase(page: Page): Locator {
  return page.locator("#showcase");
}

/** The box of the dialog the position triggers open, built from the parts. */
function positioned(page: Page): Locator {
  return page.locator("#positioned-box");
}

/** The modal a box is in, which is the outer of the two elements. */
function modal(box: Locator): Locator {
  return box.locator("..");
}

/** Dismisses the showcase, which every spec that needs the page under it starts with. */
async function dismissShowcase(page: Page): Promise<void> {
  const box = showcase(page);

  await expect(box).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(box).toBeHidden();
}

/** Opens the positioned dialog from one of the position triggers. */
async function openPositioned(page: Page, position = 0): Promise<Locator> {
  await axisTriggers(page, "placement").nth(position).click();

  const box = positioned(page);
  await expect(box).toBeVisible();

  return box;
}

/** Where a position put the box, and what it did to its corners and its size. */
function placement(box: Locator): Promise<string> {
  return box.evaluate((element) => {
    // Placement is the modal's (daisyUI puts the box somewhere by laying out
    // the grid around it) and the rest is the box's own.
    const modal = getComputedStyle(element.parentElement as Element);
    const style = getComputedStyle(element);

    return [
      modal.alignItems,
      modal.justifyItems,
      style.width,
      style.maxHeight,
      style.borderTopLeftRadius,
      style.borderBottomLeftRadius,
    ].join(" ");
  });
}

/**
 * Starts recording the transitions that run on a dialog.
 *
 * Events rather than polling: a transition is observable only while it runs,
 * and `transitionrun` fires as soon as one is created, so this catches motion
 * that a check taken a moment later would have missed, including motion on an
 * element that is on its way out of the document.
 */
async function recordMotion(page: Page): Promise<void> {
  await page.evaluate(() => {
    const running: string[] = [];
    window.motion = running;

    document.addEventListener(
      "transitionrun",
      (event) => {
        if (event.target instanceof Element && event.target.closest(".modal")) {
          running.push(event.propertyName);
        }
      },
      true,
    );
  });
}

/** The properties transitioned since recording started, or since it was forgotten. */
function recordedMotion(page: Page): Promise<string[]> {
  return page.evaluate(() => window.motion ?? []);
}

/** Empties the recording without stopping it. */
async function forgetMotion(page: Page): Promise<void> {
  await page.evaluate(() => {
    window.motion?.splice(0);
  });
}
