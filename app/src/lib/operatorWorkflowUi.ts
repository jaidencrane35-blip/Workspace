/**
 * Purpose: Programme V IC1 — Operator workflow composition (derived projection only).
 * Owner: Frontend product shell
 * Inputs: Observation load state, Arrangement selection, Preview/edit Interaction State,
 *   pre-Restore availability, in-flight op, last Restore result (session)
 * Outputs: Workflow progress steps, current focus, next-action hint, transparency line
 *
 * Composition, not orchestration:
 *   Desktop → Arrangement → Preview → Restore
 * exists because those capabilities already exist — not because a workflow runtime
 * coordinates them. Capability boundaries remain independent.
 *
 * Graceful interruption:
 *   Preview closed, Arrangement deselected, Desktop changes, Restore cancelled
 * simply change the inputs to this projection. No recovery engine, transaction
 * manager, or resumable workflow state.
 *
 * Non-goals: workflow controller, persistence, history, cache, background
 *   orchestration, implicit execution, new execution authority
 */

import type { StageDesktopLoadState } from "./stageDesktopUi";
import type { OperationalInFlight } from "./operationalConfidenceUi";

/** Canonical operator workflow steps — labels only; never persisted. */
export type OperatorWorkflowStepId =
  | "desktop"
  | "arrangement"
  | "preview"
  | "restore";

export type OperatorWorkflowStepStatus =
  | "inactive"
  | "ready"
  | "active"
  | "complete"
  | "blocked";

export interface OperatorWorkflowStep {
  id: OperatorWorkflowStepId;
  label: string;
  status: OperatorWorkflowStepStatus;
  /** Short fact for this step — derived, not stored. */
  detail: string;
  owner: "Observation" | "Desktop" | "Arrangement" | "Interaction" | "Restore";
}

export interface OperatorWorkflowProjection {
  steps: OperatorWorkflowStep[];
  /** Step that best describes “where the operator is” right now. */
  currentStepId: OperatorWorkflowStepId;
  /** Deterministic next valid action — never auto-executed. */
  nextAction: string;
  /** Explain Ownership line for the workflow as a whole. */
  line: string;
  /** Path label for chrome. */
  pathLabel: string;
}

export interface ProjectOperatorWorkflowInput {
  loadState: StageDesktopLoadState;
  hasProfile: boolean;
  arrangementCount: number;
  arrangementSelected: boolean;
  arrangementName?: string | null;
  /** Existing Preview capability (edit-session preview of saved bounds). */
  previewActive: boolean;
  layoutEditing: boolean;
  restoreAvailable: boolean;
  inFlight: OperationalInFlight;
  hasLastRestoreResult: boolean;
}

function desktopStep(
  input: ProjectOperatorWorkflowInput,
): OperatorWorkflowStep {
  if (input.loadState === "runtime_unavailable") {
    return {
      id: "desktop",
      label: "Desktop",
      status: "blocked",
      detail: "Runtime unavailable",
      owner: "Observation",
    };
  }
  if (input.loadState === "error") {
    return {
      id: "desktop",
      label: "Desktop",
      status: "blocked",
      detail: "Observation failed",
      owner: "Observation",
    };
  }
  if (input.loadState === "loading" || input.inFlight === "observe") {
    return {
      id: "desktop",
      label: "Desktop",
      status: "active",
      detail: "Observing",
      owner: "Observation",
    };
  }
  if (input.loadState === "ready") {
    return {
      id: "desktop",
      label: "Desktop",
      status: "complete",
      detail: "Observed",
      owner: "Observation",
    };
  }
  return {
    id: "desktop",
    label: "Desktop",
    status: "inactive",
    detail: "Waiting",
    owner: "Desktop",
  };
}

function arrangementStep(
  input: ProjectOperatorWorkflowInput,
): OperatorWorkflowStep {
  if (!input.hasProfile) {
    return {
      id: "arrangement",
      label: "Arrangement",
      status: "blocked",
      detail: "Needs Profile",
      owner: "Arrangement",
    };
  }
  if (input.arrangementSelected) {
    const name = input.arrangementName?.trim() || "Arrangement";
    return {
      id: "arrangement",
      label: "Arrangement",
      status: "active",
      detail: `Selected · ${name}`,
      owner: "Arrangement",
    };
  }
  if (input.arrangementCount > 0) {
    return {
      id: "arrangement",
      label: "Arrangement",
      status: "ready",
      detail: "Available · choose one",
      owner: "Arrangement",
    };
  }
  return {
    id: "arrangement",
    label: "Arrangement",
    status: "inactive",
    detail: "None saved yet",
    owner: "Arrangement",
  };
}

function previewStep(input: ProjectOperatorWorkflowInput): OperatorWorkflowStep {
  if (!input.arrangementSelected) {
    return {
      id: "preview",
      label: "Preview",
      status: "inactive",
      detail: "Needs Arrangement",
      owner: "Interaction",
    };
  }
  if (input.previewActive) {
    return {
      id: "preview",
      label: "Preview",
      status: "active",
      detail: "Active · saved bounds (visual only)",
      owner: "Interaction",
    };
  }
  if (input.layoutEditing) {
    return {
      id: "preview",
      label: "Preview",
      status: "ready",
      detail: "Available in Edit layout",
      owner: "Interaction",
    };
  }
  return {
    id: "preview",
    label: "Preview",
    status: "ready",
    detail: "Open Edit layout to Preview",
    owner: "Interaction",
  };
}

