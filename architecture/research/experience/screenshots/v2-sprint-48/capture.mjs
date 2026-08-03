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
  await shot(page, "fresh-workspace");

  const fresh = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const nodes = [...document.querySelectorAll(".semantic-field__node")];
    const phases = [
      stage?.getAttribute("data-temporal"),
      ...nodes.map((n) => n.getAttribute("data-temporal")),
    ].filter(Boolean);
    return {
      temporalOn: Boolean(stage?.getAttribute("data-temporal")),
      phaseCount: new Set(phases).size,
      hasActivePhase: phases.some((p) =>
        ["nascent", "evolving", "resumed", "waiting"].includes(p),
      ),
      nodeCount: nodes.length,
      noBadges: !document.querySelector(
        "[data-temporal-badge], .temporal-badge, .moment-age-badge",
      ),
    };
  });

  await goTab(page, "Guide");
  await page.waitForTimeout(800);
  await shot(page, "contextual-guidance-over-time");

  const guide = await page.evaluate(() => {
    const panel = document.querySelector('[data-testid="pilot-help"]');
    const annotation = document.querySelector(".semantic-cluster__annotation");
    const confidence = Number.parseFloat(
      annotation?.getAttribute("data-confidence") || "0",
    );
    return {
      clusterMode: panel?.getAttribute("data-semantic-guide") === "cluster",
      hintFades: !annotation || confidence <= 1,
      noTimeline: !document.querySelector(".timeline, .history-panel"),
    };
  });

  await goTab(page, "Home");
  await goTab(page, "Save");
  try {
    await page.locator(".moment-write__invite").click({ timeout: 4000 });
  } catch {
    /* open */
  }
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 14000, state: "attached" });
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(700);
  try {
    await page.getByRole("button", { name: /Save|Quick save/i }).first().click({
      timeout: 5000,
    });
    await page.waitForTimeout(900);
  } catch {
    await page.waitForTimeout(400);
  }
  await goTab(page, "Home");
  await page.waitForTimeout(700);
  await shot(page, "evolving-workspace");

  const evolving = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const phase = stage?.getAttribute("data-temporal");
    return {
      stageTemporal: Boolean(phase),
      fieldAlive:
        document.querySelectorAll(".semantic-field__node").length >= 1,
      notHistoryChrome: !document.querySelector(".history-panel, .timeline"),
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
    await page.waitForTimeout(900);
  } catch {
    await page.waitForTimeout(400);
  }
  await shot(page, "resumed-workspace");

  const resumed = await page.evaluate(() => {
    const body = document.querySelector(".continue-preview-body--semantic");
    const confidence = Number.parseFloat(
      body?.getAttribute("data-temporal-confidence") || "0",
    );
    return {
      restoreTemporal: Boolean(body?.getAttribute("data-temporal")),
      confidencePresent: confidence > 0,
      panes: document.querySelectorAll(".continue-window-pane--semantic")
        .length,
    };
  });

  await goTab(page, "Home");
  await page.waitForTimeout(500);
  // Prefer a far/dormant neighbour when present.
  const dormantClicked = await page.evaluate(() => {
    const dormant = document.querySelector(
      '.semantic-field__node[data-temporal="dormant"] .ws-object, .semantic-field__node[data-band="far"] .ws-object',
    );
    if (dormant instanceof HTMLElement) {
      dormant.click();
      return true;
    }
    const nodes = [...document.querySelectorAll(".semantic-field__node")];
    const last = nodes[nodes.length - 1]?.querySelector(".ws-object");
    if (last instanceof HTMLElement) {
      last.click();
      return true;
    }
    return false;
  });
  await page.waitForTimeout(900);
  await shot(page, "long-dormant-workspace");

  const dormant = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const phase = stage?.getAttribute("data-temporal");
    const nodes = [...document.querySelectorAll(".semantic-field__node")];
    return {
      selected: Boolean(phase),
      quieterPresent:
        nodes.some((n) => n.getAttribute("data-temporal") === "dormant") ||
        nodes.some((n) => n.getAttribute("data-band") === "far") ||
        phase === "dormant" ||
        phase === "waiting",
      stillVisible: nodes.length >= 0,
    };
  });

  const report = {
    temporalContinuity: {
      stageTemporal: fresh.temporalOn,
      phaseVariety: fresh.phaseCount >= 1,
      activePresence: fresh.hasActivePhase,
    },
    workspaceMemory: {
      noBadges: fresh.noBadges,
      evolvingAlive: evolving.fieldAlive && evolving.stageTemporal,
      notHistoryUi: evolving.notHistoryChrome && guide.noTimeline,
    },
    evolutionStability: {
      restoreTemporal: resumed.restoreTemporal,
      confidenceWired: resumed.confidencePresent,
      dormantQuiet: dormant.quieterPresent || dormantClicked,
    },
    predictiveRelevance: {
      guideObservational: guide.clusterMode && guide.hintFades,
      resumePanes: resumed.panes > 0,
      dormantStillPresent: dormant.stillVisible,
    },
    detail: { fresh, guide, evolving, resumed, dormant, dormantClicked },
  };

  const flat = [
    ...Object.values(report.temporalContinuity),
    ...Object.values(report.workspaceMemory),
    ...Object.values(report.evolutionStability),
    ...Object.values(report.predictiveRelevance),
  ];
  const pass = flat.every(Boolean);

  fs.writeFileSync(
    path.join(OUT, "temporal-validation.json"),
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
