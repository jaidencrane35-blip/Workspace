#!/usr/bin/env node
/**
 * P19.S1 — Tray Show ↔ ShellMode Continuity (Conversation Form restore).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-tray-shellmode: ${message}`);
  process.exit(1);
}

const trayPath = path.join(root, "app/src-tauri/src/tray.rs");
const shellWinPath = path.join(root, "app/src/lib/shellWindows.ts");
const shellRuntimePath = path.join(root, "app/src/lib/shellRuntime.ts");
const mainPath = path.join(root, "app/src/main.tsx");
const reportPath = path.join(
  root,
  "docs/capability-runtime/product-proof/P19_S1_TRAY_SHELLMODE_CONTINUITY.md",
);

for (const p of [trayPath, shellWinPath, shellRuntimePath, mainPath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const tray = fs.readFileSync(trayPath, "utf8");
const shellWin = fs.readFileSync(shellWinPath, "utf8");
const shellRuntime = fs.readFileSync(shellRuntimePath, "utf8");
const main = fs.readFileSync(mainPath, "utf8");

if (!tray.includes("workspace-show-conversation")) {
  fail("tray must emit workspace-show-conversation");
}
if (!tray.includes("Emitter")) {
  fail("tray must use Emitter for ShellMode restore signal");
}
if (!/emit\(SHOW_CONVERSATION_EVENT|emit\("workspace-show-conversation"/.test(tray)) {
  fail("show_conversation must emit restore event before/with native show");
}
if (!shellRuntime.includes('SHOW_CONVERSATION_EVENT = "workspace-show-conversation"')) {
  fail("shellRuntime must define SHOW_CONVERSATION_EVENT");
}
if (!shellWin.includes("restoreConversationShell")) {
  fail("shellWindows must expose restoreConversationShell");
}
if (!shellWin.includes("installTrayShowConversationRestore")) {
  fail("shellWindows must install tray Show listener");
}
if (!shellWin.includes("saveShellMode(1)")) {
  fail("restore must persist ShellMode Conversation (1)");
}
if (!shellWin.includes("applyShellMode(1)")) {
  fail("restore must apply Conversation Form windows");
}
if (!main.includes("installTrayShowConversationRestore")) {
  fail("main entry must install tray ShellMode restore");
}
// Must not invent a parallel lifecycle framework.
if (/class TrayLifecycle|createTraySession|new LifecycleManager/.test(shellWin)) {
  fail("must not introduce a new lifecycle framework");
}

console.log("verify-tray-shellmode: ok");
