/**
 * Sprint 68 — Workspace Presence.
 * Coherent environmental presentation derived from certified adaptations + memory evolution.
 * No AI decision-making. No new persistence. No Runtime Core / navigation changes.
 */

import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import { contentAddressedId } from "../dev/governancePrimitives";
import { validateComposition } from "./adaptationComposition";
import {
  deriveActiveMemoryEvolution,
  resolvePresentationFromRuntime,
  selectEvolutionMembers,
  type WorkspaceMemoryEvolution,
} from "./workspaceMemoryEvolution";
import type { WorkspaceDensity } from "../lib/density";
import {
  finalizeResolvedPresentation,
  identityPresentation,
  listAdaptations,
  type MotionProfile,
  type ResolvedPresentation,
} from "./workspaceAdaptation";

export type PresenceValidationFailure =
  | "no_certified_lineage"
  | "evolution_inactive"
  | "composition_regressed"
  | "composition_unstable"
  | "lineage_incomplete"
  | "presentation_bypass_attempt";

export interface PresenceResolvedEnvironment {
  /** Environmental lighting depth (maps to environmentalWeight). */
  lighting: number;
  /** Spacing rhythm (maps to spacingScale). */
  spacingRhythm: number;
  /** Atmospheric intensity (maps to environmentalWeight / atmosphere). */
  atmosphericIntensity: number;
  /** Motion cadence 0=reduced, 0.5=standard, 1=expressive. */
  motionCadence: number;
  /** Focal emphasis (maps to emphasisScale). */
  focalEmphasis: number;
  /** Depth weighting (maps to environmentalWeight). */
  depthWeighting: number;
  density: WorkspaceDensity | null;
  motionProfile: MotionProfile;
  groupingTightness: number;
}

export interface PresenceValidation {
  valid: boolean;
  failureReasons: PresenceValidationFailure[];
  compositionValidationResult: "passed" | "failed";
  composedStabilityScore: number | null;
  evolutionId: string | null;
}

/**
 * Derived presence profile — not persisted.
 * Presentation qualities only; no user content; no AI state.
 */
export interface WorkspacePresence {
  schemaVersion: 1;
  presenceId: string;
  environmentalCalm: number;
  spatialContinuity: number;
  focalGravity: number;
  contextualAtmosphere: number;
  visualBreathingRhythm: number;
  contributingAdaptationIds: string[];
  evolutionId: string | null;
  evolutionEpoch: number | null;
  certificationIds: string[];
  resolvedEnvironment: PresenceResolvedEnvironment;
  active: boolean;
  validation: PresenceValidation;
}

function clamp01(n: number): number {
  return Math.min(1, Math.max(0, n));
}

function round4(n: number): number {
  return Number(n.toFixed(4));
}

function motionCadenceOf(profile: MotionProfile | undefined): number {
  if (profile === "reduced") {
    return 0;
  }
  if (profile === "expressive") {
    return 1;
  }
  return 0.5;
}

function motionProfileOf(cadence: number): MotionProfile {
  if (cadence <= 0.25) {
    return "reduced";
  }
  if (cadence >= 0.75) {
    return "expressive";
  }
  return "standard";
}

function qualitiesFromPresentation(presentation: ResolvedPresentation): {
  environmentalCalm: number;
  spatialContinuity: number;
  focalGravity: number;
  contextualAtmosphere: number;
  visualBreathingRhythm: number;
  resolvedEnvironment: PresenceResolvedEnvironment;
} {
  const spacing = presentation.spacingScale ?? 1;
  const emphasis = presentation.emphasisScale ?? 1;
  const grouping = presentation.groupingTightness ?? 0.5;
  const env = presentation.environmentalWeight ?? 1;
  const motion = presentation.motionProfile ?? "standard";
  const cadence = motionCadenceOf(motion);

  // Derived qualities (0–1) — interpretive projection of certified presentation.
  const contextualAtmosphere = clamp01((env - 0.5) / 1);
  const environmentalCalm = clamp01(1 - Math.abs(env - 1) - cadence * 0.25);
  const spatialContinuity = clamp01(grouping);
  const focalGravity = clamp01((emphasis - 0.5) / 1);
  const visualBreathingRhythm = clamp01(
    0.5 + (spacing - 1) * 0.5 + (0.5 - Math.abs(cadence - 0.5)) * 0.25,
  );

  return {
    environmentalCalm: round4(environmentalCalm),
    spatialContinuity: round4(spatialContinuity),
    focalGravity: round4(focalGravity),
    contextualAtmosphere: round4(contextualAtmosphere),
    visualBreathingRhythm: round4(visualBreathingRhythm),
    resolvedEnvironment: {
      lighting: round4(env),
      spacingRhythm: round4(spacing),
      atmosphericIntensity: round4(env),
      motionCadence: round4(cadence),
      focalEmphasis: round4(emphasis),
      depthWeighting: round4(env),
      density: presentation.density ?? null,
      motionProfile: motion,
      groupingTightness: round4(grouping),
    },
  };
}

