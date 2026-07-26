#!/usr/bin/env node
/**
 * Verify UI components respect the Experience import boundary.
 */
import path from "node:path";
import { fileURLToPath } from "node:url";
import { auditUiExperienceBoundary } from "./ui-experience-boundary-lib.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const componentsDir = path.join(root, "app/src/components");

const violations = auditUiExperienceBoundary(componentsDir);
if (violations.length > 0) {
  console.error("verify-ui-experience-boundary: FAILED");
  for (const v of violations) {
    console.error(`  - ${v}`);
  }
  process.exit(1);
}

console.log(
  `verify-ui-experience-boundary: ok (${violations.length} violations)`,
);
