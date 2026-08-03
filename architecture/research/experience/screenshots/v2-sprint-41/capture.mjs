import { chromium } from "playwright";
import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const OUT = __dirname;
const BASE = "http://127.0.0.1:1420/";

async function shot(page, name) {
  const file = path.join(OUT, `${name}.png`);
  await page.screenshot({ path: file, fullPage: false });
  console.log("wrote", file);
}

async function goTab(page, name) {
  await page.getByRole("tab", { name, exact: true }).click();
  await page.waitForTimeout(1000);
}

async function run() {
  fs.mkdirSync(OUT, { recursive: true });
  const browser = await chromium.launch({ channel: "msedge", headless: true });
  const context = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    colorScheme: "dark",
    reducedMotion: "reduce",
  });
  const page = await context.newPage();
  await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
  await page.waitForTimeout(1200);
  await page.getByRole("heading", { name: "Atelier", exact: true }).waitFor({
    timeout: 15000,
  });

  await goTab(page, "Home");
  await shot(page, "home-desktop");

  await goTab(page, "Save");
  await shot(page, "save-desktop");

  await goTab(page, "Continue");
  const continueBtn = page
    .locator(".moment-card--hero .exp-btn.primary")
    .filter({ hasText: "Continue" })
    .first();
  if (await continueBtn.count()) {
    await continueBtn.click();
    await page.waitForTimeout(1100);
  }
  await shot(page, "continue-desktop");

  await goTab(page, "Check-in");
  await page.getByText("How’s the return feeling?").first().waitFor({
    timeout: 10000,
  });
  await shot(page, "checkin-desktop");

  await goTab(page, "Guide");
  await page.getByText("How this pilot works").first().waitFor({
    timeout: 10000,
  });
  await shot(page, "guide-desktop");

  await goTab(page, "Home");
  const markers = await page.evaluate(() => {
    const shell = document.querySelector(".ws-shell");
    return {
      continuous: shell?.getAttribute("data-place") === "continuous",
      homeRegion: !!document.querySelector(".home-place.ws-region, .ws-region.home-place"),
      ambientMoments: document.querySelectorAll(".moment-card--ambient").length,
      pageImperative: [...document.querySelectorAll("h1, h2")].some((el) =>
        (el.textContent || "").includes("Continue your work"),
      ),
      trustStripHome: !!document.querySelector(
        '[data-testid="workspace-home"] .trust-strip',
      ),
    };
  });
  fs.writeFileSync(
    path.join(OUT, "spatial-validation.json"),
    JSON.stringify(markers, null, 2),
  );
  console.log("validation", markers);
  await browser.close();
  if (!markers.continuous || markers.pageImperative) {
    process.exitCode = 1;
  }
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