function validatePresence(
  store: ExperienceStoreAdapter,
  evolution: WorkspaceMemoryEvolution | null,
  contributingAdaptationIds: string[],
): PresenceValidation {
  const failureReasons: PresenceValidationFailure[] = [];
  const composition = validateComposition(store);

  if (contributingAdaptationIds.length === 0) {
    failureReasons.push("no_certified_lineage");
  }
  if (!evolution || !evolution.active) {
    failureReasons.push("evolution_inactive");
  }
  if (composition.validationResult !== "passed") {
    failureReasons.push("composition_regressed");
  }
  if (!composition.governanceIntact) {
    failureReasons.push("lineage_incomplete");
  }
  if (
    composition.stabilityScores.length > 0 &&
    composition.stabilityScores.every((s) => s.score > 0) &&
    composition.composedStabilityScore < 0.7
  ) {
    failureReasons.push("composition_unstable");
  }
  // Presence must never bypass certified adaptations / evolution.
  if (
    evolution &&
    evolution.active &&
    contributingAdaptationIds.some(
      (id) => !evolution.originatingAdaptationIds.includes(id),
    )
  ) {
    failureReasons.push("presentation_bypass_attempt");
  }

  const unique = [...new Set(failureReasons)].sort() as PresenceValidationFailure[];
  return {
    valid: unique.length === 0,
    failureReasons: unique,
    compositionValidationResult: composition.validationResult,
    composedStabilityScore: composition.composedStabilityScore,
    evolutionId: evolution?.evolutionId ?? null,
  };
}

/**
 * Derive WorkspacePresence from runtime → pack → evolution → certified adaptations.
 * Failed validation ⇒ `active: false` (presence not applied).
 */
export function deriveWorkspacePresence(
  store: ExperienceStoreAdapter,
): WorkspacePresence | null {
  const evolution = deriveActiveMemoryEvolution(store);
  const { members, certifications } = selectEvolutionMembers(store);
  const contributingAdaptationIds = (
    evolution?.active
      ? evolution.originatingAdaptationIds
      : members.map((m) => m.adaptationId)
  )
    .slice()
    .sort((a, b) => a.localeCompare(b));

  if (contributingAdaptationIds.length === 0 && !evolution) {
    return null;
  }

  // Presentation base follows certified pack → evolution path (no bypass).
  const basePresentation = resolvePresentationFromRuntime(store);
  const qualities = qualitiesFromPresentation(basePresentation);
  const validation = validatePresence(
    store,
    evolution,
    contributingAdaptationIds,
  );

  const certificationIds = (
    evolution?.active
      ? evolution.certificationLineage.certificationIds
      : certifications.map((c) => c.certificationId)
  )
    .slice()
    .sort((a, b) => a.localeCompare(b));

  const presenceId = contentAddressedId(
    "presence",
    [
      evolution?.evolutionId ?? "noevo",
      contributingAdaptationIds.join(","),
      certificationIds.join(","),
      String(qualities.resolvedEnvironment.lighting),
      String(qualities.resolvedEnvironment.spacingRhythm),
      String(qualities.resolvedEnvironment.focalEmphasis),
      String(qualities.resolvedEnvironment.motionCadence),
    ].join("|"),
  );

  return {
    schemaVersion: 1,
    presenceId,
    environmentalCalm: qualities.environmentalCalm,
    spatialContinuity: qualities.spatialContinuity,
    focalGravity: qualities.focalGravity,
    contextualAtmosphere: qualities.contextualAtmosphere,
    visualBreathingRhythm: qualities.visualBreathingRhythm,
    contributingAdaptationIds,
    evolutionId: evolution?.evolutionId ?? null,
    evolutionEpoch: evolution?.evolutionEpoch ?? null,
    certificationIds,
    resolvedEnvironment: qualities.resolvedEnvironment,
    active: validation.valid,
    validation,
  };
}

