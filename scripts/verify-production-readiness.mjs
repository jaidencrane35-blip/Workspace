#!/usr/bin/env node
/**
 * Verify production-readiness matrix stays consistent with repository evidence.
 * Does not claim public release readiness — only that claims match the tree.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

const MATURITIES = new Set([
  "Complete",
  "MinorImprovement",
  "TrackARequired",
  "FutureScale",
  "OutOfScope",
]);
const CONFIDENCES = new Set(["Verified", "Supported", "Hypothesis", "Unknown"]);

function fail(message) {
  console.error(`verify-production-readiness: ${message}`);
  process.exit(1);
}

const reportPath = path.join(
  root,
  "docs/production/PR1_PRODUCTION_READINESS_PROGRAM.md",
);
const matrixPath = path.join(root, "docs/production/production-readiness.json");
const tauriConfPath = path.join(root, "app/src-tauri/tauri.conf.json");
const cargoPath = path.join(root, "app/src-tauri/Cargo.toml");

if (!fs.existsSync(reportPath)) {
  fail(`missing ${reportPath}`);
}
if (!fs.existsSync(matrixPath)) {
  fail(`missing ${matrixPath}`);
}

const matrix = JSON.parse(fs.readFileSync(matrixPath, "utf8"));
if (matrix.id !== "workspace-production-readiness") {
  fail("matrix.id must be workspace-production-readiness");
}
if (!Array.isArray(matrix.areas) || matrix.areas.length < 20) {
  fail("matrix.areas must list at least 20 production areas");
}

const requiredIds = [
  "installer",
  "autoUpdate",
  "codeSigning",
  "systemTray",
  "diagnostics",
  "ipcSurface",
  "releaseAutomation",
  "telemetry",
];
for (const id of requiredIds) {
  if (!matrix.areas.some((a) => a.id === id)) {
    fail(`missing required area: ${id}`);
  }
}

for (const area of matrix.areas) {
  if (!area.id || !MATURITIES.has(area.maturity)) {
    fail(`invalid area maturity for ${area.id}`);
  }
  if (!CONFIDENCES.has(area.confidence)) {
    fail(`invalid confidence for ${area.id}`);
  }
  if (!area.evidence || String(area.evidence).trim().length < 8) {
    fail(`area ${area.id} needs evidence`);
  }
}

if (matrix.productionReadyToday !== false) {
  fail("productionReadyToday must be false until Phase 2 Critical lands");
}
if (matrix.broadPublicReleaseReady !== false) {
  fail("broadPublicReleaseReady must be false until Phase 2 Critical lands");
}
if (matrix.p17Blocked !== true) {
  fail("p17Blocked must remain true while Product Proof gate is open");
}

const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
const tauriText = JSON.stringify(tauriConf);
const cargoText = fs.readFileSync(cargoPath, "utf8");

const hasUpdater =
  /updater/i.test(tauriText) ||
  /tauri-plugin-updater/i.test(cargoText) ||
  /plugin-updater/i.test(cargoText);
const autoUpdate = matrix.areas.find((a) => a.id === "autoUpdate");
if (!hasUpdater && autoUpdate.maturity === "Complete") {
  fail("autoUpdate cannot be Complete without updater evidence in tauri/Cargo");
}
if (!hasUpdater && !["TrackARequired", "FutureScale", "OutOfScope"].includes(autoUpdate.maturity)) {
  fail(
    `autoUpdate maturity ${autoUpdate.maturity} inconsistent with absent updater`,
  );
}

const hasTrayDeps =
  /tray/i.test(cargoText) && /TrayIcon|SystemTray|tray-icon/i.test(cargoText);
const tray = matrix.areas.find((a) => a.id === "systemTray");
if (!hasTrayDeps && tray.maturity === "Complete") {
  fail("systemTray cannot be Complete without tray dependencies");
}

const signingHints =
  /certificateThumbprint|signCommand|windows.*sign|signingIdentity/i.test(
    tauriText,
  );
const codeSigning = matrix.areas.find((a) => a.id === "codeSigning");
if (!signingHints && codeSigning.maturity === "Complete") {
  fail("codeSigning cannot be Complete without signing config evidence");
}

console.log(
  `verify-production-readiness: ok (${matrix.areas.length} areas; productionReadyToday=${matrix.productionReadyToday})`,
);