function restoreStep(input: ProjectOperatorWorkflowInput): OperatorWorkflowStep {
  if (input.inFlight === "restore") {
    return {
      id: "restore",
      label: "Restore",
      status: "active",
      detail: "Executing",
      owner: "Restore",
    };
  }
  if (input.hasLastRestoreResult) {
    return {
      id: "restore",
      label: "Restore",
      status: "complete",
      detail: "Complete (session result)",
      owner: "Restore",
    };
  }
  if (!input.arrangementSelected) {
    return {
      id: "restore",
      label: "Restore",
      status: "inactive",
      detail: "Needs Arrangement",
      owner: "Restore",
    };
  }
  if (input.restoreAvailable) {
    return {
      id: "restore",
      label: "Restore",
      status: "ready",
      detail: "Ready",
      owner: "Restore",
    };
  }
  return {
    id: "restore",
    label: "Restore",
    status: "blocked",
    detail: "Not ready",
    owner: "Restore",
  };
}

function pickCurrentStep(
  steps: OperatorWorkflowStep[],
  input: ProjectOperatorWorkflowInput,
): OperatorWorkflowStepId {
  if (input.inFlight === "restore") {
    return "restore";
  }
  if (input.hasLastRestoreResult && !input.previewActive) {
    return "restore";
  }
  if (input.previewActive) {
    return "preview";
  }
  if (input.arrangementSelected) {
    if (input.restoreAvailable) {
      return "restore";
    }
    return "arrangement";
  }
  if (input.loadState !== "ready") {
    return "desktop";
  }
  if (input.arrangementCount > 0) {
    return "arrangement";
  }
  // Prefer the first non-complete / non-inactive needing attention
  for (const step of steps) {
    if (step.status === "active" || step.status === "ready") {
      return step.id;
    }
  }
  return "desktop";
}

function nextActionFor(
  input: ProjectOperatorWorkflowInput,
  current: OperatorWorkflowStepId,
): string {
  if (input.inFlight === "restore") {
    return "Wait for Restore to finish — Restore owns OS positioning";
  }
  if (input.loadState === "runtime_unavailable") {
    return "Open the desktop app runtime to observe the Desktop";
  }
  if (input.loadState === "error") {
    return "Refresh Desktop observation";
  }
  if (!input.hasProfile) {
    return "Choose a Profile so Arrangements can be saved";
  }
  if (input.arrangementCount === 0) {
    return "Save an Arrangement from the Desktop selection or Arrangements panel";
  }
  if (!input.arrangementSelected) {
    return "Select an Arrangement to continue the workflow";
  }
  if (input.previewActive) {
    return input.restoreAvailable
      ? "Restore when ready — or close Preview; the workflow will re-project"
      : "Close Preview or Edit layout; workflow re-projects from current state";
  }
  if (input.layoutEditing && !input.previewActive) {
    return input.restoreAvailable
      ? "Turn Preview on, or Restore using existing Restore authority"
      : "Update Arrangement if needed, or leave Edit layout; workflow re-projects";
  }
  if (input.hasLastRestoreResult) {
    return "Select another Arrangement, Preview again, or leave Desktop — no resume required";
  }
  if (input.restoreAvailable) {
    return "Restore this Arrangement — or Edit layout to Preview saved bounds first";
  }
  if (current === "arrangement") {
    return "Edit layout to Preview, or wait until Restore becomes ready";
  }
  return "Continue when Desktop or Arrangement state changes — workflow re-projects";
}

/**
 * Project the operator workflow from existing Product + Interaction State.
 * Pure: same inputs ⇒ same projection. No stored step index.
 */
export function projectOperatorWorkflow(
  input: ProjectOperatorWorkflowInput,
): OperatorWorkflowProjection {
  const steps = [
    desktopStep(input),
    arrangementStep(input),
    previewStep(input),
    restoreStep(input),
  ];
  const currentStepId = pickCurrentStep(steps, input);
  const current = steps.find((step) => step.id === currentStepId) ?? steps[0];
  const nextAction = nextActionFor(input, currentStepId);

  const why = input.hasLastRestoreResult
    ? "Restore finished in this session and each step still reflects live Desktop / Arrangement / Interaction facts"
    : input.previewActive
      ? "Preview is showing saved Arrangement bounds without moving windows"
      : input.arrangementSelected
        ? "an Arrangement is selected and Desktop / Restore facts determine the next valid action"
        : "Desktop observation and Arrangement availability determine where the operator is";

  return {
    steps,
    currentStepId,
    nextAction,
    pathLabel: "Desktop · Arrangement · Preview · Restore",
    line: `Operator workflow · ${current.label} because ${why} · ${current.owner}`,
  };
}

/** Compact progress tokens for chrome (e.g. Desktop ✓ · Arrangement ● · …). */
export function operatorWorkflowProgressLine(
  projection: OperatorWorkflowProjection,
): string {
  return projection.steps
    .map((step) => {
      const mark =
        step.status === "complete"
          ? "✓"
          : step.status === "active"
            ? "●"
            : step.status === "ready"
              ? "○"
              : step.status === "blocked"
                ? "×"
                : "·";
      return `${step.label} ${mark}`;
    })
    .join(" → ");
}
