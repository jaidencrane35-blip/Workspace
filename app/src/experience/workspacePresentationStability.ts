/**
 * Sprint 71 — Presentation Stability.
 * Internal presentation smoothing — not a new resolver stage.
 * Calms motion / emphasis / environment using evidence variance.
 * Never changes prediction selection, Runtime, navigation, or persistence.
 */

import {
  getReplayInvocationCount,
  listEvidenceSnapshots,
  type ExperienceEvidence,
} from "../dev/experienceEvidence";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import { contentAddressedId, tipOf } from "../dev/governancePrimitives";
import { validateComposition } from "./adaptationComposition";
import { deriveWorkspaceCalibration } from "./workspaceCalibration";
import { deriveActiveMemoryEvolution } from "./workspaceMemoryEvolution";
import {
  finalizeResolvedPresentation,
  identityPresentation,
  type MotionProfile,
  type ResolvedPresentation,
} from "./workspaceAdaptation";
import { clamp01, lerp, populationVariance, round4 } from "./experienceMath";

export type StabilityValidationFailure =
  | "no_evidence"
  | "insufficient_samples"
  | "missing_replay"
  | "missing_architecture_snapshot"
  | "missing_calibration_lineage"
  | "composition_regressed"
  | "lineage_incomplete"
  | "stability_regressed";

export interface StabilityEvidenceLineage {
  evidenceSnapshotIds: string[];
  tipEvidenceId: string | null;
}

export interface StabilityReplayLineage {
  replaySessionIds: string[];
  bundleReplayInvocations: number;
}

export interface StabilityValidation {
  valid: boolean;
  failureReasons: StabilityValidationFailure[];
  compositionValidationResult: "passed" | "failed";
  composedStabilityScore: number | null;
}

/**
 * Derived presentation stability — not persisted.
 * Measures and optionally damps presentation variance.
 */
export interface WorkspacePresentationStability {
  schemaVersion: 1;
  stabilityId: string;
  /** 0–1 overall calm/continuity score. */
  presentationStabilityScore: number;
  environmentalStability: number;
  motionContinuity: number;
  focalStability: number;
  transitionConsistency: number;
  /** Population variance of normalized presentation proxies (lower = calmer). */
  presentationVariance: number;
  /** Running variance after each evidence sample. */
  varianceTrend: number[];
  evidenceLineage: StabilityEvidenceLineage;
  replayLineage: StabilityReplayLineage;
  calibrationLineageId: string | null;
  anticipationLineageId: string | null;
  architectureSnapshotIds: string[];
  /** Damping factors applied when active (0–1 pull toward calm identity). */
  motionDamping: number;
  transitionCadence: number;
  emphasisSmoothing: number;
  environmentalInterpolation: number;
  focalSettling: number;
  atmosphericContinuity: number;
  active: boolean;
  validation: StabilityValidation;
}

const MIN_SAMPLES = 2;

function proxiesFromEvidence(evidence: ExperienceEvidence): {
  environmental: number;
  focal: number;
  motion: number;
  transition: number;
} {
  return {
    environmental: clamp01(evidence.metrics.meanFrictionScore),
    focal: clamp01(evidence.metrics.topHesitationMedianGapMs / 2000),
    motion: clamp01(evidence.metrics.replayDivergenceRate),
    transition: clamp01(evidence.metrics.abandonedFlowTotal / 10),
  };
}

export interface StabilityMeasurement {
  environmentalStability: number;
  motionContinuity: number;
  focalStability: number;
  transitionConsistency: number;
  presentationVariance: number;
  varianceTrend: number[];
  presentationStabilityScore: number;
}

