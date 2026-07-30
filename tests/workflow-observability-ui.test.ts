/**
 * Purpose: Programme V IC6 — workflow observability projections (semantic diffs only).
 */

import { describe, expect, it } from "vitest";
import { projectOperatorWorkflow } from "../app/src/lib/operatorWorkflowUi";
import { projectWorkflowRecommendations } from "../app/src/lib/workflowDecisionSupportUi";
import { projectWorkflowRecoverability } from "../app/src/lib/workflowRecoverabilityUi";
import { projectWorkflowPredictability } from "../app/src/lib/workflowPredictabilityUi";
import { composeWorkflowExplainability } from "../app/src/lib/workflowExplainabilityUi";
import {
  captureWorkflowObservabilitySnapshot,
  observabilityStabilityLabel,
  projectWorkflowObservability,
  workflowObservabilityFingerprint,
  workflowObservabilityIds,
  type WorkflowObservabilitySnapshot,
} from "../app/src/lib/workflowObservabilityUi";

function buildSnapshot(overrides: {
  arrangementSelected?: boolean;
  restoreAvailable?: boolean;
  desktopDiffers?: boolean;
  inFlight?: "observe" | "save" | "update" | "restore" | null;
  hasRestoreGaps?: boolean;
}): WorkflowObservabilitySnapshot {
  const arrangementSelected = overrides.arrangementSelected ?? true;
  const restoreAvailable = overrides.restoreAvailable ?? true;
  const desktopDiffers = overrides.desktopDiffers ?? true;
  const inFlight = overrides.inFlight ?? null;
  const hasRestoreGaps = overrides.hasRestoreGaps ?? false;

  const workflow = projectOperatorWorkflow({
    loadState: "ready",
    hasProfile: true,
    arrangementCount: 2,
    arrangementSelected,
    arrangementName: "Dev",
    previewActive: false,
    layoutEditing: false,
    restoreAvailable,
    inFlight,
    hasLastRestoreResult: false,
  });
  const recommendations = projectWorkflowRecommendations({
    hasProfile: true,
    desktopReady: true,
    observedWindowCount: 4,
    arrangementCount: 2,
    arrangementSelected,
    desktopDiffersFromArrangement: desktopDiffers,
    newWindowsDetected: false,
    previewActive: false,
    layoutEditing: false,
    restoreAvailable,
    operationInFlight: Boolean(inFlight),
  });
  const recoverability = projectWorkflowRecoverability({
    loadState: "ready",
    hasProfile: true,
    observedWindowCount: 4,
    arrangementCount: 2,
    arrangementSelected,
    staleArrangementSelection: false,
    restoreAvailable,
    desktopDiffersFromArrangement: desktopDiffers,
    hasRestoreGaps,
    operationInFlight: Boolean(inFlight),
  });
  const predictability = projectWorkflowPredictability({
    loadState: "ready",
    hasProfile: true,
    observedWindowCount: 4,
    selectedWindowCount: 0,
    arrangementSelected,
    restoreAvailable,
    restoreMoveCount: desktopDiffers ? 12 : 0,
    restoreUnchangedCount: desktopDiffers ? 3 : 5,
    restoreUnavailableCount: hasRestoreGaps ? 1 : 0,
    restoreWithoutBoundsCount: 0,
    updateAddedCount: 0,
    updateRemovedCount: 0,
    updateBoundsChangedCount: desktopDiffers ? 4 : 0,
    updateZOrderChangedCount: 0,
    updateUnchangedCount: desktopDiffers ? 3 : 5,
    desktopDiffersFromArrangement: desktopDiffers,
    previewBoundsEntryCount: 15,
    previewActive: false,
    operationInFlight: Boolean(inFlight),
  });
  const explainability = composeWorkflowExplainability({
    workflow,
    recommendations,
    recoverability,
    predictability,
  });
  return captureWorkflowObservabilitySnapshot({
    workflow,
    recommendations,
    recoverability,
    predictability,
    explainability,
    inFlight,
  });
}

