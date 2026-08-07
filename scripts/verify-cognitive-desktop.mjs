/**
 * P16.33 — Cognitive desktop reasoning + registry self-description guards.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-cognitive-desktop: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/semanticIntentEngine.ts",
  "app/src/lib/capabilityRegistry.ts",
  "app/src/lib/intentGrammar.ts",
  "tests/cognitive-desktop-nl.test.ts",
  "docs/capability-runtime/product-proof/VOICE_P16_33_COGNITIVE_DESKTOP.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const engine = fs.readFileSync(
  path.join(root, "app/src/lib/semanticIntentEngine.ts"),
  "utf8",
);
const registry = fs.readFileSync(
  path.join(root, "app/src/lib/capabilityRegistry.ts"),
  "utf8",
);
const grammar = fs.readFileSync(
  path.join(root, "app/src/lib/intentGrammar.ts"),
  "utf8",
);

for (const token of [
  "reasonCognitiveDesktop",
  "generateRecoveryGuidance",
  "winMoveMonitor",
  "winEnumerate",
]) {
  if (!engine.includes(token) && token !== "generateRecoveryGuidance") {
    fail(`semantic engine must include ${token}`);
  }
}
if (!engine.includes("reasonCognitiveDesktop")) {
  fail("semantic engine must reason cognitively (reasonCognitiveDesktop)");
}
if (!registry.includes("generateRecoveryGuidance")) {
  fail("registry must generate recovery guidance");
}
if (!registry.includes("won’t overclaim") && !registry.includes("won't overclaim")) {
  fail("discovery must self-describe can/cannot (won’t overclaim)");
}
if (!registry.includes("node.requirements")) {
  fail("discovery must surface requirements from the graph");
}
if (!grammar.includes("next\\s+to") && !grammar.includes("next to")) {
  fail("grammar must reason beside/next to (not open-only)");
}
// Beside must be parsed before bare “I want X”
const besideIdx = grammar.indexOf("beside|next");
const goalIdx = grammar.indexOf("take\\s+me\\s+to|go\\s+to|i\\s+want");
if (besideIdx < 0 || goalIdx < 0 || besideIdx > goalIdx) {
  fail("beside/next-to must be parsed before bare goal phrasing");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-cognitive-desktop.mjs")) {
  fail("package.json must wire verify-cognitive-desktop into pnpm test");
}

console.log("verify-cognitive-desktop: ok");
