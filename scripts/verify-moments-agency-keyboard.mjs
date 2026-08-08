#!/usr/bin/env node
/**
 * P17.S3 — Moments agency Esc / Enter / Conversation focus return.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-moments-agency-keyboard: ${message}`);
  process.exit(1);
}

const hookPath = path.join(root, "app/src/hooks/useAgencyKeyboard.ts");
const previewPath = path.join(
  root,
  "app/src/components/objects/ContinuePreviewObject.tsx",
);
const resumePath = path.join(root, "app/src/components/ResumeContextPanel.tsx");
const operatorPath = path.join(
  root,
  "app/src/components/operator/OperatorRoot.tsx",
);
const voicePath = path.join(
  root,
  "app/src/components/operator/VoiceMicButton.tsx",
);
const reportPath = path.join(
  root,
  "docs/ui/P17_S3_MOMENTS_AGENCY_KEYBOARD_CONTINUITY.md",
);

for (const p of [hookPath, previewPath, resumePath, operatorPath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const hook = fs.readFileSync(hookPath, "utf8");
const preview = fs.readFileSync(previewPath, "utf8");
const resume = fs.readFileSync(resumePath, "utf8");
const operator = fs.readFileSync(operatorPath, "utf8");
const voice = fs.readFileSync(voicePath, "utf8");

if (!hook.includes("focusConversationInput")) {
  fail("hook must restore Conversation focus");
}
if (!hook.includes('CONVERSATION_INPUT_ID = "workspace-conversation-input"')) {
  fail("hook must target workspace-conversation-input");
}
if (!hook.includes('event.key === "Escape"')) {
  fail("hook must handle Escape dismiss");
}
if (!hook.includes('event.key === "Enter"')) {
  fail("hook must handle Enter primary");
}
if (!hook.includes("isTextEntryTarget")) {
  fail("hook must skip text-entry targets");
}
if (!preview.includes("useAgencyKeyboard")) {
  fail("preview agency card must use useAgencyKeyboard");
}
if (!preview.includes('data-agency-card="preview"')) {
  fail("preview must mark data-agency-card");
}
if (!resume.includes("useAgencyKeyboard")) {
  fail("delete confirm must use useAgencyKeyboard");
}
if (!resume.includes('data-agency-card="confirm_delete"')) {
  fail("delete confirm must mark data-agency-card");
}
if (!resume.includes("!expandHost")) {
  fail("preview must render inline when expandHost is null");
}
if (!operator.includes('id="workspace-conversation-input"')) {
  fail("composer must expose workspace-conversation-input id");
}
if (!operator.includes('data-voice-ready')) {
  fail("Soft Send data-voice-ready must remain");
}
if (!voice.includes('event.key === "Escape"') || !voice.includes("addEventListener(\"keydown\"")) {
  fail("Voice Esc/keydown ownership must remain");
}
if (!voice.includes('event.key === "Enter"') || !voice.includes('event.key === " "')) {
  fail("Voice Enter/Space stop must remain");
}

console.log("verify-moments-agency-keyboard: ok");
