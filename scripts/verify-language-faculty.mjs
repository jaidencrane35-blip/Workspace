#!/usr/bin/env node
/**
 * P24.S2 — Language Faculty Boundary (machine check).
 *
 * Enforces ADR-P24-CONFLICT-A Option C structurally:
 * Faculty may propose Meaning; it must not acquire capability selection,
 * execution, IPC, memory writes, or Kernel authority.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-language-faculty: ${msg}`);
  process.exit(1);
}

const FACULTY = "app/src/lib/languageFaculty.ts";
const BRIDGE = "app/src/lib/intentBridge.ts";
const CONTRACT = "app/src/lib/goalContract.ts";
const TESTS = "tests/language-faculty.test.ts";
const ADR = "docs/architecture/ADR-P24-CONFLICT-A-LANGUAGE-FACULTY.md";

for (const rel of [FACULTY, BRIDGE, CONTRACT, TESTS, ADR]) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const read = (rel) => fs.readFileSync(path.join(root, rel), "utf8");
const faculty = read(FACULTY);
const bridge = read(BRIDGE);
const tests = read(TESTS);

// 1. Faculty port and boundary primitives exist.
for (const token of [
  "export interface LanguageFaculty",
  "proposeMeaning",
  "export type MeaningProposal",
  "export function validateMeaningProposal",
  "export const deterministicLanguageFaculty",
  "export function comprehendViaFaculty",
  "comprehend(",
]) {
  if (!faculty.includes(token)) fail(`languageFaculty must define ${token}`);
}

// 2. MeaningProposal must not be a parallel execution type.
if (/export (?:type|interface) MeaningProposal\s*=\s*\{/.test(faculty)) {
  // Object form is only OK if it does not declare authority fields.
  const start = faculty.indexOf("MeaningProposal");
  const slice = faculty.slice(start, start + 800);
  for (const field of ["steps", "plan", "capabilityId", "provider", "operation"]) {
    if (new RegExp(`\\b${field}\\s*[?:]`).test(slice)) {
      fail(`MeaningProposal must not declare authority field "${field}"`);
    }
  }
}
if (!/export type MeaningProposal\s*=\s*GoalContract\s*;/.test(faculty)) {
  fail(
    "MeaningProposal must be exactly GoalContract (ADR: GoalContract is sufficient — no intersection, no parallel type)",
  );
}

// 3. Forbidden dependencies — Faculty must not import authority surfaces.
const forbiddenImports = [
  "./capabilityRegistry",
  "./executionPlanner",
  "./operator/intentMap",
  "./operator/runtimeBridge",
  "./ipc",
  "./substitutionProhibition",
  "./intelligenceRouting",
  "@tauri-apps",
];
for (const mod of forbiddenImports) {
  if (new RegExp(`from\\s+["']${mod.replace(/[./@]/g, "\\$&")}`).test(faculty)) {
    fail(`languageFaculty must not import ${mod}`);
  }
}

// 4. Forbidden *uses* — imports and call sites. Tokens may appear only as
// needles inside the authority-leak detector regex (that is detection, not use).
const facultyWithoutLeakNeedles = faculty
  .replace(/const AUTHORITY_LEAK[\s\S]*?;/, "")
  .replace(/const PROVIDER_LEAK[\s\S]*?;/, "")
  .replace(/\/\*\*[\s\S]*?\*\//g, "")
  .replace(/\/\/.*$/gm, "");
for (const token of [
  "CapabilityIntent",
  "execute_capability_intent",
  "invokeIpc",
  "buildExecutionPlan",
  "commitWorkspaceContext",
  "create_memory",
  "createMemory",
  "ProviderInvoke",
]) {
  if (facultyWithoutLeakNeedles.includes(token)) {
    fail(`languageFaculty must not reference ${token}`);
  }
}

// 5. No model / vendor / inference runtime in this scaffold.
for (const token of [
  "openai",
  "anthropic",
  "ollama",
  "chatgpt",
  "apiKey",
  "API_KEY",
  "inference",
  "ChatCompletion",
]) {
  if (new RegExp(token, "i").test(faculty)) {
    fail(`languageFaculty must not embed a model backend (${token})`);
  }
}

// 6. Deterministic Faculty must wrap comprehend — not reimplement cascade.
if (
  !/deterministicLanguageFaculty: LanguageFaculty = \{\s*proposeMeaning\(utterance: string\): MeaningProposal \{\s*return comprehend\(utterance\);\s*\},\s*\};/.test(
    faculty,
  )
) {
  fail(
    "deterministic Faculty proposeMeaning must be exactly `return comprehend(utterance)` — no rewriting, no invented evidence",
  );
}
if (/resolveIntentCore|browserOpen|winEnumerate/.test(faculty)) {
  fail("Faculty must not contain the intent/action cascade");
}

// 7. Validator must reject authority fields and fall back on failure.
if (!/FORBIDDEN_PROPOSAL_KEYS/.test(faculty) && !/must not carry authority field/.test(faculty)) {
  fail("validator must reject authority-bearing proposal fields");
}
if (!/deterministic_fallback|faculty_rejected/.test(faculty)) {
  fail("comprehendViaFaculty must fall back to deterministic comprehend on rejection");
}

// 8. Intent bridge enters through the Faculty boundary.
if (!bridge.includes("comprehendViaFaculty")) {
  fail("intentBridge must call comprehendViaFaculty");
}
if (!/from\s+["']\.\/languageFaculty["']/.test(bridge)) {
  fail("intentBridge must import languageFaculty");
}

// 9. Tests prove the required cases.
for (const token of [
  "What is the time in Queensland",
  "What does minimize mean",
  "Open Notepad",
  "count",
  "What application am I using",
  "Open that",
  "validateMeaningProposal",
  "CapabilityIntent",
  "faculty_rejected",
]) {
  if (!tests.includes(token)) {
    fail(`language-faculty tests must cover: ${token}`);
  }
}

const pkg = read("package.json");
if (!pkg.includes("verify-language-faculty.mjs")) {
  fail("package.json must wire verify-language-faculty");
}

console.log("verify-language-faculty: ok");
