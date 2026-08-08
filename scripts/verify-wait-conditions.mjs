#!/usr/bin/env node
/**
 * C-VER-003 — Wait Conditions (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-wait-conditions: ${msg}`);
  process.exit(1);
}

const required = [
  "packages/windows-integration/src/uia.rs",
  "packages/kernel/src/capability_runtime/window_provider.rs",
  "packages/kernel/src/capability_runtime/types.rs",
  "packages/kernel/src/operator/plan.rs",
  "packages/kernel/src/operator/compose.rs",
  "app/src/lib/intentBridge.ts",
  "tests/wait-conditions.test.ts",
  "docs/capability-runtime/product-proof/P22_S5_WAIT_CONDITIONS.md",
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const types = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/types.rs"),
  "utf8",
);
const provider = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/window_provider.rs"),
  "utf8",
);
const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
const compose = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/compose.rs"),
  "utf8",
);
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
const atlas = fs.readFileSync(
  path.join(root, "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md"),
  "utf8",
);
const report = fs.readFileSync(
  path.join(
    root,
    "docs/capability-runtime/product-proof/P22_S5_WAIT_CONDITIONS.md",
  ),
  "utf8",
);
const uia = fs.readFileSync(
  path.join(root, "packages/windows-integration/src/uia.rs"),
  "utf8",
);

if (!types.includes("WaitCondition")) {
  fail("types.rs must define WaitCondition");
}
if (!types.includes("wait_condition")) {
  fail("types.rs must parse wait_condition");
}

for (const token of [
  "WaitCondition",
  "wait_timeout_ms",
  "condition_met",
  "condition_timeout",
  "MAX_MS",
]) {
  if (!provider.includes(token)) fail(`window_provider.rs missing ${token}`);
}

if (!plan.includes("window.wait_condition")) {
  fail("plan.rs must compose window.wait_condition");
}
if (!plan.includes("click_control") || !plan.includes("type_control")) {
  fail("plan.rs must keep click_control / type_control unchanged");
}

if (!compose.includes("window.wait_condition")) {
  fail("compose.rs must Completion-Contract wait_condition");
}

if (!bridge.includes("winWaitCondition") || !bridge.includes("resolveWindowWaitCondition")) {
  fail("intentBridge must resolve wait intents");
}

if (!uia.includes("schedule_appear") || !uia.includes("schedule_disappear")) {
  fail("uia memory port must support delayed appear/disappear for wait tests");
}

if (!atlas.includes("C-VER-003") || !atlas.includes("Wait Conditions")) {
  fail("Atlas must record C-VER-003 Wait Conditions");
}

if (!report.includes("C-VER-003") || !report.includes("bounded timeout")) {
  fail("Product Proof harness must cite C-VER-003 and bounded timeout");
}
if (!report.includes("never invent") && !report.includes("never invents")) {
  fail("Product Proof must forbid invented success");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-wait-conditions.mjs")) {
  fail("package.json must wire verify-wait-conditions.mjs");
}

// Reject agent / scheduler surfaces in this milestone.
for (const banned of ["SendInput", "agent_loop", "retry_forever", "Duration::MAX"]) {
  if (provider.includes(banned)) {
    fail(`window_provider must not introduce ${banned}`);
  }
}

console.log("verify-wait-conditions: ok");
