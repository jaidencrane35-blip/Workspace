#!/usr/bin/env node
/**
 * C-OBS-003 — Desktop UI Tree (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-desktop-ui-tree: ${msg}`);
  process.exit(1);
}

const required = [
  "packages/windows-integration/src/uia.rs",
  "packages/kernel/src/capability_runtime/window_provider.rs",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/operator/intentMap.ts",
  "tests/desktop-ui-tree.test.ts",
  "docs/capability-runtime/product-proof/P22_S2_DESKTOP_UI_TREE.md",
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const uia = fs.readFileSync(
  path.join(root, "packages/windows-integration/src/uia.rs"),
  "utf8",
);
const types = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/types.rs"),
  "utf8",
);
const provider = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/window_provider.rs"),
  "utf8",
);
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
const map = fs.readFileSync(
  path.join(root, "app/src/lib/operator/intentMap.ts"),
  "utf8",
);
const atlas = fs.readFileSync(
  path.join(root, "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md"),
  "utf8",
);
const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/P22_S2_DESKTOP_UI_TREE.md",
  ),
  "utf8",
);

for (const token of [
  "UiAutomationPort",
  "enumerate_controls",
  "MemoryUiAutomationPort",
  "C-OBS-003",
]) {
  if (!uia.includes(token)) fail(`uia.rs missing ${token}`);
}

if (!types.includes("EnumerateControls")) {
  fail("types.rs must define EnumerateControls");
}
if (!types.includes('"enumerate_controls"')) {
  fail("types.rs must serialize enumerate_controls");
}

if (!provider.includes("enumerate_controls")) {
  fail("WindowProvider must expose enumerate_controls");
}
if (!provider.includes("ui_automation")) {
  fail("WindowPorts must include ui_automation");
}

if (!bridge.includes("winEnumerateControls")) {
  fail("intentBridge must resolve winEnumerateControls");
}
if (!bridge.includes("C-OBS-003")) {
  fail("intentBridge must cite C-OBS-003");
}

if (!map.includes('operation: "enumerate_controls"')) {
  fail("intentMap must map winEnumerateControls → enumerate_controls");
}

if (!atlas.includes("C-OBS-003")) {
  fail("Atlas must record C-OBS-003");
}
if (!atlas.includes("Desktop UI Tree")) {
  fail("Atlas must name Desktop UI Tree");
}

if (!report.includes("C-OBS-003")) fail("Product Proof report must cite C-OBS-003");
if (!report.includes("observation only")) {
  fail("Product Proof report must state observation-only scope");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-desktop-ui-tree.mjs")) {
  fail("package.json must wire verify-desktop-ui-tree.mjs");
}

if (!types.includes("FindControl")) {
  fail("types.rs must retain FindControl for observation");
}

console.log("verify-desktop-ui-tree: ok");
