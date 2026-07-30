/**
 * Purpose: Programme V IC6 — Workflow observability (projection only).
 * Owner: Frontend product shell
 * Inputs: Current IC1–IC5 projection outputs + immediately preceding snapshot
 *   (Interaction State only) + in-flight execution flag
 * Outputs: Meaningful transition explanations (what / why / owner / stability /
 *   source projection)
 *
 * Observability explains how and why existing projections evolved — not history.
 * Compares current projection with at most one immediately preceding snapshot.
 * No timeline, event log, replay, analytics, or persistent observability state.
 *
 * Observation minimality: surface only semantic changes that materially affect
 * operator understanding. Identical recomputation produces no new transitions.
 *
 * Stability: identical previous+current snapshots ⇒ identical transitions.
 *
 * Non-goals: event history, timeline engine, activity log, behavioural analytics,
 *   replay, audit history, persistent observability state, metrics collection
 */

import type { OperationalInFlight } from "./operationalConfidenceUi";
import type { OperatorWorkflowProjection } from "./operatorWorkflowUi";
import type { WorkflowRecommendation } from "./workflowDecisionSupportUi";
import type { WorkflowRecoverabilityCondition } from "./workflowRecoverabilityUi";
import type { WorkflowPredictabilityOutcome } from "./workflowPredictabilityUi";
import type { WorkflowExplainabilitySummary } from "./workflowExplainabilityUi";

/** Stable ids — declaration order is enumeration order, not priority. */
export type WorkflowObservabilityId =
  | "phase_changed"
  | "recommendation_changed"
  | "recoverability_changed"
  | "predictability_changed"
  | "execution_transient"
  | "execution_stable";

export type ObservabilityStability = "stable" | "transient";

export type ObservabilitySourceProjection =
  | "ic1_workflow"
  | "ic2_recommendation"
  | "ic3_recoverability"
  | "ic4_predictability"
  | "interaction";

export interface WorkflowObservabilityTransition {
  id: WorkflowObservabilityId;
  what: string;
  because: string;
  owner: string;
  stability: ObservabilityStability;
  sourceProjection: ObservabilitySourceProjection;
  sourceId: string;
  line: string;
}

/**
 * Semantic snapshot of IC1–IC5 + execution flag.
 * Held at most once as the immediately preceding Interaction State value.
 * Never persisted across sessions; never accumulated as history.
 */
export interface WorkflowObservabilitySnapshot {
  phaseId: string;
  phaseLabel: string;
  phaseDetail: string;
  phaseOwner: string;
  workflowWhy: string;
  recommendationId: string | null;
  recommendationAction: string | null;
  recommendationBecause: string | null;
  recommendationOwner: string | null;
  recoverabilityId: string | null;
  recoverabilityWhat: string | null;
  recoverabilityClassification: string | null;
  recoverabilityBecause: string | null;
  recoverabilityOwner: string | null;
  predictabilityId: string | null;
  predictabilityWhat: string | null;
  predictabilityBecause: string | null;
  predictabilityOwner: string | null;
  /** Compact semantic summary from primary predictability outcome fields. */
  predictabilitySemantics: string | null;
  /** IC5 line — for fingerprint equivalence only (not a separate transition). */
  explainabilityLine: string;
  inFlight: OperationalInFlight;
}

export interface CaptureWorkflowObservabilitySnapshotInput {
  workflow: OperatorWorkflowProjection;
  recommendations: readonly WorkflowRecommendation[];
  recoverability: readonly WorkflowRecoverabilityCondition[];
  predictability: readonly WorkflowPredictabilityOutcome[];
  explainability: WorkflowExplainabilitySummary;
  inFlight: OperationalInFlight;
}

export interface ProjectWorkflowObservabilityInput {
  previous: WorkflowObservabilitySnapshot | null;
  current: WorkflowObservabilitySnapshot;
}

function lineFor(what: string, because: string, owner: string): string {
  return `${what} because ${because} · Owner: ${owner}`;
}

function transition(
  partial: Omit<WorkflowObservabilityTransition, "line">,
): WorkflowObservabilityTransition {
  return {
    ...partial,
    line: lineFor(partial.what, partial.because, partial.owner),
  };
}

function predictabilitySemantics(
  outcome: WorkflowPredictabilityOutcome,
): string {
  return [
    outcome.what,
    ...outcome.effects,
    ...outcome.unchanged,
    ...outcome.unavailable,
    ...outcome.informationGaps,
    outcome.because,
  ].join("|");
}

