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
  await shot(page, "semantic-home");

  const home = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const field = document.querySelector('[data-testid="semantic-field"]');
    const nodes = [...document.querySelectorAll(".semantic-field__node")];
    const bands = nodes.map((n) => n.getAttribute("data-band"));
    const xs = nodes.map((n) =>
      Number.parseFloat(getComputedStyle(n).getPropertyValue("--sem-x")),
    );
    const distinctX = new Set(xs.map((v) => Math.round(v / 8))).size;
    return {
      semanticOn: stage?.getAttribute("data-semantic") === "on",
      fieldPresent: !!field,
      nodeCount: nodes.length,
      hasNear: bands.includes("near") || bands.includes("mid"),
      notRowGrid: !document.querySelector(".home-satellite--0"),
      spreadX: distinctX >= Math.min(2, nodes.length),
    };
  });

  // Guide early — before resume affinity quiets observational hints.
  await goTab(page, "Guide");
  await page.waitForTimeout(900);
  await shot(page, "contextual-guidance");

  const guide = await page.evaluate(() => {
    const panel = document.querySelector('[data-testid="pilot-help"]');
    const cluster = document.querySelector(".semantic-cluster__annotation");
    const field = document.querySelector('[data-testid="semantic-field"]');
    const hintOn = panel?.getAttribute("data-hint") === "on";
    return {
      clusterGuide: panel?.getAttribute("data-semantic-guide") === "cluster",
      annotationOnField: hintOn
        ? !!(cluster && field?.contains(cluster))
        : true,
      notOnDestinationExpand: !document.querySelector(
        ".moment-attach-host .moment-guide-hint",
      ),
      hintOn,
      hasClusterWhenHinted: !hintOn || !!cluster,
    };
  });

  await goTab(page, "Home");
  await goTab(page, "Save");
  try {
    await page.locator(".moment-write__invite").click({ timeout: 4000 });
  } catch {
    try {
      await page
        .locator(".moment-object--anchor")
        .click({ timeout: 3000, force: true });
    } catch {
      /* open */
    }
  }
  const handoff = page.locator("#saved-context-handoff");
  await handoff.waitFor({ timeout: 14000, state: "attached" });
  await handoff.fill("Finish the pricing slide, then send the PDF to Mara.");
  await page.waitForTimeout(900);
  await shot(page, "writing-field");

  const writing = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const nodes = [...document.querySelectorAll(".semantic-field__node")];
    const opacities = nodes.map((n) =>
      Number.parseFloat(n.style.getPropertyValue("--sem-opacity") || "0"),
    );
    const proximities = nodes.map((n) =>
      Number.parseFloat(n.getAttribute("data-proximity") || "0"),
    );
    const spread =
      opacities.length >= 2
        ? Math.max(...opacities) - Math.min(...opacities)
        : proximities.length >= 2
          ? Math.max(...proximities) - Math.min(...proximities)
          : 0;
    return {
      presence: stage?.getAttribute("data-presence") === "writing",
      fieldDuringWrite: nodes.length > 0,
      opacitySpread: spread > 0.04 || nodes.some((n) => n.dataset.band === "far"),
      spread,
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
  await shot(page, "restore-field");

  const restore = await page.evaluate(() => {
    const field = document.querySelector(
      '[data-testid="semantic-restore-field"]',
    );
    const panes = [
      ...document.querySelectorAll(".continue-window-pane--semantic"),
    ];
    const importances = panes.map((p) =>
      Number.parseFloat(p.getAttribute("data-importance") || "0"),
    );
    const sorted = [...importances].sort((a, b) => b - a);
    return {
      semanticRestore: !!field,
      paneCount: panes.length,
      noSymmetrySlots: !document.querySelector(".continue-window-pane--0"),
      importanceOrdered:
        importances.length === 0 ||
        importances.every((v, i) => Math.abs(v - sorted[i]) < 0.001),
    };
  });

  await goTab(page, "Check-in");
  await page.locator("#checkin-form").waitFor({ timeout: 10000 });
  await page.waitForTimeout(700);
  await shot(page, "reflection-influence");

  const reflection = await page.evaluate(() => {
    const stage = document.querySelector(
      '[data-testid="persistent-moment-stage"]',
    );
    const field = document.querySelector('[data-testid="semantic-field"]');
    return {
      reflecting: stage?.getAttribute("data-presence") === "reflecting",
      fieldPresent: !!field,
      noNewChrome: !document.querySelector(".checkin-semantic-badge"),
    };
  });

  const report = {
    semanticClarity: {
      stageSemantic: home.semanticOn,
      fieldPresent: home.fieldPresent,
      nodesPlaced: home.nodeCount >= 1,
    },
    spatialMeaning: {
      notRowGrid: home.notRowGrid,
      spreadRelationships: home.spreadX,
      writingReshape: writing.presence && writing.fieldDuringWrite,
    },
    relationshipLegibility: {
      bandVariation: home.hasNear,
      opacityDuringWrite: writing.opacitySpread,
      restoreByImportance: restore.importanceOrdered && restore.semanticRestore,
    },
    workspaceCoherence: {
      restoreSemantic: restore.semanticRestore && restore.noSymmetrySlots,
      reflectionQuiet: reflection.reflecting && reflection.noNewChrome,
      guideOnCluster: guide.clusterGuide && guide.notOnDestinationExpand,
    },
    detail: { home, writing, restore, reflection, guide },
  };

  const flat = [
    ...Object.values(report.semanticClarity),
    ...Object.values(report.spatialMeaning),
    ...Object.values(report.relationshipLegibility),
    ...Object.values(report.workspaceCoherence),
  ];
  const pass = flat.every(Boolean);

  fs.writeFileSync(
    path.join(OUT, "semantic-validation.json"),
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
