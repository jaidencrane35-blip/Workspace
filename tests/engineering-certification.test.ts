/**
 * Sprint 73 — Continuous Engineering Certification.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import { ARCHITECTURE_AUTHORITY_DOCS } from "../app/src/dev/engineeringGovernance";
import {
  ENGINEERING_INVARIANTS,
  clearEngineeringCertificationHistory,
  compareEngineeringCertifications,
  deriveSubsystemHealth,
  listEngineeringCertificationHistory,
  listEngineeringInvariants,
  runEngineeringCertification,
  type EngineeringCertificationReport,
} from "../app/src/dev/engineeringCertification";
import {
  certifyAdaptationSet,
  clearCertificationStore,
} from "../app/src/experience/adaptationCertification";
import { clearAdaptationPackStore } from "../app/src/experience/adaptationPacks";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function teardown(store: ReturnType<typeof memoryStore>) {
  clearAdaptationPackStore(store);
  clearCertificationStore(store);
  clearProductionActivationStore(store);
  clearExperimentStore(store);
}

beforeEach(() => {
  clearEngineeringCertificationHistory();
});

describe("invariant registry", () => {
  it("exposes canonical EngineeringInvariant metadata", () => {
    const list = listEngineeringInvariants();
    expect(list.length).toBe(ENGINEERING_INVARIANTS.length);
    expect(list.length).toBeGreaterThanOrEqual(10);
    for (const inv of list) {
      expect(inv.schemaVersion).toBe(1);
      expect(inv.invariantId).toMatch(/^inv-/);
      expect(inv.expectedResult).toBe("pass");
      expect(inv.authorityDocument).toMatch(/\.md$/);
      expect(inv.validationMethod.length).toBeGreaterThan(0);
      expect(inv.owningSubsystem.length).toBeGreaterThan(0);
    }
    const ids = list.map((i) => i.invariantId);
    expect(new Set(ids).size).toBe(ids.length);
  });

  it("covers required invariant families", () => {
    const ids = new Set(ENGINEERING_INVARIANTS.map((i) => i.invariantId));
    expect(ids.has("inv-resolver-stage-count")).toBe(true);
    expect(ids.has("inv-authority-chain-linkage")).toBe(true);
    expect(ids.has("inv-replay-presentation-determinism")).toBe(true);
    expect(ids.has("inv-certification-immutability-shape")).toBe(true);
    expect(ids.has("inv-storage-keys-present")).toBe(true);
    expect(ids.has("inv-public-api-exports")).toBe(true);
    expect(ids.has("inv-evidence-schema-key")).toBe(true);
  });
});

describe("certification runner", () => {
  it("runs all invariants deterministically and completes with a store", () => {
    const store = memoryStore();
    const a = runEngineeringCertification(store, {
      freezeDuration: true,
      now: 42,
      recordHistory: false,
    });
    const b = runEngineeringCertification(store, {
      freezeDuration: true,
      now: 42,
      recordHistory: false,
    });
    expect(a.schemaVersion).toBe(1);
    expect(a.complete).toBe(true);
    expect(a.failed).toEqual([]);
    expect(a.passed.length + a.skipped.length).toBe(
      ENGINEERING_INVARIANTS.length,
    );
    expect(a.results).toHaveLength(ENGINEERING_INVARIANTS.length);
    expect(a.certificationId).toBe(b.certificationId);
    expect(a.passed).toEqual(b.passed);
    expect(a.failed).toEqual(b.failed);
    expect(a.skipped).toEqual(b.skipped);
    expect(a.durationMs).toBe(0);
    expect(a.authorityReferences.length).toBeGreaterThan(0);
    expect(a.subsystems.length).toBeGreaterThan(0);
    for (const result of a.results) {
      if (result.outcome === "failed") {
        expect(result.cause).not.toBeNull();
      }
      if (result.outcome === "passed") {
        expect(result.cause).toBeNull();
      }
    }
  });

  it("skips store-dependent invariants without a store and still completes", () => {
    const report = runEngineeringCertification(null, {
      freezeDuration: true,
      now: 7,
      recordHistory: false,
    });
    expect(report.complete).toBe(true);
    expect(report.failed).toEqual([]);
    expect(report.skipped.length).toBeGreaterThan(0);
    expect(report.skipped).toContain("inv-replay-presentation-determinism");
    expect(report.skipped).toContain("inv-composition-on-store");
  });

  it("records in-memory history only", () => {
    const store = memoryStore();
    runEngineeringCertification(store, {
      freezeDuration: true,
      now: 1,
      recordHistory: true,
    });
    runEngineeringCertification(store, {
      freezeDuration: true,
      now: 2,
      recordHistory: true,
    });
    expect(listEngineeringCertificationHistory()).toHaveLength(2);
    // No new persistence surface — certification history is in-memory only.
    expect(store.getItem("ws.dev.engineering.certification.v1")).toBeNull();
  });

  it("exercises presentation replay invariant against a seeded store", () => {
    const store = memoryStore();
    const activation = runFirstProductionAdaptation(store, { now: 10 });
    expect(activation.outcome).toBe("activated");
    expect(certifyAdaptationSet(store, { now: 100 }).ok).toBe(true);
    const report = runEngineeringCertification(store, {
      freezeDuration: true,
      now: 200,
      recordHistory: false,
    });
    expect(report.failed).toEqual([]);
    expect(report.passed).toContain("inv-replay-presentation-determinism");
    expect(report.passed).toContain("inv-certification-immutability-shape");
    teardown(store);
  });
});

describe("baseline comparison", () => {
  it("reports only outcome deltas", () => {
    const previous: EngineeringCertificationReport = {
      schemaVersion: 1,
      certificationId: "prev",
      certifiedAt: 1,
      durationMs: 0,
      passed: ["inv-a", "inv-b"],
      failed: ["inv-c"],
      skipped: ["inv-d"],
      results: [],
      authorityReferences: [],
      subsystems: [],
      complete: false,
    };
    const current: EngineeringCertificationReport = {
      schemaVersion: 1,
      certificationId: "curr",
      certifiedAt: 2,
      durationMs: 0,
      passed: ["inv-a", "inv-c", "inv-e"],
      failed: ["inv-b"],
      skipped: [],
      results: [],
      authorityReferences: [],
      subsystems: [],
      complete: false,
    };
    const cmp = compareEngineeringCertifications(previous, current);
    expect(cmp.newlyFailed).toEqual(["inv-b"]);
    expect(cmp.newlyResolved).toEqual(["inv-c"]);
    expect(cmp.invariantsAdded).toEqual(["inv-e"]);
    expect(cmp.invariantsRemoved).toEqual(["inv-d"]);
  });

  it("treats null previous as full addition", () => {
    const current = runEngineeringCertification(null, {
      freezeDuration: true,
      now: 3,
      recordHistory: false,
    });
    const cmp = compareEngineeringCertifications(null, current);
    expect(cmp.newlyResolved).toEqual([]);
    expect(cmp.invariantsRemoved).toEqual([]);
    expect(cmp.invariantsAdded.length).toBe(ENGINEERING_INVARIANTS.length);
  });

  it("derives subsystem health from a report", () => {
    const report = runEngineeringCertification(memoryStore(), {
      freezeDuration: true,
      now: 4,
      recordHistory: false,
    });
    const health = deriveSubsystemHealth(report);
    expect(health.length).toBeGreaterThan(0);
    const total = health.reduce(
      (n, h) => n + h.passed + h.failed + h.skipped,
      0,
    );
    expect(total).toBe(ENGINEERING_INVARIANTS.length);
  });
});

describe("governance", () => {
  it("extends authority chain through continuous certification doc", () => {
    expect(ARCHITECTURE_AUTHORITY_DOCS).toContain(
      "64_Continuous_Engineering_Certification.md",
    );
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "63_Architectural_Simplification.md" &&
          b === "64_Continuous_Engineering_Certification.md",
      ),
    ).toBe(true);
    expect(AUTHORITY_CHAIN).toHaveLength(
      ARCHITECTURE_AUTHORITY_DOCS.length - 1,
    );
  });
});

describe("privacy audit (engineering certification)", () => {
  it("module introduces no persistence or telemetry", () => {
    const src = readFileSync(
      path.join(root, "app/src/dev/engineeringCertification.ts"),
      "utf8",
    );
    expect(src).not.toMatch(
      /(?:export\s+)?const\s+\w*STORAGE_KEY\s*=\s*["'`]/,
    );
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    expect(src).not.toMatch(/\bfetch\s*\(/);
    expect(src).not.toMatch(/sendBeacon/);
    const report = runEngineeringCertification(null, {
      freezeDuration: true,
      now: 9,
      recordHistory: false,
    });
    const raw = JSON.stringify(report);
    for (const forbidden of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${forbidden}"`);
    }
  });

  it("overlay lazy-loads certification module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("./engineeringCertification")');
    expect(dash).toContain("engineering-certification");
    expect(dash).toContain("eng-cert-report");
    expect(dash).toContain("eng-invariant-registry");
    expect(dash).toContain("eng-cert-history");
    expect(dash).toContain("eng-cert-baseline");
    expect(dash).toContain("eng-subsystem-health");
  });

  it("architecture doc records certification evidence", () => {
    const doc = readFileSync(
      path.join(root, "architecture/64_Continuous_Engineering_Certification.md"),
      "utf8",
    );
    expect(doc).toContain("EngineeringInvariant");
    expect(doc).toContain("runEngineeringCertification");
    expect(doc).toContain("compareEngineeringCertifications");
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
