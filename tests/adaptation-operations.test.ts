/**
 * Sprint 60 — Adaptation operations.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import {
  buildEvidenceFromSessions,
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
} from "../app/src/dev/experienceEvidence";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  ADAPTATION_STORAGE_KEY,
  listAdaptations,
  upsertValidatedAdaptation,
} from "../app/src/experience/workspaceAdaptation";
import {
  clearExperimentStore,
  runAdaptationExperiments,
} from "../app/src/experience/adaptationExperiments";
import {
  buildAdaptationCatalog,
  deriveOperationalHealth,
  getCatalogEntry,
  runBatchValidation,
} from "../app/src/experience/adaptationOperations";
import type {
  ExperienceEvent,
  ExperienceSession,
} from "../app/src/dev/experienceEvents";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function session(
  id: string,
  events: ExperienceEvent[],
): ExperienceSession {
  return {
    schemaVersion: 1,
    sessionId: id,
    startedAt: 1,
    events,
  };
}

function nav(
  seq: number,
  t: number,
  from: ExperienceEvent["from"],
  destination: ExperienceEvent["destination"],
): ExperienceEvent {
  return {
    seq,
    t,
    type: "navigate",
    from,
    destination,
    commandId: "dock_navigate",
  };
}

function calmSession(id: string): ExperienceSession {
  return session(id, [
    { seq: 0, t: 0, type: "session_start", destination: "home" },
    {
      seq: 1,
      t: 400,
      type: "first_meaningful_interaction",
      destination: "home",
      modality: "pointer",
    },
    nav(2, 600, "home", "save"),
    { seq: 3, t: 700, type: "flow_start", flow: "save", destination: "save" },
    { seq: 4, t: 900, type: "save_success", flow: "save", destination: "save" },
  ]);
}

describe("adaptation catalog", () => {
  it("derives catalog entries from existing governance ids", () => {
    const store = memoryStore();
    const summary = runAdaptationExperiments(store);
    const catalog = buildAdaptationCatalog(store);

    expect(catalog.entries.length).toBe(summary.results.length);
    expect(catalog.entries.map((e) => e.adaptationId).sort()).toEqual(
      summary.results.map((r) => r.adaptationId).sort(),
    );

    for (const entry of catalog.entries) {
      const adaptation = listAdaptations(store).find(
        (a) => a.adaptationId === entry.adaptationId,
      )!;
      expect(entry.proposalId).toBe(adaptation.proposalId);
      expect(entry.engineeringChangeId).toBe(adaptation.engineeringChangeId);
      expect(entry.lifecycleState).toBe(adaptation.rolloutState);
      expect(entry.latestArchitectureSnapshotId).toBeTruthy();
      expect(entry.evidenceCount).toBeGreaterThan(0);
    }

    const dist = catalog.lifecycleDistribution;
    expect(
      dist.inactive +
        dist.candidate +
        dist.rollout_candidate +
        dist.active +
        dist.rolled_back,
    ).toBe(catalog.entries.length);

    const first = catalog.entries[0]!;
    expect(getCatalogEntry(store, first.adaptationId)?.adaptationId).toBe(
      first.adaptationId,
    );

    // Derived view — catalog is not a separate persisted key.
    expect(store.getItem("ws.experience.adaptation.catalog.v1")).toBeNull();
    clearExperimentStore(store);
  });

  it("is deterministic across identical stores", () => {
    const a = buildAdaptationCatalog(
      (() => {
        const s = memoryStore();
        runAdaptationExperiments(s);
        return s;
      })(),
    );
    const b = buildAdaptationCatalog(
      (() => {
        const s = memoryStore();
        runAdaptationExperiments(s);
        return s;
      })(),
    );
    expect(a.entries).toEqual(b.entries);
    expect(a.lifecycleDistribution).toEqual(b.lifecycleDistribution);
  });
});

describe("batch validation", () => {
  it("validates multiple adaptations with atomic write semantics", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const before = listAdaptations(store).map((a) => ({
      id: a.adaptationId,
      validation: a.validation.validationResult,
      state: a.rolloutState,
    }));

    const report = runBatchValidation(store);
    expect(report.schemaVersion).toBe(1);
    expect(
      report.validated.length +
        report.failed.length +
        report.unchanged.length +
        report.skipped.length,
    ).toBe(before.length);

    // Re-run should be unchanged (identical_result).
    const second = runBatchValidation(store);
    expect(second.unchanged.length).toBe(before.length);
    expect(second.validated).toEqual([]);
    expect(second.failed).toEqual([]);
    for (const item of second.items) {
      expect(item.cause).toBe("identical_result");
    }

    clearExperimentStore(store);
  });

  it("skips rolled_back with deterministic cause", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const adaptation = listAdaptations(store)[0]!;
    upsertValidatedAdaptation(store, {
      ...adaptation,
      rolloutState: "rolled_back",
    });

    const report = runBatchValidation(store, {
      adaptationIds: [adaptation.adaptationId],
    });
    expect(report.skipped).toEqual([
      { adaptationId: adaptation.adaptationId, cause: "rolled_back" },
    ]);
    clearExperimentStore(store);
  });

  it("does not leave partial mutations when batch completes", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const ids = listAdaptations(store).map((a) => a.adaptationId).sort();
    runBatchValidation(store);
    const afterIds = listAdaptations(store).map((a) => a.adaptationId).sort();
    expect(afterIds).toEqual(ids);
    // All adaptations still present with coherent validation fields.
    for (const a of listAdaptations(store)) {
      expect(["pending", "passed", "failed"]).toContain(
        a.validation.validationResult,
      );
    }
    clearExperimentStore(store);
  });
});

describe("operational health", () => {
  it("derives backlog and stale/expired counts from existing evidence", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const health = deriveOperationalHealth(store);

    expect(health.adaptationCount).toBe(listAdaptations(store).length);
    expect(health.validationBacklog).toBe(0);
    // Experiments leave passed adaptations in candidate — not inactive validated.
    expect(health.inactiveValidatedAdaptations).toBe(0);
    expect(health.lifecycleDistribution.candidate).toBeGreaterThan(0);

    // Tip evidence beyond longitudinal latest → staleEvidence rises.
    const tip = buildEvidenceFromSessions([calmSession("s-ops-tip")], {
      tag: "ops_tip",
    });
    persistEvidenceSnapshot(store, tip);
    const staleHealth = deriveOperationalHealth(store);
    expect(staleHealth.staleEvidence).toBeGreaterThan(0);
    expect(listEvidenceSnapshots(store).length).toBeGreaterThan(2);

    // Incomplete longitudinal series counts as expired samples.
    expect(staleHealth.expiredLongitudinalSamples).toBeGreaterThan(0);

    clearExperimentStore(store);
  });

  it("counts inactive validated adaptations", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const adaptation = listAdaptations(store)[0]!;
    upsertValidatedAdaptation(store, {
      ...adaptation,
      rolloutState: "inactive",
      validation: { ...adaptation.validation, validationResult: "passed" },
    });
    const health = deriveOperationalHealth(store);
    expect(health.inactiveValidatedAdaptations).toBeGreaterThanOrEqual(1);
    clearExperimentStore(store);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through operations doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "50_Longitudinal_Adaptation_Validation.md" &&
          b === "51_Adaptation_Operations.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (adaptation operations)", () => {
  it("operations module does not persist catalog or health payloads", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    buildAdaptationCatalog(store);
    deriveOperationalHealth(store);
    runBatchValidation(store);
    expect(store.getItem("ws.experience.adaptation.catalog.v1")).toBeNull();
    expect(store.getItem("ws.experience.adaptation.health.v1")).toBeNull();
    const raw = store.getItem(ADAPTATION_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    clearExperimentStore(store);
  });

  it("experience modules introduce no network telemetry", () => {
    const dir = path.join(root, "app/src/experience");
    for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("overlay lazy-loads operations module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/adaptationOperations")');
    expect(dash).toContain("adaptation-operations");
    expect(dash).toContain("run-batch-validation");
  });

  it("architecture doc records catalog and health schemas", () => {
    const doc = readFileSync(
      path.join(root, "architecture/51_Adaptation_Operations.md"),
      "utf8",
    );
    expect(doc).toContain("AdaptationCatalog");
    expect(doc).toContain("BatchValidationReport");
    expect(doc).toContain("AdaptationOperationalHealth");
    expect(doc).toMatch(/Evidence only|no design recommendations/i);
  });
});
