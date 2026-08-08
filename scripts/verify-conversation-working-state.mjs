#!/usr/bin/env node
/**
 * P17.S4 — Conversation working-state continuity (busy from Send through IPC).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-conversation-working-state: ${message}`);
  process.exit(1);
}

const rootUi = path.join(
  root,
  "app/src/components/operator/OperatorRoot.tsx",
);
const intelligence = path.join(root, "app/src/lib/operator/intelligence.ts");
const appPath = path.join(root, "app/src/App.tsx");
const voicePath = path.join(
  root,
  "app/src/components/operator/VoiceMicButton.tsx",
);
const reportPath = path.join(
  root,
  "docs/ui/P17_S4_CONVERSATION_WORKING_STATE_CONTINUITY.md",
);

for (const p of [rootUi, intelligence, appPath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const ui = fs.readFileSync(rootUi, "utf8");
const intel = fs.readFileSync(intelligence, "utf8");
const app = fs.readFileSync(appPath, "utf8");
const voice = fs.readFileSync(voicePath, "utf8");

if (!ui.includes("WORKING_ACK") && !ui.includes("Working on that")) {
  fail("OperatorRoot must show immediate working acknowledgement");
}
if (!ui.includes("toolBusy")) {
  fail("OperatorRoot must accept toolBusy bridge from App");
}
if (!ui.includes("composerBusy")) {
  fail("OperatorRoot must combine turn busy with toolBusy");
}
if (!ui.includes("holdBusy")) {
  fail("stream reveal must support holdBusy (busy not owned by streamText)");
}
if (!ui.includes("setBusy(true)")) {
  fail("turn must setBusy(true) for Conversation work");
}
// Busy must be set in submit path before handleIntent (order check).
const submitIdx = ui.indexOf("const submitUtterance");
const setBusyInSubmit = ui.indexOf("setBusy(true)", submitIdx);
const handleIntentCall = ui.indexOf("await handleIntent(", submitIdx);
if (setBusyInSubmit < 0 || handleIntentCall < 0 || setBusyInSubmit > handleIntentCall) {
  fail("setBusy(true) must precede handleIntent in submitUtterance");
}
if (!ui.includes('data-voice-ready')) {
  fail("Soft Send data-voice-ready must remain");
}
if (!app.includes("toolBusy={busy}")) {
  fail("App must bridge Moments busy into OperatorRoot toolBusy");
}
if (!intel.includes("options?.onWorking") && !intel.includes("onWorking?.(")) {
  fail("support path must invoke onWorking before export_support_bundle");
}
const supportIdx = intel.indexOf('intent.kind === "supportBundle"');
const onWorkingIdx = intel.indexOf("onWorking", supportIdx);
const exportIdx = intel.indexOf("export_support_bundle", supportIdx);
if (supportIdx < 0 || onWorkingIdx < 0 || exportIdx < 0 || onWorkingIdx > exportIdx) {
  fail("Creating… onWorking must run before export_support_bundle");
}
if (!voice.includes('event.key === "Escape"') || !voice.includes('event.key === "Enter"')) {
  fail("Voice Esc/Enter ownership must remain");
}

console.log("verify-conversation-working-state: ok");
