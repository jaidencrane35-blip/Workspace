#!/usr/bin/env node
/** Verify Gate B1 diagnostics / support bundle wiring. */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-support-bundle: ${message}`);
  process.exit(1);
}

const libPath = path.join(root, "app/src-tauri/src/lib.rs");
const fileLogPath = path.join(root, "app/src-tauri/src/file_log.rs");
const cmdPath = path.join(
  root,
  "app/src-tauri/src/commands/support_bundle.rs",
);
const intentPath = path.join(root, "app/src/lib/intentBridge.ts");
const intelPath = path.join(root, "app/src/lib/operator/intelligence.ts");
const tiersPath = path.join(root, "docs/03-Engineering/ipc-tiers.json");
const matrixPath = path.join(root, "docs/production/production-readiness.json");
const depPath = path.join(
  root,
  "docs/production/production-gates-dependency.json",
);

for (const p of [
  libPath,
  fileLogPath,
  cmdPath,
  intentPath,
  intelPath,
  tiersPath,
  matrixPath,
  depPath,
]) {
  if (!fs.existsSync(p)) fail(`missing ${path.relative(root, p)}`);
}

const lib = fs.readFileSync(libPath, "utf8");
if (!lib.includes("export_support_bundle")) {
  fail("lib.rs must register export_support_bundle");
}
if (!lib.includes("attach_file_logger")) {
  fail("lib.rs must attach file logger in setup");
}

const cmd = fs.readFileSync(cmdPath, "utf8");
if (!cmd.includes("database_contents_included: false")) {
  fail("support bundle must never include database contents");
}
if (!cmd.includes("Moments")) {
  fail("support bundle privacy copy must mention Moments exclusion");
}

const fileLog = fs.readFileSync(fileLogPath, "utf8");
if (!fileLog.includes("MAX_LOG_BYTES") || !fileLog.includes("rotate")) {
  fail("file_log.rs must implement size rotation");
}

const intent = fs.readFileSync(intentPath, "utf8");
if (!intent.includes('kind: "supportBundle"')) {
  fail("intentBridge must expose supportBundle intent");
}

const intel = fs.readFileSync(intelPath, "utf8");
if (!intel.includes("export_support_bundle")) {
  fail("intelligence.ts must invoke export_support_bundle");
}

const tiers = JSON.parse(fs.readFileSync(tiersPath, "utf8"));
if (!tiers.diagnostic?.includes("export_support_bundle")) {
  fail("export_support_bundle must be diagnostic-tier IPC");
}

const matrix = JSON.parse(fs.readFileSync(matrixPath, "utf8"));
if (!matrix.completedGates?.includes("B1-diagnostics-support-bundle")) {
  fail("production-readiness completedGates must include B1");
}

const dep = JSON.parse(fs.readFileSync(depPath, "utf8"));
if (!dep.completed?.includes("B1-diagnostics-support-bundle")) {
  fail("dependency json completed must include B1 after land");
}

console.log("verify-support-bundle: ok");
