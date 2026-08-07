#!/usr/bin/env node
/**
 * P16.PF1 / Gate A1 — write SHA-256 sidecars beside NSIS setup.exe artifacts.
 *
 * Usage:
 *   node scripts/generate-artifact-checksums.mjs
 *   node scripts/generate-artifact-checksums.mjs --file path\to\setup.exe
 */
import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`generate-artifact-checksums: ${message}`);
  process.exit(1);
}

export function sha256File(filePath) {
  const hash = crypto.createHash("sha256");
  hash.update(fs.readFileSync(filePath));
  return hash.digest("hex");
}

export function writeSidecar(filePath) {
  const digest = sha256File(filePath);
  const base = path.basename(filePath);
  const sidecar = `${filePath}.sha256`;
  const body = `${digest}  ${base}\n`;
  fs.writeFileSync(sidecar, body, "utf8");
  return { digest, sidecar };
}

export function findNsisSetupArtifacts(repoRoot = root) {
  const dirs = [
    path.join(repoRoot, "target/release/bundle/nsis"),
    path.join(repoRoot, "app/src-tauri/target/release/bundle/nsis"),
  ];
  const out = [];
  for (const dir of dirs) {
    if (!fs.existsSync(dir)) continue;
    for (const name of fs.readdirSync(dir)) {
      if (/^Workspace_.*_x64-setup\.exe$/i.test(name)) {
        out.push(path.join(dir, name));
      }
    }
  }
  return out;
}

function parseArgs(argv) {
  const fileIdx = argv.indexOf("--file");
  if (fileIdx >= 0) {
    const p = argv[fileIdx + 1];
    if (!p) fail("--file requires a path");
    return [path.resolve(p)];
  }
  return null;
}

function main() {
  const explicit = parseArgs(process.argv.slice(2));
  const files = explicit ?? findNsisSetupArtifacts();
  if (files.length === 0) {
    fail(
      "no Workspace_*_x64-setup.exe found — build with pnpm installer:build, or pass --file",
    );
  }
  for (const file of files) {
    if (!fs.existsSync(file)) fail(`missing ${file}`);
    const { digest, sidecar } = writeSidecar(file);
    console.log(
      `generate-artifact-checksums: ${path.relative(root, sidecar)} = ${digest}`,
    );
  }
}

const isMain =
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isMain) {
  main();
}
