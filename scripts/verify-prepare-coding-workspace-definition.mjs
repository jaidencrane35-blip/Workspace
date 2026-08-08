#!/usr/bin/env node
/**
 * B-DEF-001 / C-PROC-002 — authoritative procedure-definition check.
 *
 * Definition-only: this verifier proves Atlas schema completeness and the
 * post-audit contract corrections. It does not claim runtime implementation
 * or Product Proof.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const atlasPath = path.join(
  root,
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
);

function fail(message) {
  console.error(`verify-prepare-coding-workspace-definition: ${message}`);
  process.exit(1);
}

if (!fs.existsSync(atlasPath)) {
  fail("missing Workspace Capability Atlas");
}

const atlas = fs.readFileSync(atlasPath, "utf8");
const start = atlas.indexOf("#### C-PROC-002 Prepare Coding Workspace");
const end = atlas.indexOf("#### C-PROC-003 Situation Goals", start);
if (start < 0 || end < 0) {
  fail("cannot isolate the C-PROC-002 Atlas record");
}
const procedure = atlas.slice(start, end);

for (const token of [
  // Status may be PLANNED (definition) or IMPLEMENTED (runtime eng complete; PP open).
  "C-PROC-002.1 Scope and completion",
  "C-PROC-002.2 Entry routing (Continue vs clarification vs procedure)",
  "C-PROC-002.3 Target authority and launchability",
  "C-PROC-002.4 Deterministic step table",
  "C-PROC-002.5 Authorization",
  "C-PROC-002.6 Failure rules",
  "C-PROC-002.7 Timing and retry model",
  "C-PROC-002.8 Future Owner Product Proof",
  "C-PROC-002.9 Explicit non-goals",
]) {
  if (!procedure.includes(token)) {
    fail(`C-PROC-002 missing required section/token: ${token}`);
  }
}

if (
  !procedure.includes("| **Status** | **PLANNED** |") &&
  !procedure.includes("| **Status** | **IMPLEMENTED** |")
) {
  fail("C-PROC-002 Status must be PLANNED or IMPLEMENTED");
}
if (
  procedure.includes("Trusted ................. 100%") ||
  procedure.includes("Production .............. 100%") ||
  procedure.includes("Product Proof ........... 100%")
) {
  fail("definition/runtime slice must not mark Product Proof, Trusted, or Production complete");
}

const requiredHeader =
  "| Step ID | Action | Required Capability | Target | Preconditions | Observable Success Condition | Timeout / bounded timing authority | Retry Policy | Failure Outcome |";
if (!procedure.includes(requiredHeader)) {
  fail("step table does not contain every required B-DEF-001 field");
}

const requiredSteps = ["PCW-001", "PCW-002", "PCW-003", "PCW-004"];
for (const stepId of requiredSteps) {
  const line = procedure
    .split(/\r?\n/)
    .find((candidate) => candidate.startsWith(`| **${stepId}** |`));
  if (!line) {
    fail(`missing deterministic step ${stepId}`);
  }
  const cells = line
    .slice(1, -1)
    .split("|")
    .map((cell) => cell.trim());
  if (cells.length !== 9) {
    fail(`${stepId} must have 9 populated procedure fields (got ${cells.length})`);
  }
  if (cells.some((cell) => cell.length === 0 || cell === "—")) {
    fail(`${stepId} contains an empty procedure field`);
  }
}

for (const retiredStep of ["PCW-005", "PCW-006"]) {
  if (procedure.includes(`| **${retiredStep}** |`)) {
    fail(`${retiredStep} must not remain after contract correction`);
  }
}

for (const resolutionClass of [
  "Explicitly named target",
  "Existing known target",
  "Ambiguous target",
  "Missing target",
  "Intent-known / Kernel-unexecutable",
]) {
  if (!procedure.includes(resolutionClass)) {
    fail(`missing named-target rule: ${resolutionClass}`);
  }
}

for (const dependency of [
  "C-CMP-002",
  "C-CMP-001",
  "C-ACT-001",
  "C-ACT-006",
  "C-VER-001",
  "C-VER-003",
  "C-OBS-001",
  "C-ITL-002",
  "C-ITL-003",
  "C-ITL-004",
  "C-CMP-004",
]) {
  if (!procedure.includes(dependency)) {
    fail(`missing required dependency/authority: ${dependency}`);
  }
}

const depsLine = procedure
  .split(/\r?\n/)
  .find((line) => line.startsWith("| **Dependencies** |"));
if (!depsLine) {
  fail("missing Dependencies row");
}
if (depsLine.includes("C-VER-002")) {
  fail("C-VER-002 must not remain a C-PROC-002 dependency");
}
if (!depsLine.includes("C-CMP-002") || !depsLine.includes("C-CMP-001")) {
  fail("Dependencies must compose C-CMP-002 and C-CMP-001");
}

for (const safetyToken of [
  "There is no",
  "default coding application",
  "Final-active-window state is **not** a mandatory completion criterion",
  "Providers never call each other",
  "automatic Launch/Open re-attempt is **forbidden**",
  "C-VER-002 is **out of scope**",
  "No invented operation deadline",
  "Substring/first-match title search alone is **not** completion truth",
  "hwnd",
  "Private Find loop duplicating C-ACT-001 / C-CMP-002",
  "Private Target A→B→C planner duplicating C-CMP-002",
]) {
  if (!procedure.includes(safetyToken)) {
    fail(`missing target/authority/retry safety token: ${safetyToken}`);
  }
}

for (const failure of [
  "Targetless coding/setup owned by Situation Goals",
  "Missing named target for prepare phrasing",
  "Ambiguous target",
  "Intent-known / Kernel-unexecutable",
  "Application not open",
  "Application cannot be located after open/focus",
  "Control cannot be located",
  "Interaction / permission / unsupported failure",
  "Verification timeout",
]) {
  if (!procedure.includes(`| ${failure} |`)) {
    fail(`missing required failure rule: ${failure}`);
  }
}

for (const proofToken of [
  "Prepare my coding workspace with Cursor and Notepad.",
  "Prepare my coding workspace with Notepad and Calculator.",
  "Prepare my coding workspace.",
  "I need my coding environment.",
  "Visual Studio Code",
  "NoSuchCodingAppZZZ",
  "Partial-completion case",
  "Only the Owner may mark Product Proof, Trusted, or Production.",
]) {
  if (!procedure.includes(proofToken)) {
    fail(`future Product Proof definition missing: ${proofToken}`);
  }
}
if (
  !procedure.includes("**Status:** Definition corrected; not executed; not accepted.") &&
  !procedure.includes(
    "**Status:** Engineering complete; Product Proof **not executed; not accepted**.",
  )
) {
  fail("future Product Proof status line missing or invalid");
}

const blockedRegisterStart = atlas.indexOf("## 5. Blocked Capability Register");
const resolvedRegisterStart = atlas.indexOf(
  "### 5.1 Resolved definition blockers",
  blockedRegisterStart,
);
if (blockedRegisterStart < 0 || resolvedRegisterStart < 0) {
  fail("cannot isolate active/resolved blocker registers");
}
const activeBlockers = atlas.slice(blockedRegisterStart, resolvedRegisterStart);
if (activeBlockers.includes("B-DEF-001")) {
  fail("B-DEF-001 must not remain in the active blocker register");
}
const resolvedBlockers = atlas.slice(
  resolvedRegisterStart,
  atlas.indexOf("## 6. Verification Matrix", resolvedRegisterStart),
);
if (
  !resolvedBlockers.includes("B-DEF-001") ||
  !resolvedBlockers.includes("RESOLVED") ||
  !resolvedBlockers.includes("contract corrected")
) {
  fail("resolved blocker register must retain B-DEF-001 correction history");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-prepare-coding-workspace-definition.mjs")) {
  fail("package.json must wire this verifier into repository validation");
}

console.log("verify-prepare-coding-workspace-definition: ok");
