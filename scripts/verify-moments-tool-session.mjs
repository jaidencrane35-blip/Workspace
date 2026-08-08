#!/usr/bin/env node
/**
 * P18.S2 — Moments tool session continuity (Save inline; session above remount).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-moments-tool-session: ${message}`);
  process.exit(1);
}

const sessionPath = path.join(root, "app/src/components/MomentsToolSession.tsx");
const appPath = path.join(root, "app/src/App.tsx");
const savePath = path.join(root, "app/src/components/SaveContextPanel.tsx");
const resumePath = path.join(root, "app/src/components/ResumeContextPanel.tsx");
const activePath = path.join(root, "app/src/components/ActiveMoment.tsx");
const reportPath = path.join(
  root,
  "docs/capability-runtime/product-proof/P18_S2_MOMENTS_TOOL_SESSION_CONTINUITY.md",
);

for (const p of [
  sessionPath,
  appPath,
  savePath,
  resumePath,
  activePath,
  reportPath,
]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const session = fs.readFileSync(sessionPath, "utf8");
const app = fs.readFileSync(appPath, "utf8");
const save = fs.readFileSync(savePath, "utf8");
const resume = fs.readFileSync(resumePath, "utf8");
const active = fs.readFileSync(activePath, "utf8");

if (!session.includes("MomentsToolSessionProvider")) {
  fail("session continuity holder missing");
}
if (!app.includes("MomentsToolSessionProvider")) {
  fail("App must mount MomentsToolSessionProvider");
}
if (!app.includes("MOMENTS_VIEWS")) {
  fail("App must preserve focus among Moments views");
}
if (!resume.includes("useMomentsToolSession")) {
  fail("Resume must use MomentsToolSession");
}
if (!save.includes("useMomentsToolSession")) {
  fail("Save must use MomentsToolSession");
}
if (!save.includes('data-moments-save="inline"') && !save.includes("inline")) {
  fail("Save must support inline write when expandHost null");
}
if (/expandHost \? createPortal\(writeForm, expandHost\) : null/.test(save)) {
  fail("Save must not portal-or-null (blank attach shell)");
}
if (!active.includes("momentsViews") && !active.includes('view === "home"')) {
  fail("ActiveMoment must preserve pin across Moments views");
}
if (!resume.includes("useAgencyKeyboard")) {
  fail("P17.S3 agency keyboard must remain");
}
if (!resume.includes("moments.length === 0")) {
  fail("P18.S1 browse truth must remain");
}

console.log("verify-moments-tool-session: ok");
