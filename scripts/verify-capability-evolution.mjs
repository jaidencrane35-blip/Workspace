#!/usr/bin/env node
/**
 * Verifies capability-evolution foundation artifacts exist and match schema.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-capability-evolution: ${msg}`);
  process.exit(1);
}

const registryPath = path.join(root, "docs/capability-evolution/registry.json");
const libPath = path.join(root, "app/src/lib/capabilityEvolution.ts");
const readmePath = path.join(root, "docs/capability-evolution/README.md");
const pipelinePath = path.join(
  root,
  "docs/capability-evolution/PIPELINE_ARCHITECTURE.md",
);

for (const p of [registryPath, libPath, readmePath, pipelinePath]) {
  if (!fs.existsSync(p)) {
    fail(`missing ${path.relative(root, p)}`);
  }
}

const registry = JSON.parse(fs.readFileSync(registryPath, "utf8"));
if (registry.schemaVersion !== 1) {
  fail("registry.schemaVersion must be 1");
}
const required = ["statuses", "categories", "approvedBacklog", "proposals", "rules"];
for (const key of required) {
  if (!(key in registry)) {
    fail(`registry missing ${key}`);
  }
}
if (!Array.isArray(registry.statuses) || registry.statuses.length < 5) {
  fail("registry.statuses incomplete");
}
if (!Array.isArray(registry.categories) || registry.categories.length < 4) {
  fail("registry.categories incomplete");
}

const lib = fs.readFileSync(libPath, "utf8");
for (const token of [
  "createProposal",
  "classifyEvolutionRequest",
  "setProposalStatus",
  "undoLastStatusChange",
  "must NOT rewrite",
]) {
  if (!lib.includes(token) && token !== "must NOT rewrite") {
    fail(`capabilityEvolution.ts missing ${token}`);
  }
}
if (!/must not rewrite itself/i.test(lib)) {
  fail("capabilityEvolution.ts must state non-rewrite law");
}

console.log("verify-capability-evolution: ok");
