/**
 * P16.37 — Workspace Context Model guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-workspace-context: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/workspaceContext.ts",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/intentPipeline.ts",
  "app/src/lib/capabilityRegistry.ts",
  "tests/workspace-context-battery.test.ts",
  "tests/cognitive-layer-hostile.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_37_WORKSPACE_CONTEXT.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const ctx = fs.readFileSync(path.join(root, "app/src/lib/workspaceContext.ts"), "utf8");
const bridge = fs.readFileSync(path.join(root, "app/src/lib/intentBridge.ts"), "utf8");
const pipeline = fs.readFileSync(path.join(root, "app/src/lib/intentPipeline.ts"), "utf8");
const registry = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityRegistry.ts"),
  "utf8",
);
const battery = fs.readFileSync(
  path.join(root, "tests/workspace-context-battery.test.ts"),
  "utf8",
);
const layers = fs.readFileSync(
  path.join(root, "tests/cognitive-layer-hostile.test.ts"),
  "utf8",
);

for (const token of [
  "resolveFromWorkspaceContext",
  "commitWorkspaceContext",
  "resetWorkspaceContext",
  "getWorkspaceContext",
  "lastAction",
  "unresolvedClarification",
  "again→replay_last_action",
]) {
  if (!ctx.includes(token)) {
    fail(`workspaceContext must define ${token}`);
  }
}

if (!bridge.includes("resolveFromWorkspaceContext")) {
  fail("resolveIntent must consult Workspace Context");
}
if (!bridge.includes("commitWorkspaceContext")) {
  fail("resolveIntent must commit Workspace Context");
}
if (!pipeline.includes("workspace_context")) {
  fail("pipeline must record workspace_context stage");
}
if (!battery.includes("500") || !battery.includes("resetWorkspaceContext")) {
  fail("battery must cover ≥500 contextual cases");
}
if (!layers.includes("hostile cognitive layers")) {
  fail("hostile layer suite required");
}

for (const token of [
  "ownership",
  "permissions",
  "dependencies",
  "benchmark",
  "architecturalJustification",
  "validateCapabilityGraphGovernance",
]) {
  if (!registry.includes(token)) {
    fail(`capability registry governance must include ${token}`);
  }
}

// Must not expand Goal Resolution / Situation Goals as continuity owners.
const goal = fs.readFileSync(path.join(root, "app/src/lib/goalResolution.ts"), "utf8");
const situation = fs.readFileSync(
  path.join(root, "app/src/lib/situationGoals.ts"),
  "utf8",
);
if (goal.includes("do that again") || goal.includes("put it beside")) {
  fail("continuity/again/it-beside must live in Workspace Context, not Goal Resolution");
}
if (situation.includes("do that again") || situation.includes("close that")) {
  fail("pronoun continuity must not expand Situation Goals");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-workspace-context.mjs")) {
  fail("package.json must wire verify-workspace-context");
}

console.log("verify-workspace-context: ok");
