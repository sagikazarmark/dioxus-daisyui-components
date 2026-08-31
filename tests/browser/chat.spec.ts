import { expect, type Locator, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "chat" });
});

test.describe("structure and placement", () => {
  test("both placements keep all four message parts as direct children", async ({
    page,
  }) => {
    const chats = axis(page, "placement");
    await expect(chats).toHaveCount(2);

    const expectedParts = ["chat-image", "chat-header", "chat-bubble", "chat-footer"];
    for (const [index, placement] of ["start", "end"].entries()) {
      const chat = chats.nth(index);
      await expect(classesOf(chat)).resolves.toEqual(["chat", `chat-${placement}`]);
      await expect(chat.locator(":scope > *")).toHaveCount(4);
      await expect(
        chat.locator(":scope > *").evaluateAll((parts) =>
          parts.map((part) =>
            Array.from(part.classList).find((className) => className.startsWith("chat-")),
          ),
        ),
      ).resolves.toEqual(expectedParts);
    }

    const [start, end] = await chats.evaluateAll((nodes) =>
      nodes.map((node) => {
        const chat = node.getBoundingClientRect();
        const bubble = node.querySelector(":scope > .chat-bubble")?.getBoundingClientRect();
        if (bubble === undefined) return NaN;
        return (bubble.left + bubble.width / 2 - chat.left) / chat.width;
      }),
    );
    expect(start).toBeLessThan(0.5);
    expect(end).toBeGreaterThan(0.5);
  });

  test("the bubble tail points toward the logical placement under LTR and RTL", async ({
    page,
  }) => {
    const cases = [
      ["#ltr-start", 1, true],
      ["#ltr-end", -1, false],
      ["#rtl-start", -1, true],
      ["#rtl-end", 1, false],
    ] as const;

    for (const [selector, horizontalScale, startsOutside] of cases) {
      const bubble = page.locator(`${selector} > .chat-bubble`);
      const tail = await bubble.evaluate((node) => {
        const style = getComputedStyle(node, "::before");
        const transform = style.transform === "none" ? new DOMMatrixReadOnly() : new DOMMatrixReadOnly(style.transform);
        return {
          horizontalScale: Math.round(transform.m11),
          inlineStart: parseFloat(style.getPropertyValue("inset-inline-start")),
          maskImage: style.getPropertyValue("mask-image"),
        };
      });

      expect(tail.maskImage).not.toBe("none");
      expect(tail.horizontalScale).toBe(horizontalScale);
      if (startsOutside) expect(tail.inlineStart).toBeLessThan(0);
      else expect(tail.inlineStart).toBeGreaterThan(0);
    }
  });
});

test.describe("bubble colours", () => {
  test("the unclassed default and all eight modifiers paint differently", async ({
    page,
  }) => {
    const bubbles = axis(page, "color").locator(":scope > .chat-bubble");
    const expectedModifiers = [
      undefined,
      "chat-bubble-neutral",
      "chat-bubble-primary",
      "chat-bubble-secondary",
      "chat-bubble-accent",
      "chat-bubble-info",
      "chat-bubble-success",
      "chat-bubble-warning",
      "chat-bubble-error",
    ];
    await expect(bubbles).toHaveCount(expectedModifiers.length);

    for (const [index, modifier] of expectedModifiers.entries()) {
      const classes = await classesOf(bubbles.nth(index));
      expect(classes).toEqual(modifier === undefined ? ["chat-bubble"] : ["chat-bubble", modifier]);
    }

    expectVaries(await computedStyle(bubbles, "background-color"), "bubble color");
  });
});

test("caller classes and attributes merge on all five parts", async ({ page }) => {
  const expectedClasses = new Map([
    ["chat", ["chat", "chat-end", "rounded-box", "bg-base-200"]],
    ["image", ["chat-image", "avatar", "opacity-75"]],
    ["header", ["chat-header", "font-semibold"]],
    [
      "bubble",
      ["chat-bubble", "chat-bubble-accent", "rounded-none", "italic"],
    ],
    ["footer", ["chat-footer", "opacity-60"]],
  ]);

  for (const [part, expected] of expectedClasses) {
    const element = page.locator(`#caller-${part}`);
    expect(await classesOf(element)).toEqual(expect.arrayContaining(expected));
    await expect(element).toHaveAttribute("data-owner", `caller-${part}`);
  }

  await expect(page.locator("#caller-bubble")).toHaveAttribute(
    "aria-label",
    "Caller-owned bubble",
  );
  await expect(page.locator("#caller-bubble")).toHaveCSS("border-radius", "0px");
});

async function classesOf(locator: Locator): Promise<string[]> {
  return ((await locator.getAttribute("class")) ?? "").split(/\s+/).filter(Boolean);
}
