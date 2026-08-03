/**
 * Sprint 62 — Adaptation composition.
 * Deterministic coexistence of multiple active adaptations.
 * No new adaptation primitives. No Runtime Core / navigation / persistence changes.
 */

import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  adaptationLineageRefsPresent,
  buildLineageIdCatalogs,
} from "../dev/governancePrimitives";
import {
  getStabilityReport,
  listStabilityReports,
} from "./longitudinalAdaptation";
import type { WorkspaceDensity } from "../lib/density";
import {
  identityPresentation,
  listAdaptations,
  resolvePresentationConfiguration,
  type AdaptationTargetComponent,
  type MotionProfile,
  type PresentationConfiguration,
  type ResolvedPresentation,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";

export type ConflictCause =
  | "overlapping_targets"
  | "contradictory_density"
  | "contradictory_motion"
  | "contradictory_environment"
  | "contradictory_spacing"
  | "contradictory_emphasis"
  | "contradictory_grouping"
  | "contradictory_css_var";

export type ResolutionStrategy =
  | "merge_multiply"
  | "priority_replace"
  | "union_targets"
  | "clamp_range";

export interface AdaptationConflict {
  cause: ConflictCause;
  adaptationIds: [string, string];
  field: string;
  resolutionStrategy: ResolutionStrategy;
  /** Winner under priority_replace; null for multiply/union/clamp. */
  winnerAdaptationId: string | null;
}

export interface AdaptationConflictReport {
  schemaVersion: 1;
  activeAdaptationIds: string[];
  compositionOrder: string[];
  conflicts: AdaptationConflict[];
  /** True when zero conflicts detected. */
  compatible: boolean;
}

export interface CompositionLineage {
  adaptationIds: string[];
  proposalIds: string[];
  engineeringChangeIds: string[];
  evidenceSnapshotIds: string[];
  architectureSnapshotIds: string[];
  replaySessionIds: string[];
}

export interface CompositionResult {
  schemaVersion: 1;
  presentation: ResolvedPresentation;
  compositionOrder: string[];
  /** Priority rank: 0 = applied first (lowest), n-1 = applied last (highest). */
  priority: Array<{ adaptationId: string; rank: number }>;
  conflictReport: AdaptationConflictReport;
  lineage: CompositionLineage;
}

export interface CompositionValidationReport {
  schemaVersion: 1;
  validationResult: "passed" | "failed";
  composition: CompositionResult;
  composedStabilityScore: number;
  stabilityScores: Array<{ adaptationId: string; score: number }>;
  regressionAdaptationIds: string[];
  governanceIntact: boolean;
  missingLineageAdaptationIds: string[];
}

/** Active + passed, sorted by adaptationId (deterministic; not registration order). */
export function selectComposableAdaptations(
  adaptations: WorkspaceAdaptation[],
): WorkspaceAdaptation[] {
  return adaptations
    .filter(
      (a) =>
        a.rolloutState === "active" &&
        a.validation.validationResult === "passed",
    )
    .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));
}

/** Lexicographic adaptationId rank — lower id = lower priority (applied earlier). */
export function compositionPriorityRank(
  orderedIds: string[],
  adaptationId: string,
): number {
  return orderedIds.indexOf(adaptationId);
}

function pairIds(a: string, b: string): [string, string] {
  return a.localeCompare(b) <= 0 ? [a, b] : [b, a];
}

function winnerId(a: string, b: string): string {
  // Higher priority = later in lexicographic order (applied last).
  return a.localeCompare(b) > 0 ? a : b;
}

function intersectTargets(
  a: AdaptationTargetComponent[],
  b: AdaptationTargetComponent[],
): AdaptationTargetComponent[] {
  const set = new Set(b);
  return a.filter((t) => set.has(t)).sort();
}

function definedDensity(
  p: PresentationConfiguration,
): WorkspaceDensity | null | undefined {
  return p.density;
}

function definedMotion(
  p: PresentationConfiguration,
): MotionProfile | undefined {
  return p.motionProfile;
}

/**
 * Detect conflicts across the ordered active set.
 * Every override/merge is recorded — nothing is silent.
 */
