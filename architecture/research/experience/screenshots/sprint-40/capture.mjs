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
  await page.waitForTimeout(900);
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

  // Place-first Home: workspace name is the hero identity signal.
  await page.getByRole("heading", { name: "Atelier", exact: true }).waitFor({
    timeout: 15000,
  });
  await goTab(page, "Home");
  await shot(page, "home-desktop");
  await shot(page, "home-place-after");

  await goTab(page, "Save");
  await shot(page, "save-desktop");

  await goTab(page, "Continue");
  await shot(page, "continue-desktop");

  await goTab(page, "Check-in");
  await shot(page, "checkin-desktop");

  await goTab(page, "Guide");
  await shot(page, "guide-desktop");

  await goTab(page, "Home");
  await page.getByRole("heading", { name: "Atelier", exact: true }).waitFor({
    timeout: 10000,
  });
  await page.waitForTimeout(400);

  const identity = await page.evaluate(() => {
    const title = document.querySelector(
      '[data-testid="workspace-home"] .place__identity--place .place__title',
    );
    const pulse = document.querySelector(
      '[data-testid="workspace-home"] .place__identity--place .place__pulse',
    );
    const pageImperative = [...document.querySelectorAll("h1, h2")].some((el) =>
      (el.textContent || "").includes("Continue your work"),
    );
    return {
      title: title?.textContent?.trim() || null,
      pulse: pulse?.textContent?.trim() || null,
      pageImperative,
      homePlace: !!document.querySelector(".home-place"),
    };
  });
  fs.writeFileSync(
    path.join(OUT, "place-first-validation.json"),
    JSON.stringify(identity, null, 2),
  );
  console.log("validation", identity);

  await browser.close();
  if (identity.title !== "Atelier" || identity.pageImperative) {
    process.exitCode = 1;
  }
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
