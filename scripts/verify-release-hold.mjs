#!/usr/bin/env node
/**
 * R2 — Engineering Standby & Release Lock consistency.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-release-hold: ${message}`);
  process.exit(1);
}

const holdPath = path.join(root, "docs/production/R2_RELEASE_HOLD.md");
const handoffPath = path.join(root, "docs/project/ENGINEERING_HANDOFF.md");
const milestonePath = path.join(root, "docs/engineering-milestone-report.md");
const pipelinePath = path.join(root, "docs/production/RELEASE_PIPELINE.md");
const playbookPath = path.join(
  root,
  "docs/production/A2_F2_RELEASE_EXECUTION_PLAYBOOK.md",
);
const depPath = path.join(
  root,
  "docs/production/production-gates-dependency.json",
);
const readinessPath = path.join(
  root,
  "docs/production/production-readiness.json",
);
const healthPath = path.join(root, "docs/project-health.json");

for (const p of [
  holdPath,
  handoffPath,
  milestonePath,
  pipelinePath,
  playbookPath,
  depPath,
  readinessPath,
  healthPath,
]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const hold = fs.readFileSync(holdPath, "utf8");
const handoff = fs.readFileSync(handoffPath, "utf8");
const milestone = fs.readFileSync(milestonePath, "utf8");
const pipeline = fs.readFileSync(pipelinePath, "utf8");

if (!hold.includes("Release Hold")) fail("R2_RELEASE_HOLD.md missing Release Hold");
if (!hold.includes("Stage 2") || !hold.includes("Owner Acceptance")) {
  fail("R2_RELEASE_HOLD.md missing Stage 2 — Owner Acceptance");
}
if (!hold.includes("Authenticode") || !/\bA2\b/.test(hold)) {
  fail("R2_RELEASE_HOLD.md must name Authenticode and A2");
}
for (const t of ["T1", "T2", "T3", "T4", "T5"]) {
  if (!hold.includes(t)) fail(`R2_RELEASE_HOLD.md missing resume trigger ${t}`);
}

if (!handoff.includes("Release Hold")) {
  fail("ENGINEERING_HANDOFF.md must declare Release Hold");
}
if (!handoff.includes("Stage 2")) {
  fail("ENGINEERING_HANDOFF.md must declare Stage 2 — Owner Acceptance");
}
if (!/A2|Code Signing/i.test(handoff)) {
  fail("ENGINEERING_HANDOFF.md must name A2 as next engineering event");
}
if (!milestone.includes("R2") || !milestone.includes("Release Hold")) {
  fail("engineering-milestone-report.md must stamp R2 Release Hold");
}
if (!pipeline.includes("Release Hold") && !pipeline.includes("R2")) {
  fail("RELEASE_PIPELINE.md must point at Release Hold / R2");
}

const dep = JSON.parse(fs.readFileSync(depPath, "utf8"));
if (!(dep.completed ?? []).includes("F1-ci-automation")) {
  fail("F1 must remain Complete under Release Hold");
}
const a2 = dep.units.find((u) => u.id === "A2-code-signing");
if (!a2 || a2.productionClassification !== "BlockedExternal") {
  fail("A2 must remain BlockedExternal until certificate");
}
if (dep.nextReadyNow !== "D1-ipc-quarantine") {
  fail(`nextReadyNow unexpected: ${dep.nextReadyNow}`);
}

const readiness = JSON.parse(fs.readFileSync(readinessPath, "utf8"));
if (readiness.productionReadyToday !== false) {
  fail("productionReadyToday must remain false");
}
if (readiness.broadPublicReleaseReady !== false) {
  fail("broadPublicReleaseReady must remain false");
}
if (!readiness.completedGates?.includes("F1-ci-automation")) {
  fail("production-readiness completedGates must include F1");
}

const health = JSON.parse(fs.readFileSync(healthPath, "utf8"));
if (health.handoffStatus !== "P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING") {
  fail("project-health handoffStatus must remain Product Proof pending");
}
const note = String(health.currentExecutionProgram?.note ?? "");
if (!/Release Hold|release hold|standby/i.test(note)) {
  fail("project-health currentExecutionProgram.note must mention Release Hold");
}

const pkg = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8"));
if (!String(pkg.scripts.test ?? "").includes("verify-release-hold")) {
  fail("pnpm test must include verify-release-hold");
}

console.log("verify-release-hold: ok (Release Hold authoritative)");
