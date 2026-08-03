/**
 * Governed WorkspaceAdaptation — presentation configuration only.
 * Every adaptation requires full governance lineage.
 * Does not alter navigation, features, Runtime Core, persistence, or data model.
 */

import { fnv1a } from "../dev/devHash";
import type { ExperienceEvidence } from "../dev/experienceEvidence";
import type {
  ExperienceEvidenceMetrics,
  MetricDirection,
} from "../dev/experienceEvidence";
import type { ExperienceChangeProposal } from "../dev/experienceGovernance";
import type { ArchitectureSnapshot } from "../dev/architecturalIntegrity";
import type { EngineeringChangeRecord } from "../dev/engineeringGovernance";
import type { WorkspaceDensity } from "../lib/density";
import {
  browserStore,
  memoryStore,
  type ExperienceStoreAdapter,
} from "../dev/experienceStore";

export const ADAPTATION_STORAGE_KEY = "ws.experience.adaptation.v1";
export const MAX_ADAPTATIONS = 40;
export const ADAPTATION_CHANGED_EVENT = "ws-adaptation-changed";

export type AdaptationScope =
  | "spacing"
  | "emphasis"
  | "grouping"
  | "motion"
  | "density"
  | "environment";

export type AdaptationTargetComponent =
  | "shell"
  | "home"
  | "save"
  | "resume"
  | "pilot"
  | "help"
  | "dock"
  | "canvas";

export type AdaptationRolloutState =
  | "inactive"
  | "candidate"
  | "active"
  | "rolled_back";

export type AdaptationValidationResult = "pending" | "passed" | "failed";

export type RollbackCriterion =
  | "friction_regression"
  | "abandon_increase"
  | "ttc_regression"
  | "replay_divergence";

export type MotionProfile = "reduced" | "standard" | "expressive";

/** Allowlisted CSS custom properties the resolver may set on the shell. */
export type AdaptationCssVar =
  | "--intent-space"
  | "--intent-depth"
  | "--adapt-emphasis"
  | "--adapt-group"
  | "--motion-fast"
  | "--motion-base"
  | "--motion-slow"
  | "--spatial-gutter";

export interface PresentationConfiguration {
  density?: WorkspaceDensity | null;
  /** Multiplier on intent spacing (default 1). */
  spacingScale?: number;
  /** Multiplier on emphasis (default 1). */
  emphasisScale?: number;
  /** 0–1 grouping tightness (default 0.5). */
  groupingTightness?: number;
  motionProfile?: MotionProfile;
  /** Multiplier on environmental / atmosphere depth (default 1). */
  environmentalWeight?: number;
  cssVars?: Partial<Record<AdaptationCssVar, string>>;
}

export interface AdaptationValidationContract {
  baselineEvidenceId: string;
  expectedImprovementDelta: number;
  expectedDirection: MetricDirection;
  expectedMetric: keyof ExperienceEvidenceMetrics;
  rollbackCriteria: RollbackCriterion[];
  replaySessionIds: string[];
  validationResult: AdaptationValidationResult;
}

export interface WorkspaceAdaptation {
  schemaVersion: 1;
  adaptationId: string;
  engineeringChangeId: string;
  proposalId: string;
  evidenceSnapshotId: string;
  architectureSnapshotId: string;
  targetComponents: AdaptationTargetComponent[];
  scopes: AdaptationScope[];
  expectedMetric: keyof ExperienceEvidenceMetrics;
  expectedImprovementDelta: number;
  expectedDirection: MetricDirection;
  rolloutState: AdaptationRolloutState;
  validation: AdaptationValidationContract;
  presentation: PresentationConfiguration;
}

export interface ResolvedPresentation extends PresentationConfiguration {
  appliedAdaptationIds: string[];
}

export type AdaptationLineageError =
  | "missing_engineering"
  | "engineering_not_accepted"
  | "missing_proposal"
  | "proposal_not_accepted"
  | "missing_evidence"
  | "missing_architecture_snapshot"
  | "architecture_integrity_invalid"
  | "lineage_mismatch"
  | "missing_replay"
  | "missing_presentation";

