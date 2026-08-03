/**
 * Sprint 66 — Real-world adaptation validation.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  listEvidenceSnapshots,
} from "../app/src/dev/experienceEvidence";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";
import {
  certifyAdaptationSet,
  clearCertificationStore,
  listCertifications,
} from "../app/src/experience/adaptationCertification";
import {
  certifyAdaptationPack,
  clearAdaptationPackStore,
} from "../app/src/experience/adaptationPacks";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  buildAdaptationPerformanceReport,
  buildCertificationLongevityReport,
  buildEvidenceInventory,
  buildLongitudinalTrendReport,
  buildPackEffectivenessReport,
  buildRealWorldValidationBundle,
} from "../app/src/experience/adaptationValidation";
import {
  ADAPTATION_STORAGE_KEY,
  CERTIFICATION_STORAGE_KEY,
  PACK_STORAGE_KEY,
  PRODUCTION_ACTIVATION_STORAGE_KEY,
} from "../app/src/experience";
import { EVIDENCE_STORAGE_KEY } from "../app/src/dev/experienceEvidence";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function seedRealWorld(store: ReturnType<typeof memoryStore>) {
  const activation = runFirstProductionAdaptation(store, { now: 10 });
  expect(activation.outcome).toBe("activated");
  const certA = certifyAdaptationSet(store, { now: 100 });
  expect(certA.ok).toBe(true);
  const certB = certifyAdaptationSet(store, { now: 250 });
  expect(certB.ok).toBe(true);
  const pack = certifyAdaptationPack(store, { now: 300 });
  expect(pack.ok).toBe(true);
  return { activation, certA, certB, pack };
}

function teardown(store: ReturnType<typeof memoryStore>) {
  clearAdaptationPackStore(store);
  clearCertificationStore(store);
  clearProductionActivationStore(store);
  clearExperimentStore(store);
}

describe("evidence inventory", () => {
  it("harvests every local evidence snapshot deterministically", () => {
    const store = memoryStore();
    seedRealWorld(store);

    const inventory = buildEvidenceInventory(store, { now: 999 });
    const snapshots = listEvidenceSnapshots(store);
    expect(inventory.schemaVersion).toBe(1);
    expect(inventory.harvestedAt).toBe(999);
    expect(inventory.snapshotCount).toBe(snapshots.length);
    expect(inventory.entries).toHaveLength(snapshots.length);

    for (let i = 0; i < inventory.entries.length; i++) {
      const entry = inventory.entries[i]!;
      const snap = snapshots[i]!;
      expect(entry.sequenceIndex).toBe(i);
      expect(entry.evidenceId).toBe(snap.evidenceId);
      expect(entry.interactionCount).toBe(snap.metrics.sessionCount);
      expect(entry.replayCount).toBe(snap.metrics.replayCount);
      expect(entry.fingerprint).toBe(snap.fingerprint);
      expect(entry.tag).toBe(snap.tag);
      // No user content fields.
      expect(entry).not.toHaveProperty("content");
      expect(entry).not.toHaveProperty("handoff");
    }

    // Deterministic: second harvest matches.
    const again = buildEvidenceInventory(store, { now: 999 });
    expect(again).toEqual(inventory);

    teardown(store);
  });
});

describe("adaptation performance report", () => {
  it("reports active adaptations and certified packs from existing evidence", () => {
    const store = memoryStore();
    const { activation, pack } = seedRealWorld(store);

    const report = buildAdaptationPerformanceReport(store, { now: 500 });
    expect(report.schemaVersion).toBe(1);
    expect(report.generatedAt).toBe(500);
    expect(report.evidenceSnapshotCount).toBeGreaterThan(0);
    expect(report.adaptations.length).toBeGreaterThan(0);
    expect(
      report.adaptations.some((a) => a.subjectId === activation.adaptationId),
    ).toBe(true);

    const entry = report.adaptations.find(
      (a) => a.subjectId === activation.adaptationId,
    )!;
    expect(entry.subjectKind).toBe("adaptation");
    expect(entry.activationCount).toBeGreaterThanOrEqual(1);
    expect(entry.observationCount).toBeGreaterThanOrEqual(0);
    expect(entry.stabilityTrend).toMatch(/rising|stable|falling|unknown/);
    expect(Array.isArray(entry.confidenceEvolution)).toBe(true);
    expect(entry.certificationIds.length).toBeGreaterThan(0);

    expect(report.packs.length).toBeGreaterThan(0);
    expect(report.packs[0]!.subjectId).toBe(pack.pack!.packId);
    expect(report.packs[0]!.packVersion).toBe(pack.pack!.version);

    const again = buildAdaptationPerformanceReport(store, { now: 500 });
    expect(again).toEqual(report);

    teardown(store);
  });
});

describe("pack effectiveness", () => {
  it("compares singles vs packs on objective measures only", () => {
    const store = memoryStore();
    seedRealWorld(store);

    const report = buildPackEffectivenessReport(store, { now: 600 });
    expect(report.schemaVersion).toBe(1);
    expect(report.singles.kind).toBe("single_adaptations");
    expect(report.packs.kind).toBe("certified_packs");
    expect(report.singles.subjectCount).toBeGreaterThan(0);
    expect(report.packs.subjectCount).toBeGreaterThan(0);
    expect(report.singles).toHaveProperty("meanStabilityScore");
    expect(report.singles).toHaveProperty("meanRegressionFrequency");
    expect(report.singles).toHaveProperty("totalEvidenceGrowth");
    expect(report.singles).toHaveProperty("rolloutSuccessRate");
    expect(report.delta).toHaveProperty("meanStabilityScore");
    expect(report.delta).toHaveProperty("totalEvidenceGrowth");
    // No subjective fields.
    expect(JSON.stringify(report)).not.toMatch(/quality|delight|satisfaction/i);

    teardown(store);
  });
});

describe("longitudinal validation", () => {
  it("builds trend and certification longevity from existing stores", () => {
    const store = memoryStore();
    seedRealWorld(store);

    const trend = buildLongitudinalTrendReport(store);
    expect(trend.points.length).toBe(listEvidenceSnapshots(store).length);
    expect(trend.points[0]).toHaveProperty("meanFrictionScore");
    expect(trend.points[0]).toHaveProperty("sessionCount");

    const longevity = buildCertificationLongevityReport(store);
    const certs = listCertifications(store);
    expect(longevity.entries).toHaveLength(certs.length);
    expect(longevity.entries.some((e) => e.longevityMs !== null)).toBe(true);
    expect(longevity.meanLongevityMs).not.toBeNull();

    const bundle = buildRealWorldValidationBundle(store, { now: 1 });
    expect(bundle.inventory.snapshotCount).toBe(trend.points.length);
    expect(bundle.performance.adaptations.length).toBeGreaterThan(0);
    expect(bundle.packEffectiveness.packs.subjectCount).toBeGreaterThan(0);

    teardown(store);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through real-world validation doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "56_Governance_Consolidation.md" &&
          b === "57_Real_World_Adaptation_Validation.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (adaptation validation)", () => {
  it("introduces no new persistence keys", () => {
    const store = memoryStore();
    seedRealWorld(store);
    const beforeKeys = new Set(
      [
        EVIDENCE_STORAGE_KEY,
        ADAPTATION_STORAGE_KEY,
        CERTIFICATION_STORAGE_KEY,
        PACK_STORAGE_KEY,
        PRODUCTION_ACTIVATION_STORAGE_KEY,
      ].filter((k) => store.getItem(k) !== null),
    );
    buildRealWorldValidationBundle(store, { now: 1 });
    // Reports are derived — existing keys only.
    for (const key of beforeKeys) {
      expect(store.getItem(key)).toBeTruthy();
    }
    // Module must not invent a validation storage key.
    const src = readFileSync(
      path.join(root, "app/src/experience/adaptationValidation.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    teardown(store);
  });

  it("reports omit forbidden content keys", () => {
    const store = memoryStore();
    seedRealWorld(store);
    const bundle = buildRealWorldValidationBundle(store, { now: 1 });
    const raw = JSON.stringify(bundle);
    for (const forbidden of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${forbidden}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    teardown(store);
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

  it("overlay lazy-loads validation module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/adaptationValidation")');
    expect(dash).toContain("real-world-adaptation-validation");
    expect(dash).toContain("evidence-inventory");
    expect(dash).toContain("adaptation-performance");
    expect(dash).toContain("pack-effectiveness");
    expect(dash).toContain("longitudinal-trend-graphs");
    expect(dash).toContain("certification-longevity");
  });

  it("architecture doc records inventory and comparison methodology", () => {
    const doc = readFileSync(
      path.join(root, "architecture/57_Real_World_Adaptation_Validation.md"),
      "utf8",
    );
    expect(doc).toContain("EvidenceInventory");
    expect(doc).toContain("AdaptationPerformanceReport");
    expect(doc).toContain("PackEffectivenessReport");
    expect(doc).toContain("Known evidence limitations");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
