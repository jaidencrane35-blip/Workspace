/**
 * P23.S2 — Substitution Prohibition Enforcement architecture guard.
 *
 * Enforcement may refuse a desktop effect. It may never select one, name a
 * provider, or reach the Kernel. This checks that structurally.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-substitution-prohibition: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/substitutionProhibition.ts",
  "app/src/lib/goalContract.ts",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/intelligenceRouting.ts",
  "app/src/lib/conversationGuidance.ts",
  "tests/substitution-prohibition.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const read = (rel) => fs.readFileSync(path.join(root, rel), "utf8");

const enforcement = read("app/src/lib/substitutionProhibition.ts");
const bridge = read("app/src/lib/intentBridge.ts");
const routing = read("app/src/lib/intelligenceRouting.ts");
const guidance = read("app/src/lib/conversationGuidance.ts");
const tests = read("tests/substitution-prohibition.test.ts");

// 1. Enforcement exists and is driven by the Goal Contract.
for (const token of [
  "export function enforceSubstitutionProhibition",
  "export function isAnswerOnlyGoal",
  "export function hasPositiveOutcomeEvidence",
  "GoalContract",
]) {
  if (!enforcement.includes(token)) {
    fail(`substitutionProhibition must define ${token}`);
  }
}

// 2. Enforcement must not import selection, execution, or transport surfaces.
for (const mod of [
  "./capabilityRegistry",
  "./executionPlanner",
  "./intelligenceRouting",
  "./semanticIntentEngine",
  "./operator/intentMap",
  "./operator/runtimeBridge",
  "./ipc",
  "@tauri-apps",
]) {
  if (new RegExp(`from\\s+["']${mod.replace(/[./@]/g, "\\$&")}`).test(enforcement)) {
    fail(`substitutionProhibition must not import ${mod} (enforcement never selects)`);
  }
}

// 3. Enforcement may only construct actions that speak — never an effect.
const SPEAKING = new Set([
  "unknown",
  "capabilityExplain",
  "browserExplain",
  "voiceExplain",
  "voiceStatus",
]);
for (const match of enforcement.matchAll(/\bkind:\s*"([a-zA-Z]+)"/g)) {
  if (!SPEAKING.has(match[1])) {
    fail(
      `substitutionProhibition constructs "${match[1]}" — enforcement may only remove effects, never introduce one`,
    );
  }
}

// 4. No provider identity or new handoff may originate here.
for (const token of ["chatgpt", "https://", "http://", "execute_capability_intent"]) {
  if (enforcement.toLowerCase().includes(token)) {
    fail(`substitutionProhibition must not contain ${token} (no new handoff)`);
  }
}

// 5. The independent-route mark belongs to routing alone.
if (!/informationHandoff:\s*true/.test(routing)) {
  fail("intelligenceRouting must mark its own external information handoff");
}
const marks = [];
for (const rel of fs.readdirSync(path.join(root, "app/src/lib"))) {
  if (!rel.endsWith(".ts")) continue;
  const body = read(`app/src/lib/${rel}`);
  if (/informationHandoff:\s*true/.test(body)) marks.push(rel);
}
if (marks.length !== 1 || marks[0] !== "intelligenceRouting.ts") {
  fail(
    `informationHandoff may only be set by intelligenceRouting.ts (found: ${marks.join(", ") || "none"})`,
  );
}

// 6. Enforcement runs on the live Intent entry, after the action is resolved.
if (!/enforceSubstitutionProhibition\(/.test(bridge)) {
  fail("resolveIntentWithGoal must apply the substitution prohibition");
}
if (!/isQuestionForm\(raw, text\)/.test(bridge)) {
  fail("the collapse branch must not fire on a question containing its vocabulary");
}

// 7. A bare refusal must be distinguishable from an answer.
if (!/softMiss:\s*true/.test(guidance)) {
  fail("conversationGuidance must mark bare refusals as softMiss");
}

// 8. Tests must prove the boundary, not only the classification.
for (const token of [
  "can only remove an effect, never introduce one",
  "introduces no new browser-opening behaviour",
  "does not duplicate the existing external handoff",
  "performs no IPC",
]) {
  if (!tests.includes(token)) {
    fail(`substitution-prohibition tests must prove: ${token}`);
  }
}

const pkg = read("package.json");
if (!pkg.includes("verify-substitution-prohibition.mjs")) {
  fail("package.json must wire verify-substitution-prohibition");
}

console.log("verify-substitution-prohibition: ok");