export type AdaptationTransitionError =
  | "not_found"
  | "lineage_incomplete"
  | "validation_not_passed"
  | "invalid_state"
  | "rolled_back";

interface AdaptationBundle {
  schemaVersion: 1;
  adaptations: WorkspaceAdaptation[];
}

const ID_RE = /^[a-z0-9_.:-]{1,96}$/i;

const PROPOSAL_OK = new Set([
  "accepted",
  "implemented",
  "validated",
  "closed",
]);

const ENGINEERING_OK = new Set([
  "architecturally_accepted",
  "released",
]);

function emptyBundle(): AdaptationBundle {
  return { schemaVersion: 1, adaptations: [] };
}

function emitChanged(): void {
  if (typeof window !== "undefined") {
    window.dispatchEvent(new Event(ADAPTATION_CHANGED_EVENT));
  }
}

export function identityPresentation(): ResolvedPresentation {
  return {
    density: null,
    spacingScale: 1,
    emphasisScale: 1,
    groupingTightness: 0.5,
    motionProfile: "standard",
    environmentalWeight: 1,
    cssVars: {},
    appliedAdaptationIds: [],
  };
}

function clamp(n: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, n));
}

function mergePresentation(
  base: ResolvedPresentation,
  next: PresentationConfiguration,
): ResolvedPresentation {
  const cssVars = { ...(base.cssVars ?? {}), ...(next.cssVars ?? {}) };
  return {
    density: next.density !== undefined ? next.density : base.density,
    spacingScale: clamp(
      (base.spacingScale ?? 1) * (next.spacingScale ?? 1),
      0.5,
      1.5,
    ),
    emphasisScale: clamp(
      (base.emphasisScale ?? 1) * (next.emphasisScale ?? 1),
      0.5,
      1.5,
    ),
    groupingTightness:
      next.groupingTightness !== undefined
        ? clamp(next.groupingTightness, 0, 1)
        : (base.groupingTightness ?? 0.5),
    motionProfile: next.motionProfile ?? base.motionProfile,
    environmentalWeight: clamp(
      (base.environmentalWeight ?? 1) * (next.environmentalWeight ?? 1),
      0.5,
      1.5,
    ),
    cssVars,
    appliedAdaptationIds: base.appliedAdaptationIds,
  };
}

/**
 * Deterministic resolver — presentation configuration only.
 * Active + passed adaptations apply in adaptationId order (stable).
 */
export function resolvePresentationConfiguration(
  adaptations: WorkspaceAdaptation[],
): ResolvedPresentation {
  const active = adaptations
    .filter(
      (a) =>
        a.rolloutState === "active" &&
        a.validation.validationResult === "passed",
    )
    .sort((a, b) => a.adaptationId.localeCompare(b.adaptationId));

  let resolved = identityPresentation();
  const applied: string[] = [];
  for (const adaptation of active) {
    resolved = mergePresentation(resolved, adaptation.presentation);
    applied.push(adaptation.adaptationId);
  }
  resolved.appliedAdaptationIds = applied;
  // Round for stability
  resolved.spacingScale = Number((resolved.spacingScale ?? 1).toFixed(4));
  resolved.emphasisScale = Number((resolved.emphasisScale ?? 1).toFixed(4));
  resolved.groupingTightness = Number(
    (resolved.groupingTightness ?? 0.5).toFixed(4),
  );
  resolved.environmentalWeight = Number(
    (resolved.environmentalWeight ?? 1).toFixed(4),
  );
  return resolved;
}

export interface AdaptationLineageContext {
  engineering: EngineeringChangeRecord | null;
  proposal: ExperienceChangeProposal | null;
  evidence: ExperienceEvidence | null;
  architectureSnapshot: ArchitectureSnapshot | null;
}

