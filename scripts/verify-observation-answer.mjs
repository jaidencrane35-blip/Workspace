#!/usr/bin/env node
/**
 * P23.S4 / P23.S5 — Observation Answer Bridge (machine check).
 *
 * Structural guarantees:
 *  - the observation rung declares what it needs and composes what comes back,
 *    but cannot obtain anything itself;
 *  - the needs are a fixed, named vocabulary of information, and each is
 *    translated by a total map into a request the Intent Layer already had, so
 *    the bridge cannot grow into a capability selector;
 *  - the Kernel remains the only observer: no answer source performs IPC, and
 *    the Conversation façade still uses the single Capability Runtime entry;
 *  - the composed answer is derived from observation data, never from names
 *    written into the source.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-observation-answer: ${msg}`);
  process.exit(1);
}

const LADDER = "app/src/lib/answerSource.ts";
const SOURCE = "app/src/lib/observationAnswerSource.ts";
const BRIDGE = "app/src/lib/intentBridge.ts";
const FACADE = "app/src/lib/operator/intelligence.ts";
const CONTEXT = "app/src/lib/workspaceContext.ts";
const TESTS = "tests/observation-answer.test.ts";
const ACTIVE_TESTS = "tests/active-window-answer.test.ts";

for (const rel of [LADDER, SOURCE, BRIDGE, FACADE, CONTEXT, TESTS, ACTIVE_TESTS]) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const read = (rel) => fs.readFileSync(path.join(root, rel), "utf8");
const ladder = read(LADDER);
const source = read(SOURCE);
const bridge = read(BRIDGE);
const facade = read(FACADE);
const context = read(CONTEXT);
const tests = read(TESTS);
const activeTests = read(ACTIVE_TESTS);

// --- The observation rung is split so it cannot observe for itself.
for (const token of [
  "ObservationAnswerSource",
  "ObservationNeed",
  "observationNeededFor",
  "composeObservationAnswer",
  "capability-observation",
]) {
  if (!ladder.includes(token)) fail(`${LADDER} missing ${token}`);
}
if (/\bobtain\s*\(/.test(ladder) || /\bobtain\s*\(/.test(source)) {
  fail("an answer source must not obtain an observation — only the Kernel may");
}

// --- A fixed vocabulary of information needs. Growth needs its own slice.
const PERMITTED_NEEDS = ['"open-windows"', '"active-window"'];
const needs = /export type ObservationNeed =([^;]+);/.exec(ladder);
if (!needs) fail(`${LADDER} must declare ObservationNeed`);
const declared = needs[1].split("|").map((s) => s.trim()).filter(Boolean);
const unexpected = declared.filter((need) => !PERMITTED_NEEDS.includes(need));
if (unexpected.length > 0 || declared.length !== PERMITTED_NEEDS.length) {
  fail(
    `ObservationNeed must remain exactly ${PERMITTED_NEEDS.join(" | ")} (found ${declared.join(" | ")}) — widening it needs its own slice`,
  );
}
for (const need of declared) {
  if (!/^"[a-z][a-z-]*"$/.test(need)) {
    fail(
      `${need} is not a semantic need — needs describe information, not capabilities or operations`,
    );
  }
}
// --- Each need has exactly one source, so there is nothing to select between.
const sourceNeeds = [...source.matchAll(/^\s+need: ("[a-z-]+"),$/gm)].map(
  (match) => match[1],
);
if (sourceNeeds.length !== declared.length) {
  fail(
    `every ObservationNeed must have exactly one source (${declared.length} declared, ${sourceNeeds.length} implemented)`,
  );
}
if (new Set(sourceNeeds).size !== sourceNeeds.length) {
  fail("two sources claim the same need — a request must have one meaning");
}

// --- No answer source may execute, transport, or name a capability.
for (const [rel, src] of [
  [LADDER, ladder],
  [SOURCE, source],
]) {
  for (const banned of [
    "@tauri-apps",
    "invokeIpc",
    "execute_capability_intent",
    "execute_window_operation",
    "CapabilityIntent",
    "capabilityRegistry",
  ]) {
    if (src.includes(banned)) {
      fail(`${rel} must not reference ${banned} — the Kernel owns observation`);
    }
  }
  if (/\bIntentAction\b/.test(src)) {
    fail(`${rel} references IntentAction — the ladder deals in meaning, not actions`);
  }
}

// --- The answer is derived, never recited.
if (/\b(chrome|firefox|notepad|explorer|outlook|edge)\b/i.test(source)) {
  fail(`${SOURCE} names an application — answers must come from the observation`);
}
if (!source.includes("observation.items")) {
  fail(`${SOURCE} must compose from the observed items`);
}

// --- The bridge reaches the existing authorized request and overrides nothing.
if (!bridge.includes("observationNeededFor")) {
  fail(`${BRIDGE} must carry the comprehended need to the existing observation`);
}
if (!/isSpeakingAction\(action\)\)\s*return action;/.test(bridge)) {
  fail("the bridge must never override an action the cascade already resolved");
}
// A total map cannot invent a request for a need that has none; a conditional
// chain can. Translation stays translation only while this holds.
if (!/:\s*Record<ObservationNeed,\s*IntentAction>\s*=/.test(bridge)) {
  fail(
    `${BRIDGE} must translate needs through a total Record<ObservationNeed, IntentAction> — a conditional chain would be a selector`,
  );
}
const map = /:\s*Record<ObservationNeed,\s*IntentAction>\s*=\s*\{([\s\S]*?)\n\};/.exec(
  bridge,
);
if (!map) fail(`${BRIDGE} must declare the need → request map`);
for (const need of declared) {
  if (!map[1].includes(`${need}:`)) {
    fail(`${BRIDGE} has no existing request for ${need}`);
  }
}

// --- Conversation still speaks to the Kernel through one entry.
if (!facade.includes("composeObservationAnswer")) {
  fail(`${FACADE} must compose the observation into the answer`);
}
if (!facade.includes("executeCapabilityIntent")) {
  fail(`${FACADE} must obtain observations through the Capability Runtime entry`);
}
if (/invokeIpc<[^>]*>\(\s*["']execute_window/.test(facade)) {
  fail(`${FACADE} must not call a window provider command directly`);
}

// --- Tests must prove derivation and the boundary.
for (const token of [
  "What windows are currently open?",
  "Quokka Ledger",
  "without permission",
  "execute_capability_intent",
]) {
  if (!tests.includes(token)) fail(`${TESTS} missing proof for ${token}`);
}
for (const token of [
  "Which window is active?",
  "Which one am I using?",
  "grounded_reference=prior_desktop_observation",
  "mutually exclusive",
]) {
  if (!activeTests.includes(token)) {
    fail(`${ACTIVE_TESTS} missing proof for ${token}`);
  }
}

// --- Grounding may refine meaning into perception, never into an effect.
if (!context.includes("groundGoalInContext")) {
  fail(`${CONTEXT} must own elliptical reference grounding — it owns pronouns`);
}
const grounding = /export function groundGoalInContext[\s\S]*?\n}/.exec(context);
if (!grounding) fail(`${CONTEXT} must declare groundGoalInContext`);
if (!/outcome: "PERCEIVE_MACHINE"/.test(grounding[0])) {
  fail("grounding must resolve to perception");
}
if (/REACH_STATE|kind:\s*"/.test(grounding[0])) {
  fail("grounding must refine meaning only — it must never produce an action");
}

const pkg = read("package.json");
if (!pkg.includes("verify-observation-answer.mjs")) {
  fail("package.json must wire verify-observation-answer.mjs");
}

console.log("verify-observation-answer: ok");
