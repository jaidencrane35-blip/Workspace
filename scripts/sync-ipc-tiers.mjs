#!/usr/bin/env node
/**
 * Sync IPC tier registry into the UI bundle.
 * Authorities: lib.rs generate_handler, experienceIpcCatalog, ipc-tiers.json, React invokeIpc.
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
const outPath = path.join(root, "app/src/generated/ipcTiers.ts");

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
  console.error("sync-ipc-tiers: classification errors:");
  for (const err of tiers.errors) {
    console.error(`  - ${err}`);
  }
  process.exit(1);
}

fs.mkdirSync(path.dirname(outPath), { recursive: true });
fs.writeFileSync(outPath, renderIpcTiersTs(tiers));
console.log(
  `Synced ${outPath} (registered=${tiers.counts.registered} product=${tiers.counts.product} developer=${tiers.counts.developer} diagnostic=${tiers.counts.diagnostic} experimental=${tiers.counts.experimental} quarantine=${tiers.counts.quarantine})`,
);