/** Measure stability qualities from the evidence series (deterministic). */
export function measurePresentationStability(
  snapshots: readonly ExperienceEvidence[],
): StabilityMeasurement {
  if (snapshots.length === 0) {
    return {
      environmentalStability: 0,
      motionContinuity: 0,
      focalStability: 0,
      transitionConsistency: 0,
      presentationVariance: 0,
      varianceTrend: [],
      presentationStabilityScore: 0,
    };
  }

  const envSeries = snapshots.map((s) => proxiesFromEvidence(s).environmental);
  const focalSeries = snapshots.map((s) => proxiesFromEvidence(s).focal);
  const motionSeries = snapshots.map((s) => proxiesFromEvidence(s).motion);
  const transitionSeries = snapshots.map(
    (s) => proxiesFromEvidence(s).transition,
  );

  const envVar = populationVariance(envSeries);
  const focalVar = populationVariance(focalSeries);
  const motionVar = populationVariance(motionSeries);
  const transitionVar = populationVariance(transitionSeries);

  const presentationVariance = round4(
    (envVar + focalVar + motionVar + transitionVar) / 4,
  );

  const varianceTrend: number[] = [];
  for (let i = 0; i < snapshots.length; i++) {
    const slice = snapshots.slice(0, i + 1);
    const v =
      (populationVariance(slice.map((s) => proxiesFromEvidence(s).environmental)) +
        populationVariance(slice.map((s) => proxiesFromEvidence(s).focal)) +
        populationVariance(slice.map((s) => proxiesFromEvidence(s).motion)) +
        populationVariance(
          slice.map((s) => proxiesFromEvidence(s).transition),
        )) /
      4;
    varianceTrend.push(round4(v));
  }

  const tip = tipOf(snapshots)!;
  const environmentalStability = round4(clamp01(1 - Math.sqrt(envVar)));
  const focalStability = round4(clamp01(1 - Math.sqrt(focalVar)));
  const motionContinuity = round4(
    clamp01(1 - tip.metrics.replayDivergenceRate - Math.sqrt(motionVar) * 0.5),
  );
  const transitionConsistency = round4(
    clamp01(1 - Math.sqrt(transitionVar)),
  );
  const presentationStabilityScore = round4(
    clamp01(
      (environmentalStability +
        motionContinuity +
        focalStability +
        transitionConsistency) /
        4,
    ),
  );

  return {
    environmentalStability,
    motionContinuity,
    focalStability,
    transitionConsistency,
    presentationVariance,
    varianceTrend,
    presentationStabilityScore,
  };
}

function dampMotionProfile(
  profile: MotionProfile | undefined,
  motionDamping: number,
): MotionProfile {
  if (motionDamping < 0.2) {
    return profile ?? "standard";
  }
  if (profile === "expressive") {
    return motionDamping >= 0.5 ? "standard" : "expressive";
  }
  if (profile === "standard" && motionDamping >= 0.75) {
    return "reduced";
  }
  return profile ?? "standard";
}

/**
 * Apply active stability damping toward calm identity.
 * Inactive ⇒ presentation unchanged.
 */
export function applyStabilityToPresentation(
  presentation: ResolvedPresentation,
  stability: WorkspacePresentationStability | null,
): ResolvedPresentation {
  if (!stability?.active) {
    return presentation;
  }

  const identity = identityPresentation();
  const e = stability.emphasisSmoothing;
  const envT = stability.environmentalInterpolation;
  const focal = stability.focalSettling;
  const atmo = stability.atmosphericContinuity;
  const cadence = stability.transitionCadence;

  return finalizeResolvedPresentation({
    ...presentation,
    emphasisScale: round4(
      lerp(presentation.emphasisScale ?? 1, identity.emphasisScale ?? 1, e * focal),
    ),
    environmentalWeight: round4(
      lerp(
        presentation.environmentalWeight ?? 1,
        identity.environmentalWeight ?? 1,
        envT * atmo,
      ),
    ),
    groupingTightness: round4(
      lerp(
        presentation.groupingTightness ?? 0.5,
        identity.groupingTightness ?? 0.5,
        cadence * 0.5,
      ),
    ),
    spacingScale: round4(
      lerp(
        presentation.spacingScale ?? 1,
        identity.spacingScale ?? 1,
        cadence * 0.35,
      ),
    ),
    motionProfile: dampMotionProfile(
      presentation.motionProfile,
      stability.motionDamping,
    ),
    appliedAdaptationIds: [...presentation.appliedAdaptationIds],
  });
}