export function verifyAdaptationLineage(
  adaptation: WorkspaceAdaptation,
  context: AdaptationLineageContext,
): AdaptationLineageError | null {
  if (!context.engineering) {
    return "missing_engineering";
  }
  if (!ENGINEERING_OK.has(context.engineering.state)) {
    return "engineering_not_accepted";
  }
  if (context.engineering.changeId !== adaptation.engineeringChangeId) {
    return "lineage_mismatch";
  }
  if (!context.proposal) {
    return "missing_proposal";
  }
  if (!PROPOSAL_OK.has(context.proposal.state)) {
    return "proposal_not_accepted";
  }
  if (context.proposal.proposalId !== adaptation.proposalId) {
    return "lineage_mismatch";
  }
  if (!context.engineering.proposalIds.includes(adaptation.proposalId)) {
    return "lineage_mismatch";
  }
  if (!context.evidence) {
    return "missing_evidence";
  }
  if (context.evidence.evidenceId !== adaptation.evidenceSnapshotId) {
    return "lineage_mismatch";
  }
  if (!context.architectureSnapshot) {
    return "missing_architecture_snapshot";
  }
  if (
    context.architectureSnapshot.snapshotId !==
    adaptation.architectureSnapshotId
  ) {
    return "lineage_mismatch";
  }
  if (!context.architectureSnapshot.integrity.valid) {
    return "architecture_integrity_invalid";
  }
  if (adaptation.validation.replaySessionIds.length === 0) {
    return "missing_replay";
  }
  if (
    !adaptation.presentation ||
    Object.keys(adaptation.presentation).length === 0
  ) {
    return "missing_presentation";
  }
  return null;
}

/**
 * Build an inactive adaptation from full governance lineage.
 * No self-activation.
 */
export function buildAdaptationFromLineage(
  context: AdaptationLineageContext,
  options: {
    targetComponents: AdaptationTargetComponent[];
    scopes: AdaptationScope[];
    presentation: PresentationConfiguration;
    expectedMetric: keyof ExperienceEvidenceMetrics;
    expectedImprovementDelta: number;
    expectedDirection: MetricDirection;
    rollbackCriteria: RollbackCriterion[];
  },
): WorkspaceAdaptation | null {
  const { engineering, proposal, evidence, architectureSnapshot } = context;
  if (!engineering || !proposal || !evidence || !architectureSnapshot) {
    return null;
  }
  if (!ENGINEERING_OK.has(engineering.state) || !PROPOSAL_OK.has(proposal.state)) {
    return null;
  }
  if (!architectureSnapshot.integrity.valid) {
    return null;
  }
  if (!engineering.proposalIds.includes(proposal.proposalId)) {
    return null;
  }

  const replaySessionIds = [
    ...new Set(proposal.validation.replaySessionIds),
  ].sort((a, b) => a.localeCompare(b));
  if (replaySessionIds.length === 0) {
    return null;
  }

  const targetComponents = [
    ...new Set(options.targetComponents),
  ].sort((a, b) => a.localeCompare(b)) as AdaptationTargetComponent[];
  const scopes = [...new Set(options.scopes)].sort((a, b) =>
    a.localeCompare(b),
  ) as AdaptationScope[];

  if (
    targetComponents.length === 0 ||
    scopes.length === 0 ||
    options.expectedImprovementDelta <= 0
  ) {
    return null;
  }

  const adaptationId = `adapt-${fnv1a(
    `${engineering.changeId}|${proposal.proposalId}|${evidence.evidenceId}|${architectureSnapshot.snapshotId}|${scopes.join(",")}|${options.expectedMetric}`,
  )}`;

  const adaptation: WorkspaceAdaptation = {
    schemaVersion: 1,
    adaptationId,
    engineeringChangeId: engineering.changeId,
    proposalId: proposal.proposalId,
    evidenceSnapshotId: evidence.evidenceId,
    architectureSnapshotId: architectureSnapshot.snapshotId,
    targetComponents,
    scopes,
    expectedMetric: options.expectedMetric,
    expectedImprovementDelta: options.expectedImprovementDelta,
    expectedDirection: options.expectedDirection,
    rolloutState: "inactive",
    validation: {
      baselineEvidenceId: evidence.evidenceId,
      expectedImprovementDelta: options.expectedImprovementDelta,
      expectedDirection: options.expectedDirection,
      expectedMetric: options.expectedMetric,
      rollbackCriteria: [...options.rollbackCriteria].sort((a, b) =>
        a.localeCompare(b),
      ) as RollbackCriterion[],
      replaySessionIds,
      validationResult: "pending",
    },
    presentation: { ...options.presentation },
  };

  if (verifyAdaptationLineage(adaptation, context) !== null) {
    return null;
  }
  return adaptation;
}

