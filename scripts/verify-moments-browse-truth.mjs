#!/usr/bin/env node
/**
 * P18.S1 — Home and Continue share ActiveMoment Moments list (no false empty).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-moments-browse-truth: ${message}`);
  process.exit(1);
}

const appPath = path.join(root, "app/src/App.tsx");
const activePath = path.join(root, "app/src/components/ActiveMoment.tsx");
const homePath = path.join(root, "app/src/components/HomeWorkspacePanel.tsx");
const resumePath = path.join(root, "app/src/components/ResumeContextPanel.tsx");
const reportPath = path.join(
  root,
  "docs/capability-runtime/product-proof/P18_S1_MOMENTS_BROWSE_TRUTH.md",
);

for (const p of [appPath, activePath, homePath, resumePath, reportPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const app = fs.readFileSync(appPath, "utf8");
const active = fs.readFileSync(activePath, "utf8");
const home = fs.readFileSync(homePath, "utf8");
const resume = fs.readFileSync(resumePath, "utf8");

if (!app.includes("ActiveMomentProvider")) {
  fail("App must mount ActiveMomentProvider on the product path");
}
if (!active.includes("moments: contexts") && !active.includes("moments: contexts,")) {
  // value includes moments: contexts
  if (!/moments:\s*contexts/.test(active)) {
    fail("ActiveMoment must expose moments from list_saved_contexts contexts");
  }
}
if (!active.includes("list_saved_contexts")) {
  fail("ActiveMoment must load via list_saved_contexts");
}
if (!home.includes("useActiveMoment") || !home.includes("moments")) {
  fail("Home must browse via useActiveMoment().moments");
}
if (home.includes("invokeIpc")) {
  fail("Home must not fetch Moments separately from ActiveMoment");
}
if (!resume.includes("moments.length === 0")) {
  fail("Continue empty gate must use moments.length, not stub primary");
}
if (resume.includes("!primary && step === \"browse\"")) {
  fail("Continue must not gate empty on !primary");
}
if (!resume.includes('data-moments-browse="continue"')) {
  fail("Continue must present Moments browse list");
}
if (!home.includes('data-moments-browse="home"')) {
  fail("Home must mark Moments browse list");
}
if (!resume.includes("useAgencyKeyboard")) {
  fail("P17.S3 agency keyboard must remain");
}
if (!resume.includes("deletedAck")) {
  fail("P17.S5 delete inline ack must remain");
}

console.log("verify-moments-browse-truth: ok");
