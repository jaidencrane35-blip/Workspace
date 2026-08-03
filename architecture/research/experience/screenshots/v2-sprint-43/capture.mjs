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
  await page.waitForTimeout(600);
  await shot(page, "home");

  const homeAnchor = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const anchor = document.querySelector(".moment-object--anchor");
    return {
      stage: !!stage,
      layoutId: anchor?.getAttribute("data-object-id") || null,
      presence: stage?.getAttribute("data-presence") || null,
    };
  });

  await goTab(page, "Save");
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 10000 });
  await handoff.click();
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(700);
  await shot(page, "save-while-writing");

  const saveAttach = await page.evaluate(() => ({
    host: !!document.querySelector('[data-testid="moment-attach-host"]'),
    writeInside: !!document.querySelector(
      ".moment-object--anchor .moment-write",
    ),
    presence: document
      .querySelector('[data-testid="persistent-moment-stage"]')
      ?.getAttribute("data-presence"),
    objectId: document
      .querySelector(".moment-object--anchor")
      ?.getAttribute("data-object-id"),
  }));

  await goTab(page, "Continue");
  const remember = page.getByRole("button", { name: "Remember this place" });
  try {
    await remember.waitFor({ timeout: 4000 });
    await remember.click();
  } catch {
    const continueBtn = page
      .locator(".moment-card--hero button.exp-btn.primary")
      .filter({ hasText: /^Continue/ })
      .first();
    await continueBtn.click({ force: true });
  }
  const approved = page.getByRole("button", { name: "Approve and restore" });
  try {
    await approved.waitFor({ timeout: 12000 });
    await page.waitForTimeout(900);
  } catch {
    await page.waitForTimeout(400);
  }
  await shot(page, "continue-reconstruction");

  const continueAttach = await page.evaluate(() => ({
    previewInside: !!document.querySelector(
      ".moment-object--anchor .continue-preview-body--remember",
    ),
    detachedCinema: !!document.querySelector(".continue-cinema__stage--solo"),
    presence: document
      .querySelector('[data-testid="persistent-moment-stage"]')
      ?.getAttribute("data-presence"),
    objectId: document
      .querySelector(".moment-object--anchor")
      ?.getAttribute("data-object-id"),
  }));

  await goTab(page, "Check-in");
  await page.getByText("How’s the return feeling?").first().waitFor({
    timeout: 10000,
  });
  await page.waitForTimeout(700);
  await shot(page, "checkin-attached");

  const checkinAttach = await page.evaluate(() => ({
    formInside: !!document.querySelector(
      ".moment-object--anchor #checkin-form",
    ),
    presence: document
      .querySelector('[data-testid="persistent-moment-stage"]')
      ?.getAttribute("data-presence"),
    objectId: document
      .querySelector(".moment-object--anchor")
      ?.getAttribute("data-object-id"),
  }));

  await goTab(page, "Guide");
  await page.getByText("How this pilot works").first().waitFor({
    timeout: 10000,
  });
  await page.waitForTimeout(500);
  await shot(page, "guide-contextual-hint");

  const guideAttach = await page.evaluate(() => ({
    hintInside: !!document.querySelector(
      ".moment-object--anchor .moment-guide-hint",
    ),
    presence: document
      .querySelector('[data-testid="persistent-moment-stage"]')
      ?.getAttribute("data-presence"),
    objectId: document
      .querySelector(".moment-object--anchor")
      ?.getAttribute("data-object-id"),
  }));

  const ids = [
    homeAnchor.layoutId,
    saveAttach.objectId,
    continueAttach.objectId,
    checkinAttach.objectId,
    guideAttach.objectId,
  ].filter(Boolean);
  const uniqueIds = new Set(ids);

  const report = {
    persistentStage: homeAnchor.stage,
    objectPermanence: uniqueIds.size === 1 && ids.length >= 4,
    sharedObjectId: [...uniqueIds][0] ?? null,
    crossDestinationContinuity: {
      home: homeAnchor.presence === "presence",
      save: saveAttach.presence === "writing" && saveAttach.writeInside,
      continue:
        continueAttach.presence === "restoring" &&
        continueAttach.previewInside &&
        !continueAttach.detachedCinema,
      checkin: checkinAttach.presence === "reflecting" && checkinAttach.formInside,
      guide: guideAttach.presence === "guided" && guideAttach.hintInside,
    },
    spatialIdentity: homeAnchor.stage && uniqueIds.size === 1,
    panelPerceptionReduced:
      !continueAttach.detachedCinema &&
      saveAttach.writeInside &&
      checkinAttach.formInside &&
      guideAttach.hintInside,
    detail: {
      homeAnchor,
      saveAttach,
      continueAttach,
      checkinAttach,
      guideAttach,
    },
  };

  const allPass = Object.values(report.crossDestinationContinuity).every(Boolean)
    && report.objectPermanence
    && report.spatialIdentity
    && report.panelPerceptionReduced;

  fs.writeFileSync(
    path.join(OUT, "object-continuity-validation.json"),
    JSON.stringify({ ...report, pass: allPass }, null, 2),
  );
  console.log("validation", report, "pass", allPass);
  await browser.close();
  if (!allPass) {
    process.exitCode = 1;
  }
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
