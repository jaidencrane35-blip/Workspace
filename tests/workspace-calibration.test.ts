/**
 * Sprint 70 — Workspace Calibration.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  persistEvidenceSnapshot,
  type ExperienceEvidence,
} from "../app/src/dev/experienceEvidence";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";
import {
  certifyAdaptationSet,
  clearCertificationStore,
} from "../app/src/experience/adaptationCertification";
import { clearAdaptationPackStore } from "../app/src/experience/adaptationPacks";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  deriveWorkspaceAnticipation,
  predictAnticipationFromEvidence,
} from "../app/src/experience/workspaceAnticipation";
import {
  applyConfidenceCalibration,
  confidenceCalibrationFactor,
  deriveWorkspaceCalibration,
  listCalibrationHistory,
  reliabilityBandOf,
  replayWorkspaceCalibration,
  scoreEvidenceCalibration,
} from "../app/src/experience/workspaceCalibration";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function seedCertified(store: ReturnType<typeof memoryStore>) {
  const activation = runFirstProductionAdaptation(store, { now: 10 });
  expect(activation.outcome).toBe("activated");
  const cert = certifyAdaptationSet(store, { now: 100 });
  expect(cert.ok).toBe(true);
  return { activation, cert };
}

function teardown(store: ReturnType<typeof memoryStore>) {
  clearAdaptationPackStore(store);
  clearCertificationStore(store);
  clearProductionActivationStore(store);
  clearExperimentStore(store);
}

function evidenceAt(
  id: string,
  destination: "home" | "save" | "resume" | "pilot" | "help",
  samples: number,
): ExperienceEvidence {
  return {
    schemaVersion: 1,
    evidenceId: id,
    fingerprint: `fp-${id}`,
    tag: `tag_${id}`,
    sourceSessionIds: [`s-${id}`],
    metrics: {
      sessionCount: Math.max(2, samples),
      medianTimeToConfidenceMs: 100,
      meanFrictionScore: 0.2,
      medianFrictionScore: 0.2,
      frictionMin: 0.1,
      frictionMax: 0.3,
      frictionP25: 0.15,
      frictionP75: 0.25,
      hesitationHotspotCount: 1,
      topHesitationDestination: destination,
      topHesitationMedianGapMs: 50,
      navigationLoopCount: 0,
      abandonedFlowTotal: 0,
      abandonedSave: 0,
      abandonedContinue: 0,
      recoverySuccessRate: 1,
      recoveryCount: 0,
      interruptionCount: 0,
      replayCount: 1,
      replayDivergenceRate: 0,
      saveSuccessTotal: destination === "save" ? samples : 0,
      continueSuccessTotal: destination === "resume" ? samples : 0,
    },
    hotspots: [
      { destination, medianGapMs: 50, samples },
    ],
    loops: [],
  };
}

describe("calibration scoring", () => {
  it("scores confirmed vs missed predictions across evidence pairs", () => {
    const snapshots = [
      evidenceAt("e1", "save", 3),
      evidenceAt("e2", "save", 3),
      evidenceAt("e3", "home", 3),
      evidenceAt("e4", "home", 3),
    ];
    const score = scoreEvidenceCalibration(snapshots);
    expect(score.predictionCount).toBe(3);
    expect(score.confirmedPredictions).toBe(2);
    expect(score.missedPredictions).toBe(1);
    expect(score.observedAccuracy).toBeCloseTo(2 / 3, 4);
    expect(score.reliabilityTrend.length).toBe(3);
  });

  it("applies monotonic confidence calibration", () => {
    const raw = 0.8;
    const low = {
      active: true,
      confidenceCalibration: confidenceCalibrationFactor(true, 0.2),
    } as Parameters<typeof applyConfidenceCalibration>[1];
    const high = {
      active: true,
      confidenceCalibration: confidenceCalibrationFactor(true, 0.9),
    } as Parameters<typeof applyConfidenceCalibration>[1];
    const a = applyConfidenceCalibration(raw, low);
    const b = applyConfidenceCalibration(raw, high);
    expect(b).toBeGreaterThanOrEqual(a);
    expect(reliabilityBandOf(1, 0.5)).toBe("insufficient");
    expect(reliabilityBandOf(4, 0.8)).toBe("high");
  });
});

describe("anticipation integration", () => {
  it("keeps prediction selection unchanged while calibrating confidence", () => {
    const store = memoryStore();
    seedCertified(store);
    // Append deterministic evidence series for calibration pairs.
    persistEvidenceSnapshot(store, evidenceAt("cal-a", "save", 4));
    persistEvidenceSnapshot(store, evidenceAt("cal-b", "save", 4));
    persistEvidenceSnapshot(store, evidenceAt("cal-c", "save", 4));

    const anticipation = deriveWorkspaceAnticipation(store)!;
    const tipPrediction = predictAnticipationFromEvidence(
      evidenceAt("cal-c", "save", 4),
    );
    // Tip evidence is last snapshot — predictions match tip evidence rules.
    expect(anticipation.likelyNextMoment).toBe(tipPrediction.likelyNextMoment);
    expect(anticipation.likelyContinuationTarget).toBe(
      tipPrediction.likelyContinuationTarget,
    );
    expect(anticipation.likelyFocalRegion).toBe(tipPrediction.likelyFocalRegion);
    expect(anticipation.rawConfidence).toBe(tipPrediction.rawConfidence);

    const calibration = deriveWorkspaceCalibration(store, {
      anticipationLineageId: anticipation.anticipationId,
    });
    expect(calibration).toBeTruthy();
    if (calibration?.active) {
      expect(anticipation.confidence).toBe(
        applyConfidenceCalibration(anticipation.rawConfidence, calibration),
      );
      expect(anticipation.calibrationId).toBe(calibration.calibrationId);
    } else {
      expect(anticipation.confidence).toBe(anticipation.rawConfidence);
    }

    teardown(store);
  });
});

describe("replay determinism", () => {
  it("replays identical calibration", () => {
    const store = memoryStore();
    seedCertified(store);
    persistEvidenceSnapshot(store, evidenceAt("r1", "pilot", 3));
    persistEvidenceSnapshot(store, evidenceAt("r2", "pilot", 3));
    persistEvidenceSnapshot(store, evidenceAt("r3", "pilot", 3));

    const a = replayWorkspaceCalibration(store);
    const b = replayWorkspaceCalibration(store);
    expect(a.calibration).toEqual(b.calibration);
    expect(a.history).toEqual(b.history);
    expect(listCalibrationHistory(store).length).toBeGreaterThan(0);

    teardown(store);
  });
});

describe("governance", () => {
  it("extends authority chain through calibration doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "60_Workspace_Anticipation.md" &&
          b === "61_Workspace_Calibration.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (workspace calibration)", () => {
  it("introduces no persistence and omits forbidden content", () => {
    const store = memoryStore();
    seedCertified(store);
    const src = readFileSync(
      path.join(root, "app/src/experience/workspaceCalibration.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    const raw = JSON.stringify(replayWorkspaceCalibration(store));
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

  it("overlay lazy-loads calibration module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/workspaceCalibration")');
    expect(dash).toContain("workspace-calibration");
    expect(dash).toContain("calibration-history");
    expect(dash).toContain("calibration-reliability-trend");
    expect(dash).toContain("calibration-prediction-accuracy");
    expect(dash).toContain("calibration-confidence");
    expect(dash).toContain("calibration-evidence-lineage");
    expect(dash).toContain("calibration-replay-lineage");
  });

  it("architecture doc records calibration model", () => {
    const doc = readFileSync(
      path.join(root, "architecture/61_Workspace_Calibration.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceCalibration");
    expect(doc).toContain("confidenceCalibration");
    expect(doc).toContain("predictAnticipationFromEvidence");
    expect(doc).toContain("No new resolver stage");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
