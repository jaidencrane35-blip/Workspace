#!/usr/bin/env node
/**
 * Verify docs/project-health.json exists and satisfies the engineering-state schema.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const healthPath = path.join(root, "docs/project-health.json");

function fail(message) {
  console.error(`verify-project-health: ${message}`);
  process.exit(1);
}

if (!fs.existsSync(healthPath)) {
  fail("missing docs/project-health.json — run: pnpm sync:project-health");
}

const health = JSON.parse(fs.readFileSync(healthPath, "utf8"));

const requiredTop = [
  "schemaVersion",
  "updatedAt",
  "constitution",
  "engineeringMode",
  "currentExecutionProgram",
  "completedExecutionPrograms",
  "remainingBacklog",
  "repositoryHealth",
  "engineeringHealth",
  "productReadiness",
  "currentMilestone",
  "acceptedReviews",
  "outstandingProductDebt",
  "verification",
  "validation",
  "architecturalRisks",
  "technicalDebtSummary",
  "lastMilestone",
  "handoffStatus",
];

for (const key of requiredTop) {
  if (!(key in health)) {
    fail(`missing required field: ${key}`);
  }
}

if (health.engineeringMode !== "constitutional-execution") {
  fail(`unexpected engineeringMode: ${health.engineeringMode}`);
}

if (!health.constitution?.version) {
  fail("constitution.version required");
}

if (!Array.isArray(health.completedExecutionPrograms)) {
  fail("completedExecutionPrograms must be an array");
}

if (!Array.isArray(health.remainingBacklog)) {
  fail("remainingBacklog must be an array");
}

if (!Array.isArray(health.acceptedReviews)) {
  fail("acceptedReviews must be an array");
}

if (!Array.isArray(health.outstandingProductDebt)) {
  fail("outstandingProductDebt must be an array");
}

if (!health.engineeringHealth?.overall) {
  fail("engineeringHealth.overall required");
}

if (!health.productReadiness?.status) {
  fail("productReadiness.status required");
}

if (!health.currentMilestone?.id) {
  fail("currentMilestone.id required");
}

const verifiers = health.verification?.verifiers ?? {};
for (const [name, meta] of Object.entries(verifiers)) {
  if (meta.present === false) {
    fail(`verifier script missing for ${name}: ${meta.script}`);
  }
  const abs = path.join(root, meta.script);
  if (!fs.existsSync(abs)) {
    fail(`verifier path not found: ${meta.script}`);
  }
}

const constitutionalSpec =
  "docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md";
if (!fs.existsSync(path.join(root, constitutionalSpec))) {
  fail(`constitutional specification missing: ${constitutionalSpec}`);
}
if (health.constitution?.document !== constitutionalSpec) {
  fail(
    `constitution.document must be ${constitutionalSpec} (got ${health.constitution?.document})`,
  );
}
if (
  !fs.existsSync(path.join(root, "architecture/ARCHITECTURAL_CONSTITUTION_V2.md"))
) {
  fail("subordinate engineering constitution missing");
}
if (
  !fs.existsSync(
    path.join(root, "docs/00-Constitution/ARCHITECTURE_AUTHORITY_HIERARCHY.md"),
  )
) {
  fail("architecture authority hierarchy missing");
}
if (
  !fs.existsSync(
    path.join(root, "docs/00-Constitution/CONSTITUTIONAL_COMPLIANCE_CHECKLIST.md"),
  )
) {
  fail("constitutional compliance checklist missing");
}
if (
  !fs.existsSync(
    path.join(root, "docs/00-Constitution/CONSTITUTIONAL_ALIGNMENT_AUDIT.md"),
  )
) {
  fail("constitutional alignment audit missing");
}
if (
  !fs.existsSync(
    path.join(
      root,
      "docs/00-Constitution/WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md",
    ),
  )
) {
  fail("engineering execution standard missing");
}

if (!fs.existsSync(path.join(root, "docs/engineering-milestone-report.md"))) {
  fail("milestone report missing");
}

console.log(
  `verify-project-health: ok (constitution ${health.constitution.version}, handoff ${health.handoffStatus})`,
);
