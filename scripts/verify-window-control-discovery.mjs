#!/usr/bin/env node
/**
 * C-OBS-004 — Window Control Discovery (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-window-control-discovery: ${msg}`);
  process.exit(1);
}

const required = [
  "packages/windows-integration/src/uia.rs",
  "packages/kernel/src/capability_runtime/window_provider.rs",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/operator/intentMap.ts",
  "tests/window-control-discovery.test.ts",
  "docs/capability-runtime/product-proof/P22_S3_WINDOW_CONTROL_DISCOVERY.md",
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
    "docs/capability-runtime/product-proof/P22_S3_WINDOW_CONTROL_DISCOVERY.md",
  ),
  "utf8",
);

for (const token of ["find_control", "match_control", "C-OBS-004"]) {
  if (!uia.includes(token)) fail(`uia.rs missing ${token}`);
}

if (!types.includes("FindControl")) fail("types.rs must define FindControl");
if (!types.includes('"find_control"')) {
  fail("types.rs must serialize find_control");
}

if (!provider.includes("FindControl") || !provider.includes("find_control")) {
  fail("WindowProvider must handle find_control");
}
if (!provider.includes("control_found") || !provider.includes("control_not_found")) {
  fail("WindowProvider must report truthful found / not-found");
}

if (!bridge.includes("winFindControl") || !bridge.includes("C-OBS-004")) {
  fail("intentBridge must resolve winFindControl for C-OBS-004");
}
if (!map.includes('operation: "find_control"')) {
  fail("intentMap must map winFindControl → find_control");
}

if (!atlas.includes("C-OBS-004") || !atlas.includes("Window Control Discovery")) {
  fail("Atlas must record C-OBS-004 Window Control Discovery");
}
if (!atlas.includes("Readiness")) {
  fail("Atlas must include Capability Readiness Score metadata");
}

if (!report.includes("C-OBS-004")) fail("Product Proof report must cite C-OBS-004");
if (!report.includes("observation only")) {
  fail("Product Proof report must state observation only scope");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-window-control-discovery.mjs")) {
  fail("package.json must wire verify-window-control-discovery.mjs");
}

if (types.includes("InvokeControl") || types.includes("SetControlValue")) {
  fail("C-OBS-004 must not add InvokeControl/SetControlValue (C-ACT later)");
}

console.log("verify-window-control-discovery: ok");
