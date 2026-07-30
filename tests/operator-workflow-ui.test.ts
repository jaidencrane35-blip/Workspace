/**
 * Purpose: Programme V IC1 — operator workflow projection (composition only).
 */

import { describe, expect, it } from "vitest";
import {
  operatorWorkflowProgressLine,
  projectOperatorWorkflow,
} from "../app/src/lib/operatorWorkflowUi";

describe("operatorWorkflowUi", () => {
  it("projects Desktop → Arrangement → Preview → Restore from existing facts", () => {
    const projection = projectOperatorWorkflow({
      loadState: "ready",
      hasProfile: true,
      arrangementCount: 2,
      arrangementSelected: true,
      arrangementName: "Focus coding",
      previewActive: false,
      layoutEditing: false,
      restoreAvailable: true,
      inFlight: null,
      hasLastRestoreResult: false,
    });
    expect(projection.pathLabel).toBe(
      "Desktop · Arrangement · Preview · Restore",
    );
    expect(projection.steps.map((step) => step.id)).toEqual([
      "desktop",
      "arrangement",
      "preview",
      "restore",
    ]);
    expect(projection.steps[0]?.status).toBe("complete");
    expect(projection.steps[1]?.detail).toContain("Focus coding");
    expect(projection.steps[3]?.status).toBe("ready");
    expect(projection.line).toContain("because");
    expect(projection.nextAction).toMatch(/Restore|Preview|Edit layout/i);
  });

  it("re-projects gracefully when Preview closes or Arrangement is deselected", () => {
    const withPreview = projectOperatorWorkflow({
      loadState: "ready",
      hasProfile: true,
      arrangementCount: 1,
      arrangementSelected: true,
      arrangementName: "Focus",
      previewActive: true,
      layoutEditing: true,
      restoreAvailable: true,
      inFlight: null,
      hasLastRestoreResult: false,
    });
    expect(withPreview.currentStepId).toBe("preview");
    expect(withPreview.steps.find((s) => s.id === "preview")?.status).toBe(
      "active",
    );

    const previewClosed = projectOperatorWorkflow({
      loadState: "ready",
      hasProfile: true,
      arrangementCount: 1,
      arrangementSelected: true,
      arrangementName: "Focus",
      previewActive: false,
      layoutEditing: false,
      restoreAvailable: true,
      inFlight: null,
      hasLastRestoreResult: false,
    });
    expect(previewClosed.currentStepId).not.toBe("preview");
    expect(previewClosed.steps.find((s) => s.id === "preview")?.status).toBe(
      "ready",
    );

    const deselected = projectOperatorWorkflow({
      loadState: "ready",
      hasProfile: true,
      arrangementCount: 1,
      arrangementSelected: false,
      previewActive: false,
      layoutEditing: false,
      restoreAvailable: false,
      inFlight: null,
      hasLastRestoreResult: false,
    });
    expect(deselected.currentStepId).toBe("arrangement");
    expect(deselected.nextAction).toMatch(/Select an Arrangement/i);
  });

  it("does not invent resume state after Restore complete", () => {
    const afterRestore = projectOperatorWorkflow({
      loadState: "ready",
      hasProfile: true,
      arrangementCount: 1,
      arrangementSelected: true,
      arrangementName: "Focus",
      previewActive: false,
      layoutEditing: false,
      restoreAvailable: true,
      inFlight: null,
      hasLastRestoreResult: true,
    });
    expect(afterRestore.steps.find((s) => s.id === "restore")?.status).toBe(
      "complete",
    );
    expect(afterRestore.nextAction).toMatch(/no resume/i);
  });

  it("formats a compact progress line without storing steps", () => {
    const projection = projectOperatorWorkflow({
      loadState: "ready",
      hasProfile: true,
      arrangementCount: 0,
      arrangementSelected: false,
      previewActive: false,
      layoutEditing: false,
      restoreAvailable: false,
      inFlight: null,
      hasLastRestoreResult: false,
    });
    const line = operatorWorkflowProgressLine(projection);
    expect(line).toContain("Desktop");
    expect(line).toContain("Arrangement");
    expect(line).toContain("→");
  });
});
