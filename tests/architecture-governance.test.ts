import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  CAPABILITY_AUTHORITY_OWNERS,
  FORBIDDEN_AUTHORITY_FIELDS,
  FORBIDDEN_HISTORY_FIELDS,
  FORBIDDEN_PROJECTION_COMMAND_FIELDS,
  HISTORY_STRUCTS,
  INTENTIONAL_BROAD_CAPABILITIES,
  MUTATION_COMMAND_BASELINE,
  PROJECTION_SUMMARY_STRUCTS,
  RECOVERY_DIAGNOSTIC_EVENT_TYPES,
  auditArchitectureGovernance,
  auditFailClosedExpectations,
  detectArchitectureMapDrift,
  handlerMethodsMissingPipeline,
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
    expect(audit.mutationInventory.length).toBeGreaterThanOrEqual(
      MUTATION_COMMAND_BASELINE,
    );
    for (const entry of audit.mutationInventory) {
      expect(entry.capability).toBeTruthy();
      expect(CAPABILITY_AUTHORITY_OWNERS[entry.capability]).toBe(
        entry.authority_owner,
      );
    }
  });

  it("emits fail-closed evidence assertions when inventory is complete", () => {
    expect(audit.evidence.assertions.some((a) => a.includes("history DTOs"))).toBe(
      true,
    );
    expect(
      audit.evidence.assertions.some((a) => a.includes("projection summary")),
    ).toBe(true);
    expect(
      audit.evidence.assertions.some((a) => a.includes("mutation commands")),
    ).toBe(true);
  });

  it("documents the allowed authority path", () => {
    const edgeKeys = audit.map.allowed_edges.map(([a, b]) => `${a}→${b}`);
    expect(edgeKeys).toContain("react→ipc");
    expect(edgeKeys).toContain("kernel_commands→permission_gateway");
    expect(edgeKeys).toContain("permission_gateway→services");
  });

  it("narrows services→gateway forbidden edge to require() only", () => {
    const forbidden = audit.map.forbidden_edges.map(
      (edge) => `${edge.from}→${edge.to}`,
    );
    expect(forbidden).toContain("services→permission_gateway_require");
    expect(forbidden).not.toContain("services→permission_gateway");
  });

  it("keeps authority-field denylist wired for history and projection exports", () => {
    expect(FORBIDDEN_AUTHORITY_FIELDS).toContain("execute");
    expect(FORBIDDEN_HISTORY_FIELDS).toBe(FORBIDDEN_AUTHORITY_FIELDS);
    expect(FORBIDDEN_PROJECTION_COMMAND_FIELDS).toBe(FORBIDDEN_AUTHORITY_FIELDS);
    expect(HISTORY_STRUCTS).toHaveLength(12);
    expect(PROJECTION_SUMMARY_STRUCTS).toHaveLength(12);
  });

  it("documents intentional broad capabilities without treating them as lifecycle owners", () => {
    expect(INTENTIONAL_BROAD_CAPABILITIES["work_context.write"]).toMatch(
      /lifecycle owners remain/i,
    );
  });

  it("keeps recovery diagnostics evidence-only without mutation authority", () => {
    expect(RECOVERY_DIAGNOSTIC_EVENT_TYPES).toEqual([
      "system.recovery.startup.attempted",
      "system.recovery.startup.completed",
      "system.recovery.startup.failed",
    ]);
    expect(
      audit.evidence.assertions.some((a) =>
        a.includes("recovery diagnostic event types remain evidence-only"),
      ),
    ).toBe(true);
    for (const entry of audit.mutationInventory) {
      expect(entry.command).not.toMatch(/Recover|ReconcileStale|RetryStale/i);
    }
  });

  it("reports unused catalog capabilities without failing the audit", () => {
    expect(Array.isArray(audit.unusedCapabilities)).toBe(true);
  });
});

