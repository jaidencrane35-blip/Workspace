#!/usr/bin/env node
/**
 * C-VER-002 — Bounded Retry (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-bounded-retry: ${msg}`);
  process.exit(1);
}

const required = [
  "packages/kernel/src/operator/retry.rs",
  "packages/kernel/src/operator/mod.rs",
  "packages/kernel/src/operator/compose.rs",
  "packages/kernel/src/operator/plan.rs",
  "tests/bounded-retry.test.ts",
  "docs/capability-runtime/product-proof/P22_S6_BOUNDED_RETRY.md",
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const retry = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/retry.rs"),
  "utf8",
);
const opMod = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/mod.rs"),
  "utf8",
);
const compose = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/compose.rs"),
  "utf8",
);
const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
const atlas = fs.readFileSync(
  path.join(root, "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md"),
  "utf8",
);
const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/P22_S6_BOUNDED_RETRY.md",
  ),
  "utf8",
);

for (const token of [
  "MAX_INTERACTION_ATTEMPTS",
  "may_retry",
  "is_non_retryable_status",
  "execute_interaction_with_retry",
  "WaitCondition",
  "RETRY_WAIT_DURATION",
]) {
  if (!retry.includes(token)) fail(`retry.rs missing ${token}`);
}

if (!retry.includes("control_not_found") || !retry.includes("control_click_failed")) {
  fail("retry.rs must distinguish retryable vs non-retryable statuses");
}

if (!opMod.includes("execute_interaction_with_retry")) {
  fail("operator mod must execute bounded retry for interactions");
}
if (
  !opMod.includes('window.click_control') ||
  !opMod.includes('window.type_control')
) {
  fail("operator must wire retry into click_control and type_control");
}

if (!compose.includes("last_interaction_legs")) {
  fail("compose.rs must Completion-Contract multi-attempt click/type results");
}

// Plans for click/type must remain the three-step locate→act→verify shape.
if (!plan.includes("window.click_control") || !plan.includes("window.type_control")) {
  fail("plan.rs must keep click_control and type_control compositions");
}
if (!plan.includes("window.wait_condition")) {
  fail("plan.rs must keep wait_condition (C-VER-003 intact)");
}

if (!atlas.includes("C-VER-002") || !atlas.includes("Retry")) {
  fail("Atlas must record C-VER-002 Retry");
}

if (!report.includes("C-VER-002") || !report.includes("bounded")) {
  fail("Product Proof must cite C-VER-002 and bounded retry");
}
if (!report.includes("never invent") && !report.includes("Never invent")) {
  fail("Product Proof must forbid invented success");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-bounded-retry.mjs")) {
  fail("package.json must wire verify-bounded-retry.mjs");
}

// Reject agent-loop surfaces.
for (const banned of ["Duration::MAX", "loop {}", "agent_loop", "recursive_prompt"]) {
  if (retry.includes(banned)) {
    fail(`retry.rs must not introduce ${banned}`);
  }
}

if (!/MAX_INTERACTION_ATTEMPTS:\s*u32\s*=\s*2/.test(retry)) {
  fail("MAX_INTERACTION_ATTEMPTS must be 2");
}

console.log("verify-bounded-retry: ok");
