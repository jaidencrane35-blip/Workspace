#!/usr/bin/env node
/**
 * Verify IPC tier registry is in sync and constitutionally consistent.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  computeIpcTiers,
  parseExperienceIpcCommands,
  parseGenerateHandlerCommands,
  parseReactInvokeCommands,
  renderIpcTiersTs,
} from "./ipc-tiers-lib.mjs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const libRsPath = path.join(root, "app/src-tauri/src/lib.rs");
const catalogPath = path.join(root, "app/src/demo/experienceIpcCatalog.ts");
const authorityPath = path.join(root, "docs/03-Engineering/ipc-tiers.json");
const appSrc = path.join(root, "app/src");
const tsPath = path.join(root, "app/src/generated/ipcTiers.ts");

function fail(message) {
  console.error(`verify-ipc-tiers: ${message}`);
  process.exit(1);
}

if (!fs.existsSync(tsPath)) {
  fail(`missing ${tsPath} — run: node scripts/sync-ipc-tiers.mjs`);
}

const authority = JSON.parse(fs.readFileSync(authorityPath, "utf8"));
const registered = parseGenerateHandlerCommands(
  fs.readFileSync(libRsPath, "utf8"),
);
const product = parseExperienceIpcCommands(
  fs.readFileSync(catalogPath, "utf8"),
);
const reactUsed = parseReactInvokeCommands(appSrc);
const tiers = computeIpcTiers({
  registered,
  product,
  diagnostic: authority.diagnostic ?? [],
  reactUsed,
  quarantine: authority.quarantine ?? [],
});

if (tiers.errors.length > 0) {
  for (const err of tiers.errors) {
    console.error(`verify-ipc-tiers: ${err}`);
  }
  process.exit(1);
}

const expected = renderIpcTiersTs(tiers);
const actual = fs.readFileSync(tsPath, "utf8");
if (expected !== actual) {
  fail(
    "generated ipcTiers.ts is stale — run: node scripts/sync-ipc-tiers.mjs",
  );
}

// Structural invariants
const union = new Set([
  ...tiers.product,
  ...tiers.developer,
  ...tiers.diagnostic,
  ...tiers.experimental,
]);
if (union.size !== tiers.registered.length) {
  fail(
    `tier partition incomplete: union=${union.size} registered=${tiers.registered.length}`,
  );
}
for (const cmd of tiers.registered) {
  if (!union.has(cmd)) {
    fail(`registered command missing from tiers: ${cmd}`);
  }
}

const overlapChecks = [
  ["product", "developer", tiers.product, tiers.developer],
  ["product", "diagnostic", tiers.product, tiers.diagnostic],
  ["product", "experimental", tiers.product, tiers.experimental],
  ["developer", "diagnostic", tiers.developer, tiers.diagnostic],
  ["developer", "experimental", tiers.developer, tiers.experimental],
  ["diagnostic", "experimental", tiers.diagnostic, tiers.experimental],
];
for (const [a, b, left, right] of overlapChecks) {
  const rightSet = new Set(right);
  for (const cmd of left) {
    if (rightSet.has(cmd)) {
      fail(`tier overlap ${a}∩${b}: ${cmd}`);
    }
  }
}

for (const cmd of tiers.product) {
  if (!reactUsed.includes(cmd) && cmd) {
    // Product commands may be demo-only paths; still must be registered.
  }
}

console.log(
  `verify-ipc-tiers: ok (registered=${tiers.counts.registered} product=${tiers.counts.product} developer=${tiers.counts.developer} diagnostic=${tiers.counts.diagnostic} experimental=${tiers.counts.experimental} quarantine=${tiers.counts.quarantine})`,
);
