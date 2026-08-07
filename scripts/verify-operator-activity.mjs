/**
 * P16.39 — Product Operator Intelligence / activity-state guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-operator-activity: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/situationGoals.ts",
  "app/src/lib/productIntelligence.ts",
  "tests/operator-activity-battery.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_39_OPERATOR_INTELLIGENCE.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const situation = fs.readFileSync(
  path.join(root, "app/src/lib/situationGoals.ts"),
  "utf8",
);
const intel = fs.readFileSync(
  path.join(root, "app/src/lib/productIntelligence.ts"),
  "utf8",
);
const battery = fs.readFileSync(
  path.join(root, "tests/operator-activity-battery.test.ts"),
  "utf8",
);
const ctx = fs.readFileSync(
  path.join(root, "app/src/lib/workspaceContext.ts"),
  "utf8",
);
const goal = fs.readFileSync(
  path.join(root, "app/src/lib/goalResolution.ts"),
  "utf8",
);

if (!situation.includes("classifyOperatorActivity")) {
  fail("situationGoals must export classifyOperatorActivity");
}
if (!situation.includes("coding|debugging")) {
  fail("Situation Goals must own operator work modes");
}
if (!situation.includes("get back") && !situation.includes("get\\s+back")) {
  fail("Situation Goals must own session resume operator states");
}
if (!situation.includes("Opening Continue for your")) {
  fail("work-mode replies must open Continue without inventing layouts");
}
if (!intel.includes("STATE_VS_GOAL_OWNERS")) {
  fail("productIntelligence must prove STATE_VS_GOAL_OWNERS");
}
if (!intel.includes("operator-activities")) {
  fail("PRODUCT_INTELLIGENCE_GAPS must include operator-activities");
}
if (!battery.includes("750") || !battery.includes("classifyOperatorActivity")) {
  fail("battery must cover ≥750 operator-activity cases");
}

if (/starting my day|i'?m coding|set me up/i.test(ctx)) {
  fail("operator work modes must not expand Workspace Context");
}
if (/starting my day|i'?m coding|set me up/i.test(goal)) {
  fail("operator work modes must not expand Goal Resolution");
}

if (fs.existsSync(path.join(root, "app/src/lib/operatorActivityEngine.ts"))) {
  fail("must not create a new operatorActivityEngine layer");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-operator-activity.mjs")) {
  fail("package.json must wire verify-operator-activity");
}

console.log("verify-operator-activity: ok");
