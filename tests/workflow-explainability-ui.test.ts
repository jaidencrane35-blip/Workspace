/**
 * Purpose: Programme V IC5 — unified workflow explainability (composition only).
 */

import { describe, expect, it } from "vitest";
import { projectOperatorWorkflow } from "../app/src/lib/operatorWorkflowUi";
import { projectWorkflowRecommendations } from "../app/src/lib/workflowDecisionSupportUi";
import { projectWorkflowRecoverability } from "../app/src/lib/workflowRecoverabilityUi";
import { projectWorkflowPredictability } from "../app/src/lib/workflowPredictabilityUi";
import {
  composeWorkflowExplainability,
  workflowExplainabilitySectionIds,
  workflowExplainabilitySources,
} from "../app/src/lib/workflowExplainabilityUi";

describe("workflowExplainabilityUi", () => {
  const workflow = projectOperatorWorkflow({
    loadState: "ready",
    hasProfile: true,
    arrangementCount: 2,
    arrangementSelected: true,
    arrangementName: "Dev",
    previewActive: false,
    layoutEditing: false,
    restoreAvailable: true,
    inFlight: null,
    hasLastRestoreResult: false,
  });

  const recommendations = projectWorkflowRecommendations({
    hasProfile: true,
    desktopReady: true,
    observedWindowCount: 4,
    arrangementCount: 2,
    arrangementSelected: true,
    desktopDiffersFromArrangement: true,
    newWindowsDetected: false,
    previewActive: false,
    layoutEditing: false,
    restoreAvailable: true,
    operationInFlight: false,
  });

  const recoverability = projectWorkflowRecoverability({
    loadState: "ready",
    hasProfile: true,
    observedWindowCount: 4,
    arrangementCount: 2,
    arrangementSelected: true,
    staleArrangementSelection: false,
    restoreAvailable: true,
    desktopDiffersFromArrangement: true,
    hasRestoreGaps: true,
    operationInFlight: false,
  });

  const predictability = projectWorkflowPredictability({
    loadState: "ready",
    hasProfile: true,
    observedWindowCount: 4,
    selectedWindowCount: 0,
    arrangementSelected: true,
    restoreAvailable: true,
    restoreMoveCount: 12,
    restoreUnchangedCount: 3,
    restoreUnavailableCount: 1,
    restoreWithoutBoundsCount: 0,
    updateAddedCount: 0,
    updateRemovedCount: 0,
    updateBoundsChangedCount: 4,
    updateZOrderChangedCount: 0,
    updateUnchangedCount: 3,
    desktopDiffersFromArrangement: true,
    previewBoundsEntryCount: 15,
    previewActive: false,
    operationInFlight: false,
  });

  it("composes What → Why → Owner → Next action → Recoverability → Expected outcome", () => {
    const summary = composeWorkflowExplainability({
      workflow,
      recommendations,
      recoverability,
      predictability,
    });
    expect(workflowExplainabilitySectionIds(summary)).toEqual([
      "what",
      "why",
      "owner",
      "next_action",
      "recoverability",
      "expected_outcome",
    ]);
    expect(summary.pathLabel).toBe(workflow.pathLabel);
  });

  it("traces every section to an originating IC1–IC4 projection", () => {
    const summary = composeWorkflowExplainability({
      workflow,
      recommendations,
      recoverability,
      predictability,
    });
    const byId = Object.fromEntries(
      summary.sections.map((section) => [section.id, section]),
    );
    expect(byId.what?.sourceProjection).toBe("ic1_workflow");
    expect(byId.why?.sourceProjection).toBe("ic1_workflow");
    expect(byId.owner?.sourceProjection).toBe("ic1_workflow");
    expect(byId.next_action?.sourceProjection).toBe("ic2_recommendation");
    expect(byId.recoverability?.sourceProjection).toBe("ic3_recoverability");
    expect(byId.expected_outcome?.sourceProjection).toBe("ic4_predictability");
    expect(workflowExplainabilitySources(summary)).toEqual([
      "ic1_workflow",
      "ic1_workflow",
      "ic1_workflow",
      "ic2_recommendation",
      "ic3_recoverability",
      "ic4_predictability",
    ]);
  });

  it("reuses source wording and preserves information gaps (narrative fidelity)", () => {
    const summary = composeWorkflowExplainability({
      workflow,
      recommendations,
      recoverability,
      predictability,
    });
    const current = workflow.steps.find(
      (step) => step.id === workflow.currentStepId,
    );
    expect(summary.sections.find((s) => s.id === "what")?.text).toBe(
      `${current?.label} · ${current?.detail}`,
    );
    expect(summary.sections.find((s) => s.id === "why")?.text).toBe(
      workflow.why,
    );
    expect(summary.sections.find((s) => s.id === "owner")?.text).toBe(
      current?.owner,
    );

    const firstRec = recommendations[0];
    expect(summary.sections.find((s) => s.id === "next_action")?.text).toBe(
      `${firstRec.action} because ${firstRec.because}`,
    );
    expect(summary.sections.find((s) => s.id === "next_action")?.owner).toBe(
      firstRec.owner,
    );

    const firstPred = predictability[0];
    const outcomeText = summary.sections.find(
      (s) => s.id === "expected_outcome",
    )?.text;
    expect(outcomeText).toContain(firstPred.what);
    expect(outcomeText).toContain("move 12 windows");
    expect(outcomeText).toContain("Because current comparison indicates these differences");
    expect(outcomeText).not.toMatch(/perfectly|recreate/i);

    const limited = projectWorkflowPredictability({
      loadState: "ready",
      hasProfile: true,
      observedWindowCount: 0,
      selectedWindowCount: 0,
      arrangementSelected: true,
      restoreAvailable: false,
      restoreMoveCount: 0,
      restoreUnchangedCount: 0,
      restoreUnavailableCount: 2,
      restoreWithoutBoundsCount: 1,
      updateAddedCount: 0,
      updateRemovedCount: 0,
      updateBoundsChangedCount: 0,
      updateZOrderChangedCount: 0,
      updateUnchangedCount: 0,
      desktopDiffersFromArrangement: false,
      previewBoundsEntryCount: 0,
      previewActive: false,
      operationInFlight: false,
    });
    const limitedSummary = composeWorkflowExplainability({
      workflow,
      recommendations: [],
      recoverability: [],
      predictability: limited,
    });
    expect(
      limitedSummary.sections.find((s) => s.id === "expected_outcome")?.text,
    ).toMatch(/Unavailable information/);
  });

  it("falls back to IC1 nextAction when no IC2 recommendations exist", () => {
    const summary = composeWorkflowExplainability({
      workflow,
      recommendations: [],
      recoverability: [],
      predictability: [],
    });
    expect(workflowExplainabilitySectionIds(summary)).toEqual([
      "what",
      "why",
      "owner",
      "next_action",
    ]);
    const next = summary.sections.find((s) => s.id === "next_action");
    expect(next?.sourceProjection).toBe("ic1_workflow");
    expect(next?.text).toBe(workflow.nextAction);
  });

  it("keeps composition stable for identical IC1–IC4 outputs", () => {
    const input = {
      workflow,
      recommendations,
      recoverability,
      predictability,
    };
    expect(composeWorkflowExplainability(input)).toEqual(
      composeWorkflowExplainability(input),
    );
  });

  it("selects primary recommendation / recoverability / predictability by declaration order", () => {
    const summary = composeWorkflowExplainability({
      workflow,
      recommendations,
      recoverability,
      predictability,
    });
    expect(summary.sections.find((s) => s.id === "next_action")?.sourceId).toBe(
      recommendations[0].id,
    );
    expect(
      summary.sections.find((s) => s.id === "recoverability")?.sourceId,
    ).toBe(recoverability[0].id);
    expect(
      summary.sections.find((s) => s.id === "expected_outcome")?.sourceId,
    ).toBe(predictability[0].id);
  });
});
