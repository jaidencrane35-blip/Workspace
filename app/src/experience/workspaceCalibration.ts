/**
 * Sprint 70 — Workspace Calibration.
 * Calibrates anticipation confidence against observed evidence accuracy.
 * Does not change prediction selection, Runtime, navigation, or persistence.
 * No new resolver stage — consumed internally by anticipation.
 */

import {
  getReplayInvocationCount,
  listEvidenceSnapshots,
  type ExperienceEvidence,
} from "../dev/experienceEvidence";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import { contentAddressedId, tipOf } from "../dev/governancePrimitives";
import { validateComposition } from "./adaptationComposition";
import {
  observedMomentFromEvidence,
  predictAnticipationFromEvidence,
  round4,
  clamp01,
} from "./anticipationPrediction";
import { deriveActiveMemoryEvolution } from "./workspaceMemoryEvolution";

export type ReliabilityBand =
  | "insufficient"
  | "low"
  | "moderate"
  | "high";

export type CalibrationValidationFailure =
  | "no_evidence"
  | "insufficient_pairs"
  | "missing_replay"
  | "missing_architecture_snapshot"
  | "missing_anticipation_lineage"
  | "composition_regressed"
  | "lineage_incomplete"
  | "calibration_regressed";

export interface CalibrationEvidenceLineage {
  evidenceSnapshotIds: string[];
  tipEvidenceId: string | null;
}

export interface CalibrationReplayLineage {
  replaySessionIds: string[];
  bundleReplayInvocations: number;
}

export interface CalibrationValidation {
  valid: boolean;
  failureReasons: CalibrationValidationFailure[];
  compositionValidationResult: "passed" | "failed";
  composedStabilityScore: number | null;
}

/**
 * Derived calibration — not persisted.
 * Confidence presentation only; prediction selection unchanged.
 */
export interface WorkspaceCalibration {
  schemaVersion: 1;
  calibrationId: string;
  predictionCount: number;
  confirmedPredictions: number;
  missedPredictions: number;
  /** confirmed / predictionCount (0 when none). */
  observedAccuracy: number;
  /**
   * Factor applied to raw confidence: 0.5 + 0.5 * observedAccuracy when active;
   * 1 when inactive/insufficient.
   */
  confidenceCalibration: number;
  reliabilityBand: ReliabilityBand;
  evidenceLineage: CalibrationEvidenceLineage;
  replayLineage: CalibrationReplayLineage;
  anticipationLineageId: string | null;
  architectureSnapshotIds: string[];
  /** Per-epoch observed accuracy series (append order). */
  reliabilityTrend: number[];
  active: boolean;
  validation: CalibrationValidation;
}

const MIN_PAIRS = 2;

export interface CalibrationScore {
  predictionCount: number;
  confirmedPredictions: number;
  missedPredictions: number;
  observedAccuracy: number;
  reliabilityTrend: number[];
}

/**
 * Score consecutive evidence snapshots: prediction at i vs observed Moment at i+1.
 */
export function scoreEvidenceCalibration(
  snapshots: readonly ExperienceEvidence[],
): CalibrationScore {
  let confirmed = 0;
  let missed = 0;
  const reliabilityTrend: number[] = [];

  for (let i = 0; i < snapshots.length - 1; i++) {
    const prior = snapshots[i]!;
    const next = snapshots[i + 1]!;
    const predicted = predictAnticipationFromEvidence(prior).likelyNextMoment;
    const observed = observedMomentFromEvidence(next);
    if (!predicted || !observed) {
      continue;
    }
    if (predicted === observed) {
      confirmed += 1;
    } else {
      missed += 1;
    }
    const total = confirmed + missed;
    reliabilityTrend.push(
      round4(total === 0 ? 0 : confirmed / total),
    );
  }

  const predictionCount = confirmed + missed;
  const observedAccuracy =
    predictionCount === 0 ? 0 : round4(confirmed / predictionCount);

  return {
    predictionCount,
    confirmedPredictions: confirmed,
    missedPredictions: missed,
    observedAccuracy,
    reliabilityTrend,
  };
}

