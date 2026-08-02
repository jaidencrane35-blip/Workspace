import { chromium } from "playwright";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const OUT = __dirname;
const BASE = "http://127.0.0.1:1420/";

const VIEWPORTS = {
  desktop: { width: 1440, height: 900 },
  ultrawide: { width: 2560, height: 1080 },
  narrow: { width: 1100, height: 800 },
};

async function shot(page, name) {
  const file = path.join(OUT, `${name}.png`);
  await page.screenshot({ path: file, fullPage: false });
  console.log("wrote", file);
}

async function goView(page, label) {
  await page.getByRole("tab", { name: label, exact: true }).click();
  await page.waitForTimeout(900);
}

async function prepareContinue(page) {
  await goView(page, "Continue");
  const continueBtn = page
    .locator(".moment-card--hero .exp-btn.primary")
    .filter({ hasText: "Continue" })
    .first();
  if (await continueBtn.count()) {
    await continueBtn.click();
    await page.waitForTimeout(1000);
  }
}

async function run() {
  const browser = await chromium.launch({
    channel: "msedge",
    headless: true,
  });

  for (const [vpName, viewport] of Object.entries(VIEWPORTS)) {
    const context = await browser.newContext({
      viewport,
      colorScheme: "dark",
    });
    const page = await context.newPage();
    await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
    await page.waitForTimeout(1200);

    // Ensure demo populated home (workspace name Atelier)
    await page.getByText("Atelier", { exact: false }).first().waitFor({
      timeout: 15000,
    });

    await goView(page, "Home");
    await shot(page, `home-${vpName}`);

    await goView(page, "Save");
    await page.getByText("Leave a note").waitFor({ timeout: 8000 });
    await shot(page, `save-${vpName}`);

    await prepareContinue(page);
    await shot(page, `continue-${vpName}`);

    await goView(page, "Check-in");
    await page.getByText("How’s the return feeling?").waitFor({ timeout: 8000 });
    await shot(page, `checkin-${vpName}`);

    await goView(page, "Guide");
    await page.getByText("How this pilot works").waitFor({ timeout: 8000 });
    await shot(page, `guide-${vpName}`);

    await context.close();
  }

  await browser.close();
  console.log("done");
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
