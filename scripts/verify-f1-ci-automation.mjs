#!/usr/bin/env node
/**
 * F1 — CI automation gate: v2-dev coverage + release verify wiring.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-f1-ci-automation: ${message}`);
  process.exit(1);
}

const required = [
  ".github/workflows/ci-pr.yml",
  ".github/workflows/ci-release.yml",
  "docs/production/RELEASE_PIPELINE.md",
  "docs/production/R1_RELEASE_ENGINEERING_READINESS.md",
  "docs/production/P16_F1_CI_AUTOMATION.md",
  "scripts/verify-version-consistency.mjs",
  "scripts/verify-release-pipeline.mjs",
  "scripts/emit-release-manifest.mjs",
];
for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const pr = fs.readFileSync(path.join(root, ".github/workflows/ci-pr.yml"), "utf8");
const release = fs.readFileSync(
  path.join(root, ".github/workflows/ci-release.yml"),
  "utf8",
);

if (!pr.includes("v2-dev")) {
  fail("ci-pr.yml must cover v2-dev");
}
if (!/branches:\s*\[[^\]]*main[^\]]*\]/s.test(pr) && !pr.includes("- main")) {
  fail("ci-pr.yml must still cover main");
}
if (!pr.includes("pnpm test") && !pr.includes("verify:release")) {
  fail("ci-pr.yml must run pnpm test or verify:release");
}

for (const token of [
  "v2-dev",
  "verify:release",
  "installer:build",
  "checksums:generate",
  "release:manifest",
  "Workspace_",
]) {
  if (!release.includes(token)) {
    fail(`ci-release.yml missing ${token}`);
  }
}
// verify:release already chains verify:release-pipeline; either token is enough.
if (
  !release.includes("verify:release-pipeline") &&
  !release.includes("verify:release")
) {
  fail("ci-release.yml must run verify:release or verify:release-pipeline");
}
if (/signtool|certificateThumbprint|tauri-plugin-updater|codesign/i.test(release)) {
  fail("ci-release.yml must not implement signing or updater (A2/B2 out of scope)");
}

const dep = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/production/production-gates-dependency.json"),
    "utf8",
  ),
);
if (!(dep.completed ?? []).includes("F1-ci-automation")) {
  fail("dependency json must mark F1-ci-automation Complete");
}
const f1 = dep.units.find((u) => u.id === "F1-ci-automation");
if (!f1 || f1.productionClassification !== "Complete") {
  fail("F1 unit productionClassification must be Complete");
}

const matrix = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/production/production-readiness.json"),
    "utf8",
  ),
);
if (!matrix.completedGates?.includes("F1-ci-automation")) {
  fail("production-readiness.json completedGates must include F1-ci-automation");
}
const releaseArea = matrix.areas?.find((a) => a.id === "releaseAutomation");
if (!releaseArea || releaseArea.maturity === "TrackARequired") {
  fail("releaseAutomation maturity must advance past TrackARequired");
}

const pkg = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8"));
if (!String(pkg.scripts.test ?? "").includes("verify-f1-ci-automation")) {
  fail("pnpm test must include verify-f1-ci-automation");
}
if (!String(pkg.scripts["verify:release"] ?? "").includes("verify:release-pipeline")) {
  fail("verify:release must include verify:release-pipeline");
}

console.log("verify-f1-ci-automation: ok");