export function reliabilityBandOf(
  predictionCount: number,
  observedAccuracy: number,
): ReliabilityBand {
  if (predictionCount < MIN_PAIRS) {
    return "insufficient";
  }
  if (observedAccuracy >= 0.7 && predictionCount >= 3) {
    return "high";
  }
  if (observedAccuracy >= 0.4) {
    return "moderate";
  }
  return "low";
}

export function confidenceCalibrationFactor(
  active: boolean,
  observedAccuracy: number,
): number {
  if (!active) {
    return 1;
  }
  return round4(0.5 + 0.5 * clamp01(observedAccuracy));
}

/** Apply calibration to raw heuristic confidence. Predictions are untouched. */
export function applyConfidenceCalibration(
  rawConfidence: number,
  calibration: WorkspaceCalibration | null,
): number {
  if (!calibration || !calibration.active) {
    return round4(clamp01(rawConfidence));
  }
  return round4(
    clamp01(rawConfidence * calibration.confidenceCalibration),
  );
}

function buildCalibration(
  store: ExperienceStoreAdapter,
  snapshots: readonly ExperienceEvidence[],
  options: {
    anticipationLineageId: string | null;
    architectureSnapshotIds: string[];
    replaySessionIds: string[];
  },
): WorkspaceCalibration {
  const score = scoreEvidenceCalibration(snapshots);
  const band = reliabilityBandOf(
    score.predictionCount,
    score.observedAccuracy,
  );
  const composition = validateComposition(store);
  const failureReasons: CalibrationValidationFailure[] = [];

  if (snapshots.length === 0) {
    failureReasons.push("no_evidence");
  }
  if (score.predictionCount < MIN_PAIRS) {
    failureReasons.push("insufficient_pairs");
  }
  if (options.replaySessionIds.length === 0) {
    failureReasons.push("missing_replay");
  }
  if (options.architectureSnapshotIds.length === 0) {
    failureReasons.push("missing_architecture_snapshot");
  }
  if (!options.anticipationLineageId) {
    failureReasons.push("missing_anticipation_lineage");
  }
  if (composition.validationResult !== "passed") {
    failureReasons.push("composition_regressed");
  }
  if (!composition.governanceIntact) {
    failureReasons.push("lineage_incomplete");
  }
  // Regression: accuracy collapse with enough samples.
  if (
    score.predictionCount >= MIN_PAIRS &&
    score.observedAccuracy < 0.15
  ) {
    failureReasons.push("calibration_regressed");
  }

  const unique = [
    ...new Set(failureReasons),
  ].sort() as CalibrationValidationFailure[];
  const active = unique.length === 0;
  const confidenceCalibration = confidenceCalibrationFactor(
    active,
    score.observedAccuracy,
  );

  const evidenceSnapshotIds = snapshots.map((s) => s.evidenceId);
  const calibrationId = contentAddressedId(
    "calibrate",
    [
      String(score.predictionCount),
      String(score.confirmedPredictions),
      String(score.missedPredictions),
      String(score.observedAccuracy),
      String(confidenceCalibration),
      band,
      tipOf(evidenceSnapshotIds) ?? "",
      options.anticipationLineageId ?? "none",
    ].join("|"),
  );

  return {
    schemaVersion: 1,
    calibrationId,
    predictionCount: score.predictionCount,
    confirmedPredictions: score.confirmedPredictions,
    missedPredictions: score.missedPredictions,
    observedAccuracy: score.observedAccuracy,
    confidenceCalibration,
    reliabilityBand: band,
    evidenceLineage: {
      evidenceSnapshotIds: [...evidenceSnapshotIds],
      tipEvidenceId: tipOf(evidenceSnapshotIds),
    },
    replayLineage: {
      replaySessionIds: [...options.replaySessionIds].sort(),
      bundleReplayInvocations: getReplayInvocationCount(store),
    },
    anticipationLineageId: options.anticipationLineageId,
    architectureSnapshotIds: [...options.architectureSnapshotIds].sort(),
    reliabilityTrend: score.reliabilityTrend,
    active,
    validation: {
      valid: active,
      failureReasons: unique,
      compositionValidationResult: composition.validationResult,
      composedStabilityScore: composition.composedStabilityScore,
    },
  };
}

