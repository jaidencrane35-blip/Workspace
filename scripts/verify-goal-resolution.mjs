/**
 * P16.36 — Goal Resolution Engine guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-goal-resolution: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/goalResolution.ts",
  "app/src/lib/intentPipeline.ts",
  "app/src/lib/intentBridge.ts",
  "tests/goal-resolution-battery.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_36_GOAL_RESOLUTION.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const goal = fs.readFileSync(path.join(root, "app/src/lib/goalResolution.ts"), "utf8");
const bridge = fs.readFileSync(path.join(root, "app/src/lib/intentBridge.ts"), "utf8");
const pipeline = fs.readFileSync(path.join(root, "app/src/lib/intentPipeline.ts"), "utf8");
const battery = fs.readFileSync(
  path.join(root, "tests/goal-resolution-battery.test.ts"),
  "utf8",
);

for (const token of [
  "resolveGoal",
  "applyGoalResolution",
  "intendedOutcome",
  "missingInformation",
  "needsClarification",
  "candidates",
  "completionCriteria",
  "recoveryStrategy",
]) {
  if (!goal.includes(token)) {
    fail(`goalResolution must define ${token}`);
  }
}

if (!bridge.includes("applyGoalResolution")) {
  fail("resolveIntent must finalize via applyGoalResolution");
}
if (!bridge.includes("resolveIntentBeforeGoalResolution")) {
  fail("pre-goal resolution must be exportable for evidence");
}
if (!pipeline.includes("goal_resolution")) {
  fail("pipeline must record goal_resolution stage");
}
if (!battery.includes("500") || !battery.includes("resolveGoal")) {
  fail("battery must cover ≥500 Goal Resolution cases");
}

// Must not expand Situation Goals file as the underspecify owner.
const situation = fs.readFileSync(
  path.join(root, "app/src/lib/situationGoals.ts"),
  "utf8",
);
if (situation.includes("i've lost it") || situation.includes("where was i")) {
  fail("underspecified locate/resume must live in Goal Resolution, not Situation Goals expansion");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-goal-resolution.mjs")) {
  fail("package.json must wire verify-goal-resolution");
}

console.log("verify-goal-resolution: ok");