function inFlightLabel(inFlight: Exclude<OperationalInFlight, null>): string {
  switch (inFlight) {
    case "observe":
      return "Observing Desktop";
    case "save":
      return "Capture in progress";
    case "update":
      return "Update in progress";
    case "restore":
      return "Restore executing";
    default: {
      const _exhaustive: never = inFlight;
      return _exhaustive;
    }
  }
}

/**
 * Capture a semantic snapshot from existing IC1–IC5 outputs + Interaction State.
 * Primary items follow IC5 declaration-order selection (first of each list).
 */
export function captureWorkflowObservabilitySnapshot(
  input: CaptureWorkflowObservabilitySnapshotInput,
): WorkflowObservabilitySnapshot {
  const step =
    input.workflow.steps.find(
      (item) => item.id === input.workflow.currentStepId,
    ) ?? input.workflow.steps[0];
  const recommendation = input.recommendations[0] ?? null;
  const recoverability = input.recoverability[0] ?? null;
  const predictability = input.predictability[0] ?? null;

  return {
    phaseId: step.id,
    phaseLabel: step.label,
    phaseDetail: step.detail,
    phaseOwner: step.owner,
    workflowWhy: input.workflow.why,
    recommendationId: recommendation?.id ?? null,
    recommendationAction: recommendation?.action ?? null,
    recommendationBecause: recommendation?.because ?? null,
    recommendationOwner: recommendation?.owner ?? null,
    recoverabilityId: recoverability?.id ?? null,
    recoverabilityWhat: recoverability?.what ?? null,
    recoverabilityClassification: recoverability?.classification ?? null,
    recoverabilityBecause: recoverability?.because ?? null,
    recoverabilityOwner: recoverability?.owner ?? null,
    predictabilityId: predictability?.id ?? null,
    predictabilityWhat: predictability?.what ?? null,
    predictabilityBecause: predictability?.because ?? null,
    predictabilityOwner: predictability?.owner ?? null,
    predictabilitySemantics: predictability
      ? predictabilitySemantics(predictability)
      : null,
    explainabilityLine: input.explainability.line,
    inFlight: input.inFlight,
  };
}

/** Stable fingerprint for observation minimality (semantic equivalence). */
export function workflowObservabilityFingerprint(
  snapshot: WorkflowObservabilitySnapshot,
): string {
  return [
    snapshot.phaseId,
    snapshot.phaseDetail,
    snapshot.phaseOwner,
    snapshot.workflowWhy,
    snapshot.recommendationId ?? "",
    snapshot.recommendationAction ?? "",
    snapshot.recommendationBecause ?? "",
    snapshot.recoverabilityId ?? "",
    snapshot.recoverabilityWhat ?? "",
    snapshot.recoverabilityClassification ?? "",
    snapshot.recoverabilityBecause ?? "",
    snapshot.predictabilityId ?? "",
    snapshot.predictabilitySemantics ?? "",
    snapshot.explainabilityLine,
    snapshot.inFlight ?? "",
  ].join("\u001f");
}

function phaseChanged(
  previous: WorkflowObservabilitySnapshot,
  current: WorkflowObservabilitySnapshot,
): boolean {
  return (
    previous.phaseId !== current.phaseId ||
    previous.phaseDetail !== current.phaseDetail
  );
}

function recommendationChanged(
  previous: WorkflowObservabilitySnapshot,
  current: WorkflowObservabilitySnapshot,
): boolean {
  return (
    previous.recommendationId !== current.recommendationId ||
    previous.recommendationAction !== current.recommendationAction ||
    previous.recommendationBecause !== current.recommendationBecause
  );
}

function recoverabilityChanged(
  previous: WorkflowObservabilitySnapshot,
  current: WorkflowObservabilitySnapshot,
): boolean {
  return (
    previous.recoverabilityId !== current.recoverabilityId ||
    previous.recoverabilityWhat !== current.recoverabilityWhat ||
    previous.recoverabilityClassification !==
      current.recoverabilityClassification ||
    previous.recoverabilityBecause !== current.recoverabilityBecause
  );
}

function predictabilityChanged(
  previous: WorkflowObservabilitySnapshot,
  current: WorkflowObservabilitySnapshot,
): boolean {
  return (
    previous.predictabilityId !== current.predictabilityId ||
    previous.predictabilitySemantics !== current.predictabilitySemantics
  );
}

/**
 * Project meaningful transitions from previous → current snapshots.
 * Empty when previous is null (no predecessor) or fingerprints are identical.
 */
