#!/usr/bin/env node
/**
 * P17.S5 — Moments owns lifecycle ack; no sticky Delete ok-banner; calm restore outcome.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-moments-status-ownership: ${message}`);
  process.exit(1);
}

const resumePath = path.join(root, "app/src/components/ResumeContextPanel.tsx");
const savePath = path.join(root, "app/src/components/SaveContextPanel.tsx");
const appPath = path.join(root, "app/src/App.tsx");
const reportPath = path.join(
  root,
  "docs/ui/P17_S5_MOMENTS_STATUS_OWNERSHIP.md",
);

for (const p of [resumePath, savePath, appPath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const resume = fs.readFileSync(resumePath, "utf8");
const save = fs.readFileSync(savePath, "utf8");
const app = fs.readFileSync(appPath, "utf8");

if (/onMessage\(`Deleted/.test(resume) || /onMessage\("Deleted/.test(resume)) {
  fail("Delete must not use onMessage ok-banner");
}
if (!resume.includes("deletedAck") || !resume.includes("is gone")) {
  fail("Delete must use inline Moments deletedAck");
}
if (!resume.includes("clearStatusChrome") || !resume.includes("onMessage(null)")) {
  fail("Resume must clear stale status chrome on new actions");
}
if (!resume.includes("describeRestoreOutcome")) {
  fail("Restore done must use describeRestoreOutcome Owner language");
}
if (resume.includes("Outcome:") && resume.includes("replace(/_/g")) {
  fail("Restore done must not show engineering Outcome: snake_case line");
}
if (!resume.includes("You’re back") && !resume.includes("You're back")) {
  fail("You’re back card must remain (S2)");
}
if (/onMessage\(`Saved/.test(save) || /onMessage\("Saved/.test(save)) {
  fail("Save success cohesion (S2) must remain");
}
if (!save.includes("onMessage(null)")) {
  fail("Save must clear stale ok-banner on new actions");
}
if (!app.includes("ws-toast__dismiss") || !app.includes("Dismiss")) {
  fail("Error banners must offer Dismiss");
}
if (!app.includes("setTimeout") || !app.includes("setMessage(null)")) {
  fail("Ok banners must auto-clear (ephemeral)");
}

console.log("verify-moments-status-ownership: ok");
