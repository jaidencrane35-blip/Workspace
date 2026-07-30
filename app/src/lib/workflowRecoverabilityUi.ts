/**
 * Purpose: Programme V IC3 — Workflow recoverability (explanation projections).
 * Owner: Frontend product shell
 * Inputs: Deterministic facts from WorkspaceState + Programme I/V projections
 * Outputs: Recoverability conditions (classification / what / why / owner / next step)
 *
 * Recoverability is a projection of existing truth — not a recovery subsystem.
 * Capability ownership is unchanged: Capture, Preview, Restore, and Arrangement
 * continue to own their own execution. IC3 only explains how they relate now.
 *
 * Explanation consistency: identical facts ⇒ identical classification, wording,
 * next step, and ownership attribution. No scoring, history, or automation.
 *
 * Non-goals: recovery engine, resumable workflow state, checkpoints, auto-retry,
 *   diagnostic persistence, recovery history, recovery automation
 */

import type { OperatorWorkflowStepId } from "./operatorWorkflowUi";
import type { StageDesktopLoadState } from "./stageDesktopUi";

/** Stable ids — declaration order is enumeration order, not priority. */
export type WorkflowRecoverabilityId =
  | "runtime_unavailable"
  | "observation_failed"
  | "no_desktop_windows"
  | "no_profile"
  | "arrangement_deleted"
  | "restore_data_unavailable"
  | "no_arrangement_capture"
  | "restore_unavailable_no_arrangement"
  | "preview_unavailable_no_arrangement"
  | "update_unavailable_matches"
  | "restore_partial_gaps";

/** Descriptive classification only — never persisted as runtime state. */
export type RecoverabilityClassification = "recoverable" | "non_recoverable";

export type RecoverabilityOwner =
  | "Observation"
  | "Desktop"
  | "Arrangement"
  | "Arrangement Selection"
  | "Arrangement Comparison"
  | "Interaction"
  | "Restore"
  | "Restore Projection";

/** Existing Workspace verbs only — no new actions. */
export type RecoverabilityVerb =
  | "capture"
  | "update"
  | "preview"
  | "restore"
  | "select_arrangement";

export type RecoverabilityNextStep =
  | { kind: "action"; verb: RecoverabilityVerb; label: string }
  | { kind: "none"; label: "No action required" };

export interface WorkflowRecoverabilityCondition {
  id: WorkflowRecoverabilityId;
  classification: RecoverabilityClassification;
  /** What is blocked or unavailable. */
  what: string;
  /** Why — derived facts only. */
  because: string;
  owner: RecoverabilityOwner;
  /** Prerequisite currently missing, when applicable. */
  missingPrerequisite: string | null;
  nextStep: RecoverabilityNextStep;
  workflowStep: OperatorWorkflowStepId;
  /** Explain Ownership line. */
  line: string;
}

/**
 * Facts already derived elsewhere — IC3 does not re-invent comparison.
 * Keep this shape boolean/count-based so wording stays stable across renders.
 */
export interface WorkflowRecoverabilityFacts {
  loadState: StageDesktopLoadState;
  hasProfile: boolean;
  observedWindowCount: number;
  arrangementCount: number;
  arrangementSelected: boolean;
  /** Selection id set but Arrangement no longer in the library. */
  staleArrangementSelection: boolean;
  restoreAvailable: boolean;
  /** True when IC4/IC6 change diff reports effective changes. */
  desktopDiffersFromArrangement: boolean;
  /** True when pre-Restore / meta reports missing tracked windows. */
  hasRestoreGaps: boolean;
  /** Suppress while an operation is in flight — avoids flicker, not “thinking”. */
  operationInFlight: boolean;
}

const VERB_LABEL: Record<RecoverabilityVerb, string> = {
  capture: "Capture Desktop",
  update: "Update",
  preview: "Open Preview",
  restore: "Restore",
  select_arrangement: "Select an Arrangement",
};

function lineFor(
  what: string,
  because: string,
  owner: RecoverabilityOwner,
): string {
  return `${what} because ${because} · Owner: ${owner}`;
}

function nextAction(verb: RecoverabilityVerb): RecoverabilityNextStep {
  return { kind: "action", verb, label: VERB_LABEL[verb] };
}

function noActionRequired(): RecoverabilityNextStep {
  return { kind: "none", label: "No action required" };
}

function condition(
  partial: Omit<WorkflowRecoverabilityCondition, "line">,
): WorkflowRecoverabilityCondition {
  return {
    ...partial,
    line: lineFor(partial.what, partial.because, partial.owner),
  };
}

/**
 * Project recoverability explanations from existing facts.
 * Order is fixed declaration order (stable), not ranked severity.
 */
