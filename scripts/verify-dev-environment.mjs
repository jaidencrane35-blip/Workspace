#!/usr/bin/env node
/**
 * Verifies engineering-environment hygiene for Cursor / indexing / orphan cleanup.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-dev-environment: ${msg}`);
  process.exit(1);
}

const cursorignore = path.join(root, ".cursorignore");
if (!fs.existsSync(cursorignore)) {
  fail("missing .cursorignore (required to exclude target/ and build artifacts)");
}
const ignoreText = fs.readFileSync(cursorignore, "utf8");
for (const token of ["target/", "node_modules/", "dist/", "snapshot-"]) {
  if (!ignoreText.includes(token)) {
    fail(`.cursorignore must exclude ${token}`);
  }
}

const cleanup = path.join(root, "scripts/dev-env-cleanup.mjs");
if (!fs.existsSync(cleanup)) {
  fail("missing scripts/dev-env-cleanup.mjs");
}

const gitignore = fs.readFileSync(path.join(root, ".gitignore"), "utf8");
if (!gitignore.includes("target/")) {
  fail(".gitignore must continue to ignore Rust target/");
}

const rule = fs.readFileSync(
  path.join(root, "docs/capability-runtime/PRODUCT_PROOF_RULE.md"),
  "utf8",
);
if (!rule.includes("User Adaptation Prohibition")) {
  fail("PRODUCT_PROOF_RULE must retain User Adaptation Prohibition");
}
if (!rule.includes("precise capitalization")) {
  fail("User Adaptation Prohibition must forbid requiring precise capitalization");
}

console.log("verify-dev-environment: ok");
