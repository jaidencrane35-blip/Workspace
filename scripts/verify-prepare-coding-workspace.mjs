#!/usr/bin/env node
/**
 * C-PROC-002 — Prepare Coding Workspace runtime composition check.
 *
 * Complements the definition verifier. Does not claim Product Proof.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

function fail(msg) {
  console.error(`verify-prepare-coding-workspace: ${msg}`);
  process.exit(1);
}

const required = [
  "app/src/lib/prepareCodingWorkspace.ts",
  "app/src/lib/compoundOpen.ts",
  "app/src/lib/intentBridge.ts",
  "app/src/lib/operator/intentMap.ts",
  "packages/kernel/src/operator/plan.rs",
  "packages/kernel/src/operator/mod.rs",
  "packages/kernel/src/operator/compose.rs",
  "packages/kernel/src/capability_runtime/application_provider.rs",
  "tests/prepare-coding-workspace.test.ts",
  "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md",
];

for (const rel of required) {
  if (!fs.existsSync(path.join(root, rel))) fail(`missing ${rel}`);
}

const prepare = fs.readFileSync(
  path.join(root, "app/src/lib/prepareCodingWorkspace.ts"),
  "utf8",
);
const bridge = fs.readFileSync(
  path.join(root, "app/src/lib/intentBridge.ts"),
  "utf8",
);
const map = fs.readFileSync(
  path.join(root, "app/src/lib/operator/intentMap.ts"),
  "utf8",
);
const plan = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/plan.rs"),
  "utf8",
);
const mod = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/mod.rs"),
  "utf8",
);
const compose = fs.readFileSync(
  path.join(root, "packages/kernel/src/operator/compose.rs"),
  "utf8",
);
const appProvider = fs.readFileSync(
  path.join(
    root,
    "packages/kernel/src/capability_runtime/application_provider.rs",
  ),
  "utf8",
);
const atlas = fs.readFileSync(
  path.join(root, "docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md"),
  "utf8",
);
const pkg = fs.readFileSync(path.join(root, "package.json"), "utf8");

for (const token of [
  "resolvePrepareCodingWorkspace",
  "matchPrepareCodingWorkspacePhrase",
  "prepareCodingWorkspace",
]) {
  if (!prepare.includes(token)) fail(`prepareCodingWorkspace.ts missing ${token}`);
}

if (!bridge.includes("resolvePrepareCodingWorkspace")) {
  fail("intentBridge must call resolvePrepareCodingWorkspace");
}
if (!bridge.includes("prepareCodingWorkspace")) {
  fail("intentBridge must declare prepareCodingWorkspace IntentAction");
}
if (!map.includes("prepare_coding_workspace")) {
  fail("intentMap must map prepareCodingWorkspace → prepare_coding_workspace");
}

for (const token of [
  "prepare_coding_workspace",
  "desktop.prepare_coding_workspace",
  "preflight_prepare_coding_workspace",
  "plan_encoded_open_steps",
]) {
  if (!plan.includes(token)) fail(`plan.rs missing ${token}`);
}

for (const token of [
  "desktop.prepare_coding_workspace",
  "preflight_prepare_coding_workspace",
  "window_available",
  "identity_confirmed",
  "identity_unconfirmed",
]) {
  if (!mod.includes(token)) fail(`operator/mod.rs missing ${token}`);
}

if (mod.includes("execute_interaction_with_retry") &&
    mod.indexOf("desktop.prepare_coding_workspace") > 0) {
  // Ensure prepare composition does not call C-VER-002 retry helper in its branch.
  const prepareStart = mod.indexOf('Some("desktop.prepare_coding_workspace")');
  const nextBranch = mod.indexOf("else if plan.composition_id", prepareStart + 10);
  const prepareBody = mod.slice(prepareStart, nextBranch > 0 ? nextBranch : undefined);
  if (prepareBody.includes("execute_interaction_with_retry")) {
    fail("C-PROC-002 must not invoke C-VER-002 retry for application launch");
  }
}

if (!compose.includes("desktop.prepare_coding_workspace")) {
  fail("compose.rs must aggregate prepare_coding_workspace completion");
}
if (!compose.includes("identity_confirmed")) {
  fail("compose.rs must require observed identity evidence");
}

if (!appProvider.includes("application_query_is_launchable")) {
  fail("application_provider must expose launchability probe from launch_alias");
}

const atlasStart = atlas.indexOf("#### C-PROC-002 Prepare Coding Workspace");
const atlasEnd = atlas.indexOf("#### C-PROC-003 Situation Goals", atlasStart);
if (atlasStart < 0 || atlasEnd < 0) fail("cannot isolate C-PROC-002 Atlas record");
const procedure = atlas.slice(atlasStart, atlasEnd);
if (!procedure.includes("| **Status** | **IMPLEMENTED** |")) {
  fail("Atlas C-PROC-002 status must be IMPLEMENTED after runtime slice");
}
if (procedure.includes("Product Proof ........... 100%")) {
  fail("must not mark Product Proof complete");
}
if (procedure.includes("Trusted ................. 100%")) {
  fail("must not mark Trusted complete");
}
if (procedure.includes("Production .............. 100%")) {
  fail("must not mark Production complete");
}

if (!pkg.includes("verify-prepare-coding-workspace.mjs")) {
  fail("package.json must wire verify-prepare-coding-workspace");
}

console.log("verify-prepare-coding-workspace: ok");
