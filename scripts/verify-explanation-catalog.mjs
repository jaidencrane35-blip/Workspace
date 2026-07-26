#!/usr/bin/env node
/**
 * Verify explanation catalog sync and resolver contract fixtures.
 * Fails when explanation-catalog.json != generated explanationCatalog.ts
 * or when catalog contract_fixtures do not resolve as expected.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  parseGeneratedCatalogTs,
  resolveFixture,
  stableStringify,
} from "./explanation-catalog-lib.mjs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const jsonPath = path.join(
  root,
  "packages/kernel/resources/explanation-catalog.json",
);
const tsPath = path.join(root, "app/src/generated/explanationCatalog.ts");

function fail(message) {
  console.error(`verify-explanation-catalog: ${message}`);
  process.exit(1);
}

if (!fs.existsSync(tsPath)) {
  fail(
    `missing ${tsPath} — run: node scripts/sync-explanation-catalog.mjs`,
  );
}

const sourceCatalog = JSON.parse(fs.readFileSync(jsonPath, "utf8"));
const generatedCatalog = parseGeneratedCatalogTs(fs.readFileSync(tsPath, "utf8"));

if (stableStringify(sourceCatalog) !== stableStringify(generatedCatalog)) {
  fail(
    "generated explanationCatalog.ts is stale — run: node scripts/sync-explanation-catalog.mjs",
  );
}

for (const fixture of sourceCatalog.contract_fixtures ?? []) {
  const resolved = resolveFixture(sourceCatalog, fixture);
  if (resolved.known !== fixture.known) {
    fail(
      `fixture ${fixture.key}: expected known=${fixture.known}, got ${resolved.known}`,
    );
  }
  if (resolved.title !== fixture.title) {
    fail(
      `fixture ${fixture.key}: title mismatch\n  expected: ${fixture.title}\n  actual:   ${resolved.title}`,
    );
  }
  if (resolved.description !== fixture.description) {
    fail(
      `fixture ${fixture.key}: description mismatch\n  expected: ${fixture.description}\n  actual:   ${resolved.description}`,
    );
  }
}

console.log(
  `verify-explanation-catalog: ok (version ${sourceCatalog.version}, ${sourceCatalog.contract_fixtures?.length ?? 0} fixtures)`,
);
