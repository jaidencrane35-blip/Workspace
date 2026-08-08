#!/usr/bin/env node
/**
 * P23.S4 — Observation Answer Bridge (machine check).
 *
 * Structural guarantees:
 *  - the observation rung declares what it needs and composes what comes back,
 *    but cannot obtain anything itself;
 *  - the bridge asks for exactly one observation, so it cannot grow into a
 *    capability selector;
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
const TESTS = "tests/observation-answer.test.ts";

for (const rel of [LADDER, SOURCE, BRIDGE, FACADE, TESTS]) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const read = (rel) => fs.readFileSync(path.join(root, rel), "utf8");
const ladder = read(LADDER);
const source = read(SOURCE);
const bridge = read(BRIDGE);
const facade = read(FACADE);
const tests = read(TESTS);

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

// --- One need. More than one would make this a selector.
const needs = /export type ObservationNeed =([^;]+);/.exec(ladder);
if (!needs) fail(`${LADDER} must declare ObservationNeed`);
const declared = needs[1].split("|").map((s) => s.trim()).filter(Boolean);
if (declared.length !== 1 || declared[0] !== '"open-windows"') {
  fail(
    `ObservationNeed must remain the single semantic need "open-windows" (found ${declared.join(" | ")}) — widening it needs its own slice`,
  );
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

const pkg = read("package.json");
if (!pkg.includes("verify-observation-answer.mjs")) {
  fail("package.json must wire verify-observation-answer.mjs");
}

console.log("verify-observation-answer: ok");
