/**
 * Purpose: Programme V IC4 — Workflow predictability (explanation projections).
 * Owner: Frontend product shell
 * Inputs: Deterministic counts/flags from Programme I comparison + pre-Restore
 *   projections and WorkspaceState readiness (never re-computed here)
 * Outputs: Before-action outcome projections (what / unchanged / unavailable /
 *   information gaps / why / owner)
 *
 * Predictability explains deterministic consequences already implied by current
 * state — it does not predict the future, simulate, or plan.
 *
 * Prediction fidelity: never promise more precision than underlying facts support.
 * Uncertainty from missing facts is itself a deterministic projection.
 *
 * Stability: identical inputs ⇒ identical wording, counts, ordering, ownership.
 *
 * Non-goals: simulation engine, planner, speculative execution, forecasting,
 *   prediction cache, execution scheduling, optimisation, behavioural inference,
 *   duplicate comparison / restore-planning logic
 */

import type { OperatorWorkflowStepId } from "./operatorWorkflowUi";
import type { StageDesktopLoadState } from "./stageDesktopUi";

/** Stable ids — declaration order is enumeration order, not priority. */
export type WorkflowPredictabilityId =
  | "outcome_unavailable"
  | "restore_outcome"
  | "restore_outcome_limited"
  | "update_outcome"
  | "update_noop"
  | "capture_outcome"
  | "preview_outcome";

export type PredictabilityOwner =
  | "Observation"
  | "Arrangement"
  | "Arrangement Comparison"
  | "Interaction"
  | "Restore Projection";

export type PredictabilityVerb =
  | "restore"
  | "update"
  | "capture"
  | "preview"
  | "none";

export interface WorkflowPredictabilityOutcome {
  id: WorkflowPredictabilityId;
  verb: PredictabilityVerb;
  /** What action / outcome is being explained. */
  what: string;
  /** Expected affected effects (count-based; fidelity-limited). */
  effects: string[];
  /** Expected unchanged entities. */
  unchanged: string[];
  /** Expected skipped / cannot-apply items. */
  unavailable: string[];
  /** Facts that cannot be determined from current state. */
  informationGaps: string[];
  because: string;
  owner: PredictabilityOwner;
  workflowStep: OperatorWorkflowStepId;
  /** Explain Ownership line. */
  line: string;
}

/**
 * Facts already derived by Programme I / Stage — IC4 interprets only.
 * Counts must come from existing change-diff / pre-Restore / meta projections.
 */
export interface WorkflowPredictabilityFacts {
  loadState: StageDesktopLoadState;
  hasProfile: boolean;
  observedWindowCount: number;
  /** Windows selected for Capture — Stage selection only. */
  selectedWindowCount: number;
  arrangementSelected: boolean;
  restoreAvailable: boolean;
  /** From existing change diff / pre-Restore plan. */
  restoreMoveCount: number;
  restoreUnchangedCount: number;
  restoreUnavailableCount: number;
  restoreWithoutBoundsCount: number;
  /** From existing Arrangement vs desktop change diff. */
  updateAddedCount: number;
  updateRemovedCount: number;
  updateBoundsChangedCount: number;
  updateZOrderChangedCount: number;
  updateUnchangedCount: number;
  desktopDiffersFromArrangement: boolean;
  /** Saved bounds entries Preview can represent (meta; not a simulator). */
  previewBoundsEntryCount: number;
  previewActive: boolean;
  /** Suppress while an operation is in flight — avoids flicker, not “thinking”. */
  operationInFlight: boolean;
}

function countLabel(count: number, singular: string, plural: string): string {
  return `${count} ${count === 1 ? singular : plural}`;
}

function lineFor(
  what: string,
  because: string,
  owner: PredictabilityOwner,
): string {
  return `${what} because ${because} · Owner: ${owner}`;
}

function outcome(
  partial: Omit<WorkflowPredictabilityOutcome, "line">,
): WorkflowPredictabilityOutcome {
  return {
    ...partial,
    line: lineFor(partial.what, partial.because, partial.owner),
  };
}

/**
 * Project before-action outcome explanations from existing comparison facts.
 * Order is fixed declaration order (stable), not ranked importance.
 */
