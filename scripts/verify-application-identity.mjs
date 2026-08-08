#!/usr/bin/env node
/**
 * P23.S6 — Observed Application Identity (machine check).
 *
 * One claim is enforced here: when Workspace names the application behind a
 * window, that name was observed, not worked out.
 *
 * The check follows the value along the only path it may travel — Windows
 * reports an executable image basename during capture, the capture hands it to
 * the window snapshot, the snapshot to the item, the item across IPC, and the
 * answer source reads it — and then proves the two ways that chain could be
 * cheated are absent: an application name written into the source, and an
 * application name derived from the window title.
 *
 * It also holds the observation to the process right it already had. Naming an
 * application must not become a reason to ask Windows for more.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-application-identity: ${msg}`);
  process.exit(1);
}

const WIN32 = "packages/windows-integration/src/win32.rs";
const CAPTURE = "packages/windows-integration/src/capture.rs";
const SNAPSHOT = "packages/windows-integration/src/enumerator.rs";
const ITEM = "packages/kernel/src/capability_runtime/types.rs";
const WINDOW_PROVIDER = "packages/kernel/src/capability_runtime/window_provider.rs";
const APPLICATION_PROVIDER =
  "packages/kernel/src/capability_runtime/application_provider.rs";
const IPC_VIEW = "app/src/lib/operator/types.ts";
const LADDER = "app/src/lib/answerSource.ts";
const SOURCE = "app/src/lib/observationAnswerSource.ts";
const TESTS = "tests/application-identity.test.ts";

const files = {};
for (const rel of [
  WIN32,
  CAPTURE,
  SNAPSHOT,
  ITEM,
  WINDOW_PROVIDER,
  APPLICATION_PROVIDER,
  IPC_VIEW,
  LADDER,
  SOURCE,
  TESTS,
]) {
  const abs = path.join(root, rel);
  if (!fs.existsSync(abs)) fail(`missing ${rel}`);
  files[rel] = fs.readFileSync(abs, "utf8");
}

// ── 1. The chain is unbroken ────────────────────────────────────────────────
// Each link must carry the field. A link that drops it is exactly the defect
// P23.S4 and P23.S5 hit: the value existed and never arrived.
const links = [
  [WIN32, /process_name: unsafe \{ process_image_basename\(process_id\) \}/, "Windows capture must read the process image basename"],
  [CAPTURE, /pub process_name: Option<String>/, "the captured window must declare the observed application"],
  [CAPTURE, /process_name: self\.process_name\.clone\(\)/, "the snapshot conversion must carry the observed application"],
  [SNAPSHOT, /pub process_name: Option<String>/, "the window snapshot must declare the observed application"],
  [ITEM, /pub process_name: Option<String>/, "the window item must declare the observed application"],
  [WINDOW_PROVIDER, /process_name: window\.process_name\.clone\(\)/, "the window provider must copy the observed application onto the item"],
  [APPLICATION_PROVIDER, /process_name: window\.process_name\.clone\(\)/, "the application provider must copy the observed application onto the item"],
  [IPC_VIEW, /processName\?: string \| null/, "the IPC view must carry the observed application"],
  [LADDER, /processName\?: string \| null/, "the observation result must carry the observed application"],
  [SOURCE, /processName/, "the answer source must read the observed application"],
];
for (const [rel, pattern, message] of links) {
  if (!pattern.test(files[rel])) fail(`${message} (${rel})`);
}

// ── 2. Absence stays absent ─────────────────────────────────────────────────
// Windows withholds this for protected processes. Optionality is the whole
// safeguard: a default would turn "not reported" into a claim.
if (/process_name: Option<String>\s*=|process_name\.unwrap_or/.test(files[ITEM] + files[SNAPSHOT] + files[WINDOW_PROVIDER])) {
  fail("an unreported application must stay unreported — no default may stand in for it");
}
if (!/if \(!application\)/.test(files[SOURCE])) {
  fail("the answer source must handle an observation that named no application");
}

// ── 3. Nothing is inferred ──────────────────────────────────────────────────
// Two cheats would satisfy every behavioural test while destroying the claim:
// a name written into the source, and a name read out of the title.
const APPLICATION_LITERAL = /["'`][^"'`\n]*\.exe["'`]/;
if (APPLICATION_LITERAL.test(files[SOURCE])) {
  fail("the answer source must not contain an application name of its own");
}
if (/(?:application|process)\w*\s*[=:]\s*[^\n]*\btitle\b/i.test(files[SOURCE])) {
  fail("an application name must never be derived from a window title");
}
for (const rel of [WINDOW_PROVIDER, APPLICATION_PROVIDER]) {
  if (/process_name:\s*Some\(/.test(files[rel])) {
    fail(`${rel} must pass the observed application through, never construct one`);
  }
}

// ── 4. The process right is unchanged ───────────────────────────────────────
// The identity comes from the query right the capture already held. Anything
// broader would be this slice quietly buying new authority over processes.
const opens = files[WIN32].match(/OpenProcess\(\s*([A-Z_]+)/g) ?? [];
if (opens.length !== 1 || !opens[0].includes("PROCESS_QUERY_LIMITED_INFORMATION")) {
  fail("the observation must keep using only PROCESS_QUERY_LIMITED_INFORMATION");
}
for (const forbidden of [
  /\bPROCESS_ALL_ACCESS\b/,
  /\bPROCESS_VM_READ\b/,
  /\bPROCESS_TERMINATE\b/,
  /\bTerminateProcess\s*\(/,
]) {
  if (forbidden.test(files[WIN32])) {
    fail(`observing an application must not reach for ${forbidden.source}`);
  }
}

// ── 5. The claim is proven by test ──────────────────────────────────────────
// Synthetic identities that appear nowhere else: an answer containing them can
// only have read them from the observation it was given.
for (const token of [
  "quokka-editor.exe",
  "zarnak-browser.exe",
  "didn’t tell me which application",
]) {
  if (!files[TESTS].includes(token)) {
    fail(`the identity tests must exercise ${token}`);
  }
}

console.log("verify-application-identity: ok");
