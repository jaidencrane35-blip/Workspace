/**
 * Sprint 69 — Workspace Anticipation (+ Sprint 70 confidence calibration).
 * Predicts likely next interaction; never executes actions.
 * Calibration adjusts confidence only — prediction selection unchanged.
 * No AI decision-making. No new persistence. No Runtime Core / navigation changes.
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
  predictAnticipationFromEvidence,
  type AnticipatedMoment,
} from "./anticipationPrediction";
import { clamp01, round4 } from "./experienceMath";
import {
  applyConfidenceCalibration,
  deriveWorkspaceCalibration,
  type ReliabilityBand,
  type WorkspaceCalibration,
} from "./workspaceCalibration";
import {
  deriveActiveMemoryEvolution,
  type WorkspaceMemoryEvolution,
} from "./workspaceMemoryEvolution";
import {
  deriveWorkspacePresence,
  resolvePresentationWithPresence,
  type WorkspacePresence,
} from "./workspacePresence";
import {
  finalizeResolvedPresentation,
  type AdaptationTargetComponent,
  type ResolvedPresentation,
} from "./workspaceAdaptation";
import { resolvePresentationWithStability } from "./workspacePresentationStability";

export type { AnticipatedMoment } from "./anticipationPrediction";
export { predictAnticipationFromEvidence } from "./anticipationPrediction";

export type AnticipationValidationFailure =
  | "no_evidence"
  | "no_certified_lineage"
  | "presence_inactive"
  | "evolution_inactive"
  | "missing_replay"
  | "missing_architecture_snapshot"
  | "composition_regressed"
  | "lineage_incomplete"
  | "confidence_unstable"
  | "prediction_bypass_attempt";

export interface AnticipationEvidenceLineage {
  evidenceSnapshotIds: string[];
  tipEvidenceId: string | null;
}

export interface AnticipationReplayLineage {
  replaySessionIds: string[];
  bundleReplayInvocations: number;
}

export interface AnticipationReadiness {
  /** Subtle emphasis multiplier relative to presence baseline (1 = unchanged). */
  subtleEmphasis: number;
  /** Environmental weighting for pre-attention (maps to environmentalWeight). */
  environmentalWeighting: number;
  /** Pre-attentive focus 0–1 (informational — does not move objects). */
  preAttentiveFocus: number;
  /** Object readiness 0–1 (informational — does not execute). */
  objectReadiness: number;
  /** Motion preparation 0–1 (informational cadence cue). */
  motionPreparation: number;
}

export interface AnticipationValidation {
  valid: boolean;
  failureReasons: AnticipationValidationFailure[];
  compositionValidationResult: "passed" | "failed";
  composedStabilityScore: number | null;
  presenceId: string | null;
  evolutionId: string | null;
}

/**
 * Derived anticipation — not persisted.
 * The system predicts. The user decides. Nothing executes automatically.
 */
export interface WorkspaceAnticipation {
  schemaVersion: 1;
  anticipationId: string;
  likelyFocalRegion: AdaptationTargetComponent | null;
  likelyNextMoment: AnticipatedMoment | null;
  likelyContinuationTarget: AnticipatedMoment | null;
  /** Calibrated confidence when calibration active; else raw. */
  confidence: number;
  /** Pre-calibration heuristic confidence. */
  rawConfidence: number;
  calibrationId: string | null;
  reliabilityBand: ReliabilityBand | null;
  evidenceLineage: AnticipationEvidenceLineage;
  replayLineage: AnticipationReplayLineage;
  architectureSnapshotIds: string[];
  contributingAdaptationIds: string[];
  certificationIds: string[];
  presenceId: string | null;
  evolutionId: string | null;
  readiness: AnticipationReadiness;
  active: boolean;
  validation: AnticipationValidation;
}

function tipEvidence(store: ExperienceStoreAdapter): ExperienceEvidence | null {
  return tipOf(listEvidenceSnapshots(store));
}

function readinessFrom(
  presentation: ResolvedPresentation,
  confidence: number,
  presence: WorkspacePresence | null,
): AnticipationReadiness {
  const emphasis = presentation.emphasisScale ?? 1;
  const env = presentation.environmentalWeight ?? 1;
  return {
    subtleEmphasis: round4(emphasis),
    environmentalWeighting: round4(env),
    preAttentiveFocus: round4(
      clamp01((presence?.focalGravity ?? 0.5) * 0.7 + confidence * 0.3),
    ),
    objectReadiness: round4(clamp01(confidence)),
    motionPreparation: round4(
      clamp01(presence?.visualBreathingRhythm ?? 0.5) * 0.6 + confidence * 0.4,
    ),
  };
}

