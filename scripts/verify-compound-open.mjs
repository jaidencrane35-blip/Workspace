#!/usr/bin/env node
/**
 * P21.S2 — Compound Goal Decomposition (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-compound-open: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/compoundOpen.ts",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/operator/intentMap.ts",
  "packages/kernel/src/operator/plan.rs",
  "packages/kernel/src/operator/mod.rs",
  "packages/kernel/src/operator/compose.rs",
  "tests/compound-open.test.ts",
  "docs/capability-runtime/product-proof/P21_S2_COMPOUND_GOAL_DECOMPOSITION.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const compound = fs.readFileSync(
  path.join(root, "app/src/lib/compoundOpen.ts"),
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
const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
const mod = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/mod.rs"),
  "utf8",
);
const compose = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/compose.rs"),
  "utf8",
);
const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/P21_S2_COMPOUND_GOAL_DECOMPOSITION.md",
  ),
  "utf8",
);

for (const token of [
  "resolveCompoundOpen",
  "encodeCompoundOpenTargets",
  "CompoundOpenTarget",
]) {
  if (!compound.includes(token)) fail(`compoundOpen.ts missing ${token}`);
}

if (!bridge.includes("resolveCompoundOpen")) {
  fail("intentBridge must call resolveCompoundOpen");
}
if (!bridge.includes("compoundOpen")) {
  fail("intentBridge must declare compoundOpen IntentAction");
}
if (!map.includes("open_compound")) {
  fail("intentMap must map compoundOpen → open_compound");
}
if (!plan.includes("desktop.open_compound") || !plan.includes("open_compound")) {
  fail("plan must compose desktop.open_compound");
}
if (!mod.includes("desktop.open_compound")) {
  fail("operator execute must handle desktop.open_compound");
}
if (!compose.includes("desktop.open_compound")) {
  fail("compose must Completion-Contract compound opens");
}

for (const token of [
  "Compound Goal Decomposition",
  "Open Cursor and Chrome",
  "Capability Completion Contract",
  "partial",
]) {
  if (!report.includes(token)) fail(`P21.S2 report missing ${token}`);
}

console.log("verify-compound-open: ok");
