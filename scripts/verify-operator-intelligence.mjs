#!/usr/bin/env node
/**
 * Verifies Kernel Operator authority + Presentation Purity (P12 Finalization).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-operator-intelligence: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/operator/OPERATOR_AUTHORITY_RULE.md",
  "docs/operator/CAPABILITY_COMPOSITION_RULE.md",
  "docs/operator/PRESENTATION_PURITY_RULE.md",
  "docs/operator/KERNEL_AUTHORITY_RULE.md",
  "docs/operator/OPERATOR_INTELLIGENCE_FOUNDATION.md",
  "packages/kernel/src/operator/mod.rs",
  "packages/kernel/src/commands/capability_intent.rs",
  "app/src-tauri/src/commands/capability_intent.rs",
  "app/src/lib/operator/intelligence.ts",
  "app/src/lib/operator/intentMap.ts",
  "app/src/lib/operator/runtimeBridge.ts",
  "tests/operator-intelligence.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);
for (const token of [
  "Operator Authority Rule",
  "Capability Composition Rule",
  "Presentation Purity Rule",
  "Kernel Authority Rule",
]) {
  if (!protocol.includes(token)) {
    fail(`protocol must document ${token}`);
  }
}

const libRs = fs.readFileSync(path.join(root, "app/src-tauri/src/lib.rs"), "utf8");
if (!libRs.includes("execute_capability_intent")) {
  fail("Tauri generate_handler must register execute_capability_intent");
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
if (rootUi.includes("invokeIpc")) {
  fail("OperatorRoot must not call invokeIpc (Presentation Purity)");
}
if (!rootUi.includes("handleOperatorUtterance")) {
  fail("OperatorRoot must speak through handleOperatorUtterance");
}

const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/operator/runtimeBridge.ts"),
  "utf8",
);
if (!bridge.includes("execute_capability_intent")) {
  fail("runtimeBridge must use single execute_capability_intent IPC");
}
if (/invokeIpc<[^>]+>\(\s*"(read_clipboard|write_clipboard|execute_)/.test(bridge)) {
  fail("runtimeBridge must not invoke provider-specific commands");
}

const intelligence = fs.readFileSync(
  path.join(root, "app/src/lib/operator/intelligence.ts"),
  "utf8",
);
if (intelligence.includes("compositionId") || intelligence.includes("planFromIntent")) {
  fail("TS intelligence must not own composition/planning");
}
for (const banned of [
  "app/src/lib/operator/planner.ts",
  "app/src/lib/operator/compose.ts",
]) {
  if (fs.existsSync(path.join(root, banned))) {
    fail(`${banned} must not exist (composition belongs in Kernel Operator)`);
  }
}

const kernelOp = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/mod.rs"),
  "utf8",
);
if (!kernelOp.includes("KernelOperator") || !kernelOp.includes("app.open_or_focus")) {
  fail("Kernel Operator must own open composition");
}

const router = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CAPABILITY_ROUTER_SPECIFICATION.md"),
  "utf8",
);
if (!router.includes("Kernel Operator") && !router.includes("execute_capability_intent")) {
  fail("Router spec must reference Kernel Operator / single IPC");
}

const intentBridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
if (/through Capability Runtime|through Window Provider|through Application Provider/i.test(intentBridge)) {
  fail("intentBridge must not leak provider/runtime terminology into replies");
}

console.log("verify-operator-intelligence: ok");
