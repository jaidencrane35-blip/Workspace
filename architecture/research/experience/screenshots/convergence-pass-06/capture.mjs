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

async function a11ySmoke(page, label) {
  const issues = await page.evaluate(() => {
    const out = [];
    const tabs = document.querySelectorAll('[role="tab"]');
    if (tabs.length < 5) out.push(`expected ≥5 tabs, got ${tabs.length}`);
    tabs.forEach((tab) => {
      const name = (tab.getAttribute("aria-label") || tab.textContent || "").trim();
      if (!name) out.push("tab missing accessible name");
    });
    const mainButtons = [...document.querySelectorAll("button")].filter(
      (b) => b.offsetParent !== null,
    );
    for (const b of mainButtons.slice(0, 50)) {
      const name = (b.getAttribute("aria-label") || b.textContent || "").trim();
      if (!name) out.push("visible button missing name");
    }
    if (!document.querySelector("h1, h2")) out.push("no heading");
    return out;
  });
  console.log(`a11y:${label}`, issues.length ? issues : "ok");
  return issues;
}

async function run() {
  fs.mkdirSync(OUT, { recursive: true });
  const browser = await chromium.launch({ channel: "msedge", headless: true });
  const allIssues = [];
  const context = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    colorScheme: "dark",
  });
  const page = await context.newPage();
  await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
  await page.waitForTimeout(1200);
  await page.getByText("Continue your work").first().waitFor({ timeout: 15000 });

  await goTab(page, "Home");
  allIssues.push(...(await a11ySmoke(page, "home")));
  await shot(page, "home-desktop");

  await goTab(page, "Continue");
  // Browse first (Inspect recessed) then expand preview
  await page.waitForTimeout(400);
  await shot(page, "continue-browse-desktop");
  const continueBtn = page
    .locator(".moment-card--hero .exp-btn.primary")
    .filter({ hasText: "Continue" })
    .first();
  if (await continueBtn.count()) {
    await continueBtn.click();
    await page.waitForTimeout(1100);
  }
  allIssues.push(...(await a11ySmoke(page, "continue")));
  await shot(page, "continue-desktop");

  await goTab(page, "Check-in");
  await page.getByText("How’s the return feeling?").first().waitFor({ timeout: 10000 });
  await page.waitForTimeout(600);
  allIssues.push(...(await a11ySmoke(page, "checkin")));
  await shot(page, "checkin-desktop");

  await context.close();
  await browser.close();
  const unique = [...new Set(allIssues)];
  if (unique.length) {
    console.error("a11y smoke issues:", unique);
    process.exitCode = 2;
  } else {
    console.log("a11y smoke: pass");
  }
  console.log("done");
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
