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
  await page.waitForTimeout(1100);
}

async function run() {
  fs.mkdirSync(OUT, { recursive: true });
  const browser = await chromium.launch({ channel: "msedge", headless: true });
  const context = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    colorScheme: "dark",
    reducedMotion: "no-preference",
  });
  const page = await context.newPage();
  await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
  await page.waitForTimeout(1400);
  await page.getByRole("heading", { name: "Atelier", exact: true }).waitFor({
    timeout: 15000,
  });

  await goTab(page, "Home");
  await page.waitForTimeout(500);
  await shot(page, "idle-workspace");

  const idle = await page.evaluate(() => {
    const region = document.querySelector(".home-place--invisible");
    const titles = [...document.querySelectorAll(".place__title")].filter(
      (el) => el.offsetParent !== null,
    );
    const kickers = [...document.querySelectorAll(".exp-kicker")].filter(
      (el) => el.offsetParent !== null,
    );
    return {
      invisibleHome: !!region,
      visiblePlaceTitles: titles.length,
      visibleKickers: kickers.length,
      quietAffordance: !!document.querySelector(".moment-card__actions--quiet"),
      primaryButtons: document.querySelectorAll(
        ".ws-place-world .exp-btn.primary",
      ).length,
    };
  });

  await goTab(page, "Save");
  const invite = page.locator(".moment-write__invite");
  try {
    await invite.click({ timeout: 4000 });
  } catch {
    /* tools may already be open */
  }
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 10000 });
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(700);
  await shot(page, "writing");

  const writing = await page.evaluate(() => ({
    toolsOn:
      document.querySelector('.moment-write[data-writing="on"]') != null,
    writeInside: !!document.querySelector(
      ".moment-object--anchor .moment-write",
    ),
  }));

  await goTab(page, "Continue");
  const continueBtn = page
    .locator(".moment-object--anchor button")
    .filter({ hasText: /^Continue/ })
    .first();
  await continueBtn.click({ force: true });
  const approved = page.getByRole("button", { name: "Approve and restore" });
  try {
    await approved.waitFor({ timeout: 12000 });
    await page.waitForTimeout(900);
  } catch {
    await page.waitForTimeout(400);
  }
  await shot(page, "restore");

  const restore = await page.evaluate(() => ({
    previewInside: !!document.querySelector(
      ".moment-object--anchor .continue-preview-body--remember",
    ),
    numericQualityVisible: [...document.querySelectorAll(".continue-preview__quality")]
      .some((el) => el.offsetParent !== null && (el.textContent || "").match(/\d/)),
    rememberChromeAbsent: !document.querySelector(
      'button.exp-btn.primary',
    )?.textContent?.includes("Remember"),
  }));

  await goTab(page, "Check-in");
  await page.locator("#checkin-form").waitFor({ timeout: 10000 });
  await page.waitForTimeout(600);
  await shot(page, "reflection");

  const reflection = await page.evaluate(() => ({
    formInside: !!document.querySelector(
      ".moment-object--anchor #checkin-form",
    ),
    visibleCheckinHeading: [...document.querySelectorAll("#checkin-title")].some(
      (el) => {
        const r = el.getBoundingClientRect();
        return r.width > 2 && r.height > 2;
      },
    ),
    metricsFolded: !!document.querySelector(".checkin-evidence__fold"),
  }));

  await goTab(page, "Guide");
  await page.getByText("How this pilot works").first().waitFor({
    timeout: 10000,
    state: "attached",
  });
  await page.waitForTimeout(500);
  await shot(page, "contextual-guidance");

  const guide = await page.evaluate(() => ({
    hintInside: !!document.querySelector(
      ".moment-object--anchor .moment-guide-hint",
    ),
    chipRailAbsent: !document.querySelector(".guide-experience__rail"),
    visibleGuideTitle: [...document.querySelectorAll(".place__title")].some(
      (el) =>
        el.offsetParent !== null &&
        (el.textContent || "").includes("How this pilot"),
    ),
  }));

  const report = {
    interfaceInvisibility: {
      idleHomeSilent: idle.invisibleHome && idle.visiblePlaceTitles === 0,
      quietAffordance: idle.quietAffordance,
      fewPrimaryButtonsIdle: idle.primaryButtons === 0,
      guideChromeGone: guide.chipRailAbsent && !guide.visibleGuideTitle,
      checkinHeadingHidden: !reflection.visibleCheckinHeading,
    },
    environmentalCommunication: {
      writingInObject: writing.writeInside && writing.toolsOn,
      restoreInObject: restore.previewInside,
      reflectionInObject: reflection.formInside,
      guideInObject: guide.hintInside,
    },
    interactionDiscoverability: {
      quietContinuePresent: idle.quietAffordance,
      writingInviteWorks: writing.toolsOn,
      restoreApprovePresent: restore.previewInside,
    },
    cognitiveLoad: {
      numericQualityHidden: !restore.numericQualityVisible,
      metricsFolded: reflection.metricsFolded,
      rememberButtonGone: restore.rememberChromeAbsent,
    },
    detail: { idle, writing, restore, reflection, guide },
  };

  const flat = [
    ...Object.values(report.interfaceInvisibility),
    ...Object.values(report.environmentalCommunication),
    ...Object.values(report.interactionDiscoverability),
    ...Object.values(report.cognitiveLoad),
  ];
  const pass = flat.every(Boolean);

  fs.writeFileSync(
    path.join(OUT, "interface-visibility-audit.json"),
    JSON.stringify({ ...report, pass }, null, 2),
  );
  console.log("validation", report, "pass", pass);
  await browser.close();
  if (!pass) process.exitCode = 1;
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
