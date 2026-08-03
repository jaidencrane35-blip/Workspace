/**
 * Sprint 61 — First governed production adaptation.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  listAdaptations,
  resolvePresentationConfiguration,
} from "../app/src/experience/workspaceAdaptation";
import {
  clearExperimentStore,
  runAdaptationExperiments,
} from "../app/src/experience/adaptationExperiments";
import {
  PRODUCTION_ACTIVATION_STORAGE_KEY,
  clearProductionActivationStore,
  getProductionActivationRecord,
  rollbackProductionAdaptation,
  runFirstProductionAdaptation,
  selectProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import { buildAdaptationCatalog } from "../app/src/experience/adaptationOperations";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("production adaptation activation", () => {
  it("activates exactly one eligible adaptation with full lineage", () => {
    const store = memoryStore();
    const record = runFirstProductionAdaptation(store, { now: 1_700_000_000_000 });

    expect(record.outcome).toBe("activated");
    expect(record.adaptationId).toBeTruthy();
    expect(record.activatedAt).toBe(1_700_000_000_000);
    expect(record.blockReasons).toEqual([]);
    expect(record.governanceIntact).toBe(true);
    expect(record.integrityValid).toBe(true);
    expect(record.rollbackAvailable).toBe(true);
    expect(record.regression).toBe(false);
    expect(record.preEvidenceId).toBeTruthy();
    expect(record.postEvidenceId).toBeTruthy();
    expect(record.preEvidenceId).not.toBe(record.postEvidenceId);
    expect(record.proposalId).toBeTruthy();
    expect(record.engineeringChangeId).toBeTruthy();
    expect(record.architectureSnapshotId).toBeTruthy();
    expect(record.replaySessionIds.length).toBeGreaterThan(0);
    expect(record.stabilityScore).toBeGreaterThanOrEqual(0.7);
    expect(record.expectedMetric).toBeTruthy();

    const active = listAdaptations(store).filter(
      (a) => a.rolloutState === "active",
    );
    expect(active).toHaveLength(1);
    expect(active[0]!.adaptationId).toBe(record.adaptationId);
    expect(
      resolvePresentationConfiguration(listAdaptations(store))
        .appliedAdaptationIds,
    ).toEqual([record.adaptationId]);

    // Exactly one — others remain non-active.
    expect(
      listAdaptations(store).filter((a) => a.rolloutState === "active"),
    ).toHaveLength(1);

    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("selection prefers highest stability score", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    // Prepare via full run path without activating twice: call select after ensure via run then rollback.
    const activated = runFirstProductionAdaptation(store, { now: 2 });
    expect(activated.outcome).toBe("activated");
    const catalog = buildAdaptationCatalog(store);
    const selectedEntry = catalog.entries.find(
      (e) => e.adaptationId === activated.adaptationId,
    );
    expect(selectedEntry).toBeTruthy();
    const maxScore = Math.max(...catalog.entries.map((e) => e.stabilityScore));
    expect(selectedEntry!.stabilityScore).toBe(maxScore);

    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("records evidence comparison metrics", () => {
    const store = memoryStore();
    const record = runFirstProductionAdaptation(store, { now: 3 });
    expect(record.outcome).toBe("activated");
    expect(typeof record.preMetricValue).toBe("number");
    expect(typeof record.postMetricValue).toBe("number");
    expect(typeof record.metricDelta).toBe("number");
    // Stored record matches getter.
    expect(getProductionActivationRecord(store)?.adaptationId).toBe(
      record.adaptationId,
    );
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });
});

describe("production rollback", () => {
  it("rolls back via existing pathway and clears presentation apply", () => {
    const store = memoryStore();
    const record = runFirstProductionAdaptation(store, { now: 4 });
    expect(record.outcome).toBe("activated");

    const rolled = rollbackProductionAdaptation(store);
    expect(rolled.ok).toBe(true);
    if (rolled.ok) {
      expect(rolled.record.outcome).toBe("rolled_back");
      expect(rolled.record.rollbackAvailable).toBe(true);
    }

    const adaptation = listAdaptations(store).find(
      (a) => a.adaptationId === record.adaptationId,
    );
    expect(adaptation?.rolloutState).toBe("rolled_back");
    expect(
      resolvePresentationConfiguration(listAdaptations(store))
        .appliedAdaptationIds,
    ).toEqual([]);

    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });
});

describe("blocked activation evidence", () => {
  it("blocks with deterministic reasons when catalog is empty", () => {
    const store = memoryStore();
    // Empty store: runFirst still seeds experiments — force select on empty after clear.
    const selected = selectProductionAdaptation(store);
    expect(selected.ok).toBe(false);
    if (!selected.ok) {
      expect(selected.reasons).toContain("empty_catalog");
      expect(selected.catalogSize).toBe(0);
    }
  });
});

describe("governance + authority", () => {
  it("extends authority chain through first production adaptation doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "51_Adaptation_Operations.md" &&
          b === "52_First_Production_Adaptation.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (production adaptation)", () => {
  it("production record omits forbidden content keys", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 5 });
    const raw = store.getItem(PRODUCTION_ACTIVATION_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    clearProductionActivationStore(store);
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

  it("overlay lazy-loads production activation", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/productionAdaptation")');
    expect(dash).toContain("production-adaptation");
    expect(dash).toContain("run-production-adaptation");
    expect(dash).toContain("active-adaptation");
  });

  it("architecture doc records activation evidence fields", () => {
    const doc = readFileSync(
      path.join(root, "architecture/52_First_Production_Adaptation.md"),
      "utf8",
    );
    expect(doc).toContain("activateAdaptation");
    expect(doc).toContain("ProductionActivationRecord");
    expect(doc).toContain("rollbackAvailable");
    expect(doc).toMatch(/Evidence only|no subjective/i);
  });
});
