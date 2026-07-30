/**
 * Purpose: Programme V IC5 — Workflow explainability (composition only).
 * Owner: Frontend product shell
 * Inputs: Outputs of Programme V IC1–IC4 projections (never re-derived here)
 * Outputs: Unified What → Why → Owner → Next action → Recoverability →
 *   Expected outcome explanation with source-projection traceability
 *
 * Narrative is composition. IC5 consumes IC1–IC4; it does not own facts,
 * recompute comparison, or invent claims absent from source projections.
 *
 * Narrative fidelity: uncertainty / information gaps from sources are preserved.
 * Explanation traceability: every section names its originating projection.
 *
 * Stability: identical IC1–IC4 outputs ⇒ identical summary sections and wording.
 * Primary item selection uses fixed declaration order — never scoring.
 *
 * Non-goals: narrative engine, summary cache, explanation persistence,
 *   AI-generated summaries, NL inference, workflow memory, new ownership
 */

import type { OperatorWorkflowProjection } from "./operatorWorkflowUi";
import type { WorkflowRecommendation } from "./workflowDecisionSupportUi";
import type { WorkflowRecoverabilityCondition } from "./workflowRecoverabilityUi";
import { recoverabilityClassificationLabel } from "./workflowRecoverabilityUi";
import type { WorkflowPredictabilityOutcome } from "./workflowPredictabilityUi";

/** Fixed section sequence — always this order when present. */
export type ExplainabilitySectionId =
  | "what"
  | "why"
  | "owner"
  | "next_action"
  | "recoverability"
  | "expected_outcome";

/** Architectural traceability — which projection produced the statement. */
export type ExplainabilitySourceProjection =
  | "ic1_workflow"
  | "ic2_recommendation"
  | "ic3_recoverability"
  | "ic4_predictability";

export interface WorkflowExplainabilitySection {
  id: ExplainabilitySectionId;
  label: string;
  /** Text taken from source projection fields — not paraphrased into new claims. */
  text: string;
  sourceProjection: ExplainabilitySourceProjection;
  /** Source item id (step / recommendation / condition / outcome id). */
  sourceId: string;
  /** Owner from the source projection when applicable. */
  owner: string | null;
}

export interface WorkflowExplainabilitySummary {
  pathLabel: string;
  sections: WorkflowExplainabilitySection[];
  /**
   * Compact single-line composition for chrome / tests.
   * Built only from section texts already composed from sources.
   */
  line: string;
}

/**
 * Inputs are already-projected Programme V outputs.
 * IC5 must not call IC1–IC4 projectors or Programme I comparison helpers.
 */
export interface ComposeWorkflowExplainabilityInput {
  workflow: OperatorWorkflowProjection;
  recommendations: readonly WorkflowRecommendation[];
  recoverability: readonly WorkflowRecoverabilityCondition[];
  predictability: readonly WorkflowPredictabilityOutcome[];
}

function currentStep(workflow: OperatorWorkflowProjection) {
  return (
    workflow.steps.find((step) => step.id === workflow.currentStepId) ??
    workflow.steps[0]
  );
}

function formatPredictabilityText(
  outcome: WorkflowPredictabilityOutcome,
): string {
  const parts: string[] = [outcome.what];
  for (const effect of outcome.effects) {
    parts.push(effect);
  }
  for (const line of outcome.unchanged) {
    parts.push(`Unchanged · ${line}`);
  }
  for (const line of outcome.unavailable) {
    parts.push(`Unavailable · ${line}`);
  }
  for (const gap of outcome.informationGaps) {
    parts.push(`Unavailable information · ${gap}`);
  }
  parts.push(`Because ${outcome.because}`);
  return parts.join(" · ");
}

function formatRecoverabilityText(
  condition: WorkflowRecoverabilityCondition,
): string {
  const parts = [
    condition.what,
    recoverabilityClassificationLabel(condition.classification),
    `Because ${condition.because}`,
  ];
  if (condition.missingPrerequisite) {
    parts.push(`Missing · ${condition.missingPrerequisite}`);
  }
  parts.push(`Next action · ${condition.nextStep.label}`);
  return parts.join(" · ");
}

/**
 * Compose a unified workflow explanation from IC1–IC4 projection outputs.
 * Fixed section order; primary items = first in each source array (declaration order).
 */
export function composeWorkflowExplainability(
  input: ComposeWorkflowExplainabilityInput,
): WorkflowExplainabilitySummary {
  const step = currentStep(input.workflow);
  const sections: WorkflowExplainabilitySection[] = [];

  // 1. What — IC1
  sections.push({
    id: "what",
    label: "What",
    text: `${step.label} · ${step.detail}`,
    sourceProjection: "ic1_workflow",
    sourceId: step.id,
    owner: null,
  });

  // 2. Why — IC1
  sections.push({
    id: "why",
    label: "Why",
    text: input.workflow.why,
    sourceProjection: "ic1_workflow",
    sourceId: step.id,
    owner: null,
  });

  // 3. Owner — IC1 (text is the owner; no secondary owner suffix)
  sections.push({
    id: "owner",
    label: "Owner",
    text: step.owner,
    sourceProjection: "ic1_workflow",
    sourceId: step.id,
    owner: null,
  });

  // 4. Next action — IC2 first recommendation, else IC1 nextAction
  const recommendation = input.recommendations[0];
  if (recommendation) {
    sections.push({
      id: "next_action",
      label: "Next action",
      text: `${recommendation.action} because ${recommendation.because}`,
      sourceProjection: "ic2_recommendation",
      sourceId: recommendation.id,
      owner: recommendation.owner,
    });
  } else if (input.workflow.nextAction) {
    sections.push({
      id: "next_action",
      label: "Next action",
      text: input.workflow.nextAction,
      sourceProjection: "ic1_workflow",
      sourceId: "next_action",
      owner: step.owner,
    });
  }

  // 5. Recoverability — first IC3 condition (declaration order), if any
  const recoverability = input.recoverability[0];
  if (recoverability) {
    sections.push({
      id: "recoverability",
      label: "Recoverability",
      text: formatRecoverabilityText(recoverability),
      sourceProjection: "ic3_recoverability",
      sourceId: recoverability.id,
      owner: recoverability.owner,
    });
  }

  // 6. Expected outcome — first IC4 outcome (declaration order), if any
  const predictability = input.predictability[0];
  if (predictability) {
    sections.push({
      id: "expected_outcome",
      label: "Expected outcome",
      text: formatPredictabilityText(predictability),
      sourceProjection: "ic4_predictability",
      sourceId: predictability.id,
      owner: predictability.owner,
    });
  }

  const line = sections
    .map((section) => `${section.label} · ${section.text}`)
    .join(" | ");

  return {
    pathLabel: input.workflow.pathLabel,
    sections,
    line,
  };
}

/** Stable section-id list for tests. */
export function workflowExplainabilitySectionIds(
  summary: WorkflowExplainabilitySummary,
): ExplainabilitySectionId[] {
  return summary.sections.map((section) => section.id);
}

/** Stable source-projection list for traceability tests. */
export function workflowExplainabilitySources(
  summary: WorkflowExplainabilitySummary,
): ExplainabilitySourceProjection[] {
  return summary.sections.map((section) => section.sourceProjection);
}
