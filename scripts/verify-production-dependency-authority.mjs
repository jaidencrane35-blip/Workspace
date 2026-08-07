#!/usr/bin/env node
/** Verify Production Dependency Authority sequencing remains coherent. */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-production-dependency-authority: ${message}`);
  process.exit(1);
}

const authPath = path.join(
  root,
  "docs/production/PRODUCTION_DEPENDENCY_AUTHORITY.md",
);
const jsonPath = path.join(
  root,
  "docs/production/production-gates-dependency.json",
);
const gatesPath = path.join(root, "docs/production/PRODUCTION_GATES.md");

for (const p of [authPath, jsonPath, gatesPath]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const gatesMd = fs.readFileSync(gatesPath, "utf8");
if (!gatesMd.includes("PRODUCTION_DEPENDENCY_AUTHORITY.md")) {
  fail("PRODUCTION_GATES.md must point at Dependency Authority");
}

const auth = fs.readFileSync(authPath, "utf8");
for (const phrase of [
  "Engineering Complete",
  "Production Ready",
  "Release Ready",
  "canonical execution order",
  "PRODUCTION_GATE_SPECIFICATION.md",
]) {
  if (!auth.includes(phrase)) fail(`authority missing phrase: ${phrase}`);
}

const dep = JSON.parse(fs.readFileSync(jsonPath, "utf8"));
if (dep.id !== "workspace-production-gates-dependency") {
  fail("bad dependency json id");
}
if (!Array.isArray(dep.canonicalOrder) || dep.canonicalOrder.length < 5) {
  fail("canonicalOrder incomplete");
}
if (dep.canonicalOrder[0] !== "B1-diagnostics-support-bundle") {
  fail("canonicalOrder must still start with B1 (historical priority)");
}

const completed = new Set(dep.completed ?? []);
const byId = new Map((dep.units ?? []).map((u) => [u.id, u]));
const expectedNext = dep.canonicalOrder.find((id) => {
  const unit = byId.get(id);
  if (!unit) return false;
  if (completed.has(id) || unit.productionClassification === "Complete") {
    return false;
  }
  return unit.productionClassification === "ReadyNow";
});
if (!expectedNext) fail("no ReadyNow unit left in canonical order");
if (dep.nextReadyNow !== expectedNext) {
  fail(
    `nextReadyNow is ${dep.nextReadyNow}, expected first incomplete ReadyNow: ${expectedNext}`,
  );
}

console.log(
  `verify-production-dependency-authority: ok (nextReadyNow=${dep.nextReadyNow})`,
);
