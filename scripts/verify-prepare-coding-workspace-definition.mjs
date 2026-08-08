#!/usr/bin/env node
/**
 * B-DEF-001 / C-PROC-002 — authoritative procedure-definition check.
 *
 * Definition-only: this verifier proves Atlas schema completeness and safety
 * constraints. It does not claim runtime implementation or Product Proof.
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
  "| **Status** | **PLANNED** |",
  "definition complete; implementation not started",
  "C-PROC-002.1 Scope and completion",
  "C-PROC-002.2 Named-target resolution",
  "C-PROC-002.3 Deterministic step table",
  "C-PROC-002.4 Authorization",
  "C-PROC-002.5 Failure rules",
  "C-PROC-002.6 Retry boundary",
  "C-PROC-002.7 Future Owner Product Proof",
  "C-PROC-002.8 Explicit non-goals",
]) {
  if (!procedure.includes(token)) {
    fail(`C-PROC-002 missing required section/token: ${token}`);
  }
}

const requiredHeader =
  "| Step ID | Action | Required Capability | Target | Preconditions | Observable Success Condition | Timeout | Retry Policy | Failure Outcome |";
if (!procedure.includes(requiredHeader)) {
  fail("step table does not contain every required B-DEF-001 field");
}

const stepIds = [
  "PCW-001",
  "PCW-002",
  "PCW-003",
  "PCW-004",
  "PCW-005",
  "PCW-006",
];
for (const stepId of stepIds) {
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

for (const resolutionClass of [
  "Explicitly named target",
  "Existing known target",
  "Ambiguous target",
  "Missing target",
]) {
  if (!procedure.includes(resolutionClass)) {
    fail(`missing named-target rule: ${resolutionClass}`);
  }
}

for (const safetyToken of [
  "There is no",
  "default coding application",
  "does **not** authorize selection",
  "All target references are resolved atomically before the first desktop effect",
  "Providers never call each other",
  "Permission denial is never retried around",
  "MAX_INTERACTION_ATTEMPTS = 2",
  "current C-VER-002 runtime wiring is click/type-only",
]) {
  if (!procedure.includes(safetyToken)) {
    fail(`missing target/authority/retry safety token: ${safetyToken}`);
  }
}

for (const failure of [
  "Application not open",
  "Application cannot be located after launch/focus",
  "Control cannot be located",
  "Interaction failure",
  "Verification timeout",
  "Retry exhaustion",
]) {
  if (!procedure.includes(`| ${failure} |`)) {
    fail(`missing required failure rule: ${failure}`);
  }
}

for (const proofToken of [
  "**Status:** Definition complete; not executed; not accepted.",
  "Prepare my coding workspace with Notepad and Calculator.",
  "Prepare my coding workspace.",
  "NoSuchCodingAppZZZ",
  "Partial-completion behaviour",
  "Only the",
  "Owner may mark Product Proof, Trusted, or Production.",
]) {
  if (!procedure.includes(proofToken)) {
    fail(`future Product Proof definition missing: ${proofToken}`);
  }
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
  !resolvedBlockers.includes("RESOLVED")
) {
  fail("resolved blocker register must retain B-DEF-001 history");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-prepare-coding-workspace-definition.mjs")) {
  fail("package.json must wire this verifier into repository validation");
}

console.log("verify-prepare-coding-workspace-definition: ok");
