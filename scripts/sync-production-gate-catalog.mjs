#!/usr/bin/env node
/**
 * Generate PRODUCTION_GATE_CATALOG.md from production-gates-dependency.json
 * so every gate shares an identical human-readable schema.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const jsonPath = path.join(
  root,
  "docs/production/production-gates-dependency.json",
);
const outPath = path.join(root, "docs/production/PRODUCTION_GATE_CATALOG.md");

const dep = JSON.parse(fs.readFileSync(jsonPath, "utf8"));
const order = dep.canonicalOrder ?? [];
const byId = new Map((dep.units ?? []).map((u) => [u.id, u]));
const completed = new Set(dep.completed ?? []);

const lines = [];
lines.push("# Production Gate Catalog");
lines.push("");
lines.push("| Field | Value |");
lines.push("| --- | --- |");
lines.push("| **Generated from** | `production-gates-dependency.json` |");
lines.push(
  "| **Schema** | `PRODUCTION_GATE_SPECIFICATION.md` |",
);
lines.push(
  "| **Sequencing** | `PRODUCTION_DEPENDENCY_AUTHORITY.md` |",
);
lines.push(`| **nextReadyNow** | \`${dep.nextReadyNow}\` |`);
lines.push("| **Do not edit by hand** | Run `node scripts/sync-production-gate-catalog.mjs` |");
lines.push("");
lines.push(
  "Every unit below uses the **same field set**. Operational Acceptance is human-trust criteria (not Product Proof, not CI).",
);
lines.push("");

function renderUnit(u) {
  const status = completed.has(u.id) || u.productionClassification === "Complete"
    ? "Complete"
    : u.productionClassification;
  lines.push(`## ${u.id}`);
  lines.push("");
  lines.push(`| Field | Value |`);
  lines.push(`| --- | --- |`);
  lines.push(`| **Gate family** | ${u.gate} |`);
  lines.push(`| **Production classification** | ${status} |`);
  lines.push(`| **Purpose** | ${u.purpose} |`);
  lines.push(`| **User value** | ${u.userValue} |`);
  lines.push(`| **Engineering value** | ${u.engineeringValue} |`);
  lines.push(
    `| **Constitutional justification** | ${u.constitutionalJustification} |`,
  );
  lines.push(
    `| **Prerequisites** | ${(u.prerequisites ?? []).join(", ") || "—"} |`,
  );
  lines.push(
    `| **Dependents** | ${(u.dependents ?? []).join(", ") || "—"} |`,
  );
  lines.push(
    `| **Blocking conditions** | ${(u.blockingConditions ?? []).join("; ") || "—"} |`,
  );
  lines.push(
    `| **Entry criteria** | ${(u.entryCriteria ?? []).map((x) => `\`${x}\``).join("; ")} |`,
  );
  lines.push(
    `| **Exit criteria** | ${(u.exitCriteria ?? []).map((x) => `\`${x}\``).join("; ")} |`,
  );
  lines.push(
    `| **Verification** | ${(u.verification ?? []).map((x) => `\`${x}\``).join("; ")} |`,
  );
  lines.push(
    `| **Production readiness delta** | ${u.productionReadinessDelta} |`,
  );
  lines.push(
    `| **Regression risks** | ${(u.regressionRisks ?? []).join("; ")} |`,
  );
  lines.push(`| **Rollback strategy** | ${u.rollbackStrategy} |`);
  lines.push(
    `| **Success metrics** | ${(u.successMetrics ?? []).join("; ")} |`,
  );
  lines.push("");
  lines.push("### Operational Acceptance");
  lines.push("");
  for (const item of u.operationalAcceptance ?? []) {
    lines.push(`- [ ] ${item}`);
  }
  lines.push("");
}

// Completed first in stable order, then canonical remaining
const seen = new Set();
for (const id of [
  ...["A0-installer-foundation", "C0-single-instance", "B1-diagnostics-support-bundle"],
  ...order,
]) {
  if (seen.has(id)) continue;
  const u = byId.get(id);
  if (!u) continue;
  seen.add(id);
  renderUnit(u);
}

for (const u of dep.units ?? []) {
  if (seen.has(u.id)) continue;
  renderUnit(u);
}

fs.writeFileSync(outPath, lines.join("\n") + "\n", "utf8");
console.log(`sync-production-gate-catalog: wrote ${outPath}`);