describe("workflowObservabilityUi", () => {
  it("returns no transitions without an immediately preceding snapshot", () => {
    const current = buildSnapshot({});
    expect(
      projectWorkflowObservability({ previous: null, current }),
    ).toEqual([]);
  });

  it("returns no transitions for identical recomputation (observation minimality)", () => {
    const snapshot = buildSnapshot({});
    expect(
      projectWorkflowObservability({ previous: snapshot, current: snapshot }),
    ).toEqual([]);
    expect(workflowObservabilityFingerprint(snapshot)).toBe(
      workflowObservabilityFingerprint({ ...snapshot }),
    );
  });

  it("surfaces recommendation change with What → Why → Owner", () => {
    const previous = buildSnapshot({ desktopDiffers: false });
    const current = buildSnapshot({ desktopDiffers: true });
    const transitions = projectWorkflowObservability({ previous, current });
    expect(workflowObservabilityIds(transitions)).toContain(
      "recommendation_changed",
    );
    const changed = transitions.find((item) => item.id === "recommendation_changed");
    expect(changed?.what).toBe("Recommendation changed");
    expect(changed?.because).toBe(
      "the current desktop differs from the saved Arrangement",
    );
    expect(changed?.owner).toBe("Arrangement Comparison");
    expect(changed?.stability).toBe("stable");
    expect(changed?.sourceProjection).toBe("ic2_recommendation");
    expect(changed?.line).toContain("Owner: Arrangement Comparison");
  });

  it("surfaces recoverability when an Arrangement becomes selected", () => {
    const previous = buildSnapshot({
      arrangementSelected: false,
      restoreAvailable: false,
      desktopDiffers: false,
    });
    const current = buildSnapshot({
      arrangementSelected: true,
      restoreAvailable: true,
      desktopDiffers: false,
    });
    const transitions = projectWorkflowObservability({ previous, current });
    expect(workflowObservabilityIds(transitions)).toContain(
      "recoverability_changed",
    );
    expect(workflowObservabilityIds(transitions)).toContain("phase_changed");
    const recoverability = transitions.find(
      (item) => item.id === "recoverability_changed",
    );
    expect(recoverability?.sourceProjection).toBe("ic3_recoverability");
    expect(recoverability?.because.length).toBeGreaterThan(0);
  });

  it("distinguishes transient execution from stable projection", () => {
    const previous = buildSnapshot({ inFlight: null });
    const current = buildSnapshot({ inFlight: "restore" });
    const transitions = projectWorkflowObservability({ previous, current });
    expect(workflowObservabilityIds(transitions)).toContain(
      "execution_transient",
    );
    const transient = transitions.find((item) => item.id === "execution_transient");
    expect(transient?.what).toBe("Workflow became transient");
    expect(transient?.because).toBe("Restore executing");
    expect(transient?.stability).toBe("transient");
    expect(transient?.owner).toBe("Interaction");
    expect(observabilityStabilityLabel("transient")).toBe("Transient");

    const after = projectWorkflowObservability({
      previous: current,
      current: buildSnapshot({ inFlight: null }),
    });
    expect(workflowObservabilityIds(after)).toContain("execution_stable");
    expect(after.find((item) => item.id === "execution_stable")?.stability).toBe(
      "stable",
    );
  });

  it("surfaces predictability changes from comparison semantics", () => {
    const previous = buildSnapshot({ desktopDiffers: false, hasRestoreGaps: false });
    const current = buildSnapshot({ desktopDiffers: true, hasRestoreGaps: true });
    const transitions = projectWorkflowObservability({ previous, current });
    expect(workflowObservabilityIds(transitions)).toContain(
      "predictability_changed",
    );
    expect(
      transitions.find((item) => item.id === "predictability_changed")
        ?.sourceProjection,
    ).toBe("ic4_predictability");
  });

  it("keeps transition wording stable for identical previous/current pairs", () => {
    const previous = buildSnapshot({ arrangementSelected: false, restoreAvailable: false });
    const current = buildSnapshot({ arrangementSelected: true, restoreAvailable: true });
    const a = projectWorkflowObservability({ previous, current });
    const b = projectWorkflowObservability({ previous, current });
    expect(a).toEqual(b);
  });

  it("uses fixed declaration order for meaningful transitions", () => {
    const previous = buildSnapshot({
      arrangementSelected: true,
      restoreAvailable: true,
      desktopDiffers: false,
      hasRestoreGaps: false,
      inFlight: null,
    });
    const current = buildSnapshot({
      arrangementSelected: true,
      restoreAvailable: true,
      desktopDiffers: true,
      hasRestoreGaps: true,
      inFlight: "restore",
    });
    const ids = workflowObservabilityIds(
      projectWorkflowObservability({ previous, current }),
    );
    const recommendation = ids.indexOf("recommendation_changed");
    const recoverability = ids.indexOf("recoverability_changed");
    const predictability = ids.indexOf("predictability_changed");
    const transient = ids.indexOf("execution_transient");
    expect(recommendation).toBeGreaterThanOrEqual(0);
    expect(recoverability).toBeGreaterThan(recommendation);
    expect(predictability).toBeGreaterThan(recoverability);
    expect(transient).toBeGreaterThan(predictability);
  });
});
