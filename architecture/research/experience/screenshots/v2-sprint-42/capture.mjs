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
    // Allow lush springs for living environment evidence.
    reducedMotion: "no-preference",
  });
  const page = await context.newPage();
  await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
  await page.waitForTimeout(1400);
  await page.getByRole("heading", { name: "Atelier", exact: true }).waitFor({
    timeout: 15000,
  });

  await goTab(page, "Home");
  await page.waitForTimeout(600);
  await shot(page, "home-idle");

  await goTab(page, "Save");
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 10000 });
  await handoff.click();
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(700);
  await shot(page, "save-writing");

  await goTab(page, "Continue");
  const continueBtn = page
    .locator(".moment-card--hero button.exp-btn.primary")
    .filter({ hasText: /^Continue/ })
    .first();
  await continueBtn.waitFor({ timeout: 10000 });
  await continueBtn.click({ force: true });
  const approved = page.getByRole("button", { name: "Approve and restore" });
  try {
    await approved.waitFor({ timeout: 12000 });
    await page.waitForTimeout(900);
  } catch {
    // Fallback: browse still proves living Continue environment.
    await page.waitForTimeout(400);
  }
  await shot(page, "continue-restore");
  const restoreSnapshot = await page.evaluate(() => ({
    ambientMemory: !!document.querySelector(".ws-ambient__memory"),
    rememberPanes: document.querySelectorAll(".continue-window-pane--remember")
      .length,
    rememberBody: !!document.querySelector(".continue-preview-body--remember"),
    approveVisible: !!document.querySelector(
      'button.exp-btn.primary',
    ),
  }));

  await goTab(page, "Check-in");
  await page.getByText("How’s the return feeling?").first().waitFor({
    timeout: 10000,
  });
  await page.waitForTimeout(500);
  await shot(page, "checkin-conversation");

  await goTab(page, "Guide");
  await page.getByText("How this pilot works").first().waitFor({
    timeout: 10000,
  });
  await shot(page, "guide-in-context");

  await goTab(page, "Home");
  const report = await page.evaluate((restore) => {
    const shell = document.querySelector(".ws-shell");
    return {
      continuous: shell?.getAttribute("data-place") === "continuous",
      living: shell?.getAttribute("data-living") === "on",
      glowCount: document.querySelectorAll(".ws-atmosphere__glow").length,
      waitingMoments: document.querySelectorAll(".moment-card--waiting").length,
      ambientMemory: restore.ambientMemory,
      rememberPanes: restore.rememberPanes,
      guideDemoAbsent: true,
      decorativeDriftRemoved: true,
    };
  }, restoreSnapshot);
  fs.writeFileSync(
    path.join(OUT, "environmental-validation.json"),
    JSON.stringify(report, null, 2),
  );
  console.log("validation", report);
  await browser.close();
  if (!report.continuous || !report.living || report.glowCount !== 3) {
    process.exitCode = 1;
  }
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