function validateAnticipation(
  store: ExperienceStoreAdapter,
  presence: WorkspacePresence | null,
  evolution: WorkspaceMemoryEvolution | null,
  evidence: ExperienceEvidence | null,
  contributingAdaptationIds: string[],
  architectureSnapshotIds: string[],
  replaySessionIds: string[],
  confidence: number,
): AnticipationValidation {
  const failureReasons: AnticipationValidationFailure[] = [];
  const composition = validateComposition(store);

  if (!evidence) {
    failureReasons.push("no_evidence");
  }
  if (contributingAdaptationIds.length === 0) {
    failureReasons.push("no_certified_lineage");
  }
  if (!presence?.active) {
    failureReasons.push("presence_inactive");
  }
  if (!evolution?.active) {
    failureReasons.push("evolution_inactive");
  }
  if (replaySessionIds.length === 0) {
    failureReasons.push("missing_replay");
  }
  if (architectureSnapshotIds.length === 0) {
    failureReasons.push("missing_architecture_snapshot");
  }
  if (composition.validationResult !== "passed") {
    failureReasons.push("composition_regressed");
  }
  if (!composition.governanceIntact) {
    failureReasons.push("lineage_incomplete");
  }
  if (confidence < 0.15 && contributingAdaptationIds.length > 0) {
    failureReasons.push("confidence_unstable");
  }
  if (
    evolution?.active &&
    contributingAdaptationIds.some(
      (id) => !evolution.originatingAdaptationIds.includes(id),
    )
  ) {
    failureReasons.push("prediction_bypass_attempt");
  }

  const unique = [
    ...new Set(failureReasons),
  ].sort() as AnticipationValidationFailure[];

  return {
    valid: unique.length === 0,
    failureReasons: unique,
    compositionValidationResult: composition.validationResult,
    composedStabilityScore: composition.composedStabilityScore,
    presenceId: presence?.presenceId ?? null,
    evolutionId: evolution?.evolutionId ?? null,
  };
}

function anticipationLineageIdOf(parts: {
  presenceId: string | null;
  evolutionId: string | null;
  likelyNextMoment: string | null;
  likelyContinuationTarget: string | null;
  likelyFocalRegion: string | null;
  contributingAdaptationIds: string[];
  tipEvidenceId: string | null;
}): string {
  return contentAddressedId(
    "anticipate",
    [
      parts.presenceId ?? "nopresence",
      parts.evolutionId ?? "noevo",
      parts.likelyNextMoment ?? "none",
      parts.likelyContinuationTarget ?? "none",
      parts.likelyFocalRegion ?? "none",
      parts.contributingAdaptationIds.join(","),
      parts.tipEvidenceId ?? "",
    ].join("|"),
  );
}

/**
 * Derive WorkspaceAnticipation. Predicts only — never executes.
 * Confidence may be calibrated (Sprint 70); prediction selection is unchanged.
 * Invalid anticipation ⇒ `active: false`.
 */
export function deriveWorkspaceAnticipation(
  store: ExperienceStoreAdapter,
): WorkspaceAnticipation | null {
  const presence = deriveWorkspacePresence(store);
  const evolution = deriveActiveMemoryEvolution(store);
  const evidence = tipEvidence(store);
  const snapshots = listEvidenceSnapshots(store);

  const contributingAdaptationIds = (
    presence?.active
      ? presence.contributingAdaptationIds
      : evolution?.active
        ? evolution.originatingAdaptationIds
        : []
  )
    .slice()
    .sort((a, b) => a.localeCompare(b));

  if (!evidence && contributingAdaptationIds.length === 0 && !presence) {
    return null;
  }

  const prediction = predictAnticipationFromEvidence(evidence);
  const evidenceSnapshotIds = [
    ...new Set([
      ...snapshots.map((s) => s.evidenceId),
      ...(presence?.active && evidence ? [evidence.evidenceId] : []),
      ...(evolution?.evidenceLineage.evidenceSnapshotIds ?? []),
    ]),
  ].sort();
  const tipEvidenceId = tipOf(evidenceSnapshotIds);

  const lineageId = anticipationLineageIdOf({
    presenceId: presence?.presenceId ?? null,
    evolutionId: evolution?.evolutionId ?? null,
    likelyNextMoment: prediction.likelyNextMoment,
    likelyContinuationTarget: prediction.likelyContinuationTarget,
    likelyFocalRegion: prediction.likelyFocalRegion,
    contributingAdaptationIds,
    tipEvidenceId,
  });

  // Internal calibration — not a new resolver stage.
  const calibration: WorkspaceCalibration | null = deriveWorkspaceCalibration(
    store,
    { anticipationLineageId: lineageId },
  );
  const confidence = applyConfidenceCalibration(
    prediction.rawConfidence,
    calibration,
  );

  const basePresentation = resolvePresentationWithPresence(store);
  const readiness = readinessFrom(basePresentation, confidence, presence);

  const replaySessionIds = [
    ...new Set([
      ...(evolution?.validation.replaySessionIds ?? []),
      ...(evidence?.sourceSessionIds ?? []),
    ]),
  ].sort();

  const architectureSnapshotIds = [
    ...new Set(evolution?.architectureSnapshotIds ?? []),
  ].sort();

  const certificationIds = [
    ...new Set(
      presence?.active
        ? presence.certificationIds
        : (evolution?.certificationLineage.certificationIds ?? []),
    ),
  ].sort();

  const validation = validateAnticipation(
    store,
    presence,
    evolution,
    evidence,
    contributingAdaptationIds,
    architectureSnapshotIds,
    replaySessionIds,
    confidence,
  );

  return {
    schemaVersion: 1,
    anticipationId: lineageId,
    likelyFocalRegion: prediction.likelyFocalRegion,
    likelyNextMoment: prediction.likelyNextMoment,
    likelyContinuationTarget: prediction.likelyContinuationTarget,
    confidence,
    rawConfidence: prediction.rawConfidence,
    calibrationId: calibration?.active ? calibration.calibrationId : null,
    reliabilityBand: calibration?.reliabilityBand ?? null,
    evidenceLineage: {
      evidenceSnapshotIds,
      tipEvidenceId,
    },
    replayLineage: {
      replaySessionIds,
      bundleReplayInvocations: getReplayInvocationCount(store),
    },
    architectureSnapshotIds,
    contributingAdaptationIds,
    certificationIds,
    presenceId: presence?.presenceId ?? null,
    evolutionId: evolution?.evolutionId ?? null,
    readiness,
    active: validation.valid,
    validation,
  };
}

