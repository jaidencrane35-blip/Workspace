#!/usr/bin/env node
/**
 * Verify P16.PI1 Slice 1 — Installer Foundation is present and coherent.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-installer-foundation: ${message}`);
  process.exit(1);
}

const tauriPath = path.join(root, "app/src-tauri/tauri.conf.json");
const hooksPath = path.join(root, "app/src-tauri/windows/hooks.nsh");
const docsPath = path.join(root, "docs/production/INSTALLER.md");
const slicePath = path.join(
  root,
  "docs/production/P16_PI1_SLICE1_INSTALLER_FOUNDATION.md",
);
const matrixPath = path.join(root, "docs/production/production-readiness.json");

for (const p of [tauriPath, hooksPath, docsPath, slicePath, matrixPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const conf = JSON.parse(fs.readFileSync(tauriPath, "utf8"));
const bundle = conf.bundle;
if (!bundle?.active) fail("bundle.active must be true");

const targets = bundle.targets;
const targetOk =
  targets === "nsis" ||
  (Array.isArray(targets) && targets.includes("nsis"));
if (!targetOk) fail('bundle.targets must include "nsis"');

for (const key of [
  "publisher",
  "shortDescription",
  "longDescription",
  "copyright",
]) {
  if (!bundle[key] || String(bundle[key]).trim().length < 3) {
    fail(`bundle.${key} required for production installer metadata`);
  }
}

const win = bundle.windows;
if (!win) fail("bundle.windows required");
if (win.allowDowngrades !== false) {
  fail("bundle.windows.allowDowngrades must be false (upgrade path)");
}
if (win.webviewInstallMode?.type !== "embedBootstrapper") {
  fail("webviewInstallMode.type must be embedBootstrapper (prerequisites)");
}
if (!win.minimumWebview2Version) {
  fail("minimumWebview2Version required");
}
if (!win.nsis?.installerHooks) {
  fail("nsis.installerHooks required");
}
if (win.nsis.installMode !== "currentUser" && win.nsis.installMode !== "both") {
  fail("nsis.installMode must be currentUser or both");
}

const hooksRel = String(win.nsis.installerHooks).replace(/^\.\//, "");
const hooksResolved = path.join(root, "app/src-tauri", hooksRel);
if (!fs.existsSync(hooksResolved)) {
  fail(`installerHooks path missing: ${hooksRel}`);
}

const hooks = fs.readFileSync(hooksPath, "utf8");
for (const macro of [
  "NSIS_HOOK_PREINSTALL",
  "NSIS_HOOK_POSTINSTALL",
  "NSIS_HOOK_PREUNINSTALL",
  "NSIS_HOOK_POSTUNINSTALL",
]) {
  if (!hooks.includes(macro)) fail(`hooks.nsh missing ${macro}`);
}
if (!hooks.includes("install-manifest.json")) {
  fail("hooks.nsh must write install-manifest.json (version detection)");
}
if (!hooks.includes("taskkill") || !hooks.includes("workspace-app.exe")) {
  fail("hooks.nsh must close workspace-app.exe for upgrade/uninstall");
}
if (!hooks.includes("com.workspace.app")) {
  fail("hooks.nsh must reference user-data identifier for cleanup policy");
}

const matrix = JSON.parse(fs.readFileSync(matrixPath, "utf8"));
const installer = matrix.areas?.find((a) => a.id === "installer");
if (!installer) fail("production-readiness.json missing installer area");
if (installer.maturity === "TrackARequired") {
  fail(
    "installer maturity still TrackARequired — update matrix after Slice 1 land",
  );
}
if (!["MinorImprovement", "Complete"].includes(installer.maturity)) {
  fail(`unexpected installer maturity: ${installer.maturity}`);
}

const packageJson = JSON.parse(
  fs.readFileSync(path.join(root, "package.json"), "utf8"),
);
if (!packageJson.scripts?.["installer:build"]) {
  fail("package.json missing installer:build script");
}
if (!packageJson.scripts?.["verify:installer-foundation"]) {
  fail("package.json missing verify:installer-foundation script");
}

const artifactDirs = [
  path.join(root, "target/release/bundle/nsis"),
  path.join(root, "app/src-tauri/target/release/bundle/nsis"),
];
let setupExe = null;
for (const dir of artifactDirs) {
  if (!fs.existsSync(dir)) continue;
  const hit = fs
    .readdirSync(dir)
    .find((f) => f.toLowerCase().endsWith("-setup.exe"));
  if (hit) {
    setupExe = path.join(dir, hit);
    break;
  }
}

if (setupExe) {
  console.log(
    `verify-installer-foundation: ok (config + hooks + docs; package present: ${path.relative(root, setupExe)})`,
  );
} else {
  console.log(
    "verify-installer-foundation: ok (config + hooks + docs; run pnpm installer:build to produce setup.exe)",
  );
}
