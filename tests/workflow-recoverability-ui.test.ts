/**
 * Purpose: Programme V IC3 — workflow recoverability projections (stable, declarative).
 */

import { describe, expect, it } from "vitest";
import {
  projectWorkflowRecoverability,
  recoverabilityClassificationLabel,
  workflowRecoverabilityIds,
  type WorkflowRecoverabilityFacts,
} from "../app/src/lib/workflowRecoverabilityUi";

function facts(
  overrides: Partial<WorkflowRecoverabilityFacts> = {},
): WorkflowRecoverabilityFacts {
  return {
    loadState: "ready",
    hasProfile: true,
    observedWindowCount: 3,
    arrangementCount: 1,
    arrangementSelected: true,
    staleArrangementSelection: false,
    restoreAvailable: true,
    desktopDiffersFromArrangement: false,
    hasRestoreGaps: false,
    operationInFlight: false,
    ...overrides,
  };
}

describe("workflowRecoverabilityUi", () => {
  it("projects Restore unavailable when no Arrangement is selected", () => {
    const result = projectWorkflowRecoverability(
      facts({
        arrangementSelected: false,
        arrangementCount: 2,
        restoreAvailable: false,
      }),
    );
    expect(workflowRecoverabilityIds(result)).toContain(
      "restore_unavailable_no_arrangement",
    );
    const blocked = result.find(
      (item) => item.id === "restore_unavailable_no_arrangement",
    );
    expect(blocked?.classification).toBe("recoverable");
    expect(blocked?.what).toBe("Restore unavailable");
    expect(blocked?.because).toBe("no Arrangement is selected");
    expect(blocked?.owner).toBe("Arrangement Selection");
    expect(blocked?.missingPrerequisite).toBe("Arrangement selection");
    expect(blocked?.nextStep).toEqual({
      kind: "action",
      verb: "select_arrangement",
      label: "Select an Arrangement",
    });
    expect(blocked?.line).toContain("Owner: Arrangement Selection");
  });

  it("projects Update unavailable when the desktop matches the Arrangement", () => {
    const result = projectWorkflowRecoverability(
      facts({
        desktopDiffersFromArrangement: false,
        restoreAvailable: true,
      }),
    );
    const update = result.find((item) => item.id === "update_unavailable_matches");
    expect(update?.classification).toBe("recoverable");
    expect(update?.what).toBe("Update unavailable");
    expect(update?.because).toBe(
      "the current desktop matches the saved Arrangement",
    );
    expect(update?.owner).toBe("Arrangement Comparison");
    expect(update?.nextStep).toEqual({
      kind: "none",
      label: "No action required",
    });
  });

  it("classifies missing desktop / Arrangement / restore data as non-recoverable", () => {
    const noWindows = projectWorkflowRecoverability(
      facts({
        observedWindowCount: 0,
        arrangementSelected: false,
        restoreAvailable: false,
      }),
    );
    expect(workflowRecoverabilityIds(noWindows)).toContain("no_desktop_windows");
    expect(
      noWindows.find((item) => item.id === "no_desktop_windows")?.classification,
    ).toBe("non_recoverable");

    const deleted = projectWorkflowRecoverability(
      facts({
        arrangementSelected: false,
        staleArrangementSelection: true,
        restoreAvailable: false,
      }),
    );
    expect(workflowRecoverabilityIds(deleted)).toContain("arrangement_deleted");
    expect(
      deleted.find((item) => item.id === "arrangement_deleted")?.nextStep.kind,
    ).toBe("none");

    const noData = projectWorkflowRecoverability(
      facts({
        restoreAvailable: false,
        desktopDiffersFromArrangement: true,
      }),
    );
    expect(workflowRecoverabilityIds(noData)).toContain(
      "restore_data_unavailable",
    );
    expect(
      noData.find((item) => item.id === "restore_data_unavailable")
        ?.classification,
    ).toBe("non_recoverable");
  });

  it("projects Capture when no Arrangement is saved", () => {
    const result = projectWorkflowRecoverability(
      facts({
        arrangementCount: 0,
        arrangementSelected: false,
        restoreAvailable: false,
      }),
    );
    expect(workflowRecoverabilityIds(result)).toContain("no_arrangement_capture");
    const capture = result.find((item) => item.id === "no_arrangement_capture");
    expect(capture?.classification).toBe("recoverable");
    expect(capture?.nextStep).toEqual({
      kind: "action",
      verb: "capture",
      label: "Capture Desktop",
    });
  });

  it("projects Restore partial gaps as recoverable with Restore next step", () => {
    const result = projectWorkflowRecoverability(
      facts({
        restoreAvailable: true,
        hasRestoreGaps: true,
        desktopDiffersFromArrangement: true,
      }),
    );
    const partial = result.find((item) => item.id === "restore_partial_gaps");
    expect(partial?.classification).toBe("recoverable");
    expect(partial?.nextStep).toEqual({
      kind: "action",
      verb: "restore",
      label: "Restore",
    });
    expect(recoverabilityClassificationLabel("recoverable")).toBe("Recoverable");
    expect(recoverabilityClassificationLabel("non_recoverable")).toBe(
      "Non-recoverable",
    );
  });

  it("keeps recoverability wording stable for identical facts", () => {
    const input = facts({
      arrangementSelected: false,
      arrangementCount: 2,
      restoreAvailable: false,
    });
    const a = projectWorkflowRecoverability(input);
    const b = projectWorkflowRecoverability(input);
    expect(a).toEqual(b);
    expect(a.map((item) => item.line)).toEqual(b.map((item) => item.line));
    expect(
      a.map((item) => [item.classification, item.nextStep, item.owner]),
    ).toEqual(
      b.map((item) => [item.classification, item.nextStep, item.owner]),
    );
  });

  it("uses fixed declaration order, not scored ranking", () => {
    const ids = workflowRecoverabilityIds(
      projectWorkflowRecoverability(
        facts({
          arrangementSelected: false,
          arrangementCount: 0,
          observedWindowCount: 2,
          restoreAvailable: false,
        }),
      ),
    );
    expect(ids.indexOf("no_arrangement_capture")).toBeLessThan(
      ids.indexOf("preview_unavailable_no_arrangement"),
    );
  });

  it("suppresses projections while an operation is in flight", () => {
    expect(
      projectWorkflowRecoverability(facts({ operationInFlight: true })),
    ).toEqual([]);
  });

  it("returns only runtime / observation terminal conditions when blocked", () => {
    expect(
      workflowRecoverabilityIds(
        projectWorkflowRecoverability(
          facts({ loadState: "runtime_unavailable" }),
        ),
      ),
    ).toEqual(["runtime_unavailable"]);
    expect(
      workflowRecoverabilityIds(
        projectWorkflowRecoverability(facts({ loadState: "error" })),
      ),
    ).toEqual(["observation_failed"]);
  });
});
