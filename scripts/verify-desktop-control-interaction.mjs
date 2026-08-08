#!/usr/bin/env node
/**
 * C-ACT-004 / C-ACT-005 — Desktop control interaction (machine check).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-desktop-control-interaction: ${msg}`);
  process.exit(1);
}

const required = [
  "packages/windows-integration/src/uia.rs",
  "packages/kernel/src/capability_runtime/window_provider.rs",
  "packages/kernel/src/operator/plan.rs",
  "packages/kernel/src/operator/compose.rs",
  "app/src/lib/intentBridge.ts",
  "tests/desktop-control-interaction.test.ts",
  "docs/capability-runtime/product-proof/P22_S4_DESKTOP_CONTROL_INTERACTION.md",
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const uia = fs.readFileSync(
  path.join(root, "packages/windows-integration/src/uia.rs"),
  "utf8",
);
const types = fs.readFileSync(
  path.join(root, "packages/kernel/src/capability_runtime/types.rs"),
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
    "docs/capability-runtime/product-proof/P22_S4_DESKTOP_CONTROL_INTERACTION.md",
  ),
  "utf8",
);

for (const token of [
  "invoke_control",
  "set_control_value",
  "UiInteractionOutcome",
  "verified",
]) {
  if (!uia.includes(token)) fail(`uia.rs missing ${token}`);
}

if (!types.includes("InvokeControl") || !types.includes("SetControlValue")) {
  fail("types.rs must define InvokeControl and SetControlValue");
}

if (!plan.includes("window.click_control") || !plan.includes("window.type_control")) {
  fail("plan.rs must compose click_control and type_control");
}
if (!plan.includes("FindControl") || !plan.includes("InvokeControl")) {
  fail("plan must locate before interact");
}

if (
  !compose.includes("window.click_control") ||
  !compose.includes("window.type_control")
) {
  fail("compose.rs must Completion-Contract click/type compositions");
}

if (!bridge.includes("winClickControl") || !bridge.includes("winTypeControl")) {
  fail("intentBridge must resolve click/type intents");
}
if (!bridge.includes("resolveWindowControlInteraction")) {
  fail("intentBridge must resolve interaction before semantic soft-miss");
}

if (!atlas.includes("C-ACT-004") || !atlas.includes("C-ACT-005")) {
  fail("Atlas must record C-ACT-004 and C-ACT-005");
}
if (!atlas.includes("Capability Pair Rule")) {
  fail("Atlas must keep Capability Pair Rule");
}

if (!report.includes("C-ACT-004") || !report.includes("C-ACT-005")) {
  fail("Product Proof must cite both capability IDs");
}
if (!report.includes("Locate") || !report.includes("never invent")) {
  fail("Product Proof must require locate → interact → verify honesty");
}

const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");
if (!pkg.includes("verify-desktop-control-interaction.mjs")) {
  fail("package.json must wire verify-desktop-control-interaction.mjs");
}

// Reject agent-loop surface in this milestone.
if (uia.includes("SendInput") || uia.includes("mouse_event")) {
  fail("milestone must use UIA patterns, not raw input injection");
}

console.log("verify-desktop-control-interaction: ok");
