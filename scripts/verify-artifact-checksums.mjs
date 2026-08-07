#!/usr/bin/env node
/**
 * P16.PF1 / Gate A1 — artifact checksum tooling + optional live verify.
 */
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  findNsisSetupArtifacts,
  sha256File,
  writeSidecar,
} from "./generate-artifact-checksums.mjs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(message) {
  console.error(`verify-artifact-checksums: ${message}`);
  process.exit(1);
}

function readSidecarDigest(sidecarPath) {
  const text = fs.readFileSync(sidecarPath, "utf8").trim();
  const match = text.match(/^([a-f0-9]{64})\s{2,}(.+)$/i);
  if (!match) {
    fail(`malformed sidecar ${path.relative(root, sidecarPath)}`);
  }
  return { digest: match[1].toLowerCase(), name: match[2].trim() };
}

const required = [
  "scripts/generate-artifact-checksums.mjs",
  "scripts/verify-artifact-checksums.mjs",
  "docs/production/INSTALLER.md",
  "docs/production/P16_PF1_PREMIUM_FINISH_AUDIT.md",
  "docs/production/production-gates-dependency.json",
];
for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const pkg = JSON.parse(
  fs.readFileSync(path.join(root, "package.json"), "utf8"),
);
if (!pkg.scripts?.["verify:artifact-checksums"]) {
  fail("package.json missing verify:artifact-checksums");
}
if (!pkg.scripts?.["checksums:generate"]) {
  fail("package.json missing checksums:generate");
}
if (!String(pkg.scripts.test ?? "").includes("verify-artifact-checksums")) {
  fail("pnpm test must include verify-artifact-checksums");
}

const installerDoc = fs.readFileSync(
  path.join(root, "docs/production/INSTALLER.md"),
  "utf8",
);
for (const token of [
  "SHA-256",
  "checksums:generate",
  "verify:artifact-checksums",
  ".sha256",
]) {
  if (!installerDoc.includes(token)) {
    fail(`INSTALLER.md missing artifact checksum docs: ${token}`);
  }
}

const dep = JSON.parse(
  fs.readFileSync(
    path.join(root, "docs/production/production-gates-dependency.json"),
    "utf8",
  ),
);
if (!(dep.completed ?? []).includes("A1-artifact-checksums")) {
  fail("production-gates-dependency.json must mark A1-artifact-checksums Complete");
}
const a1 = dep.units.find((u) => u.id === "A1-artifact-checksums");
if (!a1 || a1.productionClassification !== "Complete") {
  fail("A1 unit productionClassification must be Complete");
}
if (!String(a1.verification?.join(" ") ?? "").includes("verify:artifact-checksums")) {
  fail("A1 verification must list verify:artifact-checksums");
}

// Self-test: generate + verify round-trip without requiring a release build.
const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "ws-a1-"));
const sample = path.join(tmpDir, "Workspace_0.0.0_x64-setup.exe");
fs.writeFileSync(sample, "workspace-a1-checksum-fixture\n");
const { digest, sidecar } = writeSidecar(sample);
const again = sha256File(sample);
if (again !== digest) fail("self-test hash mismatch");
const parsed = readSidecarDigest(sidecar);
if (parsed.digest !== digest) fail("self-test sidecar parse mismatch");
if (parsed.name !== path.basename(sample)) fail("self-test sidecar name mismatch");
fs.rmSync(tmpDir, { recursive: true, force: true });

const artifacts = findNsisSetupArtifacts(root);
for (const file of artifacts) {
  const side = `${file}.sha256`;
  if (!fs.existsSync(side)) {
    fail(
      `missing sidecar for ${path.relative(root, file)} — run pnpm checksums:generate`,
    );
  }
  const { digest: expected, name } = readSidecarDigest(side);
  if (name !== path.basename(file)) {
    fail(`sidecar name mismatch for ${path.relative(root, file)}`);
  }
  const actual = sha256File(file);
  if (actual !== expected) {
    fail(
      `checksum mismatch for ${path.relative(root, file)} — regenerate with pnpm checksums:generate`,
    );
  }
  console.log(
    `verify-artifact-checksums: matched ${path.relative(root, file)}`,
  );
}

console.log(
  `verify-artifact-checksums: ok (self-test pass; live artifacts=${artifacts.length})`,
);
