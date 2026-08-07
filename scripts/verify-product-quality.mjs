#!/usr/bin/env node
/**
 * P16.PQ1 — Workspace Product Quality Standard machine check.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-product-quality: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/ui/WORKSPACE_PRODUCT_QUALITY_STANDARD.md",
  "docs/ui/WORKSPACE_INTERACTION_LANGUAGE.md",
  "docs/ui/PRODUCT_GRAVITY_RULE.md",
  "app/src/components/operator/OperatorRoot.tsx",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const standard = fs.readFileSync(
  path.join(root, "docs/ui/WORKSPACE_PRODUCT_QUALITY_STANDARD.md"),
  "utf8",
);
for (const token of [
  "Interaction Quality",
  "Conversation Quality",
  "Voice Quality",
  "Motion Quality",
  "Visual Quality",
  "Capability Quality",
  "Reliability Quality",
  "Production Quality",
  "Delight",
  "Product Quality Checklist",
  "Calm over clever",
  "Trust over spectacle",
  "One obvious next action",
  "review-before-send",
  "Does it reduce effort?",
  "Does it feel premium?",
]) {
  if (!standard.includes(token)) {
    fail(`WORKSPACE_PRODUCT_QUALITY_STANDARD.md missing: ${token}`);
  }
}

const rootUi = fs.readFileSync(
  path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
  "utf8",
);
if (!rootUi.includes("Send when you're ready.")) {
  fail("OperatorRoot must use calm post-dictation cue (P16.PQ1)");
}
if (rootUi.includes("Review your words, then Send")) {
  fail("OperatorRoot must not restore instructional review tutorial copy");
}
if (
  /onVoiceTranscript[\s\S]*?submitUtterance\(transcript\)/.test(rootUi)
) {
  fail("F10: onVoiceTranscript must not auto-submit");
}

console.log("verify-product-quality: ok");
