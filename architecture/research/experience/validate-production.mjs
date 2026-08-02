/**
 * Production workflow + interaction validation against populated demo.
 * Outputs JSON observations to stdout; screenshots optional.
 */
import { chromium } from "playwright";
import path from "node:path";
import { fileURLToPath } from "node:url";
import fs from "node:fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const BASE = "http://127.0.0.1:1420/";
const OUT = path.join(__dirname, "screenshots", "production-validation");

const friction = [];
const workflow = { journeys: [], interactions: [], performance: [] };

function note(list, item) {
  list.push(item);
  console.log(JSON.stringify(item));
}

async function goTab(page, name) {
  const tab = page.getByRole("tab", { name, exact: true });
  await tab.click();
  await page.waitForTimeout(700);
  return tab;
}

async function visibleText(page, text) {
  return page.getByText(text, { exact: false }).first().isVisible().catch(() => false);
}

async function run() {
  fs.mkdirSync(OUT, { recursive: true });
  const browser = await chromium.launch({ channel: "msedge", headless: true });
  const context = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    colorScheme: "dark",
  });
  const page = await context.newPage();

  // CLS / layout shift observer
  await page.addInitScript(() => {
    window.__cls = 0;
    window.__longTasks = 0;
    try {
      new PerformanceObserver((list) => {
        for (const e of list.getEntries()) {
          if (e.hadRecentInput) continue;
          window.__cls += e.value || 0;
        }
      }).observe({ type: "layout-shift", buffered: true });
    } catch {}
  });

  const t0 = Date.now();
  await page.goto(BASE, { waitUntil: "networkidle", timeout: 45000 });
  await page.waitForTimeout(1000);

  const hasHome = await visibleText(page, "Continue your work");
  if (!hasHome) {
    note(friction, {
      journey: "boot",
      step: "load demo",
      expected: "Populated Home with Continue your work",
      actual: "Home title not found",
      component: "HomeWorkspacePanel",
      severity: 5,
      fix: "L",
    });
  }

  // ─── Journey 1: Home → Save → Home → Continue → Preview ───
  const j1 = { id: 1, steps: [], ok: true };
  await goTab(page, "Home");
  j1.steps.push({ step: "Home", ok: await visibleText(page, "Continue your work") });

  await goTab(page, "Save");
  const saveTitle = await visibleText(page, "Save");
  const handoff = page.locator("textarea, [contenteditable='true'], input").first();
  const handoffCount = await page.locator("textarea").count();
  j1.steps.push({ step: "Save view", ok: saveTitle || handoffCount > 0, handoffFields: handoffCount });

  if (handoffCount > 0) {
    await page.locator("textarea").first().click();
    await page.locator("textarea").first().fill(
      "Validation handoff — finish pricing slide review.",
    );
    // name field if present
    const nameInput = page.locator('input[type="text"], input:not([type])').first();
    if (await nameInput.count()) {
      await nameInput.fill("Validation moment · Client");
    }
    const saveBtn = page
      .getByRole("button", { name: /Save this context|Save|Quick save/i })
      .first();
    if (await saveBtn.count()) {
      await saveBtn.click();
      await page.waitForTimeout(900);
      const savedOk =
        (await visibleText(page, "Saved")) ||
        (await visibleText(page, "saved")) ||
        (await visibleText(page, "Validation moment"));
      j1.steps.push({ step: "Save moment", ok: savedOk });
      if (!savedOk) {
        note(friction, {
          journey: 1,
          step: "Save a new Moment",
          expected: "Confirmation that moment was saved",
          actual: "No clear saved confirmation in view after submit",
          component: "SaveContextPanel",
          severity: 3,
          fix: "S",
        });
        j1.ok = false;
      }
    } else {
      note(friction, {
        journey: 1,
        step: "Save a new Moment",
        expected: "Primary Save action available",
        actual: "No Save button found",
        component: "SaveContextPanel",
        severity: 4,
        fix: "M",
      });
      j1.ok = false;
    }
  } else {
    // Demo without Tauri may show different save UI
    const body = await page.locator("main, .ws-content, .spatial-frame").first().innerText().catch(() => "");
    note(friction, {
      journey: 1,
      step: "Save a new Moment",
      expected: "Writable handoff / name fields for new Moment",
      actual: `Save surface lacked textarea (snippet: ${body.slice(0, 160).replace(/\s+/g, " ")})`,
      component: "SaveContextPanel",
      severity: 4,
      fix: "M",
    });
    j1.ok = false;
  }

  await goTab(page, "Home");
  j1.steps.push({ step: "Return Home", ok: await visibleText(page, "Continue your work") });

  await goTab(page, "Continue");
  const continueBrowse = await visibleText(page, "What were you doing?");
  j1.steps.push({ step: "Continue browse", ok: continueBrowse });

  const continueCta = page
    .locator(".moment-card--hero .exp-btn.primary")
    .filter({ hasText: "Continue" })
    .first();
  if (await continueCta.count()) {
    await continueCta.click();
    await page.waitForTimeout(1100);
    const preview =
      (await visibleText(page, "Approve and restore")) ||
      (await visibleText(page, "This place is ready"));
    j1.steps.push({ step: "Restore Preview", ok: preview });
    if (!preview) {
      note(friction, {
        journey: 1,
        step: "Restore Preview",
        expected: "Approve and restore preview for selected Moment",
        actual: "Preview actions not visible after Continue",
        component: "ResumeContextPanel / ContinuePreviewBody",
        severity: 4,
        fix: "M",
      });
      j1.ok = false;
    }
  } else {
    note(friction, {
      journey: 1,
      step: "Continue → Preview",
      expected: "Continue CTA on featured Moment",
      actual: "Continue CTA missing",
      component: "MomentCard",
      severity: 4,
      fix: "M",
    });
    j1.ok = false;
  }
  await page.screenshot({ path: path.join(OUT, "journey1-preview.png") });
  note(workflow.journeys, j1);

  // ─── Journey 2: Home → Continue → Check-in → Home ───
  const j2 = { id: 2, steps: [], ok: true };
  await goTab(page, "Home");
  j2.steps.push({ step: "Home", ok: true });
  await goTab(page, "Continue");
  if (await continueCta.count()) {
    // may already be previewing
    const approve = page.getByRole("button", { name: /Approve and restore/i });
    if (!(await approve.isVisible().catch(() => false))) {
      await continueCta.click().catch(() => {});
      await page.waitForTimeout(800);
    }
  }
  j2.steps.push({
    step: "Continue existing Moment",
    ok:
      (await visibleText(page, "Approve and restore")) ||
      (await visibleText(page, "What were you doing?")),
  });

  await goTab(page, "Check-in");
  const checkin = await visibleText(page, "How’s the return feeling?");
  j2.steps.push({ step: "Check-in", ok: checkin });
  if (!checkin) {
    const consent = await visibleText(page, "consent");
    note(friction, {
      journey: 2,
      step: "Check-in",
      expected: "Active check-in pulse (or clear consent gate)",
      actual: consent ? "Consent gate shown instead of active pulse" : "Check-in title missing",
      component: "PilotMeasurementPanel",
      severity: consent ? 2 : 4,
      fix: "S",
    });
    if (!consent) j2.ok = false;
  }

  // dock labels hidden — verify aria names still work
  const checkinTab = page.getByRole("tab", { name: "Check-in", exact: true });
  const checkinNamed = await checkinTab.count();
  if (!checkinNamed) {
    note(friction, {
      journey: 2,
      step: "Dock",
      expected: "Check-in tab reachable by accessible name",
      actual: "Tab name Check-in not found",
      component: "WorkspaceShell dock",
      severity: 5,
      fix: "S",
    });
  }

  await goTab(page, "Home");
  j2.steps.push({ step: "Return Home", ok: await visibleText(page, "Continue your work") });
  await page.screenshot({ path: path.join(OUT, "journey2-home.png") });
  note(workflow.journeys, j2);

  // ─── Journey 3: Guide → Home → Save → Continue ───
  const j3 = { id: 3, steps: [], ok: true };
  await goTab(page, "Guide");
  j3.steps.push({
    step: "Guide",
    ok: await visibleText(page, "How this pilot works"),
  });
  await goTab(page, "Home");
  j3.steps.push({ step: "Home", ok: await visibleText(page, "Continue your work") });
  await goTab(page, "Save");
  j3.steps.push({
    step: "Save",
    ok: (await page.locator("textarea").count()) > 0 || (await visibleText(page, "Save")),
  });
  await goTab(page, "Continue");
  j3.steps.push({
    step: "Continue",
    ok:
      (await visibleText(page, "What were you doing?")) ||
      (await visibleText(page, "Approve and restore")),
  });
  await page.screenshot({ path: path.join(OUT, "journey3-continue.png") });
  note(workflow.journeys, j3);

  // ─── Interaction: keyboard-only dock ───
  await goTab(page, "Home");
  await page.keyboard.press("Tab");
  await page.waitForTimeout(100);
  let focusTrail = [];
  for (let i = 0; i < 12; i++) {
    const info = await page.evaluate(() => {
      const el = document.activeElement;
      if (!el) return null;
      return {
        tag: el.tagName,
        role: el.getAttribute("role"),
        name:
          el.getAttribute("aria-label") ||
          (el.textContent || "").trim().slice(0, 40),
        className: (el.className || "").toString().slice(0, 60),
      };
    });
    focusTrail.push(info);
    await page.keyboard.press("Tab");
    await page.waitForTimeout(40);
  }
  note(workflow.interactions, { kind: "focus-order-sample", focusTrail });

  const dockInTrail = focusTrail.some(
    (f) => f && (f.role === "tab" || /Home|Save|Continue|Check-in|Guide/.test(f.name || "")),
  );
  if (!dockInTrail) {
    note(friction, {
      journey: "interaction",
      step: "keyboard-only navigation",
      expected: "Dock tabs appear in Tab focus order",
      actual: `After 12 Tabs, no dock tab focused. Trail: ${focusTrail
        .filter(Boolean)
        .map((f) => f.name || f.className)
        .join(" → ")}`,
      component: "WorkspaceShell",
      severity: 3,
      fix: "M",
    });
  }

  // Arrow keys between tabs (shell supports Left/Right)
  await page.getByRole("tab", { name: "Home", exact: true }).focus();
  await page.keyboard.press("ArrowRight");
  await page.waitForTimeout(400);
  const afterArrow = await page.evaluate(() => {
    const t = document.querySelector('[role="tab"][aria-selected="true"]');
    return t?.getAttribute("aria-label") || t?.textContent?.trim();
  });
  if (afterArrow !== "Save") {
    note(friction, {
      journey: "interaction",
      step: "dock keyboard arrows",
      expected: "ArrowRight from Home selects Save",
      actual: `Active tab became ${afterArrow}`,
      component: "WorkspaceShell",
      severity: 2,
      fix: "S",
    });
  }
  note(workflow.interactions, { kind: "dock-arrow", afterArrow });

  // Writing flow focus
  await goTab(page, "Save");
  const ta = page.locator("textarea").first();
  if (await ta.count()) {
    await ta.focus();
    const writingClass = await page.evaluate(() =>
      document.body.className + " " + (document.documentElement.dataset.density || "") + " " +
      (document.querySelector("[data-writing], .is-writing, [data-ambient='input']")?.className || ""),
    );
    note(workflow.interactions, { kind: "writing-focus", writingClass: writingClass.slice(0, 120) });
  }

  // Resize
  await page.setViewportSize({ width: 1100, height: 800 });
  await page.waitForTimeout(400);
  await goTab(page, "Home");
  const homeOkNarrow = await visibleText(page, "Continue your work");
  const overflow = await page.evaluate(() => ({
    scrollWidth: document.documentElement.scrollWidth,
    clientWidth: document.documentElement.clientWidth,
    bodyOverflowX: getComputedStyle(document.body).overflowX,
  }));
  if (overflow.scrollWidth > overflow.clientWidth + 8) {
    note(friction, {
      journey: "interaction",
      step: "screen resizing 1100px",
      expected: "No horizontal page overflow",
      actual: `scrollWidth ${overflow.scrollWidth} > clientWidth ${overflow.clientWidth}`,
      component: "layout / App.css",
      severity: 2,
      fix: "S",
    });
  }
  note(workflow.interactions, { kind: "resize-1100", homeOkNarrow, overflow });

  await page.setViewportSize({ width: 720, height: 900 });
  await page.waitForTimeout(400);
  const dockVisible = await page.locator(".ws-dock, .tabs.exp-nav").first().isVisible();
  const mobileHome = await visibleText(page, "Continue your work");
  note(workflow.interactions, { kind: "resize-720", dockVisible, mobileHome });
  if (!dockVisible) {
    note(friction, {
      journey: "interaction",
      step: "screen resizing 720px",
      expected: "Dock remains usable",
      actual: "Dock not visible",
      component: "ws-dock",
      severity: 3,
      fix: "M",
    });
  }

  await page.setViewportSize({ width: 1440, height: 900 });

  // Rapid dock navigation
  const rapidStart = Date.now();
  for (const name of ["Home", "Save", "Continue", "Check-in", "Guide", "Home", "Continue", "Home"]) {
    await goTab(page, name);
  }
  const rapidMs = Date.now() - rapidStart;
  note(workflow.interactions, { kind: "rapid-dock", rapidMs });
  if (rapidMs > 12000) {
    note(friction, {
      journey: "interaction",
      step: "rapid dock navigation",
      expected: "Dock switches feel snappy under repeated use",
      actual: `8 navigations took ${rapidMs}ms`,
      component: "WorkspaceShell / view panels",
      severity: 2,
      fix: "M",
    });
  }

  // Empty↔populated: Home still populated after journeys
  await goTab(page, "Home");
  const stillPopulated =
    (await page.locator(".moment-card--hero").count()) > 0 ||
    (await visibleText(page, "Northwind"));
  note(workflow.interactions, { kind: "populated-stable", stillPopulated });

  // Inspect recessed but reachable
  await goTab(page, "Continue");
  // back to browse if previewing
  const notNow = page.getByRole("button", { name: /Not now/i });
  if (await notNow.isVisible().catch(() => false)) {
    await notNow.click();
    await page.waitForTimeout(500);
  }
  const inspect = page.locator("summary", { hasText: "Inspect" });
  const inspectVisible = await inspect.isVisible().catch(() => false);
  note(workflow.interactions, { kind: "inspect-entry", inspectVisible });

  // Continue Inspect near dock may collide with dock click targets
  if (inspectVisible) {
    const boxes = await page.evaluate(() => {
      const s = document.querySelector(".continue-inspect-entry");
      const d = document.querySelector(".ws-dock");
      if (!s || !d) return null;
      const a = s.getBoundingClientRect();
      const b = d.getBoundingClientRect();
      const overlap = !(a.right < b.left || a.left > b.right || a.bottom < b.top || a.top > b.bottom);
      return { inspect: { top: a.top, bottom: a.bottom }, dock: { top: b.top, bottom: b.bottom }, overlap };
    });
    if (boxes?.overlap) {
      note(friction, {
        journey: 1,
        step: "Continue browse — Inspect vs dock",
        expected: "Inspect control clear of dock hit target",
        actual: "Inspect details overlaps dock bounds",
        component: "ResumeContextPanel / continue-inspect-entry",
        severity: 3,
        fix: "S",
      });
    }
    note(workflow.interactions, { kind: "inspect-dock-geometry", boxes });
  }

  // A11y smoke
  const a11y = await page.evaluate(() => {
    const out = [];
    document.querySelectorAll('[role="tab"]').forEach((tab) => {
      const name = (tab.getAttribute("aria-label") || tab.textContent || "").trim();
      if (!name) out.push("tab missing name");
    });
    if (!document.querySelector("h1, h2")) out.push("no heading");
    // visible buttons without name
    [...document.querySelectorAll("button")].forEach((b) => {
      if (!b.offsetParent) return;
      const name = (b.getAttribute("aria-label") || b.textContent || "").trim();
      if (!name) out.push("unnamed button");
    });
    return out;
  });
  note(workflow.interactions, { kind: "a11y-smoke", a11y });
  if (a11y.length) {
    note(friction, {
      journey: "interaction",
      step: "accessibility smoke",
      expected: "Named tabs/buttons and a heading",
      actual: a11y.join("; "),
      component: "various",
      severity: 3,
      fix: "S",
    });
  }

  const cls = await page.evaluate(() => window.__cls || 0);
  note(workflow.performance, {
    kind: "cls",
    cls,
    elapsedMs: Date.now() - t0,
  });
  if (cls > 0.1) {
    note(friction, {
      journey: "performance",
      step: "layout shifts",
      expected: "CLS under 0.1 during workflow",
      actual: `CLS ${cls.toFixed(3)}`,
      component: "layout motion",
      severity: 2,
      fix: "M",
    });
  }

  // Check-in Record CTA vs dock clearance
  await goTab(page, "Check-in");
  const ctaClear = await page.evaluate(() => {
    const btn = [...document.querySelectorAll("button")].find((b) =>
      /Record this leave/i.test(b.textContent || ""),
    );
    const dock = document.querySelector(".ws-dock");
    if (!btn || !dock) return null;
    const a = btn.getBoundingClientRect();
    const b = dock.getBoundingClientRect();
    return {
      gap: b.top - a.bottom,
      btnBottom: a.bottom,
      dockTop: b.top,
      viewport: window.innerHeight,
    };
  });
  note(workflow.interactions, { kind: "checkin-cta-dock", ctaClear });
  if (ctaClear && ctaClear.gap < 12) {
    note(friction, {
      journey: 2,
      step: "Check-in Record CTA",
      expected: "Clear space between Record CTA and dock",
      actual: `Gap ${ctaClear.gap?.toFixed?.(1) ?? ctaClear.gap}px`,
      component: "PilotMeasurementPanel / checkin-narrative",
      severity: 3,
      fix: "S",
    });
  }

  fs.writeFileSync(
    path.join(__dirname, "validation-raw.json"),
    JSON.stringify({ friction, workflow }, null, 2),
  );

  await context.close();
  await browser.close();
  console.log("FRICTION_COUNT", friction.length);
  console.log("done");
}

run().catch((e) => {
  console.error(e);
  process.exit(1);
});
