import path from "node:path";
import { fileURLToPath } from "node:url";
import { auditIpcContracts } from "./ipc-contract-lib.mjs";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const audit = auditIpcContracts(rootDir);

if (audit.violations.length > 0) {
  console.error("IPC contract verification failed:");
  for (const violation of audit.violations) {
    console.error(`- ${violation}`);
  }
  process.exit(1);
}

console.log(
  `IPC contracts verified: ${audit.registered.length} commands, ` +
    `${audit.invoked.length} frontend consumers, ` +
    `${audit.publicErrorCodes.length} public error codes.`,
);
