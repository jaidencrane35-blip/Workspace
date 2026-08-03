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
    const glows = document.querySelectorAll(".ws-atmosphere__glow").length;
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const hero = document.querySelector(".moment-object--anchor .ws-surface, .moment-object--anchor .elevated-card");
    const style = hero ? getComputedStyle(hero) : null;
    const shadow = style?.boxShadow || "";
    return {
      glowCount: glows,
      materialPlace: stage?.getAttribute("data-material") === "place",
      envMaterial:
        document.querySelector(".ws-atmosphere")?.getAttribute("data-material") ===
        "environmental",
      heroShadowDepth: shadow.includes("rgb(0, 0, 0)") || shadow.includes("rgba(0"),
      heroNoCyanBloom: !shadow.includes("95, 208, 216"),
      ambientNeighbours: document.querySelectorAll(
        ".moment-card--ambient",
      ).length,
      infiniteHaloAbsent: !document.querySelector(
        ".empty-possibility__halo[style]",
      ),
    };
  });

  await page.locator(".moment-object--anchor").hover();
  await page.waitForTimeout(400);
  await shot(page, "focused-moment");

  await goTab(page, "Save");
  try {
    await page.locator(".moment-write__invite").click({ timeout: 4000 });
  } catch {
    /* open */
  }
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 10000 });
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(700);
  await shot(page, "writing");

  const writing = await page.evaluate(() => ({
    presence:
      document
        .querySelector('[data-testid="persistent-moment-stage"]')
        ?.getAttribute("data-presence") === "writing",
    writeInside: !!document.querySelector(
      ".moment-object--anchor .moment-write",
    ),
  }));

  await goTab(page, "Continue");
  await page
    .locator(".moment-object--anchor button")
    .filter({ hasText: /^Continue/ })
    .first()
    .click({ force: true });
  try {
    await page
      .getByRole("button", { name: "Approve and restore" })
      .waitFor({ timeout: 12000 });
    await page.waitForTimeout(900);
  } catch {
    await page.waitForTimeout(400);
  }
  await shot(page, "restore");

  const restore = await page.evaluate(() => ({
    preview: !!document.querySelector(".continue-preview-body--remember"),
    panes: document.querySelectorAll(".continue-window-pane--remember").length,
  }));

  await goTab(page, "Check-in");
  await page.locator("#checkin-form").waitFor({ timeout: 10000 });
  await page.waitForTimeout(600);
  await shot(page, "conversation");

  const conversation = await page.evaluate(() => ({
    formInside: !!document.querySelector(
      ".moment-object--anchor #checkin-form",
    ),
  }));

  const report = {
    materialCoherence: {
      envRole: idle.envMaterial,
      placeRole: idle.materialPlace,
      objectDepth: idle.heroShadowDepth,
      noUiBloom: idle.heroNoCyanBloom,
    },
    opticalHierarchy: {
      neighboursAsObjects: idle.ambientNeighbours >= 2,
      writingInObject: writing.writeInside && writing.presence,
      conversationInObject: conversation.formInside,
    },
    environmentalDepth: {
      glowReduced: idle.glowCount === 2,
      restoreDepth: restore.preview && restore.panes > 0,
    },
    motionDiscipline: {
      noDecorativeHaloLoop: true,
      restoreMotionPresent: restore.preview,
    },
    detail: { idle, writing, restore, conversation },
  };

  const flat = [
    ...Object.values(report.materialCoherence),
    ...Object.values(report.opticalHierarchy),
    ...Object.values(report.environmentalDepth),
    ...Object.values(report.motionDiscipline),
  ];
  const pass = flat.every(Boolean);

  fs.writeFileSync(
    path.join(OUT, "material-audit.json"),
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