export function getActiveWorkspacePresence(
  store: ExperienceStoreAdapter,
): WorkspacePresence | null {
  const presence = deriveWorkspacePresence(store);
  if (!presence || !presence.active) {
    return null;
  }
  return presence;
}

export function validateWorkspacePresence(
  store: ExperienceStoreAdapter,
  presence: WorkspacePresence,
): PresenceValidation {
  const evolution = deriveActiveMemoryEvolution(store);
  return validatePresence(store, evolution, presence.contributingAdaptationIds);
}

/**
 * Map presence resolved environment onto presentation fields.
 * Only environmental lighting, spacing rhythm, atmosphere, motion, focal, depth.
 */
export function presentationFromPresence(
  presence: WorkspacePresence,
  base: ResolvedPresentation,
): ResolvedPresentation {
  if (!presence.active) {
    return base;
  }
  const env = presence.resolvedEnvironment;
  return finalizeResolvedPresentation({
    ...base,
    density: env.density,
    spacingScale: env.spacingRhythm,
    emphasisScale: env.focalEmphasis,
    groupingTightness: env.groupingTightness,
    motionProfile: env.motionProfile ?? motionProfileOf(env.motionCadence),
    environmentalWeight: env.depthWeighting,
    appliedAdaptationIds: [...presence.contributingAdaptationIds].sort((a, b) =>
      a.localeCompare(b),
    ),
  });
}

/**
 * Resolver ordering:
 * Runtime → Certified Pack → Memory Evolution → Workspace Presence → Presentation
 *
 * Presence never bypasses certified adaptations. Failed validation skips presence.
 */
export function resolvePresentationWithPresence(
  store: ExperienceStoreAdapter,
): ResolvedPresentation {
  const base = resolvePresentationFromRuntime(store);
  const presence = deriveWorkspacePresence(store);
  if (!presence?.active) {
    return base;
  }
  return presentationFromPresence(presence, base);
}

/** Replay — identical store ⇒ identical presence + presentation. */
export function replayWorkspacePresence(store: ExperienceStoreAdapter): {
  presence: WorkspacePresence | null;
  evolution: WorkspaceMemoryEvolution | null;
  presentation: ResolvedPresentation;
  baselinePresentation: ResolvedPresentation;
} {
  const evolution = deriveActiveMemoryEvolution(store);
  const presence = deriveWorkspacePresence(store);
  const baselinePresentation = resolvePresentationFromRuntime(store);
  const presentation = resolvePresentationWithPresence(store);
  return { presence, evolution, presentation, baselinePresentation };
}

/** Identity presence (no environmental influence). */
export function identityPresence(): WorkspacePresence {
  const base = identityPresentation();
  const qualities = qualitiesFromPresentation(base);
  return {
    schemaVersion: 1,
    presenceId: "presence-identity",
    environmentalCalm: qualities.environmentalCalm,
    spatialContinuity: qualities.spatialContinuity,
    focalGravity: qualities.focalGravity,
    contextualAtmosphere: qualities.contextualAtmosphere,
    visualBreathingRhythm: qualities.visualBreathingRhythm,
    contributingAdaptationIds: [],
    evolutionId: null,
    evolutionEpoch: null,
    certificationIds: [],
    resolvedEnvironment: qualities.resolvedEnvironment,
    active: false,
    validation: {
      valid: false,
      failureReasons: ["no_certified_lineage"],
      compositionValidationResult: "failed",
      composedStabilityScore: null,
      evolutionId: null,
    },
  };
}

/** Ensure presence path does not invent adaptations outside the store. */
export function presenceContributorsExist(store: ExperienceStoreAdapter): boolean {
  const presence = deriveWorkspacePresence(store);
  if (!presence) {
    return true;
  }
  const ids = new Set(listAdaptations(store).map((a) => a.adaptationId));
  return presence.contributingAdaptationIds.every((id) => ids.has(id));
}