export function deriveWorkspacePresentationStability(
  store: ExperienceStoreAdapter,
  options: {
    anticipationLineageId?: string | null;
  } = {},
): WorkspacePresentationStability | null {
  const snapshots = listEvidenceSnapshots(store);
  if (snapshots.length === 0) {
    return null;
  }

  const evolution = deriveActiveMemoryEvolution(store);
  const calibration = deriveWorkspaceCalibration(store, {
    anticipationLineageId: options.anticipationLineageId ?? null,
  });
  const measurement = measurePresentationStability(snapshots);
  const composition = validateComposition(store);

  const failureReasons: StabilityValidationFailure[] = [];
  if (snapshots.length < MIN_SAMPLES) {
    failureReasons.push("insufficient_samples");
  }
  const replaySessionIds = [
    ...new Set([
      ...(evolution?.validation.replaySessionIds ?? []),
      ...snapshots.flatMap((s) => s.sourceSessionIds),
    ]),
  ].sort();
  if (replaySessionIds.length === 0) {
    failureReasons.push("missing_replay");
  }
  const architectureSnapshotIds = [
    ...new Set(evolution?.architectureSnapshotIds ?? []),
  ].sort();
  if (architectureSnapshotIds.length === 0) {
    failureReasons.push("missing_architecture_snapshot");
  }
  if (!calibration) {
    failureReasons.push("missing_calibration_lineage");
  }
  if (composition.validationResult !== "passed") {
    failureReasons.push("composition_regressed");
  }
  if (!composition.governanceIntact) {
    failureReasons.push("lineage_incomplete");
  }
  // Regression: high variance with enough samples.
  if (
    snapshots.length >= MIN_SAMPLES &&
    measurement.presentationVariance > 0.35
  ) {
    failureReasons.push("stability_regressed");
  }

  const unique = [
    ...new Set(failureReasons),
  ].sort() as StabilityValidationFailure[];
  const active = unique.length === 0;

  const score = measurement.presentationStabilityScore;
  // Stronger damping when score is high (calm evidence) — settle the room.
  const motionDamping = active ? round4(clamp01(score * 0.65)) : 0;
  const transitionCadence = active ? round4(clamp01(score * 0.55)) : 0;
  const emphasisSmoothing = active
    ? round4(clamp01(measurement.focalStability * 0.5))
    : 0;
  const environmentalInterpolation = active
    ? round4(clamp01(measurement.environmentalStability * 0.55))
    : 0;
  const focalSettling = active
    ? round4(clamp01(measurement.focalStability * 0.6))
    : 0;
  const atmosphericContinuity = active
    ? round4(clamp01(measurement.environmentalStability * 0.5))
    : 0;

  const evidenceSnapshotIds = snapshots.map((s) => s.evidenceId);
  const stabilityId = contentAddressedId(
    "stability",
    [
      String(measurement.presentationStabilityScore),
      String(measurement.presentationVariance),
      String(motionDamping),
      String(emphasisSmoothing),
      tipOf(evidenceSnapshotIds) ?? "",
      calibration?.calibrationId ?? "nocal",
      options.anticipationLineageId ?? "noant",
    ].join("|"),
  );

  return {
    schemaVersion: 1,
    stabilityId,
    presentationStabilityScore: measurement.presentationStabilityScore,
    environmentalStability: measurement.environmentalStability,
    motionContinuity: measurement.motionContinuity,
    focalStability: measurement.focalStability,
    transitionConsistency: measurement.transitionConsistency,
    presentationVariance: measurement.presentationVariance,
    varianceTrend: measurement.varianceTrend,
    evidenceLineage: {
      evidenceSnapshotIds,
      tipEvidenceId: tipOf(evidenceSnapshotIds),
    },
    replayLineage: {
      replaySessionIds,
      bundleReplayInvocations: getReplayInvocationCount(store),
    },
    calibrationLineageId: calibration?.calibrationId ?? null,
    anticipationLineageId: options.anticipationLineageId ?? null,
    architectureSnapshotIds,
    motionDamping,
    transitionCadence,
    emphasisSmoothing,
    environmentalInterpolation,
    focalSettling,
    atmosphericContinuity,
    active,
    validation: {
      valid: active,
      failureReasons: unique,
      compositionValidationResult: composition.validationResult,
      composedStabilityScore: composition.composedStabilityScore,
    },
  };
}

export function getActivePresentationStability(
  store: ExperienceStoreAdapter,
  options: {
    anticipationLineageId?: string | null;
  } = {},
): WorkspacePresentationStability | null {
  const stability = deriveWorkspacePresentationStability(store, options);
  if (!stability?.active) {
    return null;
  }
  return stability;
}

export function validatePresentationStability(
  store: ExperienceStoreAdapter,
  stability: WorkspacePresentationStability,
): StabilityValidation {
  const fresh = deriveWorkspacePresentationStability(store, {
    anticipationLineageId: stability.anticipationLineageId,
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

/**
 * Internal resolver hook — apply stability after anticipation presentation.
 * Not a new pipeline stage.
 */
export function resolvePresentationWithStability(
  store: ExperienceStoreAdapter,
  presentation: ResolvedPresentation,
  options: {
    anticipationLineageId?: string | null;
  } = {},
): ResolvedPresentation {
  const stability = deriveWorkspacePresentationStability(store, options);
  return applyStabilityToPresentation(presentation, stability);
}

export function replayPresentationStability(store: ExperienceStoreAdapter): {
  stability: WorkspacePresentationStability | null;
  measurement: StabilityMeasurement;
} {
  const snapshots = listEvidenceSnapshots(store);
  const stability = deriveWorkspacePresentationStability(store);
  return {
    stability,
    measurement: measurePresentationStability(snapshots),
  };
}
