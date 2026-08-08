#!/usr/bin/env node
/**
 * P21.S1 — Capability Completion Contract (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-capability-completion: ${msg}`);
  process.exit(1);
}

const required = [
  "packages/kernel/src/operator/plan.rs",
  "packages/kernel/src/operator/compose.rs",
  "packages/kernel/src/operator/mod.rs",
  "docs/capability-runtime/product-proof/P21_A1_GOAL_COMPLETION_AUDIT.md",
  "docs/capability-runtime/product-proof/P21_S1_CAPABILITY_COMPLETION_CONTRACT.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
const compose = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/compose.rs"),
  "utf8",
);
const mod = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/mod.rs"),
  "utf8",
);
const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/P21_S1_CAPABILITY_COMPLETION_CONTRACT.md",
  ),
  "utf8",
);

if (!plan.includes("browser.open_beside")) {
  fail("plan must compose browser.open_beside");
}
if (!plan.includes("CapabilityOperation::Snap")) {
  fail("open_beside plan must include Window Snap");
}
if (!plan.includes("CapabilityOperation::Enumerate")) {
  fail("open_beside plan must include layout verify Enumerate");
}
if (!plan.includes('Some("left"') && !plan.includes('Some("left".into())')) {
  fail("open_beside must snap beside target left");
}
if (!plan.includes('Some("right"') && !plan.includes('Some("right".into())')) {
  fail("open_beside must snap opened surface right");
}

if (!compose.includes("compose_completion")) {
  fail("compose must implement Completion Contract helper");
}
if (!compose.includes("beside_layout_verified")) {
  fail("compose must verify observable beside layout");
}
if (!compose.includes('"completed"') || !compose.includes('"partial"')) {
  fail("compose must distinguish completed vs partial");
}
if (!compose.includes("open_beside_never_claims_layout_after_open_only")) {
  fail("compose tests must prove Conversation cannot overclaim beside");
}

if (!mod.includes("plans_browser_open_beside_completion_contract")) {
  fail("operator tests must cover open_beside multi-step plan");
}

for (const token of [
  "Capability Completion Contract",
  "Opened beside",
  "browser.open_beside",
  "partial",
]) {
  if (!report.includes(token)) {
    fail(`P21.S1 report missing ${token}`);
  }
}

console.log("verify-capability-completion: ok");
