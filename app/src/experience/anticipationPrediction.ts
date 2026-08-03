/**
 * Shared deterministic anticipation prediction from ExperienceEvidence.
 * Used by anticipation (selection) and calibration (accuracy scoring).
 * No persistence. No AI. No execution.
 */

import type { ExperienceDestination } from "../dev/experienceEvents";
import type { ExperienceEvidence } from "../dev/experienceEvidence";
import type { AdaptationTargetComponent } from "./workspaceAdaptation";
import { clamp01, round4 } from "./experienceMath";

export type AnticipatedMoment = Exclude<ExperienceDestination, "unknown">;

/** Re-export shared math for existing call sites. */
export { clamp01, round4 } from "./experienceMath";

const MOMENTS: readonly AnticipatedMoment[] = [
  "home",
  "save",
  "resume",
  "pilot",
  "help",
] as const;

export function isAnticipatedMoment(
  value: string | null | undefined,
): value is AnticipatedMoment {
  return !!value && (MOMENTS as readonly string[]).includes(value);
}

export function destinationToRegion(
  destination: AnticipatedMoment | null,
): AdaptationTargetComponent | null {
  if (!destination) {
    return null;
  }
  return destination;
}

export interface AnticipationPrediction {
  likelyNextMoment: AnticipatedMoment | null;
  likelyContinuationTarget: AnticipatedMoment | null;
  likelyFocalRegion: AdaptationTargetComponent | null;
  /** Raw heuristic confidence (pre-calibration). */
  rawConfidence: number;
}

/** Observed focal Moment from an evidence snapshot (for calibration scoring). */
export function observedMomentFromEvidence(
  evidence: ExperienceEvidence,
): AnticipatedMoment | null {
  const hotspot = [...evidence.hotspots].sort((a, b) => {
    if (b.samples !== a.samples) {
      return b.samples - a.samples;
    }
    return a.destination.localeCompare(b.destination);
  })[0];
  const raw =
    evidence.metrics.topHesitationDestination ?? hotspot?.destination ?? null;
  return isAnticipatedMoment(raw) ? raw : null;
}

/**
 * Deterministic prediction from a single evidence snapshot.
 * Selection rules are fixed — calibration must not alter these fields.
 */
export function predictAnticipationFromEvidence(
  evidence: ExperienceEvidence | null,
): AnticipationPrediction {
  if (!evidence) {
    return {
      likelyNextMoment: null,
      likelyContinuationTarget: null,
      likelyFocalRegion: null,
      rawConfidence: 0,
    };
  }

  const hotspot = [...evidence.hotspots].sort((a, b) => {
    if (b.samples !== a.samples) {
      return b.samples - a.samples;
    }
    return a.destination.localeCompare(b.destination);
  })[0];

  const nextRaw =
    evidence.metrics.topHesitationDestination ?? hotspot?.destination ?? null;
  const likelyNextMoment = isAnticipatedMoment(nextRaw) ? nextRaw : null;

  const continueN = evidence.metrics.continueSuccessTotal;
  const saveN = evidence.metrics.saveSuccessTotal;
  let likelyContinuationTarget: AnticipatedMoment | null = likelyNextMoment;
  if (continueN > saveN && continueN > 0) {
    likelyContinuationTarget = "resume";
  } else if (saveN >= continueN && saveN > 0) {
    likelyContinuationTarget = "save";
  }

  const likelyFocalRegion = destinationToRegion(
    likelyNextMoment ?? likelyContinuationTarget,
  );

  const sessionFactor = clamp01(evidence.metrics.sessionCount / 4);
  const replayFactor = clamp01(1 - evidence.metrics.replayDivergenceRate);
  const hotspotFactor = hotspot
    ? clamp01(hotspot.samples / Math.max(1, evidence.metrics.sessionCount))
    : 0.25;
  const rawConfidence = round4(
    clamp01(sessionFactor * 0.45 + replayFactor * 0.35 + hotspotFactor * 0.2),
  );

  return {
    likelyNextMoment,
    likelyContinuationTarget,
    likelyFocalRegion,
    rawConfidence,
  };
}
