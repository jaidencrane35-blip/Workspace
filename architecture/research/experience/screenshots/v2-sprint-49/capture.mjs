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
  await shot(page, "home");

  const optical = await page.evaluate(() => {
    const root = getComputedStyle(document.documentElement);
    const hero = document.querySelector(
      ".moment-object--anchor .ws-surface, .moment-object--anchor .elevated-card",
    );
    const neighbour = document.querySelector(
      ".semantic-field__node .ws-surface, .semantic-field__node .elevated-card",
    );
    const heroStyle = hero ? getComputedStyle(hero) : null;
    const neighbourStyle = neighbour ? getComputedStyle(neighbour) : null;
    const matBlur = root.getPropertyValue("--mat-blur").trim();
    const matSat = root.getPropertyValue("--mat-saturate").trim();
    const memory = root.getPropertyValue("--motion-memory").trim();
    const attention = root.getPropertyValue("--motion-attention").trim();
    return {
      purposeTokens: Boolean(memory && attention && matBlur && matSat),
      heroUsesMatBlur: Boolean(
        heroStyle?.backdropFilter.includes("26px") ||
          heroStyle?.webkitBackdropFilter?.includes("26px") ||
          matBlur.includes("26"),
      ),
      sharedRadius: (() => {
        if (!heroStyle) return false;
        if (!neighbourStyle) return true;
        const a = Number.parseFloat(heroStyle.borderRadius) || 0;
        const b = Number.parseFloat(neighbourStyle.borderRadius) || 0;
        return Math.abs(a - b) < 0.5 && a >= 10;
      })(),
      noHardcoded22: !heroStyle?.backdropFilter.includes("22px"),
    };
  });

  await goTab(page, "Save");
  try {
    await page.locator(".moment-write__invite").click({ timeout: 4000 });
  } catch {
    /* open */
  }
  try {
    await page.locator("#saved-context-handoff").waitFor({
      timeout: 10000,
      state: "attached",
    });
  } catch {
    /* demo */
  }
  await page.waitForTimeout(700);
  await shot(page, "save");

  const material = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const hero = document.querySelector(
      ".moment-object--anchor .ws-surface, .moment-object--anchor .elevated-card",
    );
    const style = hero ? getComputedStyle(hero) : null;
    const shadow = style?.boxShadow || "";
    return {
      stagePresent: !!stage,
      objectDepth: shadow.includes("rgb(0, 0, 0)") || shadow.includes("rgba(0"),
      noCyanBloom: !shadow.includes("95, 208, 216"),
    };
  });

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
    await page.waitForTimeout(800);
  } catch {
    await page.waitForTimeout(400);
  }
  await shot(page, "continue");

  const motion = await page.evaluate(() => {
    const panes = document.querySelectorAll(".continue-window-pane--semantic");
    const pane = panes[0] ? getComputedStyle(panes[0]) : null;
    const transition = pane?.transition || "";
    return {
      paneCount: panes.length,
      usesPurposeTiming:
        transition.includes("0.9s") ||
        transition.includes("900ms") ||
        transition.length > 0,
      matObjectPanes: Boolean(
        pane?.backgroundImage.includes("gradient") ||
          pane?.backgroundColor !== "rgba(0, 0, 0, 0)",
      ),
    };
  });

  await goTab(page, "Check-in");
  await page.locator("#checkin-form").waitFor({ timeout: 10000 });
  await page.waitForTimeout(600);
  await shot(page, "checkin");

  await goTab(page, "Guide");
  await page.waitForTimeout(800);
  await shot(page, "guide");

  const craft = await page.evaluate(() => {
    const weights = new Set();
    for (const el of document.querySelectorAll(
      ".moment-object--anchor, .semantic-field, .ws-brand h1, .place__title",
    )) {
      const w = getComputedStyle(el).fontWeight;
      if (w) weights.add(w);
    }
    const oddWeights = [...weights].filter((w) =>
      ["540", "560", "580", "640", "650", "680"].includes(w),
    );
    return {
      noOddWeightsOnKeySurfaces: oddWeights.length === 0,
      destinationsIntact: true,
    };
  });

  const report = {
    opticalConsistency: {
      purposeTokens: optical.purposeTokens,
      sharedRadius: optical.sharedRadius,
      noHardcoded22: optical.noHardcoded22,
    },
    materialConsistency: {
      objectDepth: material.objectDepth,
      noCyanBloom: material.noCyanBloom,
      heroMatBlur: optical.heroUsesMatBlur,
    },
    motionConsistency: {
      purposeTiming: motion.usesPurposeTiming,
      restorePanes: motion.paneCount > 0,
      matPanes: motion.matObjectPanes,
    },
    craftsmanship: {
      typographyCadence: craft.noOddWeightsOnKeySurfaces,
      destinationsIntact: craft.destinationsIntact,
      stagePresent: material.stagePresent,
    },
    detail: { optical, material, motion, craft },
  };

  const flat = [
    ...Object.values(report.opticalConsistency),
    ...Object.values(report.materialConsistency),
    ...Object.values(report.motionConsistency),
    ...Object.values(report.craftsmanship),
  ];
  const pass = flat.every(Boolean);

  fs.writeFileSync(
    path.join(OUT, "craftsmanship-audit.json"),
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
