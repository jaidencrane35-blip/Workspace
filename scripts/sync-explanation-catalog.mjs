#!/usr/bin/env node
/**
 * Sync Experience explanation catalog into the UI bundle.
 * Canonical source: packages/kernel/resources/explanation-catalog.json
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const src = path.join(root, "packages/kernel/resources/explanation-catalog.json");
const out = path.join(root, "app/src/generated/explanationCatalog.ts");

const catalog = JSON.parse(fs.readFileSync(src, "utf8"));
fs.mkdirSync(path.dirname(out), { recursive: true });
const header = `/** AUTO-GENERATED — do not edit. Source: packages/kernel/resources/explanation-catalog.json
 * Regenerate: node scripts/sync-explanation-catalog.mjs
 */
`;
fs.writeFileSync(
  out,
  `${header}export default ${JSON.stringify(catalog, null, 2)} as const;\n`,
);
console.log(`Synced ${out}`);