export function projectWorkflowPredictability(
  facts: WorkflowPredictabilityFacts,
): WorkflowPredictabilityOutcome[] {
  if (facts.operationInFlight) {
    return [];
  }

  const outcomes: WorkflowPredictabilityOutcome[] = [];

  // 1. Observation / runtime not ready — uncertainty as projection
  if (facts.loadState === "runtime_unavailable") {
    outcomes.push(
      outcome({
        id: "outcome_unavailable",
        verb: "none",
        what: "Outcome unavailable",
        effects: [],
        unchanged: [],
        unavailable: [],
        informationGaps: [
          "workflow outcomes cannot be projected without the app runtime",
        ],
        because: "Desktop observation requires the app runtime",
        owner: "Observation",
        workflowStep: "desktop",
      }),
    );
    return outcomes;
  }

  if (facts.loadState === "error") {
    outcomes.push(
      outcome({
        id: "outcome_unavailable",
        verb: "none",
        what: "Outcome unavailable",
        effects: [],
        unchanged: [],
        unavailable: [],
        informationGaps: [
          "workflow outcomes cannot be projected because Desktop observation failed",
        ],
        because: "Desktop observation failed",
        owner: "Observation",
        workflowStep: "desktop",
      }),
    );
    return outcomes;
  }

  if (facts.loadState !== "ready") {
    outcomes.push(
      outcome({
        id: "outcome_unavailable",
        verb: "none",
        what: "Outcome unavailable",
        effects: [],
        unchanged: [],
        unavailable: [],
        informationGaps: [
          "workflow outcomes cannot be projected until Desktop observation is ready",
        ],
        because: "Desktop observation is not ready",
        owner: "Observation",
        workflowStep: "desktop",
      }),
    );
    return outcomes;
  }

  // 2. Restore outcome — from existing pre-Restore / comparison counts only
  if (facts.arrangementSelected && facts.restoreAvailable) {
    const effects: string[] = [];
    const unchanged: string[] = [];
    const unavailable: string[] = [];

    if (facts.restoreMoveCount > 0) {
      effects.push(
        `move ${countLabel(facts.restoreMoveCount, "window", "windows")}`,
      );
    }
    if (facts.restoreUnchangedCount > 0) {
      unchanged.push(
        `leave ${countLabel(facts.restoreUnchangedCount, "window", "windows")} unchanged`,
      );
    }
    if (facts.restoreUnavailableCount > 0) {
      unavailable.push(
        `skip ${countLabel(facts.restoreUnavailableCount, "unavailable window", "unavailable windows")}`,
      );
    }
    if (facts.restoreWithoutBoundsCount > 0) {
      unavailable.push(
        `skip ${countLabel(facts.restoreWithoutBoundsCount, "window without stored bounds", "windows without stored bounds")}`,
      );
    }
    if (
      effects.length === 0 &&
      unchanged.length === 0 &&
      unavailable.length === 0
    ) {
      effects.push("apply saved bounds where matching windows are available");
    }

    outcomes.push(
      outcome({
        id: "restore_outcome",
        verb: "restore",
        what: "Restore will",
        effects,
        unchanged,
        unavailable,
        informationGaps: [],
        because: "current comparison indicates these differences",
        owner: "Restore Projection",
        workflowStep: "restore",
      }),
    );
  }

  // 3. Restore limited — fidelity: explain that outcomes cannot be fully projected
  if (facts.arrangementSelected && !facts.restoreAvailable) {
    const informationGaps: string[] = [
      "a complete Restore outcome cannot be projected because Restore is not ready",
    ];
    if (facts.observedWindowCount === 0) {
      informationGaps.push("no desktop windows are observed for comparison");
    }
    if (facts.restoreWithoutBoundsCount > 0) {
      informationGaps.push(
        `${countLabel(facts.restoreWithoutBoundsCount, "tracked window lacks", "tracked windows lack")} stored bounds`,
      );
    }
    if (facts.restoreUnavailableCount > 0) {
      informationGaps.push(
        `${countLabel(facts.restoreUnavailableCount, "tracked window is", "tracked windows are")} missing from the desktop`,
      );
    }

    outcomes.push(
      outcome({
        id: "restore_outcome_limited",
        verb: "restore",
        what: "Restore outcome limited",
        effects: [],
        unchanged: [],
        unavailable: [],
        informationGaps,
        because: "required Restore facts are not available in current state",
        owner: "Restore Projection",
        workflowStep: "restore",
      }),
    );
  }

  // 4. Update outcome — from existing change-diff counts only
  if (facts.arrangementSelected && facts.desktopDiffersFromArrangement) {
    const effects: string[] = [];
    const unchanged: string[] = [];
    if (facts.updateAddedCount > 0) {
      effects.push(
        `add ${countLabel(facts.updateAddedCount, "window", "windows")} to the Arrangement`,
      );
    }
    if (facts.updateRemovedCount > 0) {
      effects.push(
        `remove ${countLabel(facts.updateRemovedCount, "window", "windows")} from the Arrangement`,
      );
    }
    if (facts.updateBoundsChangedCount > 0) {
      effects.push(
        `update bounds for ${countLabel(facts.updateBoundsChangedCount, "window", "windows")}`,
      );
    }
    if (facts.updateZOrderChangedCount > 0) {
      effects.push(
        `update z-order for ${countLabel(facts.updateZOrderChangedCount, "window", "windows")}`,
      );
    }
    if (facts.updateUnchangedCount > 0) {
      unchanged.push(
        `leave ${countLabel(facts.updateUnchangedCount, "window", "windows")} unchanged in the Arrangement`,
      );
    }
    if (effects.length === 0) {
      effects.push("record effective desktop differences into the Arrangement");
    }

    outcomes.push(
      outcome({
        id: "update_outcome",
        verb: "update",
        what: "Update will",
        effects,
        unchanged,
        unavailable: [],
        informationGaps: [],
        because: "the saved Arrangement differs from the current desktop",
        owner: "Arrangement Comparison",
        workflowStep: "arrangement",
      }),
    );
  }

  // 5. Update noop — desktop matches
  if (facts.arrangementSelected && !facts.desktopDiffersFromArrangement) {
    const unchanged: string[] = [];
    if (facts.updateUnchangedCount > 0) {
      unchanged.push(
        `leave ${countLabel(facts.updateUnchangedCount, "window", "windows")} unchanged`,
      );
    } else {
      unchanged.push("leave the saved Arrangement unchanged");
    }

    outcomes.push(
      outcome({
        id: "update_noop",
        verb: "update",
        what: "Update will",
        effects: ["make no effective Arrangement changes"],
        unchanged,
        unavailable: [],
        informationGaps: [],
        because: "the current desktop matches the saved Arrangement",
        owner: "Arrangement Comparison",
        workflowStep: "arrangement",
      }),
    );
  }

  // 6. Capture outcome — selection count only (Stage Capture path)
  if (facts.hasProfile && facts.selectedWindowCount > 0) {
    outcomes.push(
      outcome({
        id: "capture_outcome",
        verb: "capture",
        what: "Capture will",
        effects: [
          `save ${countLabel(facts.selectedWindowCount, "selected window", "selected windows")} as a new Arrangement`,
        ],
        unchanged: ["leave desktop window positions unchanged"],
        unavailable: [],
        informationGaps: [],
        because: "selected Desktop windows are available to save",
        owner: "Arrangement",
        workflowStep: "arrangement",
      }),
    );
  }

  // 7. Preview outcome — Interaction capability fact, not a simulator
  if (facts.arrangementSelected && !facts.previewActive) {
    const effects: string[] = [
      "show saved Arrangement bounds as overlays",
    ];
    if (facts.previewBoundsEntryCount > 0) {
      effects.push(
        `represent up to ${countLabel(facts.previewBoundsEntryCount, "saved bounds entry", "saved bounds entries")}`,
      );
    }
    const informationGaps: string[] = [];
    if (facts.previewBoundsEntryCount === 0) {
      informationGaps.push(
        "no saved bounds entries are available for Preview overlays",
      );
    }

    outcomes.push(
      outcome({
        id: "preview_outcome",
        verb: "preview",
        what: "Preview will",
        effects,
        unchanged: ["leave desktop window positions unchanged"],
        unavailable: [],
        informationGaps,
        because:
          "Preview displays saved bounds without applying them to the desktop",
        owner: "Interaction",
        workflowStep: "preview",
      }),
    );
  }

  return outcomes;
}

/** Stable list signature for tests — ids only, declaration order. */
export function workflowPredictabilityIds(
  outcomes: readonly WorkflowPredictabilityOutcome[],
): WorkflowPredictabilityId[] {
  return outcomes.map((item) => item.id);
}
