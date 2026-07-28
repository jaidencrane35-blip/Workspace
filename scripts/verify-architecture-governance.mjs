import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  auditArchitectureGovernance,
  canonicalMapJson,
  detectArchitectureMapDrift,
  writeArchitectureMap,
} from "./architecture-governance-lib.mjs";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const mapPath = path.join(
  rootDir,
  "scripts/generated/architecture-map.json",
);

const writeMode = process.argv.includes("--write");

if (writeMode) {
  const audit = writeArchitectureMap(rootDir, mapPath);
  if (audit.violations.length > 0) {
    console.error("Architecture governance verification failed:");
    for (const violation of audit.violations) {
      console.error(`- ${violation}`);
    }
    process.exit(1);
  }
  console.log(
    `Architecture map written to ${path.relative(rootDir, mapPath)}.`,
  );
  for (const line of audit.evidence.assertions) {
    console.log(`✓ ${line}`);
  }
  console.log(
    `Architecture governance verified: ${audit.map.stats.mutation_commands} mutations, ` +
      `${audit.map.stats.history_dtos_found} history DTOs, ` +
      `${audit.map.stats.projection_dtos_found} projection DTOs, ` +
      `${audit.map.stats.gateway_require_sites} PermissionGateway.require sites.`,
  );
  process.exit(0);
}

const { audit, driftViolations } = detectArchitectureMapDrift(rootDir, mapPath);
const allViolations = [...audit.violations, ...driftViolations];

if (allViolations.length > 0) {
  console.error("Architecture governance verification failed:");
  for (const violation of allViolations) {
    console.error(`- ${violation}`);
  }
  process.exit(1);
}

for (const line of audit.evidence.assertions) {
  console.log(`✓ ${line}`);
}
console.log(
  `Architecture governance verified (map drift clean): ${audit.map.stats.mutation_commands} mutations, ` +
    `${audit.map.stats.history_dtos_found} history DTOs, ` +
    `${audit.map.stats.projection_dtos_found} projection DTOs. ` +
    `Canonical map matches ${path.relative(rootDir, mapPath)}.`,
);

// Touch canonical form to ensure generator stability (no write).
void canonicalMapJson(audit);
