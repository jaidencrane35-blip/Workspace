import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { auditIpcContracts } from "../scripts/ipc-contract-lib.mjs";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);

describe("IPC contract boundary", () => {
  const audit = auditIpcContracts(rootDir);

  it("keeps command definitions, registration, and frontend calls aligned", () => {
    expect(audit.violations.filter((item) => item.includes("command"))).toEqual(
      [],
    );
  });

  it("keeps public kernel error codes aligned with TypeScript", () => {
    expect(
      audit.violations.filter((item) => item.includes("error code")),
    ).toEqual([]);
  });

  it("keeps RecommendationOutcome fields aligned with domain serialization", () => {
    expect(
      audit.violations.filter((item) =>
        item.includes("RecommendationOutcome field"),
      ),
    ).toEqual([]);
  });
});
