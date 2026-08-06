#!/usr/bin/env node
/**
 * Verifies two-form Zero-Trap shell + click-restore Desktop Operator.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-shell-zero-trap: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/shellStateMachine.ts",
  "app/src/lib/shellRuntime.ts",
  "app/src/lib/shellWindows.ts",
  "app/src/components/operator/DesktopOperator.tsx",
  "docs/execution-program-desktop-operator-shell-completion.md",
  "docs/shell/FUTURE_INPUT_ARCHITECTURE.md",
  "app/src-tauri/src/commands/shell.rs",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const machine = fs.readFileSync(
  path.join(root, "app/src/lib/shellStateMachine.ts"),
  "utf8",
);
for (const token of [
  "SHELL_TRANSITIONS",
  "SHELL_EXITS",
  "verifyZeroTrapGraph",
  "Zero-Trap",
]) {
  if (!machine.includes(token)) {
    fail(`shellStateMachine missing ${token}`);
  }
}

if (machine.includes("Expand Workspace") || machine.includes("Settings")) {
  fail("shellStateMachine must not expose Expanded/Settings as shell exits");
}

const desktop = fs.readFileSync(
  path.join(root, "app/src/components/operator/DesktopOperator.tsx"),
  "utf8",
);
for (const token of ["openConversation", "startOperatorDrag", "onDoubleClick"]) {
  if (!desktop.includes(token)) {
    fail(`DesktopOperator missing ${token}`);
  }
}
if (desktop.includes("onContextMenu") || desktop.includes("op-desktop-menu")) {
  fail("DesktopOperator must not require a context menu to restore");
}
if (desktop.includes("openExpanded") || desktop.includes("Hide")) {
  fail("DesktopOperator must not offer Expand/Hide shell paths");
}
// Drag must not steal click — startDragging only after move threshold.
if (/onPointerDown[\s\S]{0,400}startOperatorDrag/.test(desktop)) {
  fail("startOperatorDrag must not run on pointerdown (breaks click restore)");
}

for (const token of ["Open Conversation", "Exit Workspace", "Collapse"]) {
  if (!machine.includes(token)) {
    fail(`shellStateMachine missing exit label ${token}`);
  }
}

const runtime = fs.readFileSync(
  path.join(root, "app/src/lib/shellRuntime.ts"),
  "utf8",
);
if (!runtime.includes("normalizeShellMode")) {
  fail("shellRuntime must migrate legacy modes");
}
if (runtime.includes("export type ShellMode = 0 | 1 | 2 | 3")) {
  fail("ShellMode must be two-form (0 | 1) only");
}

const windows = fs.readFileSync(
  path.join(root, "app/src/lib/shellWindows.ts"),
  "utf8",
);
if (!windows.includes("installMainCloseCollapse")) {
  fail("shellWindows must install main close → operator");
}
if (!windows.includes("exitWorkspace")) {
  fail("shellWindows must support exitWorkspace");
}
if (!windows.includes("setSkipTaskbar(true)")) {
  fail("collapsed conversation must leave the taskbar (mode switch, not resize)");
}
if (windows.includes("hideShellToTaskbar")) {
  fail("hide-to-taskbar is not a user-facing shell form");
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
if (rootUi.includes("Expand") && rootUi.includes("op-shell__btn--primary")) {
  fail("OperatorRoot must not expose Expand chrome");
}
if (rootUi.includes("OperatorSettingsPanel") || rootUi.includes("Settings")) {
  fail("OperatorRoot must not expose Settings chrome");
}

console.log("verify-shell-zero-trap: ok");
