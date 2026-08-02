import { chromium } from "playwright";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const OUT = __dirname;
const BASE = "http://127.0.0.1:1420/";

const VIEWPORTS = {
  desktop: { width: 1440, height: 900 },
  ultrawide: { width: 2560, height: 1080 },
};

async function shot(page, name) {
  const file = path.join(OUT, `${name}.png`);
  await page.screenshot({ path: file, fullPage: false });
  console.log("wrote", file);
}

async function run() {
  const browser = await chromium.launch({ channel: "msedge", headless: true });

  for (const [vpName, viewport] of Object.entries(VIEWPORTS)) {
    const context = await browser.newContext({
      viewport,
      colorScheme: "dark",
    });
    const page = await context.newPage();
    await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
    await page.waitForTimeout(1200);
    await page.getByText("Continue your work").first().waitFor({ timeout: 15000 });

    await page.getByRole("tab", { name: "Home", exact: true }).click();
    await page.waitForTimeout(800);
    await shot(page, `home-${vpName}`);

    await page.getByRole("tab", { name: "Continue", exact: true }).click();
    await page.waitForTimeout(700);
    const continueBtn = page
      .locator(".moment-card--hero .exp-btn.primary")
      .filter({ hasText: "Continue" })
      .first();
    if (await continueBtn.count()) {
      await continueBtn.click();
      await page.waitForTimeout(1100);
    }
    await shot(page, `continue-${vpName}`);
    await context.close();
  }

  await browser.close();
  console.log("done");
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
