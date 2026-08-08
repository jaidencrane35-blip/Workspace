#!/usr/bin/env node
/**
 * P17.S2 — Moments Save/Restore success: one Owner acknowledgement (no ok-banner duplex).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-moments-success-cohesion: ${message}`);
  process.exit(1);
}

const resumePath = path.join(root, "app/src/components/ResumeContextPanel.tsx");
const savePath = path.join(root, "app/src/components/SaveContextPanel.tsx");
const reportPath = path.join(
  root,
  "docs/ui/P17_S2_MOMENTS_SUCCESS_COHESION.md",
);

for (const p of [resumePath, savePath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const resume = fs.readFileSync(resumePath, "utf8");
const save = fs.readFileSync(savePath, "utf8");

if (resume.includes("Resume finished")) {
  fail("Resume success must not emit Resume finished ok-banner");
}
if (!resume.includes("You’re back") && !resume.includes("You're back")) {
  fail("Resume done card must keep You’re back acknowledgement");
}
if (!resume.includes("execute_resume_plan")) {
  fail("Resume approve path must still execute_resume_plan");
}
// P17.S5: delete ack is inline Moments-owned (not sticky ok-banner).
if (/onMessage\(`Deleted/.test(resume) || /onMessage\("Deleted/.test(resume)) {
  fail("Delete success must not emit Deleted ok-banner");
}
if (!resume.includes("deletedAck") && !resume.includes("is gone")) {
  fail("Delete acknowledgement must remain as Moments-owned inline ack");
}
if (!resume.includes("onError(formatError")) {
  fail("Resume failure path must still use onError");
}

if (/onMessage\(`Saved/.test(save) || /onMessage\("Saved/.test(save)) {
  fail("Save success must not emit Saved ok-banner");
}
if (!save.includes("Saved into this place")) {
  fail("Save success must keep Saved into this place acknowledgement");
}
if (!save.includes("save_workspace_context")) {
  fail("Save path must still call save_workspace_context");
}
if (!save.includes("onError(formatError")) {
  fail("Save failure path must still use onError");
}

console.log("verify-moments-success-cohesion: ok");
