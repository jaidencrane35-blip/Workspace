import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  auditArchitectureGovernance,
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

const audit = writeArchitectureMap(rootDir, mapPath);

if (audit.violations.length > 0) {
  console.error("Architecture governance verification failed:");
  for (const violation of audit.violations) {
    console.error(`- ${violation}`);
  }
  process.exit(1);
}

console.log(
  `Architecture governance verified: ${audit.map.stats.mutation_commands} mutation commands, ` +
    `${audit.map.stats.capability_catalog} capabilities, ` +
    `${audit.map.stats.gateway_require_sites} PermissionGateway.require sites. ` +
    `Map written to ${path.relative(rootDir, mapPath)}.`,
);