export function getActiveWorkspaceAnticipation(
  store: ExperienceStoreAdapter,
): WorkspaceAnticipation | null {
  const anticipation = deriveWorkspaceAnticipation(store);
  if (!anticipation?.active) {
    return null;
  }
  return anticipation;
}

export function validateWorkspaceAnticipation(
  store: ExperienceStoreAdapter,
  anticipation: WorkspaceAnticipation,
): AnticipationValidation {
  const presence = deriveWorkspacePresence(store);
  const evolution = deriveActiveMemoryEvolution(store);
  const evidence = tipEvidence(store);
  return validateAnticipation(
    store,
    presence,
    evolution,
    evidence,
    anticipation.contributingAdaptationIds,
    anticipation.architectureSnapshotIds,
    anticipation.replayLineage.replaySessionIds,
    anticipation.confidence,
  );
}

/**
 * Apply anticipation readiness onto presentation.
 * Subtle emphasis / environmental weighting only — never moves objects or executes.
 * Values remain those of the certified presence baseline (no autonomous drift).
 */
export function presentationFromAnticipation(
  anticipation: WorkspaceAnticipation,
  base: ResolvedPresentation,
): ResolvedPresentation {
  if (!anticipation.active) {
    return base;
  }
  return finalizeResolvedPresentation({
    ...base,
    emphasisScale: anticipation.readiness.subtleEmphasis,
    environmentalWeight: anticipation.readiness.environmentalWeighting,
    appliedAdaptationIds: [...anticipation.contributingAdaptationIds].sort(
      (a, b) => a.localeCompare(b),
    ),
  });
}

/**
 * Resolver ordering (unchanged — no new stage):
 * Runtime → Pack → Evolution → Presence → Anticipation → Presentation
 *
 * Calibration adjusts confidence internally.
 * Presentation stability damps variance internally (Sprint 71).
 */
export function resolvePresentationWithAnticipation(
  store: ExperienceStoreAdapter,
): ResolvedPresentation {
  const base = resolvePresentationWithPresence(store);
  const anticipation = deriveWorkspaceAnticipation(store);
  const anticipated = anticipation?.active
    ? presentationFromAnticipation(anticipation, base)
    : base;
  return resolvePresentationWithStability(store, anticipated, {
    anticipationLineageId: anticipation?.anticipationId ?? null,
  });
}

/** Replay — identical store ⇒ identical anticipation + presentation. */
export function replayWorkspaceAnticipation(store: ExperienceStoreAdapter): {
  anticipation: WorkspaceAnticipation | null;
  presence: WorkspacePresence | null;
  evolution: WorkspaceMemoryEvolution | null;
  presentation: ResolvedPresentation;
  baselinePresentation: ResolvedPresentation;
} {
  const evolution = deriveActiveMemoryEvolution(store);
  const presence = deriveWorkspacePresence(store);
  const anticipation = deriveWorkspaceAnticipation(store);
  const baselinePresentation = resolvePresentationWithPresence(store);
  const presentation = resolvePresentationWithAnticipation(store);
  return {
    anticipation,
    presence,
    evolution,
    presentation,
    baselinePresentation,
  };
}
