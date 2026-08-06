#!/usr/bin/env node
/**
 * Verifies P12.7 Operator Intelligence Foundation + Operator Authority Rule.
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
  "docs/operator/OPERATOR_INTELLIGENCE_FOUNDATION.md",
  "docs/operator/OPERATOR_CONTRACTS.md",
  "docs/operator/CONVERSATION_OPERATOR_PROTOCOL.md",
  "docs/operator/OPERATOR_RUNTIME_PROTOCOL.md",
  "docs/operator/OPERATOR_STATE_MACHINE.md",
  "docs/operator/COMPOSITION_CATALOGUE.md",
  "docs/operator/policies/CLARIFICATION_POLICY.md",
  "docs/operator/policies/TRUTHFULNESS_POLICY.md",
  "docs/operator/policies/ORCHESTRATION_POLICY.md",
  "docs/operator/policies/PERMISSION_POLICY.md",
  "docs/operator/policies/RESPONSE_COMPOSITION_POLICY.md",
  "docs/operator/policies/CONTEXT_LIFETIME_POLICY.md",
  "docs/operator/product-proof/OPERATOR_PRODUCT_PROOF.md",
  "app/src/lib/operator/intelligence.ts",
  "app/src/lib/operator/runtimeBridge.ts",
  "app/src/lib/operator/planner.ts",
  "tests/operator-intelligence.test.ts",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const authority = fs.readFileSync(
  path.join(root, "docs/operator/OPERATOR_AUTHORITY_RULE.md"),
  "utf8",
);
for (const token of [
  "Conversation never invokes providers directly",
  "Operator alone",
  "Providers remain completely independent",
]) {
  if (!authority.includes(token)) {
    fail(`OPERATOR_AUTHORITY_RULE missing: ${token}`);
  }
}

const composition = fs.readFileSync(
  path.join(root, "docs/operator/CAPABILITY_COMPOSITION_RULE.md"),
  "utf8",
);
if (!composition.includes("Independently useful") || !composition.includes("Composable")) {
  fail("CAPABILITY_COMPOSITION_RULE must require independent + composable");
}

const protocol = fs.readFileSync(
  path.join(root, ".cursor/rules/constitutional-execution-protocol.mdc"),
  "utf8",
);
if (!protocol.includes("Operator Authority Rule")) {
  fail("protocol must document Operator Authority Rule");
}
if (!protocol.includes("Capability Composition Rule")) {
  fail("protocol must document Capability Composition Rule");
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
for (const banned of [
  "read_clipboard",
  "write_clipboard",
  "execute_application_operation",
  "execute_window_operation",
  "invokeIpc",
]) {
  if (rootUi.includes(banned)) {
    fail(`OperatorRoot must not call ${banned} (Operator Authority)`);
  }
}
if (!rootUi.includes("handleOperatorUtterance")) {
  fail("OperatorRoot must speak only through handleOperatorUtterance");
}

const router = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CAPABILITY_ROUTER_SPECIFICATION.md"),
  "utf8",
);
if (!router.includes("Operator Intelligence")) {
  fail("Capability Router spec must include Operator Intelligence in pipeline");
}

console.log("verify-operator-intelligence: ok");
