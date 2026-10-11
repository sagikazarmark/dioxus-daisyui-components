import { chromium, type FullConfig } from "@playwright/test";

/**
 * Waits for a wired Preview service to serve a page before any spec runs.
 *
 * `dagger check playwright:test` binds the Preview the preview module serves
 * and hands its address over as PLAYWRIGHT_BASE_URL. The service counts as up
 * once its port accepts connections, which is minutes before the first page
 * finishes loading. The first specs the workers pick up then time out on
 * `page.goto` on every attempt (in CI, accordion's first two), while the same
 * specs pass seconds later. Loading the home page once here, with a budget of
 * its own, warms the service so every spec starts on one that answers.
 *
 * A local run needs none of this: the webServer block in playwright.config.ts
 * already waits for the server it starts.
 */
export default async function globalSetup(config: FullConfig): Promise<void> {
  const base = process.env.PLAYWRIGHT_BASE_URL;
  if (!base) {
    return;
  }

  const deadline = Date.now() + 10 * 60 * 1000;
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    for (;;) {
      try {
        await page.goto(base, { waitUntil: "load", timeout: 60 * 1000 });
        return;
      } catch (error) {
        if (Date.now() > deadline) {
          throw new Error(`the Preview at ${base} did not load within ten minutes`, { cause: error });
        }
        await new Promise((resolve) => setTimeout(resolve, 5 * 1000));
      }
    }
  } finally {
    await browser.close();
  }
}
