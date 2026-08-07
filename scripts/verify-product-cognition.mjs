/**
 * P16.35 — Product Cognition gap analysis guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-product-cognition: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/situationGoals.ts",
  "app/src/lib/productCognition.ts",
  "app/src/lib/capabilityRegistry.ts",
  "tests/product-cognition-battery.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_35_PRODUCT_COGNITION.md",
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
const registry = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityRegistry.ts"),
  "utf8",
);
const engine = fs.readFileSync(
  path.join(root, "app/src/lib/semanticIntentEngine.ts"),
  "utf8",
);
const evolution = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityEvolution.ts"),
  "utf8",
);
const battery = fs.readFileSync(
  path.join(root, "tests/product-cognition-battery.test.ts"),
  "utf8",
);

if (!situation.includes("resolveSituationGoal")) {
  fail("situationGoals must export resolveSituationGoal");
}
if (!engine.includes("resolveSituationGoal")) {
  fail("semantic engine must call Situation Goals");
}
if (!registry.includes("discoverability") || !registry.includes("alternatives")) {
  fail("Capability Graph must declare discoverability + alternatives");
}
if (!registry.includes("similar:")) {
  fail("Capability Graph must declare similar capabilities");
}
if (!evolution.includes("everything you (can|know)")) {
  fail("evolution classifier must exclude capability discovery phrasing");
}
if (!battery.includes("250") || !battery.includes("assessCognition")) {
  fail("hostile battery must assess ≥250 requests");
}
if (!battery.includes("misunderstood")) {
  fail("battery must classify misunderstandings");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-product-cognition.mjs")) {
  fail("package.json must wire verify-product-cognition");
}

console.log("verify-product-cognition: ok");
