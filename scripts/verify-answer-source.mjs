#!/usr/bin/env node
/**
 * P23.S3 — Answer Source Ladder (machine check).
 *
 * Structural guarantees:
 *  - the ladder answers "which trusted source knows this", never "which
 *    capability should execute";
 *  - no answer source performs IPC, names a capability or provider, or
 *    constructs an action;
 *  - the local clock reads the runtime's time-zone database rather than
 *    arithmetic on offsets;
 *  - there is exactly one time-zone authority in the repository.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-answer-source: ${msg}`);
  process.exit(1);
}

const LADDER = "app/src/lib/answerSource.ts";
const TEMPORAL = "app/src/lib/temporalAnswerSource.ts";
const BRIDGE = "app/src/lib/intentBridge.ts";
const ROUTING = "app/src/lib/intelligenceRouting.ts";
const TESTS = "tests/answer-source.test.ts";

for (const rel of [LADDER, TEMPORAL, BRIDGE, ROUTING, TESTS]) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const read = (rel) => fs.readFileSync(path.join(root, rel), "utf8");
const ladder = read(LADDER);
const temporal = read(TEMPORAL);
const bridge = read(BRIDGE);
const routing = read(ROUTING);
const tests = read(TESTS);

// --- The ladder is a source contract, not a plan.
for (const token of [
  "AnswerRung",
  "AnswerSource",
  "resolveAnswer",
  "answerForGoal",
  "deterministic-local",
  "authorized-external",
  "honest-limitation",
]) {
  if (!ladder.includes(token)) fail(`${LADDER} missing ${token}`);
}

// --- No answer source may execute, select, or transport anything.
for (const [rel, src] of [
  [LADDER, ladder],
  [TEMPORAL, temporal],
]) {
  for (const banned of [
    "@tauri-apps",
    "invokeIpc",
    "execute_capability_intent",
    "CapabilityIntent",
    "capabilityRegistry",
    "executionPlan",
    "resolveDesktopEntity",
  ]) {
    if (src.includes(banned)) {
      fail(`${rel} must not reference ${banned} — answer sources never execute or select`);
    }
  }
  if (/kind:\s*"(appOpen|browserOpen|collapse|navigate|win[A-Z]|screenshot)/.test(src)) {
    fail(`${rel} constructs a desktop action — answer sources produce text only`);
  }
  if (/\bIntentAction\b/.test(src)) {
    fail(`${rel} references IntentAction — the ladder deals in meaning, not actions`);
  }
}

// --- Daylight saving belongs to the runtime, never to us.
if (!temporal.includes("Intl.DateTimeFormat")) {
  fail(`${TEMPORAL} must obtain time from Intl.DateTimeFormat`);
}
if (!temporal.includes("timeZone:")) {
  fail(`${TEMPORAL} must resolve times through an IANA timeZone`);
}
if (/getTimezoneOffset|UTC\s*\+\s*\d|60\s*\*\s*60\s*\*\s*1000/.test(temporal)) {
  fail(`${TEMPORAL} must not compute offsets — DST is the time-zone database's job`);
}

// --- One time-zone authority.
if (/Australia\/|America\/|Europe\//.test(routing)) {
  fail(`${ROUTING} must not carry its own zone table — delegate to ${TEMPORAL}`);
}
if (!routing.includes("resolveRegionZone")) {
  fail(`${ROUTING} must resolve regions through the temporal answer source`);
}

// --- Wired at the single Intent entry point, ahead of the desktop cascade.
if (!bridge.includes("answerForGoal")) {
  fail(`${BRIDGE} must consult the answer ladder`);
}
const entry = bridge.slice(bridge.indexOf("export function resolveIntentWithGoal"));
if (entry.indexOf("answerForGoal") > entry.indexOf("resolveIntentCore")) {
  fail("the answer ladder must be consulted before the desktop cascade runs");
}

// --- Tests must prove the Owner-visible failure and the boundary.
for (const token of [
  "What is the time in Queensland, Australia?",
  "Australia/Brisbane",
  "deterministic-local",
  "chatgpt",
]) {
  if (!tests.includes(token)) fail(`${TESTS} missing proof for ${token}`);
}

const pkg = read("package.json");
if (!pkg.includes("verify-answer-source.mjs")) {
  fail("package.json must wire verify-answer-source.mjs");
}

console.log("verify-answer-source: ok");
