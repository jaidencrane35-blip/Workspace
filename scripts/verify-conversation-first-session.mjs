#!/usr/bin/env node
/**
 * P20.S1 — Empty Conversation First-Session Cue.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-conversation-first-session: ${message}`);
  process.exit(1);
}

const operatorPath = path.join(
  root,
  "app/src/components/operator/OperatorRoot.tsx",
);
const voicePath = path.join(
  root,
  "app/src/components/operator/VoiceMicButton.tsx",
);
const cssPath = path.join(root, "app/src/App.css");
const reportPath = path.join(
  root,
  "docs/capability-runtime/product-proof/P20_S1_EMPTY_CONVERSATION_FIRST_SESSION_CUE.md",
);

for (const p of [operatorPath, voicePath, cssPath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const operator = fs.readFileSync(operatorPath, "utf8");
const voice = fs.readFileSync(voicePath, "utf8");
const css = fs.readFileSync(cssPath, "utf8");

if (!operator.includes("showFirstSessionCue")) {
  fail("OperatorRoot must gate first-session cue");
}
if (!operator.includes("data-first-session")) {
  fail("empty invite must mark data-first-session");
}
if (!operator.includes("FIRST_SESSION_INVITE") && !operator.includes("Say or type what you need")) {
  fail("must invite action with calm copy");
}
if (!operator.includes('placeholder={COMPOSER_PLACEHOLDER}') && !operator.includes('placeholder="Say or type')) {
  fail("composer must present a meaningful placeholder");
}
if (/placeholder=""/.test(operator)) {
  fail("composer must not keep an empty placeholder");
}
if (!operator.includes("messages.length === 0")) {
  fail("cue must require empty Conversation");
}
if (!operator.includes("!composerBusy")) {
  fail("cue must hide while busy");
}
if (!operator.includes("!momentsActive") && !operator.includes("momentsActive")) {
  fail("cue must consider Moments-active suppression");
}
if (!operator.includes("!voiceCapturing")) {
  fail("cue must hide during voice capture");
}
if (!voice.includes("onCaptureActiveChange")) {
  fail("VoiceMicButton must report capture activity");
}
if (!css.includes("op-shell__invite")) {
  fail("invite styles missing");
}
// No onboarding / catalogue / modal framework.
for (const bad of [
  "OnboardingWizard",
  "CapabilityCatalog",
  "Walkthrough",
  "TutorialModal",
  "feature list",
]) {
  if (operator.includes(bad)) fail(`must not introduce ${bad}`);
}

console.log("verify-conversation-first-session: ok");
