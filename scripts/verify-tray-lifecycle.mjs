#!/usr/bin/env node
/**
 * P16.PR2 / Gate E1 — tray lifecycle wiring (Show Conversation / Exit Workspace).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-tray-lifecycle: ${message}`);
  process.exit(1);
}

const required = [
  "app/src-tauri/src/tray.rs",
  "app/src-tauri/src/lib.rs",
  "app/src-tauri/Cargo.toml",
  "docs/production/P16_PR2_PRODUCTION_EXPERIENCE_AUDIT.md",
  "docs/production/production-gates-dependency.json",
  "docs/production/production-readiness.json",
];
for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const cargo = fs.readFileSync(path.join(root, "app/src-tauri/Cargo.toml"), "utf8");
if (!cargo.includes("tray-icon")) {
  fail("Cargo.toml must enable tauri tray-icon feature");
}

const tray = fs.readFileSync(path.join(root, "app/src-tauri/src/tray.rs"), "utf8");
for (const token of [
  "Show Conversation",
  "Exit Workspace",
  "TrayIconBuilder",
  "show_conversation",
  "install_tray",
]) {
  if (!tray.includes(token)) fail(`tray.rs missing ${token}`);
}
if (
  /\bOpen Settings\b/.test(tray) ||
  /\bCapability Catalog\b/i.test(tray) ||
  /MenuItem::with_id\([^)]*settings/i.test(tray)
) {
  fail("tray must not add catalogue/settings chrome (Product Gravity)");
}

const lib = fs.readFileSync(path.join(root, "app/src-tauri/src/lib.rs"), "utf8");
if (!lib.includes("mod tray")) fail("lib.rs must declare mod tray");
if (!lib.includes("tray::install_tray")) fail("lib.rs setup must call tray::install_tray");
if (!lib.includes("tray::show_conversation")) {
  fail("single-instance focus must reuse tray::show_conversation");
}

const dep = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/production/production-gates-dependency.json"),
    "utf8",
  ),
);
if (!(dep.completed ?? []).includes("E1-tray-lifecycle")) {
  fail("dependency json must mark E1-tray-lifecycle Complete");
}
const e1 = dep.units.find((u) => u.id === "E1-tray-lifecycle");
if (!e1 || e1.productionClassification !== "Complete") {
  fail("E1 unit must be Complete");
}

const matrix = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/production/production-readiness.json"),
    "utf8",
  ),
);
if (!matrix.completedGates?.includes("E1-tray-lifecycle")) {
  fail("production-readiness.json completedGates must include E1-tray-lifecycle");
}
const trayArea = matrix.areas?.find((a) => a.id === "systemTray");
if (!trayArea || trayArea.maturity === "TrackARequired") {
  fail("systemTray maturity must advance past TrackARequired");
}
if (!String(trayArea.evidence ?? "").toLowerCase().includes("tray")) {
  fail("systemTray evidence must mention tray wiring");
}

console.log("verify-tray-lifecycle: ok");
