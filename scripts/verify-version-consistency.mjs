#!/usr/bin/env node
/**
 * F1 — fail early when release version metadata drifts across manifests.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-version-consistency: ${message}`);
  process.exit(1);
}

function readJson(rel) {
  return JSON.parse(fs.readFileSync(path.join(root, rel), "utf8"));
}

function cargoWorkspaceVersion() {
  const text = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
  const section = text.match(/\[workspace\.package\]([\s\S]*?)(\n\[|\s*$)/);
  if (!section) fail("Cargo.toml missing [workspace.package]");
  const m = section[1].match(/^\s*version\s*=\s*"([^"]+)"/m);
  if (!m) fail("Cargo.toml workspace.package.version missing");
  return m[1];
}

function tauriAppVersion() {
  const text = fs.readFileSync(
    path.join(root, "app/src-tauri/Cargo.toml"),
    "utf8",
  );
  const m = text.match(/^\s*version\s*=\s*"([^"]+)"/m);
  if (!m) fail("app/src-tauri/Cargo.toml version missing");
  return m[1];
}

const rootPkg = readJson("package.json");
const appPkg = readJson("app/package.json");
const tauriConf = readJson("app/src-tauri/tauri.conf.json");
const cargoWs = cargoWorkspaceVersion();
const cargoApp = tauriAppVersion();

const expected = rootPkg.version;
if (!expected || typeof expected !== "string") {
  fail("root package.json version missing");
}

const pairs = [
  ["package.json", expected],
  ["app/package.json", appPkg.version],
  ["app/src-tauri/tauri.conf.json", tauriConf.version],
  ["Cargo.toml [workspace.package]", cargoWs],
  ["app/src-tauri/Cargo.toml", cargoApp],
];

const drift = pairs.filter(([, v]) => v !== expected);
if (drift.length) {
  fail(
    `version drift (expected ${expected}): ${drift
      .map(([name, v]) => `${name}=${v}`)
      .join("; ")}`,
  );
}

if (!/^\d+\.\d+\.\d+/.test(expected)) {
  fail(`version must be semver-like major.minor.patch (got ${expected})`);
}

console.log(`verify-version-consistency: ok (${expected})`);
