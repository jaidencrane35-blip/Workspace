/**
 * Sprint 58 — Adaptation experiments.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { listProposals } from "../app/src/dev/experienceGovernance";
import { listEngineeringRecords } from "../app/src/dev/engineeringGovernance";
import { listArchitectureSnapshots } from "../app/src/dev/architecturalIntegrity";
import {
  ADAPTATION_EXPERIMENT_SPECS,
  EXPERIMENT_STORAGE_KEY,
  clearExperimentStore,
  experimentRollbackReady,
  getExperimentSummary,
  listExperimentResults,
  runAdaptationExperiments,
  selectEligibleProposals,
  selectExperimentSpecs,
  toggleAdaptationExperiment,
} from "../app/src/experience/adaptationExperiments";
import {
  listAdaptations,
  resolvePresentationConfiguration,
} from "../app/src/experience/workspaceAdaptation";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("adaptation experiment selection", () => {
  it("selects up to three specs by improvement then risk", () => {
    const selected = selectExperimentSpecs(ADAPTATION_EXPERIMENT_SPECS, 3);
    expect(selected).toHaveLength(3);
    expect(selected.map((s) => s.experimentKey)).toEqual([
      "environment_quiet",
      "spacing_tighten",
      "density_balanced",
    ]);
  });

  it("returns empty eligible proposals from an empty store", () => {
    expect(selectEligibleProposals([])).toEqual([]);
  });
});

describe("adaptation experiment pipeline", () => {
  it("materialises lineage, validates before/after, never auto-activates", () => {
    const store = memoryStore();
    const summary = runAdaptationExperiments(store);
    expect(summary.experimentsSelected).toBe(3);
    expect(summary.proposalsConsidered).toBeGreaterThanOrEqual(1);
    expect(summary.selectionNote).toMatch(/materialised|Selected from approved/i);
    expect(summary.results).toHaveLength(3);

    for (const result of summary.results) {
      expect(result.proposalId).toBeTruthy();
      expect(result.engineeringChangeId).toBeTruthy();
      expect(result.evidenceBaselineId).toBeTruthy();
      expect(result.evidenceAfterId).toBeTruthy();
      expect(result.architectureSnapshotId).toBeTruthy();
      expect(result.replaySessionIds.length).toBeGreaterThan(0);
      expect(result.adaptationId).toBeTruthy();
      expect(["passed", "failed", "skipped"]).toContain(result.validationOutcome);
    }

    const passed = summary.results.filter((r) => r.validationOutcome === "passed");
    expect(passed.length).toBeGreaterThan(0);
    for (const result of passed) {
      expect(result.experimentStatus).toBe("validated");
      expect(result.rolloutDisposition).toBe("activate");
      expect(result.regressionCheck).toBe("clear");
    }

    // Governance artefacts exist.
    expect(listProposals(store).length).toBeGreaterThan(0);
    expect(listEngineeringRecords(store).length).toBeGreaterThan(0);
    expect(listArchitectureSnapshots(store).length).toBeGreaterThan(0);

    // Adaptations exist but none are active without manual toggle.
    const adaptations = listAdaptations(store);
    expect(adaptations.length).toBeGreaterThanOrEqual(passed.length);
    expect(
      resolvePresentationConfiguration(adaptations).appliedAdaptationIds,
    ).toEqual([]);

    for (const adaptation of adaptations) {
      expect(adaptation.rolloutState).not.toBe("active");
      if (adaptation.validation.validationResult === "passed") {
        expect(adaptation.rolloutState).toBe("candidate");
      } else {
        expect(adaptation.rolloutState).toBe("inactive");
      }
    }

    clearExperimentStore(store);
  });

  it("compares evidence metrics deterministically across runs", () => {
    const a = runAdaptationExperiments(memoryStore());
    const b = runAdaptationExperiments(memoryStore());
    expect(a.results.map((r) => r.experimentKey)).toEqual(
      b.results.map((r) => r.experimentKey),
    );
    for (let i = 0; i < a.results.length; i++) {
      expect(a.results[i]!.baselineMetricValue).toBe(
        b.results[i]!.baselineMetricValue,
      );
      expect(a.results[i]!.observedMetricValue).toBe(
        b.results[i]!.observedMetricValue,
      );
      expect(a.results[i]!.validationOutcome).toBe(
        b.results[i]!.validationOutcome,
      );
    }
  });
});

describe("governance validation + toggle", () => {
  it("toggles validated experiments individually and supports rollback readiness", () => {
    const store = memoryStore();
    const summary = runAdaptationExperiments(store);
    const validated = summary.results.find(
      (r) => r.experimentStatus === "validated",
    );
    expect(validated).toBeTruthy();
    expect(experimentRollbackReady(store, validated!.experimentId)).toBe(true);

    const on = toggleAdaptationExperiment(store, validated!.adaptationId);
    expect(on.ok).toBe(true);
    if (on.ok) {
      expect(on.adaptation.rolloutState).toBe("active");
    }
    expect(
      resolvePresentationConfiguration(listAdaptations(store))
        .appliedAdaptationIds,
    ).toContain(validated!.adaptationId);

    const off = toggleAdaptationExperiment(store, validated!.adaptationId);
    expect(off.ok).toBe(true);
    if (off.ok) {
      expect(off.adaptation.rolloutState).toBe("inactive");
    }
    expect(
      resolvePresentationConfiguration(listAdaptations(store))
        .appliedAdaptationIds,
    ).not.toContain(validated!.adaptationId);

    // Failed / missing cannot activate.
    const failed = summary.results.find((r) => r.validationOutcome === "failed");
    if (failed?.adaptationId) {
      const blocked = toggleAdaptationExperiment(store, failed.adaptationId);
      expect(blocked.ok).toBe(false);
    }

    expect(getExperimentSummary(store)?.experimentsSelected).toBe(3);
    clearExperimentStore(store);
  });

  it("keeps authority chain including experiments doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "48_Adaptive_Workspace.md" &&
          b === "49_Adaptation_Experiments.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (adaptation experiments)", () => {
  it("experiment storage omits forbidden content keys", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const raw = store.getItem(EXPERIMENT_STORAGE_KEY)!;
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

  it("overlay lazy-loads experiment runner", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/adaptationExperiments")');
    expect(dash).toContain("adaptation-experiments");
    expect(dash).toContain("run-adaptation-experiments");
  });

  it("architecture doc records experiment evidence fields", () => {
    const doc = readFileSync(
      path.join(root, "architecture/49_Adaptation_Experiments.md"),
      "utf8",
    );
    expect(doc).toContain("spacing_tighten");
    expect(doc).toContain("density_balanced");
    expect(doc).toContain("environment_quiet");
    expect(doc).toContain("experimentStatus");
    expect(doc).toMatch(/Evidence only|No subjective/i);
  });
});
