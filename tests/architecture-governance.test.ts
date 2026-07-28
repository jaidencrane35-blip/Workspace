import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  CAPABILITY_AUTHORITY_OWNERS,
  FORBIDDEN_HISTORY_FIELDS,
  auditArchitectureGovernance,
} from "../scripts/architecture-governance-lib.mjs";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);

describe("architecture governance enforcement", () => {
  const audit = auditArchitectureGovernance(rootDir);

  it("has no architectural invariant violations", () => {
    expect(audit.violations).toEqual([]);
  });

  it("maps every mutation command to a capability and authority owner", () => {
    expect(audit.mutationInventory.length).toBeGreaterThan(0);
    for (const entry of audit.mutationInventory) {
      expect(entry.capability).toBeTruthy();
      expect(CAPABILITY_AUTHORITY_OWNERS[entry.capability]).toBe(
        entry.authority_owner,
      );
    }
  });

  it("documents the allowed Domain → Services → Commands → Gateway → Execution path", () => {
    const edgeKeys = audit.map.allowed_edges.map(([a, b]) => `${a}→${b}`);
    expect(edgeKeys).toContain("react→ipc");
    expect(edgeKeys).toContain("ipc→kernel_commands");
    expect(edgeKeys).toContain("kernel_commands→permission_gateway");
    expect(edgeKeys).toContain("permission_gateway→services");
    expect(edgeKeys).toContain("services→repositories");
    expect(edgeKeys).toContain("services→execution");
  });

  it("documents forbidden edges including React→Database and RE→Process Spawn", () => {
    const forbidden = audit.map.forbidden_edges.map(
      (edge) => `${edge.from}→${edge.to}`,
    );
    expect(forbidden).toContain("react→repositories");
    expect(forbidden).toContain("domain→execution");
    expect(forbidden).toContain("repositories→services");
  });

  it("keeps history executable-field denylist non-empty", () => {
    expect(FORBIDDEN_HISTORY_FIELDS).toContain("execute");
    expect(FORBIDDEN_HISTORY_FIELDS).toContain("command_envelope");
  });
});