function metricValue(
  evidence: ExperienceEvidence,
  metric: keyof ExperienceEvidenceMetrics,
): number {
  return Number(evidence.metrics[metric]);
}

function improved(
  baseline: number,
  after: number,
  direction: MetricDirection,
  delta: number,
): boolean {
  if (direction === "lower_better") {
    return baseline - after >= delta - 1e-9;
  }
  return after - baseline >= delta - 1e-9;
}

function rollbackTriggered(
  baseline: ExperienceEvidence,
  after: ExperienceEvidence,
  criteria: RollbackCriterion[],
): boolean {
  for (const c of criteria) {
    if (c === "friction_regression") {
      if (after.metrics.meanFrictionScore > baseline.metrics.meanFrictionScore + 0.03) {
        return true;
      }
    }
    if (c === "abandon_increase") {
      if (after.metrics.abandonedFlowTotal > baseline.metrics.abandonedFlowTotal) {
        return true;
      }
    }
    if (c === "ttc_regression") {
      if (
        after.metrics.medianTimeToConfidenceMs >
        baseline.metrics.medianTimeToConfidenceMs + 250
      ) {
        return true;
      }
    }
    if (c === "replay_divergence") {
      if (after.metrics.replayDivergenceRate > 0) {
        return true;
      }
    }
  }
  return false;
}

/**
 * Compare before/after evidence. Failed adaptations stay / become inactive.
 * No self-promotion to active.
 */
export function validateAdaptationEvidence(
  adaptation: WorkspaceAdaptation,
  baseline: ExperienceEvidence,
  after: ExperienceEvidence,
): {
  validationResult: AdaptationValidationResult;
  rollback: boolean;
  adaptation: WorkspaceAdaptation;
} {
  const rollback = rollbackTriggered(
    baseline,
    after,
    adaptation.validation.rollbackCriteria,
  );
  const baseVal = metricValue(baseline, adaptation.expectedMetric);
  const afterVal = metricValue(after, adaptation.expectedMetric);
  const ok =
    !rollback &&
    improved(
      baseVal,
      afterVal,
      adaptation.expectedDirection,
      adaptation.expectedImprovementDelta,
    );

  const validationResult: AdaptationValidationResult = ok ? "passed" : "failed";
  const next: WorkspaceAdaptation = {
    ...adaptation,
    validation: {
      ...adaptation.validation,
      baselineEvidenceId: baseline.evidenceId,
      validationResult,
    },
    rolloutState:
      validationResult === "failed"
        ? adaptation.rolloutState === "active"
          ? "rolled_back"
          : "inactive"
        : adaptation.rolloutState === "active"
          ? "active"
          : "candidate",
  };

  return { validationResult, rollback, adaptation: next };
}

export function loadAdaptationBundle(
  store: ExperienceStoreAdapter,
): AdaptationBundle {
  const raw = store.getItem(ADAPTATION_STORAGE_KEY);
  if (!raw) {
    return emptyBundle();
  }
  try {
    const parsed = JSON.parse(raw) as AdaptationBundle;
    if (parsed?.schemaVersion !== 1 || !Array.isArray(parsed.adaptations)) {
      return emptyBundle();
    }
    return {
      schemaVersion: 1,
      adaptations: parsed.adaptations.slice(-MAX_ADAPTATIONS),
    };
  } catch {
    return emptyBundle();
  }
}

