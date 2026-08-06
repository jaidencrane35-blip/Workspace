#!/usr/bin/env node
/**
 * Verifies P9 Capability Runtime research pack artifacts and key laws.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-capability-runtime-research: ${msg}`);
  process.exit(1);
}

const required = [
  "docs/capability-runtime/00_INDEX.md",
  "docs/capability-runtime/CAPABILITY_RUNTIME_RESEARCH_REPORT.md",
  "docs/capability-runtime/OPEN_SOURCE_ADOPTION_MATRIX.md",
  "docs/capability-runtime/CAPABILITY_DOMAIN_CATALOGUE.md",
  "docs/capability-runtime/CAPABILITY_CONTRACTS.md",
  "docs/capability-runtime/ADOPTION_RISK_ASSESSMENT.md",
  "docs/capability-runtime/RECOMMENDED_IMPLEMENTATION_ORDER.md",
  "docs/ui/UI_ARCHITECTURE_SPECIFICATION.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) {
    fail(`missing ${rel}`);
  }
}

const domains = [
  "Application Control",
  "Window Management",
  "Clipboard",
  "File Operations",
  "Desktop Observation",
  "Screenshots",
  "OCR",
  "Voice Input",
  "Automation",
  "Memory",
  "Search",
  "Notifications",
  "Terminal",
  "Browser",
  "Workflow",
];

const catalogue = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CAPABILITY_DOMAIN_CATALOGUE.md"),
  "utf8",
);
for (const d of domains) {
  if (!catalogue.includes(d)) {
    fail(`catalogue missing domain: ${d}`);
  }
}

const matrix = fs.readFileSync(
  path.join(root, "docs/capability-runtime/OPEN_SOURCE_ADOPTION_MATRIX.md"),
  "utf8",
);
for (const token of ["ADOPT", "ADAPT", "WRAP", "STUDY", "REJECT", "Kiro"]) {
  if (!matrix.includes(token)) {
    fail(`adoption matrix missing ${token}`);
  }
}
if (!matrix.includes("**0**") && !matrix.includes("ADOPT (platform-as-product) | **0**")) {
  // Ensure we explicitly claim zero platform ADOPT
  if (!/ADOPT \(platform-as-product\)[\s\S]{0,80}\*\*0\*\*/.test(matrix)) {
    fail("matrix must record zero platform ADOPT");
  }
}

const contracts = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CAPABILITY_CONTRACTS.md"),
  "utf8",
);
for (const token of [
  "Permissions",
  "Audit",
  "IPC boundary",
  "Rust ownership",
  "TypeScript ownership",
  "UI ownership",
  "Rollback",
]) {
  if (!contracts.includes(token)) {
    fail(`contracts missing ${token}`);
  }
}

const report = fs.readFileSync(
  path.join(root, "docs/capability-runtime/CAPABILITY_RUNTIME_RESEARCH_REPORT.md"),
  "utf8",
);
if (!report.includes("None") && !report.includes("**None**")) {
  fail("research report must state no implementation in this program");
}
if (!report.includes("UI Architecture")) {
  fail("research report must reference frozen UI Architecture");
}

console.log("verify-capability-runtime-research: ok");