describe("architecture map drift detection", () => {
  it("passes when committed map matches live canonical inventory", () => {
    const mapPath = path.join(
      rootDir,
      "scripts/generated/architecture-map.json",
    );
    const { audit, driftViolations } = detectArchitectureMapDrift(
      rootDir,
      mapPath,
    );
    expect(audit.violations).toEqual([]);
    expect(driftViolations).toEqual([]);
  });

  it("fails when committed map is stale", () => {
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "gov-map-"));
    const mapPath = path.join(tmp, "architecture-map.json");
    fs.writeFileSync(mapPath, `${JSON.stringify({ stale: true }, null, 2)}\n`);
    // Point detect at repo root audit but compare against stale file by
    // temporarily swapping — use detect with rootDir and fake map path.
    const { driftViolations } = detectArchitectureMapDrift(rootDir, mapPath);
    expect(driftViolations.length).toBeGreaterThan(0);
    expect(driftViolations[0]).toMatch(/drift/i);
  });
});

describe("fail-closed verifier behaviour", () => {
  it("fails when history DTO targets disappear from a synthetic tree", () => {
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "gov-empty-"));
    // Minimal skeleton so path checks run but domain has no history structs.
    for (const rel of [
      "app/src",
      "app/src-tauri",
      "packages/kernel/src/commands",
      "packages/kernel/src/security",
      "packages/kernel/src/services",
      "packages/domain/src",
      "packages/database/src",
      "packages/windows-integration/src",
    ]) {
      fs.mkdirSync(path.join(tmp, rel), { recursive: true });
      fs.writeFileSync(path.join(tmp, rel, "placeholder.rs"), "// empty\n");
      if (rel.startsWith("app/")) {
        fs.writeFileSync(path.join(tmp, rel, "placeholder.ts"), "// empty\n");
      }
    }
    fs.mkdirSync(path.join(tmp, "packages/domain/src/capability"), {
      recursive: true,
    });
    fs.writeFileSync(
      path.join(tmp, "packages/domain/src/capability/mod.rs"),
      `impl CapabilityId { fn x() { CapabilityId::new("workspace.write"); } }\n`,
    );
    // Fake required kernel files
    fs.writeFileSync(
      path.join(tmp, "packages/kernel/src/commands/handler.rs"),
      "pub struct CommandHandler;\n",
    );
    fs.writeFileSync(
      path.join(tmp, "packages/kernel/src/commands/pipeline.rs"),
      "pub struct CommandPipeline;\n",
    );
    fs.writeFileSync(
      path.join(tmp, "packages/kernel/src/security/gateway.rs"),
      "pub struct PermissionGateway;\n",
    );
    fs.writeFileSync(
      path.join(tmp, "packages/domain/Cargo.toml"),
      "[package]\nname = \"workspace-domain\"\n",
    );
    fs.writeFileSync(
      path.join(tmp, "packages/database/Cargo.toml"),
      "[package]\nname = \"workspace-database\"\n",
    );

    const audit = auditFailClosedExpectations(tmp);
    expect(
      audit.violations.some((v) =>
        /No history DTO structure|cannot establish compliance/i.test(v),
      ),
    ).toBe(true);
    expect(
      audit.violations.some((v) =>
        /Mutation command inventory dropped below baseline/i.test(v),
      ),
    ).toBe(true);
  });

  it("fails when a required source tree is missing", () => {
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "gov-miss-"));
    fs.mkdirSync(path.join(tmp, "packages/domain/src"), { recursive: true });
    const audit = auditFailClosedExpectations(tmp);
    expect(
      audit.violations.some((v) => /Required source tree missing/i.test(v)),
    ).toBe(true);
  });
});

describe("handler pipeline detection", () => {
  it("rejects mutation handlers that only call non-inner Self helpers", () => {
    const fake = `
impl CommandHandler {
    pub fn create_widget(kernel: &WorkspaceKernel) -> Result<()> {
        Self::helper(kernel)
    }
    fn helper(kernel: &WorkspaceKernel) -> Result<()> {
        Ok(())
    }
}
`;
    expect(handlerMethodsMissingPipeline(fake)).toContain("create_widget");
  });

  it("accepts handlers that reach CommandPipeline through *_inner chain", () => {
    const fake = `
impl CommandHandler {
    pub fn submit_ai_application_launch(kernel: &WorkspaceKernel) -> Result<()> {
        Self::submit_ai_application_launch_inner(kernel)
    }
    fn submit_ai_application_launch_inner(kernel: &WorkspaceKernel) -> Result<()> {
        CommandPipeline::new(kernel.command_context()).execute_mutation(LaunchApplication::new())
    }
}
`;
    expect(handlerMethodsMissingPipeline(fake)).toEqual([]);
  });
});
