#!/usr/bin/env node
/**
 * Verify Product Proof TypeScript contracts match a fresh Rust export.
 */
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const committed = path.join(root, "app/src/generated/productContracts.ts");

function fail(message) {
  console.error(`verify-product-contracts: ${message}`);
  process.exit(1);
}

if (!fs.existsSync(committed)) {
  fail(`missing ${committed} — run: pnpm sync:contracts`);
}

const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "ws-product-contracts-"));
const tmpOut = path.join(tmpDir, "productContracts.ts");

const result = spawnSync(
  "cargo",
  [
    "run",
    "-q",
    "-p",
    "workspace-domain",
    "--bin",
    "export_product_contracts",
    "--",
    tmpOut,
  ],
  { cwd: root, encoding: "utf8", shell: true },
);

if (result.status !== 0) {
  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);
  fail("cargo export failed");
}

const expected = fs.readFileSync(tmpOut, "utf8");
const actual = fs.readFileSync(committed, "utf8");

try {
  fs.rmSync(tmpDir, { recursive: true, force: true });
} catch {
  // best-effort cleanup
}

if (expected !== actual) {
  fail(
    "generated productContracts.ts is stale — run: pnpm sync:contracts",
  );
}

const required = [
  "export type Workspace =",
  "export type SavedContext =",
  "export type ResumePlanPreview =",
  "export type ActionOperationResult =",
  "export type PilotMeasurementScope =",
  "export type PilotMeasurementSnapshot =",
];
for (const marker of required) {
  if (!actual.includes(marker)) {
    fail(`missing required contract export: ${marker}`);
  }
}

console.log(
  `verify-product-contracts: ok (${actual.split(/\r?\n/).length} lines)`,
);
