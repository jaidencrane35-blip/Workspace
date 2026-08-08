#!/usr/bin/env node
/**
 * F1 — Release pipeline validation stages (engineering confidence, unsigned).
 * Does not build the installer; CI release workflow builds separately.
 */
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-release-pipeline: ${message}`);
  process.exit(1);
}

function runNode(scriptRel) {
  const script = path.join(root, scriptRel);
  if (!fs.existsSync(script)) fail(`missing ${scriptRel}`);
  const result = spawnSync(process.execPath, [script], {
    cwd: root,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    process.stdout.write(result.stdout ?? "");
    process.stderr.write(result.stderr ?? "");
    fail(`stage failed: ${scriptRel}`);
  }
  const line = (result.stdout ?? "").trim().split(/\r?\n/).filter(Boolean).pop();
  console.log(`  ok — ${scriptRel}${line ? ` (${line})` : ""}`);
}

const stages = [
  ["version-consistency", "scripts/verify-version-consistency.mjs"],
  ["project-health", "scripts/verify-project-health.mjs"],
  ["installer-foundation", "scripts/verify-installer-foundation.mjs"],
  ["artifact-checksums-tooling", "scripts/verify-artifact-checksums.mjs"],
  ["support-bundle", "scripts/verify-support-bundle.mjs"],
];

console.log("verify-release-pipeline: stages");
for (const [name, script] of stages) {
  console.log(`- ${name}`);
  runNode(script);
}

const doc = path.join(root, "docs/production/RELEASE_PIPELINE.md");
if (!fs.existsSync(doc)) fail("missing docs/production/RELEASE_PIPELINE.md");

const pkg = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8"));
for (const key of [
  "verify:version-consistency",
  "verify:release-pipeline",
  "verify:f1-ci-automation",
  "release:manifest",
  "installer:build",
  "checksums:generate",
  "verify:release",
]) {
  if (!pkg.scripts?.[key]) fail(`package.json missing script ${key}`);
}

console.log("verify-release-pipeline: ok (all stages)");
