/**
 * Purpose: Programme V IC2 — Workflow decision support (recommendation projections).
 * Owner: Frontend product shell
 * Inputs: Deterministic facts from WorkspaceState + Programme I/V projections
 * Outputs: Independent recommendation projections (what / why / owner / step)
 *
 * Recommendations are projections of existing truth — not decisions, not automation.
 * Operator remains the decision-maker. Capture / Update / Restore / Preview remain
 * owned by their existing capabilities.
 *
 * Stability: same inputs ⇒ same recommendations and identical wording.
 * No scoring, ranking algorithms, confidence, urgency, or persistence.
 *
 * Non-goals: recommendation engines, workflow AI, automation, execution,
 *   recommendation history/learning, background decision making
 */

import type { OperatorWorkflowStepId } from "./operatorWorkflowUi";

/** Stable ids — declaration order is enumeration order, not priority. */
export type WorkflowRecommendationId =
  | "preview_unavailable"
  | "capture_recommended"
  | "update_recommended"
  | "preview_recommended"
  | "restore_available";

export type WorkflowRecommendationOwner =
  | "Observation"
  | "Arrangement"
  | "Arrangement Comparison"
  | "Interaction"
  | "Restore Projection";

export interface WorkflowRecommendation {
  id: WorkflowRecommendationId;
  /** What is recommended. */
  action: string;
  /** Why it is recommended (facts only). */
  because: string;
  owner: WorkflowRecommendationOwner;
  workflowStep: OperatorWorkflowStepId;
  /** Full Explain Ownership line. */
  line: string;
}

/**
 * Facts already derived elsewhere — IC2 does not re-invent comparison.
 * Keep this shape boolean/count-based so wording stays stable across renders.
 */
export interface WorkflowRecommendationFacts {
  hasProfile: boolean;
  desktopReady: boolean;
  observedWindowCount: number;
  arrangementCount: number;
  arrangementSelected: boolean;
  /** True when IC4/IC6 change diff reports effective changes. */
  desktopDiffersFromArrangement: boolean;
  /** True when change diff reports windows on desktop not in Arrangement. */
  newWindowsDetected: boolean;
  previewActive: boolean;
  layoutEditing: boolean;
  restoreAvailable: boolean;
  /** Suppress while an operation is in flight — avoids flicker, not “thinking”. */
  operationInFlight: boolean;
}

function lineFor(action: string, because: string, owner: WorkflowRecommendationOwner): string {
  return `${action} because ${because} · Owner: ${owner}`;
}

/**
 * Project independent recommendations from existing facts.
 * Order is fixed declaration order (stable), not ranked priority.
 */
export function projectWorkflowRecommendations(
  facts: WorkflowRecommendationFacts,
): WorkflowRecommendation[] {
  if (facts.operationInFlight) {
    return [];
  }

  const recommendations: WorkflowRecommendation[] = [];

  // 1. Preview unavailable
  if (!facts.arrangementSelected) {
    recommendations.push({
      id: "preview_unavailable",
      action: "Preview unavailable",
      because: "no Arrangement is selected",
      owner: "Interaction",
      workflowStep: "preview",
      line: lineFor(
        "Preview unavailable",
        "no Arrangement is selected",
        "Interaction",
      ),
    });
  }

  // 2. Capture recommended
  if (
    facts.hasProfile &&
    facts.desktopReady &&
    facts.observedWindowCount > 0 &&
    (facts.arrangementCount === 0 ||
      (facts.arrangementSelected && facts.newWindowsDetected))
  ) {
    const because =
      facts.arrangementCount === 0
        ? "Desktop has observed windows and this Profile has no saved Arrangement"
        : "new windows have been detected on the Desktop that are not in the selected Arrangement";
    recommendations.push({
      id: "capture_recommended",
      action: "Capture recommended",
      because,
      owner: "Observation",
      workflowStep: "arrangement",
      line: lineFor("Capture recommended", because, "Observation"),
    });
  }

  // 3. Update recommended
  if (
    facts.arrangementSelected &&
    facts.desktopDiffersFromArrangement
  ) {
    const because =
      "the current desktop differs from the saved Arrangement";
    recommendations.push({
      id: "update_recommended",
      action: "Update recommended",
      because,
      owner: "Arrangement Comparison",
      workflowStep: "arrangement",
      line: lineFor("Update recommended", because, "Arrangement Comparison"),
    });
  }

  // 4. Preview recommended
  if (
    facts.arrangementSelected &&
    !facts.previewActive &&
    facts.desktopReady
  ) {
    const because = facts.layoutEditing
      ? "an Arrangement is selected and Preview is not active in Edit layout"
      : "an Arrangement is selected and Preview is not active — open Edit layout to Preview saved bounds";
    recommendations.push({
      id: "preview_recommended",
      action: "Preview recommended",
      because,
      owner: "Interaction",
      workflowStep: "preview",
      line: lineFor("Preview recommended", because, "Interaction"),
    });
  }

  // 5. Restore available
  if (facts.arrangementSelected && facts.restoreAvailable) {
    const because =
      "a complete Arrangement is selected and Restore readiness projects ready";
    recommendations.push({
      id: "restore_available",
      action: "Restore available",
      because,
      owner: "Restore Projection",
      workflowStep: "restore",
      line: lineFor("Restore available", because, "Restore Projection"),
    });
  }

  return recommendations;
}

/** Stable list signature for tests — ids only, declaration order. */
export function workflowRecommendationIds(
  recommendations: readonly WorkflowRecommendation[],
): WorkflowRecommendationId[] {
  return recommendations.map((item) => item.id);
}