export function projectWorkflowObservability(
  input: ProjectWorkflowObservabilityInput,
): WorkflowObservabilityTransition[] {
  const { previous, current } = input;
  if (!previous) {
    return [];
  }
  if (
    workflowObservabilityFingerprint(previous) ===
    workflowObservabilityFingerprint(current)
  ) {
    return [];
  }

  const transitions: WorkflowObservabilityTransition[] = [];
  const baselineStability: ObservabilityStability = current.inFlight
    ? "transient"
    : "stable";

  // 1. Phase changed — IC1
  if (phaseChanged(previous, current)) {
    transitions.push(
      transition({
        id: "phase_changed",
        what: "Workflow phase changed",
        because: `workflow phase moved from ${previous.phaseLabel} to ${current.phaseLabel}`,
        owner: current.phaseOwner,
        stability: baselineStability,
        sourceProjection: "ic1_workflow",
        sourceId: current.phaseId,
      }),
    );
  }

  // 2. Recommendation changed — IC2
  if (recommendationChanged(previous, current)) {
    const because = current.recommendationBecause
      ? current.recommendationBecause
      : previous.recommendationId
        ? "recommendation projection cleared from current facts"
        : "recommendation projection is empty";
    const owner =
      current.recommendationOwner ??
      previous.recommendationOwner ??
      current.phaseOwner;
    transitions.push(
      transition({
        id: "recommendation_changed",
        what: "Recommendation changed",
        because,
        owner,
        stability: baselineStability,
        sourceProjection: "ic2_recommendation",
        sourceId: current.recommendationId ?? "none",
      }),
    );
  }

  // 3. Recoverability changed — IC3
  if (recoverabilityChanged(previous, current)) {
    const because = current.recoverabilityBecause
      ? current.recoverabilityBecause
      : previous.recoverabilityId
        ? "recoverability projection cleared from current facts"
        : "recoverability projection is empty";
    const owner =
      current.recoverabilityOwner ??
      previous.recoverabilityOwner ??
      current.phaseOwner;
    const what =
      previous.recoverabilityClassification === "non_recoverable" &&
      current.recoverabilityClassification === "recoverable"
        ? "Workflow became recoverable"
        : previous.recoverabilityClassification === "recoverable" &&
            current.recoverabilityClassification === "non_recoverable"
          ? "Workflow became non-recoverable"
          : "Recoverability changed";
    transitions.push(
      transition({
        id: "recoverability_changed",
        what,
        because,
        owner,
        stability: baselineStability,
        sourceProjection: "ic3_recoverability",
        sourceId: current.recoverabilityId ?? "none",
      }),
    );
  }

  // 4. Predictability changed — IC4
  if (predictabilityChanged(previous, current)) {
    const because = current.predictabilityBecause
      ? current.predictabilityBecause
      : previous.predictabilityId
        ? "predictability projection cleared from current facts"
        : "predictability projection is empty";
    const owner =
      current.predictabilityOwner ??
      previous.predictabilityOwner ??
      current.phaseOwner;
    transitions.push(
      transition({
        id: "predictability_changed",
        what: "Predictability changed",
        because,
        owner,
        stability: baselineStability,
        sourceProjection: "ic4_predictability",
        sourceId: current.predictabilityId ?? "none",
      }),
    );
  }

  // 5–6. Stable vs transient execution — Interaction State
  if (!previous.inFlight && current.inFlight) {
    transitions.push(
      transition({
        id: "execution_transient",
        what: "Workflow became transient",
        because: inFlightLabel(current.inFlight),
        owner: "Interaction",
        stability: "transient",
        sourceProjection: "interaction",
        sourceId: current.inFlight,
      }),
    );
  } else if (previous.inFlight && !current.inFlight) {
    transitions.push(
      transition({
        id: "execution_stable",
        what: "Workflow became stable",
        because: `${inFlightLabel(previous.inFlight)} finished — projections re-project from current facts`,
        owner: "Interaction",
        stability: "stable",
        sourceProjection: "interaction",
        sourceId: previous.inFlight,
      }),
    );
  } else if (
    previous.inFlight &&
    current.inFlight &&
    previous.inFlight !== current.inFlight
  ) {
    transitions.push(
      transition({
        id: "execution_transient",
        what: "Transient execution changed",
        because: `${inFlightLabel(previous.inFlight)} → ${inFlightLabel(current.inFlight)}`,
        owner: "Interaction",
        stability: "transient",
        sourceProjection: "interaction",
        sourceId: current.inFlight,
      }),
    );
  }

  return transitions;
}

/** Stable list signature for tests — ids only, declaration order. */
export function workflowObservabilityIds(
  transitions: readonly WorkflowObservabilityTransition[],
): WorkflowObservabilityId[] {
  return transitions.map((item) => item.id);
}

/** Stability label for chrome. */
export function observabilityStabilityLabel(
  stability: ObservabilityStability,
): string {
  return stability === "stable" ? "Stable" : "Transient";
}