/**
 * Active calibration for the full local evidence series.
 * Invalid ⇒ inactive (anticipation keeps raw confidence).
 */
export function deriveWorkspaceCalibration(
  store: ExperienceStoreAdapter,
  options: {
    anticipationLineageId?: string | null;
  } = {},
): WorkspaceCalibration | null {
  const snapshots = listEvidenceSnapshots(store);
  if (snapshots.length === 0) {
    return null;
  }
  const evolution = deriveActiveMemoryEvolution(store);
  const replaySessionIds = [
    ...new Set([
      ...(evolution?.validation.replaySessionIds ?? []),
      ...snapshots.flatMap((s) => s.sourceSessionIds),
    ]),
  ].sort();
  const architectureSnapshotIds = [
    ...new Set(evolution?.architectureSnapshotIds ?? []),
  ].sort();

  return buildCalibration(store, snapshots, {
    anticipationLineageId: options.anticipationLineageId ?? null,
    architectureSnapshotIds,
    replaySessionIds,
  });
}

/** Calibration history — one entry per evidence prefix length ≥ 2. */
export function listCalibrationHistory(
  store: ExperienceStoreAdapter,
  options: {
    anticipationLineageId?: string | null;
  } = {},
): WorkspaceCalibration[] {
  const snapshots = listEvidenceSnapshots(store);
  if (snapshots.length < 2) {
    return [];
  }
  const evolution = deriveActiveMemoryEvolution(store);
  const replaySessionIds = [
    ...new Set([
      ...(evolution?.validation.replaySessionIds ?? []),
      ...snapshots.flatMap((s) => s.sourceSessionIds),
    ]),
  ].sort();
  const architectureSnapshotIds = [
    ...new Set(evolution?.architectureSnapshotIds ?? []),
  ].sort();

  const history: WorkspaceCalibration[] = [];
  for (let end = 2; end <= snapshots.length; end++) {
    history.push(
      buildCalibration(store, snapshots.slice(0, end), {
        anticipationLineageId: options.anticipationLineageId ?? null,
        architectureSnapshotIds,
        replaySessionIds,
      }),
    );
  }
  return history;
}

export function getActiveWorkspaceCalibration(
  store: ExperienceStoreAdapter,
  options: {
    anticipationLineageId?: string | null;
  } = {},
): WorkspaceCalibration | null {
  const calibration = deriveWorkspaceCalibration(store, options);
  if (!calibration?.active) {
    return null;
  }
  return calibration;
}

export function validateWorkspaceCalibration(
  store: ExperienceStoreAdapter,
  calibration: WorkspaceCalibration,
): CalibrationValidation {
  const fresh = deriveWorkspaceCalibration(store, {
    anticipationLineageId: calibration.anticipationLineageId,
  });
  return (
    fresh?.validation ?? {
      valid: false,
      failureReasons: ["no_evidence"],
      compositionValidationResult: "failed",
      composedStabilityScore: null,
    }
  );
}

/** Replay — identical store ⇒ identical calibration. */
export function replayWorkspaceCalibration(store: ExperienceStoreAdapter): {
  calibration: WorkspaceCalibration | null;
  history: WorkspaceCalibration[];
} {
  const calibration = deriveWorkspaceCalibration(store);
  const history = listCalibrationHistory(store);
  return { calibration, history };
}
