/**
 * Sprint 74 — Architectural Maintainability.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import { ARCHITECTURE_AUTHORITY_DOCS } from "../app/src/dev/engineeringGovernance";
import { ENGINEERING_INVARIANTS } from "../app/src/dev/engineeringCertification";
import {
  MAINTAINABILITY_BASELINE,
  MODULE_INVENTORY,
  buildDependencyHealthReport,
  buildMaintainabilityReport,
  compareMaintainabilityTrend,
} from "../app/src/dev/architecturalMaintainability";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("maintainability report", () => {
  it("reports deterministic implementation metrics", () => {
    const a = buildMaintainabilityReport();
    const b = buildMaintainabilityReport();
    expect(a).toEqual(b);
    expect(a.schemaVersion).toBe(1);
    expect(a.reportId).toBe("arch-maintainability-v1");
    expect(a.moduleCount).toBe(MODULE_INVENTORY.length);
    expect(a.moduleCount).toBeGreaterThan(MAINTAINABILITY_BASELINE.moduleIds.length);
    expect(a.dependencyFanOutTotal).toBeGreaterThan(0);
    expect(a.dependencyDepthMax).toBeGreaterThan(0);
    expect(a.exportedSymbolCount).toBeGreaterThan(0);
    expect(a.averageModuleSize).toBeGreaterThan(0);
    expect(a.largestModules.length).toBeGreaterThan(0);
    expect(a.duplicatedImplementation.clampRoundSites).toBe(0);
    expect(a.duplicatedImplementation.varianceSites).toBe(0);
    expect(a.validationCoverage.invariantCount).toBe(
      ENGINEERING_INVARIANTS.length,
    );
    expect(a.invariantCoverage.invariantCount).toBe(
      ENGINEERING_INVARIANTS.length,
    );
    expect(a.subsystems.length).toBeGreaterThan(0);
  });

  it("inventory ids are unique and sorted", () => {
    const ids = MODULE_INVENTORY.map((m) => m.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect([...ids].sort()).toEqual(ids);
  });
});

describe("dependency analysis", () => {
  it("produces deterministic dependency health", () => {
    const a = buildDependencyHealthReport();
    const b = buildDependencyHealthReport();
    expect(a).toEqual(b);
    expect(a.schemaVersion).toBe(1);
    expect(a.reportId).toBe("dependency-health-v1");
    expect(a.circularDependencyCount).toBe(a.circularDependencies.length);
    expect(a.circularDependencyCount).toBeGreaterThanOrEqual(1);
    expect(a.edgeCount).toBe(
      MODULE_INVENTORY.reduce((s, m) => s + m.imports.length, 0),
    );
    expect(a.architecturalBoundaryCrossings.length).toBeGreaterThan(0);
    expect(a.subsystemOwnership.length).toBeGreaterThan(0);
    for (const crossing of a.architecturalBoundaryCrossings) {
      expect(["experience_to_dev", "dev_to_experience"]).toContain(
        crossing.kind,
      );
    }
  });

  it("flags high fan-out and fan-in modules", () => {
    const health = buildDependencyHealthReport();
    expect(health.highFanOutModules.length).toBeGreaterThan(0);
    expect(health.highFanInModules.length).toBeGreaterThan(0);
    expect(health.highFanOutModules.every((m) => m.count >= 8)).toBe(true);
    expect(health.highFanInModules.every((m) => m.count >= 10)).toBe(true);
  });
});

describe("trend comparison", () => {
  it("reports only engineering metric deltas vs baseline", () => {
    const report = buildMaintainabilityReport();
    const health = buildDependencyHealthReport();
    const trend = compareMaintainabilityTrend(report, health);
    expect(trend.schemaVersion).toBe(1);
    expect(trend.reportId).toBe("maintainability-trend-v1");
    expect(trend.modulesAdded).toContain(
      "dev/architecturalMaintainability.ts",
    );
    expect(trend.modulesRemoved).toEqual([]);
    expect(trend.dependencyIncreases).toBeGreaterThanOrEqual(0);
    expect(trend.dependencyReductions).toBeGreaterThanOrEqual(0);
    expect(
      trend.dependencyIncreases === 0 || trend.dependencyReductions === 0,
    ).toBe(true);
    expect(typeof trend.invariantCoverageDelta).toBe("number");
    expect(typeof trend.validationCoverageDelta).toBe("number");
  });
});

describe("governance", () => {
  it("extends authority chain through maintainability doc", () => {
    expect(ARCHITECTURE_AUTHORITY_DOCS).toContain(
      "65_Architectural_Maintainability.md",
    );
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "64_Continuous_Engineering_Certification.md" &&
          b === "65_Architectural_Maintainability.md",
      ),
    ).toBe(true);
    expect(AUTHORITY_CHAIN).toHaveLength(
      ARCHITECTURE_AUTHORITY_DOCS.length - 1,
    );
  });
});

describe("privacy audit (architectural maintainability)", () => {
  it("module introduces no persistence or telemetry", () => {
    const src = readFileSync(
      path.join(root, "app/src/dev/architecturalMaintainability.ts"),
      "utf8",
    );
    expect(src).not.toMatch(
      /(?:export\s+)?const\s+\w*STORAGE_KEY\s*=\s*["'`]/,
    );
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    expect(src).not.toMatch(/\bfetch\s*\(/);
    expect(src).not.toMatch(/sendBeacon/);
    const raw = JSON.stringify({
      maintainability: buildMaintainabilityReport(),
      dependency: buildDependencyHealthReport(),
      trend: compareMaintainabilityTrend(),
    });
    for (const forbidden of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${forbidden}"`);
    }
  });

  it("overlay lazy-loads maintainability module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("./architecturalMaintainability")');
    expect(dash).toContain("architectural-maintainability");
    expect(dash).toContain("maintainability-report");
    expect(dash).toContain("maintainability-metrics");
    expect(dash).toContain("dependency-health");
    expect(dash).toContain("maintainability-trend");
    expect(dash).toContain("maintainability-subsystems");
  });

  it("architecture doc records maintainability evidence", () => {
    const doc = readFileSync(
      path.join(root, "architecture/65_Architectural_Maintainability.md"),
      "utf8",
    );
    expect(doc).toContain("MaintainabilityReport");
    expect(doc).toContain("DependencyHealthReport");
    expect(doc).toContain("compareMaintainabilityTrend");
    expect(doc).toContain("Subsystem ownership");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });

  it("dev tree still has no network telemetry", () => {
    const dir = path.join(root, "app/src/dev");
    for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });
});
