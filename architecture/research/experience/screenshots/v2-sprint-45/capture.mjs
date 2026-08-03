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
  await shot(page, "passive-workspace");

  const passive = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const neighbours = [
      ...document.querySelectorAll(".ws-object-stage__neighbours .moment-card"),
    ];
    return {
      cognitive: stage?.getAttribute("data-cognitive") === "on",
      neighbourCount: neighbours.length,
      fieldCognitive: !!document.querySelector(".home-field--cognitive"),
    };
  });

  await goTab(page, "Save");
  try {
    await page.locator(".moment-write__invite").click({ timeout: 4000 });
  } catch {
    /* already open */
  }
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 10000 });
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(800);
  await shot(page, "active-writing");

  const writing = await page.evaluate(() => ({
    presence:
      document
        .querySelector('[data-testid="persistent-moment-stage"]')
        ?.getAttribute("data-presence") === "writing",
    toolsOn: !!document.querySelector('.moment-write[data-writing="on"]'),
  }));

  // Guide before resume — observational usefulness while workspace is still unfamiliar.
  await goTab(page, "Guide");
  await page.waitForTimeout(700);
  await shot(page, "contextual-guidance");

  const guide = await page.evaluate(() => {
    const section = document.querySelector(".guide-place--cognitive");
    return {
      cognitive: !!section,
      hintAttr: section?.getAttribute("data-hint"),
      hintInside: !!document.querySelector(
        ".moment-object--anchor .moment-guide-hint",
      ),
      howThisPilot: document.body.innerText.includes("How this pilot works") ||
        !!document.querySelector(".guide-place"),
    };
  });

  await goTab(page, "Continue");
  const continueBtn = page
    .locator(".moment-object--anchor button")
    .filter({ hasText: /^Continue/ })
    .first();
  await continueBtn.click({ force: true });
  try {
    await page
      .getByRole("button", { name: "Approve and restore" })
      .waitFor({ timeout: 12000 });
    await page.waitForTimeout(900);
  } catch {
    await page.waitForTimeout(400);
  }
  await shot(page, "resumed-workspace");

  const resumed = await page.evaluate(() => {
    const panes = [
      ...document.querySelectorAll(
        ".continue-preview-body--cognitive .continue-window-pane",
      ),
    ];
    return {
      cognitivePreview: !!document.querySelector(
        ".continue-preview-body--cognitive",
      ),
      paneCount: panes.length,
      presence:
        document
          .querySelector('[data-testid="persistent-moment-stage"]')
          ?.getAttribute("data-presence") === "restoring",
    };
  });

  await goTab(page, "Check-in");
  await page.locator("#checkin-form").waitFor({ timeout: 10000 });
  await page.waitForTimeout(700);
  await shot(page, "reflection-influence");

  const reflection = await page.evaluate(() => ({
    formInside: !!document.querySelector(
      ".moment-object--anchor #checkin-form",
    ),
    presence:
      document
        .querySelector('[data-testid="persistent-moment-stage"]')
        ?.getAttribute("data-presence") === "reflecting",
  }));

  await goTab(page, "Home");
  await page.waitForTimeout(700);
  const organised = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    return {
      backHomeCognitive: stage?.getAttribute("data-cognitive") === "on",
      presence: stage?.getAttribute("data-presence"),
      neighbours: document.querySelectorAll(
        ".ws-object-stage__neighbours .moment-card",
      ).length,
    };
  });

  const report = {
    cognitiveCoherence: {
      stageMarked: passive.cognitive,
      writingPresence: writing.presence && writing.toolsOn,
      restoreCognitive: resumed.cognitivePreview && resumed.presence,
      reflectionPresence: reflection.presence && reflection.formInside,
    },
    environmentalIntelligence: {
      neighbourField: passive.fieldCognitive && passive.neighbourCount > 0,
      guideObservational: guide.cognitive && guide.howThisPilot,
      guideRespectsConfidence:
        guide.hintAttr === "off" || guide.hintInside === true,
    },
    attentionStability: {
      writingFocused: writing.presence,
      restoreFocused: resumed.presence,
      homeReturns: organised.presence === "presence",
    },
    workspaceSelfOrganisation: {
      cognitiveHome: organised.backHomeCognitive,
      neighboursPresent: organised.neighbours > 0,
      panesPrioritised: resumed.paneCount > 0,
    },
    detail: { passive, writing, guide, resumed, reflection, organised },
  };

  const flat = [
    ...Object.values(report.cognitiveCoherence),
    ...Object.values(report.environmentalIntelligence),
    ...Object.values(report.attentionStability),
    ...Object.values(report.workspaceSelfOrganisation),
  ];
  const pass = flat.every(Boolean);

  fs.writeFileSync(
    path.join(OUT, "cognitive-behaviour-validation.json"),
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
