/**
 * P16.38 — Product Intelligence Boundary guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-product-intelligence: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/productIntelligence.ts",
  "app/src/lib/capabilityRegistry.ts",
  "app/src/lib/workspaceContext.ts",
  "tests/product-intelligence-battery.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_38_PRODUCT_INTELLIGENCE.md",
  "docs/capability-runtime/PRODUCT_PROOF_RULE.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const intel = fs.readFileSync(
  path.join(root, "app/src/lib/productIntelligence.ts"),
  "utf8",
);
const registry = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityRegistry.ts"),
  "utf8",
);
const discovery = registry;
const battery = fs.readFileSync(
  path.join(root, "tests/product-intelligence-battery.test.ts"),
  "utf8",
);
const proof = fs.readFileSync(
  path.join(root, "docs/capability-runtime/PRODUCT_PROOF_RULE.md"),
  "utf8",
);
const ctx = fs.readFileSync(
  path.join(root, "app/src/lib/workspaceContext.ts"),
  "utf8",
);

for (const token of [
  "PRODUCT_INTELLIGENCE_GAPS",
  "hasEngineeringLeak",
  "classifyProductExperience",
  "OutsideWorkspace",
]) {
  if (!intel.includes(token)) {
    fail(`productIntelligence must define ${token}`);
  }
}

if (!registry.includes("purpose:")) {
  fail("CapabilityNode must declare purpose");
}
if (!discovery.includes("Related / similar / alternatives")) {
  fail("discovery must surface related/similar/alternatives from Registry");
}
if (discovery.includes("Capability graph declares")) {
  fail("Owner-facing discovery must not mention Capability graph");
}
if (!battery.includes("500") || !battery.includes("hasEngineeringLeak")) {
  fail("battery must cover ≥500 product interactions with leak checks");
}
if (!proof.includes("Repository Evidence Before Architectural Confidence")) {
  fail("PRODUCT_PROOF_RULE must adopt Repository Evidence Before Architectural Confidence");
}
if (!ctx.includes("back|previous") && !ctx.includes("back|previous|go back")) {
  // pattern is go back|back|previous in alternation
  if (!/back\|previous|previous\|go back|go back\|back\|previous/.test(ctx)) {
    fail("Context must own Back/Previous continuity");
  }
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-product-intelligence.mjs")) {
  fail("package.json must wire verify-product-intelligence");
}

console.log("verify-product-intelligence: ok");
