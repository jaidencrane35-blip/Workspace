#!/usr/bin/env node
/**
 * Verify the Tauri shell ships a production-safe Content Security Policy.
 */
import path from "node:path";
import { fileURLToPath } from "node:url";
import { auditContentSecurityPolicy } from "./content-security-policy-lib.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const violations = auditContentSecurityPolicy({
  configPath: path.join(root, "app/src-tauri/tauri.conf.json"),
  htmlPaths: [
    path.join(root, "app/index.html"),
    path.join(root, "app/dist/index.html"),
  ],
});

if (violations.length > 0) {
  console.error("verify-content-security-policy: FAILED");
  for (const v of violations) {
    console.error(`  - ${v}`);
  }
  process.exit(1);
}

console.log(
  `verify-content-security-policy: ok (${violations.length} violations)`,
);