export function analyzeAdaptationConflicts(
  ordered: WorkspaceAdaptation[],
): AdaptationConflictReport {
  const compositionOrder = ordered.map((a) => a.adaptationId);
  const conflicts: AdaptationConflict[] = [];

  for (let i = 0; i < ordered.length; i++) {
    for (let j = i + 1; j < ordered.length; j++) {
      const a = ordered[i]!;
      const b = ordered[j]!;
      const ids = pairIds(a.adaptationId, b.adaptationId);
      const win = winnerId(a.adaptationId, b.adaptationId);

      const overlap = intersectTargets(a.targetComponents, b.targetComponents);
      if (overlap.length > 0) {
        conflicts.push({
          cause: "overlapping_targets",
          adaptationIds: ids,
          field: `targets:${overlap.join(",")}`,
          resolutionStrategy: "union_targets",
          winnerAdaptationId: null,
        });
      }

      const dA = definedDensity(a.presentation);
      const dB = definedDensity(b.presentation);
      if (
        dA !== undefined &&
        dB !== undefined &&
        dA !== null &&
        dB !== null &&
        dA !== dB
      ) {
        conflicts.push({
          cause: "contradictory_density",
          adaptationIds: ids,
          field: "density",
          resolutionStrategy: "priority_replace",
          winnerAdaptationId: win,
        });
      }

      const mA = definedMotion(a.presentation);
      const mB = definedMotion(b.presentation);
      if (mA !== undefined && mB !== undefined && mA !== mB) {
        conflicts.push({
          cause: "contradictory_motion",
          adaptationIds: ids,
          field: "motionProfile",
          resolutionStrategy: "priority_replace",
          winnerAdaptationId: win,
        });
      }

      const eA = a.presentation.environmentalWeight;
      const eB = b.presentation.environmentalWeight;
      if (eA !== undefined && eB !== undefined && eA !== eB) {
        conflicts.push({
          cause: "contradictory_environment",
          adaptationIds: ids,
          field: "environmentalWeight",
          resolutionStrategy: "merge_multiply",
          winnerAdaptationId: null,
        });
        // Product may clamp.
        if ((eA * eB < 0.5 || eA * eB > 1.5) && eA !== 1 && eB !== 1) {
          conflicts.push({
            cause: "contradictory_environment",
            adaptationIds: ids,
            field: "environmentalWeight:clamp",
            resolutionStrategy: "clamp_range",
            winnerAdaptationId: null,
          });
        }
      }

      const sA = a.presentation.spacingScale;
      const sB = b.presentation.spacingScale;
      if (sA !== undefined && sB !== undefined && sA !== sB) {
        conflicts.push({
          cause: "contradictory_spacing",
          adaptationIds: ids,
          field: "spacingScale",
          resolutionStrategy: "merge_multiply",
          winnerAdaptationId: null,
        });
      }

      const emA = a.presentation.emphasisScale;
      const emB = b.presentation.emphasisScale;
      if (emA !== undefined && emB !== undefined && emA !== emB) {
        conflicts.push({
          cause: "contradictory_emphasis",
          adaptationIds: ids,
          field: "emphasisScale",
          resolutionStrategy: "merge_multiply",
          winnerAdaptationId: null,
        });
      }

      const gA = a.presentation.groupingTightness;
      const gB = b.presentation.groupingTightness;
      if (gA !== undefined && gB !== undefined && gA !== gB) {
        conflicts.push({
          cause: "contradictory_grouping",
          adaptationIds: ids,
          field: "groupingTightness",
          resolutionStrategy: "priority_replace",
          winnerAdaptationId: win,
        });
      }

      const cssA = a.presentation.cssVars ?? {};
      const cssB = b.presentation.cssVars ?? {};
      for (const key of Object.keys(cssA).sort()) {
        if (
          cssB[key as keyof typeof cssB] !== undefined &&
          cssA[key as keyof typeof cssA] !== cssB[key as keyof typeof cssB]
        ) {
          conflicts.push({
            cause: "contradictory_css_var",
            adaptationIds: ids,
            field: key,
            resolutionStrategy: "priority_replace",
            winnerAdaptationId: win,
          });
        }
      }
    }
  }

  // Stable conflict order.
  conflicts.sort((x, y) => {
    const c = x.cause.localeCompare(y.cause);
    if (c !== 0) return c;
    const a = x.adaptationIds[0].localeCompare(y.adaptationIds[0]);
    if (a !== 0) return a;
    const b = x.adaptationIds[1].localeCompare(y.adaptationIds[1]);
    if (b !== 0) return b;
    return x.field.localeCompare(y.field);
  });

  return {
    schemaVersion: 1,
    activeAdaptationIds: [...compositionOrder],
    compositionOrder,
    conflicts,
    compatible: conflicts.length === 0,
  };
}