function saveAdaptationBundle(
  store: ExperienceStoreAdapter,
  bundle: AdaptationBundle,
): void {
  store.setItem(
    ADAPTATION_STORAGE_KEY,
    JSON.stringify({
      schemaVersion: 1,
      adaptations: bundle.adaptations.slice(-MAX_ADAPTATIONS),
    }),
  );
  emitChanged();
}

export function listAdaptations(
  store: ExperienceStoreAdapter,
): WorkspaceAdaptation[] {
  return loadAdaptationBundle(store).adaptations.map((a) => ({
    ...a,
    validation: { ...a.validation },
    presentation: { ...a.presentation },
  }));
}

export function getAdaptation(
  store: ExperienceStoreAdapter,
  adaptationId: string,
): WorkspaceAdaptation | null {
  const found = loadAdaptationBundle(store).adaptations.find(
    (a) => a.adaptationId === adaptationId,
  );
  return found
    ? {
        ...found,
        validation: { ...found.validation },
        presentation: { ...found.presentation },
      }
    : null;
}

/**
 * Persist adaptation only when lineage verifies. Starts inactive.
 */
export function persistAdaptation(
  store: ExperienceStoreAdapter,
  adaptation: WorkspaceAdaptation,
  context: AdaptationLineageContext,
):
  | { ok: true; adaptation: WorkspaceAdaptation }
  | { ok: false; error: AdaptationLineageError } {
  const lineageError = verifyAdaptationLineage(adaptation, context);
  if (lineageError) {
    return { ok: false, error: lineageError };
  }
  if (!ID_RE.test(adaptation.adaptationId)) {
    return { ok: false, error: "lineage_mismatch" };
  }

  const bundle = loadAdaptationBundle(store);
  const existing = bundle.adaptations.find(
    (a) => a.adaptationId === adaptation.adaptationId,
  );
  if (existing) {
    return { ok: true, adaptation: { ...existing } };
  }

  // Never persist as active / passed on create — no self-promotion.
  const stored: WorkspaceAdaptation = {
    ...adaptation,
    rolloutState: "inactive",
    validation: {
      ...adaptation.validation,
      validationResult:
        adaptation.validation.validationResult === "passed"
          ? "pending"
          : adaptation.validation.validationResult,
    },
  };

  bundle.adaptations = [...bundle.adaptations, stored];
  saveAdaptationBundle(store, bundle);
  return { ok: true, adaptation: { ...stored } };
}

export function upsertValidatedAdaptation(
  store: ExperienceStoreAdapter,
  adaptation: WorkspaceAdaptation,
): void {
  const bundle = loadAdaptationBundle(store);
  const index = bundle.adaptations.findIndex(
    (a) => a.adaptationId === adaptation.adaptationId,
  );
  const copy = {
    ...adaptation,
    validation: { ...adaptation.validation },
    presentation: { ...adaptation.presentation },
  };
  if (index >= 0) {
    const next = [...bundle.adaptations];
    next[index] = copy;
    bundle.adaptations = next;
  } else {
    bundle.adaptations = [...bundle.adaptations, copy];
  }
  saveAdaptationBundle(store, bundle);
}

/**
 * Activate only when validation passed. No automatic promotion.
 */
export function activateAdaptation(
  store: ExperienceStoreAdapter,
  adaptationId: string,
):
  | { ok: true; adaptation: WorkspaceAdaptation }
  | { ok: false; error: AdaptationTransitionError } {
  const bundle = loadAdaptationBundle(store);
  const index = bundle.adaptations.findIndex(
    (a) => a.adaptationId === adaptationId,
  );
  if (index < 0) {
    return { ok: false, error: "not_found" };
  }
  const current = bundle.adaptations[index]!;
  if (current.rolloutState === "rolled_back") {
    return { ok: false, error: "rolled_back" };
  }
  if (current.validation.validationResult !== "passed") {
    return { ok: false, error: "validation_not_passed" };
  }
  if (
    current.rolloutState !== "inactive" &&
    current.rolloutState !== "candidate"
  ) {
    return { ok: false, error: "invalid_state" };
  }
  const updated: WorkspaceAdaptation = {
    ...current,
    rolloutState: "active",
  };
  const next = [...bundle.adaptations];
  next[index] = updated;
  bundle.adaptations = next;
  saveAdaptationBundle(store, bundle);
  return { ok: true, adaptation: { ...updated } };
}

