#!/usr/bin/env node
/**
 * Enforce Canonical Production Gate Specification on every catalog unit.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-production-gate-specification: ${message}`);
  process.exit(1);
}

const specPath = path.join(
  root,
  "docs/production/PRODUCTION_GATE_SPECIFICATION.md",
);
const jsonPath = path.join(
  root,
  "docs/production/production-gates-dependency.json",
);
const catalogPath = path.join(
  root,
  "docs/production/PRODUCTION_GATE_CATALOG.md",
);
const authPath = path.join(
  root,
  "docs/production/PRODUCTION_DEPENDENCY_AUTHORITY.md",
);

for (const p of [specPath, jsonPath, catalogPath, authPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const spec = fs.readFileSync(specPath, "utf8");
for (const phrase of [
  "Operational Acceptance",
  "Mandatory schema",
  "Normalize catalog only",
  "rollbackStrategy",
]) {
  if (!spec.includes(phrase) && phrase === "rollbackStrategy") {
    if (!spec.includes("Rollback strategy")) fail(`Gate Spec missing ${phrase}`);
    continue;
  }
  if (!spec.includes(phrase) && phrase !== "rollbackStrategy") {
    fail(`Gate Spec missing phrase: ${phrase}`);
  }
}
if (!spec.includes("Rollback strategy") && !spec.includes("rollbackStrategy")) {
  fail("Gate Spec must define Rollback strategy");
}

const auth = fs.readFileSync(authPath, "utf8");
if (!auth.includes("PRODUCTION_GATE_SPECIFICATION.md")) {
  fail("Dependency Authority must reference Gate Specification");
}

const dep = JSON.parse(fs.readFileSync(jsonPath, "utf8"));
if (dep.schemaVersion !== 2) fail("schemaVersion must be 2");
if (!dep.gateSpecification?.includes("PRODUCTION_GATE_SPECIFICATION")) {
  fail("dependency json must point at gate specification");
}

const required = dep.requiredFields;
if (!Array.isArray(required) || required.length < 15) {
  fail("requiredFields incomplete");
}

const classes = new Set([
  "ReadyNow",
  "BlockedExternal",
  "BlockedByGate",
  "OptionalPolish",
  "ReleaseOnly",
  "Complete",
]);

for (const unit of dep.units ?? []) {
  for (const field of required) {
    if (unit[field] === undefined || unit[field] === null) {
      fail(`${unit.id} missing field ${field}`);
    }
  }
  if (!classes.has(unit.productionClassification)) {
    fail(`${unit.id} invalid productionClassification`);
  }
  if (
    !Array.isArray(unit.operationalAcceptance) ||
    unit.operationalAcceptance.length < 1
  ) {
    fail(`${unit.id} needs operationalAcceptance items`);
  }
  for (const item of unit.operationalAcceptance) {
    if (
      typeof item !== "string" ||
      !/^A (user|release engineer|developer) can /i.test(item)
    ) {
      fail(
        `${unit.id} operationalAcceptance must be “A <role> can …” (${item})`,
      );
    }
  }
}

const catalog = fs.readFileSync(catalogPath, "utf8");
for (const unit of dep.units) {
  if (!catalog.includes(`## ${unit.id}`)) {
    fail(
      `catalog missing section for ${unit.id} — run sync-production-gate-catalog`,
    );
  }
}
if (!catalog.includes("### Operational Acceptance")) {
  fail("catalog missing Operational Acceptance headings");
}

const a1 = dep.units.find((u) => u.id === "A1-artifact-checksums");
if (!a1 || a1.productionClassification !== "ReadyNow") {
  fail("A1 must remain ReadyNow (not implemented in PI4)");
}
if ((dep.completed ?? []).includes("A1-artifact-checksums")) {
  fail("A1 must not be marked completed in PI4 normalize-only");
}

console.log(
  `verify-production-gate-specification: ok (${dep.units.length} units schema-complete; A1 not implemented)`,
);