function buildCompositionLineage(
  ordered: WorkspaceAdaptation[],
): CompositionLineage {
  const adaptationIds = ordered.map((a) => a.adaptationId);
  const proposalIds = [
    ...new Set(ordered.map((a) => a.proposalId).filter(Boolean)),
  ].sort();
  const engineeringChangeIds = [
    ...new Set(ordered.map((a) => a.engineeringChangeId).filter(Boolean)),
  ].sort();
  const evidenceSnapshotIds = [
    ...new Set(ordered.map((a) => a.evidenceSnapshotId).filter(Boolean)),
  ].sort();
  const architectureSnapshotIds = [
    ...new Set(ordered.map((a) => a.architectureSnapshotId).filter(Boolean)),
  ].sort();
  const replaySessionIds = [
    ...new Set(ordered.flatMap((a) => a.validation.replaySessionIds)),
  ].sort();
  return {
    adaptationIds,
    proposalIds,
    engineeringChangeIds,
    evidenceSnapshotIds,
    architectureSnapshotIds,
    replaySessionIds,
  };
}

/**
 * Compose N active adaptations into one resolved presentation.
 * Merge order = adaptationId ascending (priority rank ascending).
 */
export function composeAdaptations(
  adaptations: WorkspaceAdaptation[],
): CompositionResult {
  const ordered = selectComposableAdaptations(adaptations);
  const compositionOrder = ordered.map((a) => a.adaptationId);
  const priority = compositionOrder.map((adaptationId, rank) => ({
    adaptationId,
    rank,
  }));
  const conflictReport = analyzeAdaptationConflicts(ordered);
  const presentation =
    ordered.length === 0
      ? identityPresentation()
      : resolvePresentationConfiguration(adaptations);
  const lineage = buildCompositionLineage(ordered);

  return {
    schemaVersion: 1,
    presentation,
    compositionOrder,
    priority,
    conflictReport,
    lineage,
  };
}

export function composeAdaptationsFromStore(
  store: ExperienceStoreAdapter,
): CompositionResult {
  return composeAdaptations(listAdaptations(store));
}

/**
 * Validate composed active set: lineage, stability, regressions, conflicts recorded.
 */
export function validateComposition(
  store: ExperienceStoreAdapter,
): CompositionValidationReport {
  const adaptations = listAdaptations(store);
  const composition = composeAdaptations(adaptations);
  const ordered = selectComposableAdaptations(adaptations);
  const catalogs = buildLineageIdCatalogs(store);

  const missingLineageAdaptationIds: string[] = [];
  const stabilityScores: Array<{ adaptationId: string; score: number }> = [];
  const regressionAdaptationIds: string[] = [];

  for (const adaptation of ordered) {
    const intact = adaptationLineageRefsPresent(adaptation, catalogs);
    if (!intact) {
      missingLineageAdaptationIds.push(adaptation.adaptationId);
    }
    const report = getStabilityReport(store, adaptation.adaptationId);
    const score = report?.stabilityScore ?? 0;
    stabilityScores.push({
      adaptationId: adaptation.adaptationId,
      score,
    });
    if (report && report.regressionEvents.length > 0) {
      regressionAdaptationIds.push(adaptation.adaptationId);
    }
  }

  stabilityScores.sort((a, b) =>
    a.adaptationId.localeCompare(b.adaptationId),
  );
  missingLineageAdaptationIds.sort();
  regressionAdaptationIds.sort();

  const composedStabilityScore =
    stabilityScores.length === 0
      ? 1
      : Number(
          (
            stabilityScores.reduce((s, x) => s + x.score, 0) /
            stabilityScores.length
          ).toFixed(4),
        );

  const governanceIntact = missingLineageAdaptationIds.length === 0;
  // Conflicts are allowed if explicitly reported; fail only on lineage/regression.
  const validationResult =
    governanceIntact && regressionAdaptationIds.length === 0
      ? "passed"
      : "failed";

  // Touch listStabilityReports to keep derivation from store artefacts.
  void listStabilityReports(store);

  return {
    schemaVersion: 1,
    validationResult,
    composition,
    composedStabilityScore,
    stabilityScores,
    regressionAdaptationIds,
    governanceIntact,
    missingLineageAdaptationIds,
  };
}
