#!/usr/bin/env node
/**
 * Sync Product Proof TypeScript contracts from Rust domain types.
 */
import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const out = path.join(root, "app/src/generated/productContracts.ts");

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
    out,
  ],
  { cwd: root, encoding: "utf8", shell: true },
);

if (result.stdout) process.stdout.write(result.stdout);
if (result.stderr) process.stderr.write(result.stderr);

if (result.status !== 0) {
  console.error("sync-product-contracts: cargo export failed");
  process.exit(result.status ?? 1);
}

console.log(`sync-product-contracts: ok → ${out}`);
