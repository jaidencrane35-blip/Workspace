/**
 * P23.S1 — Goal Contract (Outcome-First Comprehension) architecture guard.
 *
 * Enforces the authority boundary structurally: comprehension may represent
 * meaning, and must not acquire capability selection, execution, or IPC.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-goal-contract: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/goalContract.ts",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/workspaceContext.ts",
  "app/src/lib/operator/intelligence.ts",
  "app/src/lib/operator/types.ts",
  "tests/goal-contract.test.ts",
  "docs/architecture/INTELLIGENCE_LAYER_BEHAVIORAL_CONSTITUTION.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const read = (rel) => fs.readFileSync(path.join(root, rel), "utf8");

const contract = read("app/src/lib/goalContract.ts");
const bridge = read("app/src/lib/intentBridge.ts");
const context = read("app/src/lib/workspaceContext.ts");
const facade = read("app/src/lib/operator/intelligence.ts");
const types = read("app/src/lib/operator/types.ts");
const tests = read("tests/goal-contract.test.ts");

// 1. Comprehension represents meaning.
for (const token of [
  "GoalContract",
  "GoalOutcome",
  "GoalMode",
  "GoalDomain",
  "export function comprehend",
  "requestedResult",
  "clarificationNeeded",
  "uncertainties",
  "references",
  "compound",
]) {
  if (!contract.includes(token)) {
    fail(`goalContract must define ${token}`);
  }
}

// 2. Comprehension must not import planning, provider, or transport surfaces.
const forbiddenImports = [
  "./executionPlanner",
  "./capabilityRegistry",
  "./operator/intentMap",
  "./operator/runtimeBridge",
  "./ipc",
  "@tauri-apps",
];
for (const mod of forbiddenImports) {
  if (new RegExp(`from\\s+["']${mod.replace(/[./@]/g, "\\$&")}`).test(contract)) {
    fail(`goalContract must not import ${mod} (Meaning must not select or execute)`);
  }
}

// 3. Comprehension must not carry execution or provider vocabulary.
for (const token of [
  "CapabilityIntent",
  "execute_capability_intent",
  "buildExecutionPlan",
  "capabilityId",
  "invokeIpc",
  "Provider",
]) {
  if (contract.includes(token)) {
    fail(`goalContract must not reference ${token} (Kernel Operator authority)`);
  }
}

// 4. The contract shape must not become a plan.
const shape = contract.slice(
  contract.indexOf("export interface GoalContract"),
  contract.indexOf("export interface GoalContract") + 1400,
);
for (const field of ["steps", "capabilities", "plan", "operation", "provider"]) {
  if (new RegExp(`\\n\\s+${field}[?]?:`).test(shape)) {
    fail(`GoalContract must not declare execution field "${field}"`);
  }
}

// 5. Meaning is comprehended before desktop matching and is preserved.
if (!bridge.includes("export function resolveIntentWithGoal")) {
  fail("intentBridge must expose resolveIntentWithGoal");
}
const declaration = bridge.indexOf("export function resolveIntentWithGoal");
const after = bridge.slice(declaration);
const nextDeclaration = after.indexOf("\nexport ", 1);
const withGoal = nextDeclaration > 0 ? after.slice(0, nextDeclaration) : after;
// Meaning comes first through the Language Faculty Boundary (P24.S2), then
// P23.S5 grounding may refine it. Nothing else may come between.
if (
  !/^\s*const goal = (?:groundGoalInContext\()?(?:comprehendViaFaculty|comprehend)\(raw\)\)?;/m.test(
    withGoal,
  )
) {
  fail("resolveIntentWithGoal must comprehend before resolving an action");
}
const meaningCall = withGoal.includes("comprehendViaFaculty(raw)")
  ? "comprehendViaFaculty(raw)"
  : "comprehend(raw)";
if (withGoal.indexOf(meaningCall) > withGoal.indexOf("resolveIntentCore(raw)")) {
  fail("meaning must be comprehended before desktop matching runs");
}
if (!/commitWorkspaceContext\([^)]*goal\)/s.test(bridge)) {
  fail("resolveIntentWithGoal must commit the Goal Contract (never discard it)");
}
if (!context.includes("currentGoal")) {
  fail("Workspace Context must preserve currentGoal");
}

// 6. The Conversation façade carries meaning to the boundary.
if (!facade.includes("resolveIntentWithGoal")) {
  fail("Conversation façade must use resolveIntentWithGoal");
}
if (!/goal,/.test(facade)) {
  fail("Conversation façade must return the comprehended goal");
}

// 7. Boundary: meaning must not cross the Kernel IPC contract.
const intentShape = types.slice(
  types.indexOf("export interface CapabilityIntent"),
  types.indexOf("export interface OperatorTurnResult"),
);
for (const field of ["goal", "outcome", "mode", "contract"]) {
  if (new RegExp(`\\n\\s+${field}[?]?:`).test(intentShape)) {
    fail(
      `CapabilityIntent must not carry "${field}" — Kernel Operator owns capability selection (see constitution §30 Conflict C)`,
    );
  }
}

// 8. Tests must prove the boundary, not just the classification.
for (const token of [
  "performs no IPC",
  "selects no capability",
  "never invents a canonical identity",
]) {
  if (!tests.includes(token)) {
    fail(`goal-contract tests must prove: ${token}`);
  }
}

const pkg = read("package.json");
if (!pkg.includes("verify-goal-contract.mjs")) {
  fail("package.json must wire verify-goal-contract");
}

console.log("verify-goal-contract: ok");