export function projectWorkflowRecoverability(
  facts: WorkflowRecoverabilityFacts,
): WorkflowRecoverabilityCondition[] {
  if (facts.operationInFlight) {
    return [];
  }

  const conditions: WorkflowRecoverabilityCondition[] = [];

  // 1. Runtime unavailable
  if (facts.loadState === "runtime_unavailable") {
    conditions.push(
      condition({
        id: "runtime_unavailable",
        classification: "non_recoverable",
        what: "Workflow unavailable",
        because: "Desktop observation requires the app runtime",
        owner: "Observation",
        missingPrerequisite: "App runtime",
        nextStep: noActionRequired(),
        workflowStep: "desktop",
      }),
    );
    return conditions;
  }

  // 2. Observation failed
  if (facts.loadState === "error") {
    conditions.push(
      condition({
        id: "observation_failed",
        classification: "non_recoverable",
        what: "Workflow unavailable",
        because: "Desktop observation failed",
        owner: "Observation",
        missingPrerequisite: "Desktop observation",
        nextStep: noActionRequired(),
        workflowStep: "desktop",
      }),
    );
    return conditions;
  }

  if (facts.loadState !== "ready") {
    return conditions;
  }

  // 3. No desktop windows observed
  if (facts.observedWindowCount === 0) {
    conditions.push(
      condition({
        id: "no_desktop_windows",
        classification: "non_recoverable",
        what: "Capture unavailable",
        because: "no desktop windows are observed",
        owner: "Observation",
        missingPrerequisite: "Desktop windows",
        nextStep: noActionRequired(),
        workflowStep: "desktop",
      }),
    );
  }

  // 4. No Profile
  if (!facts.hasProfile) {
    conditions.push(
      condition({
        id: "no_profile",
        classification: "non_recoverable",
        what: "Arrangement save unavailable",
        because: "no Profile is selected",
        owner: "Arrangement",
        missingPrerequisite: "Profile",
        nextStep: noActionRequired(),
        workflowStep: "arrangement",
      }),
    );
  }

  // 5. Arrangement deleted / missing from library
  if (facts.staleArrangementSelection) {
    conditions.push(
      condition({
        id: "arrangement_deleted",
        classification: "non_recoverable",
        what: "Restore unavailable",
        because: "the selected Arrangement is no longer available",
        owner: "Arrangement",
        missingPrerequisite: "Arrangement",
        nextStep: noActionRequired(),
        workflowStep: "restore",
      }),
    );
  }

  // 6. Restore data unavailable (selected Arrangement cannot project ready)
  if (
    facts.arrangementSelected &&
    !facts.restoreAvailable &&
    facts.observedWindowCount > 0
  ) {
    conditions.push(
      condition({
        id: "restore_data_unavailable",
        classification: "non_recoverable",
        what: "Restore unavailable",
        because: "Restore data is unavailable for the selected Arrangement",
        owner: "Restore Projection",
        missingPrerequisite: "Restore data",
        nextStep: noActionRequired(),
        workflowStep: "restore",
      }),
    );
  }

  // 7. No Arrangement saved — Capture Desktop
  if (
    facts.hasProfile &&
    facts.arrangementCount === 0 &&
    facts.observedWindowCount > 0 &&
    !facts.staleArrangementSelection
  ) {
    conditions.push(
      condition({
        id: "no_arrangement_capture",
        classification: "recoverable",
        what: "Restore unavailable",
        because: "no Arrangement is saved for this Profile",
        owner: "Arrangement",
        missingPrerequisite: "Arrangement",
        nextStep: nextAction("capture"),
        workflowStep: "arrangement",
      }),
    );
  }

  // 8. Restore unavailable — no Arrangement selected (library has Arrangements)
  if (
    !facts.arrangementSelected &&
    !facts.staleArrangementSelection &&
    facts.arrangementCount > 0
  ) {
    conditions.push(
      condition({
        id: "restore_unavailable_no_arrangement",
        classification: "recoverable",
        what: "Restore unavailable",
        because: "no Arrangement is selected",
        owner: "Arrangement Selection",
        missingPrerequisite: "Arrangement selection",
        nextStep: nextAction("select_arrangement"),
        workflowStep: "restore",
      }),
    );
  }

  // 9. Preview unavailable — no Arrangement selected
  if (
    !facts.arrangementSelected &&
    !facts.staleArrangementSelection &&
    (facts.arrangementCount > 0 || facts.hasProfile)
  ) {
    conditions.push(
      condition({
        id: "preview_unavailable_no_arrangement",
        classification: "recoverable",
        what: "Preview unavailable",
        because: "no Arrangement is selected",
        owner: "Arrangement Selection",
        missingPrerequisite: "Arrangement selection",
        nextStep:
          facts.arrangementCount > 0
            ? nextAction("select_arrangement")
            : nextAction("capture"),
        workflowStep: "preview",
      }),
    );
  }

  // 10. Update unavailable — desktop matches saved Arrangement
  if (facts.arrangementSelected && !facts.desktopDiffersFromArrangement) {
    conditions.push(
      condition({
        id: "update_unavailable_matches",
        classification: "recoverable",
        what: "Update unavailable",
        because: "the current desktop matches the saved Arrangement",
        owner: "Arrangement Comparison",
        missingPrerequisite: null,
        nextStep: noActionRequired(),
        workflowStep: "arrangement",
      }),
    );
  }

  // 11. Restore may proceed with gaps — still an existing Restore action
  if (
    facts.arrangementSelected &&
    facts.restoreAvailable &&
    facts.hasRestoreGaps
  ) {
    conditions.push(
      condition({
        id: "restore_partial_gaps",
        classification: "recoverable",
        what: "Restore partial",
        because: "some tracked windows are missing from the desktop",
        owner: "Restore Projection",
        missingPrerequisite: "Matching desktop windows",
        nextStep: nextAction("restore"),
        workflowStep: "restore",
      }),
    );
  }

  return conditions;
}

/** Stable list signature for tests — ids only, declaration order. */
export function workflowRecoverabilityIds(
  conditions: readonly WorkflowRecoverabilityCondition[],
): WorkflowRecoverabilityId[] {
  return conditions.map((item) => item.id);
}

/** Classification label for chrome. */
export function recoverabilityClassificationLabel(
  classification: RecoverabilityClassification,
): string {
  return classification === "recoverable" ? "Recoverable" : "Non-recoverable";
}
