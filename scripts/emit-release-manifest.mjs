#!/usr/bin/env node
/**
 * F1 — emit release metadata JSON beside NSIS artifacts (unsigned pipeline).
 *
 * Usage:
 *   node scripts/emit-release-manifest.mjs
 *   node scripts/emit-release-manifest.mjs --out path\to\release-manifest.json
 */
import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  findNsisSetupArtifacts,
  sha256File,
} from "./generate-artifact-checksums.mjs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`emit-release-manifest: ${message}`);
  process.exit(1);
}

function gitSha() {
  try {
    return execSync("git rev-parse HEAD", { cwd: root, encoding: "utf8" }).trim();
  } catch {
    return null;
  }
}

function parseArgs(argv) {
  const outIdx = argv.indexOf("--out");
  return {
    out:
      outIdx >= 0
        ? path.resolve(argv[outIdx + 1] ?? "")
        : null,
  };
}

const { out } = parseArgs(process.argv.slice(2));
const version = JSON.parse(
  fs.readFileSync(path.join(root, "package.json"), "utf8"),
).version;
const artifacts = findNsisSetupArtifacts(root);
if (artifacts.length === 0) {
  fail(
    "no Workspace_*_x64-setup.exe — run pnpm installer:build first (or skip emit)",
  );
}

const files = artifacts.map((filePath) => {
  const name = path.basename(filePath);
  if (!/^Workspace_.*_x64-setup\.exe$/i.test(name)) {
    fail(`artifact naming rejected: ${name}`);
  }
  const sidecar = `${filePath}.sha256`;
  const digest = fs.existsSync(sidecar)
    ? fs.readFileSync(sidecar, "utf8").trim().split(/\s+/)[0]
    : sha256File(filePath);
  return {
    name,
    path: path.relative(root, filePath).replace(/\\/g, "/"),
    sha256: digest.toLowerCase(),
    sidecar: fs.existsSync(sidecar)
      ? path.relative(root, sidecar).replace(/\\/g, "/")
      : null,
  };
});

const manifest = {
  schemaVersion: 1,
  productName: "Workspace",
  version,
  channel: "unsigned",
  signed: false,
  gitSha: gitSha(),
  generatedAt: new Date().toISOString(),
  artifacts: files,
  notes:
    "F1 unsigned release manifest. Signing (A2) and updater (B2) are out of scope.",
};

const target =
  out ||
  path.join(path.dirname(artifacts[0]), `Workspace_${version}_release-manifest.json`);
fs.writeFileSync(target, `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
console.log(`emit-release-manifest: wrote ${path.relative(root, target)}`);