/** Rollback without data loss — record retained, presentation removed from resolver. */
export function rollbackAdaptation(
  store: ExperienceStoreAdapter,
  adaptationId: string,
):
  | { ok: true; adaptation: WorkspaceAdaptation }
  | { ok: false; error: AdaptationTransitionError } {
  const bundle = loadAdaptationBundle(store);
  const index = bundle.adaptations.findIndex(
    (a) => a.adaptationId === adaptationId,
  );
  if (index < 0) {
    return { ok: false, error: "not_found" };
  }
  const updated: WorkspaceAdaptation = {
    ...bundle.adaptations[index]!,
    rolloutState: "rolled_back",
  };
  const next = [...bundle.adaptations];
  next[index] = updated;
  bundle.adaptations = next;
  saveAdaptationBundle(store, bundle);
  return { ok: true, adaptation: { ...updated } };
}

export function deactivateAdaptation(
  store: ExperienceStoreAdapter,
  adaptationId: string,
):
  | { ok: true; adaptation: WorkspaceAdaptation }
  | { ok: false; error: AdaptationTransitionError } {
  const bundle = loadAdaptationBundle(store);
  const index = bundle.adaptations.findIndex(
    (a) => a.adaptationId === adaptationId,
  );
  if (index < 0) {
    return { ok: false, error: "not_found" };
  }
  const current = bundle.adaptations[index]!;
  if (current.rolloutState === "rolled_back") {
    return { ok: false, error: "rolled_back" };
  }
  const updated: WorkspaceAdaptation = {
    ...current,
    rolloutState: "inactive",
  };
  const next = [...bundle.adaptations];
  next[index] = updated;
  bundle.adaptations = next;
  saveAdaptationBundle(store, bundle);
  return { ok: true, adaptation: { ...updated } };
}

/** Shell CSS vars derived from resolved presentation (presentation only). */
export function presentationToShellStyle(
  baseIntentSpace: number,
  baseIntentDepth: number,
  presentation: ResolvedPresentation,
): Record<string, string> {
  const space = baseIntentSpace * (presentation.spacingScale ?? 1);
  const depth = baseIntentDepth * (presentation.environmentalWeight ?? 1);
  const style: Record<string, string> = {
    "--intent-space": String(Number(space.toFixed(4))),
    "--intent-depth": String(Number(depth.toFixed(4))),
    "--adapt-emphasis": String(presentation.emphasisScale ?? 1),
    "--adapt-group": String(presentation.groupingTightness ?? 0.5),
  };

  if (presentation.motionProfile === "reduced") {
    style["--motion-fast"] = "0ms";
    style["--motion-base"] = "0ms";
    style["--motion-slow"] = "0ms";
  } else if (presentation.motionProfile === "expressive") {
    style["--motion-fast"] = "200ms";
    style["--motion-base"] = "360ms";
    style["--motion-slow"] = "520ms";
  }

  if (presentation.groupingTightness !== undefined) {
    const g = presentation.groupingTightness;
    style["--spatial-gutter"] = `${Number((1.25 - g * 0.5).toFixed(3))}rem`;
  }

  for (const [key, value] of Object.entries(presentation.cssVars ?? {})) {
    if (value !== undefined) {
      style[key] = value;
    }
  }
  return style;
}

export function clearAdaptationStore(store: ExperienceStoreAdapter): void {
  store.removeItem(ADAPTATION_STORAGE_KEY);
  emitChanged();
}

export function defaultAdaptationStore(): ExperienceStoreAdapter {
  return browserStore() ?? memoryStore();
}
