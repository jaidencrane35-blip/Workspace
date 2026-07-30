/**
 * Purpose: Programme V IC2 — workflow recommendation projections (stable, declarative).
 */

import { describe, expect, it } from "vitest";
import {
  projectWorkflowRecommendations,
  workflowRecommendationIds,
  type WorkflowRecommendationFacts,
} from "../app/src/lib/workflowDecisionSupportUi";

function facts(
  overrides: Partial<WorkflowRecommendationFacts> = {},
): WorkflowRecommendationFacts {
  return {
    hasProfile: true,
    desktopReady: true,
    observedWindowCount: 3,
    arrangementCount: 1,
    arrangementSelected: true,
    desktopDiffersFromArrangement: false,
    newWindowsDetected: false,
    previewActive: false,
    layoutEditing: false,
    restoreAvailable: false,
    operationInFlight: false,
    ...overrides,
  };
}

describe("workflowDecisionSupportUi", () => {
  it("projects Preview unavailable when no Arrangement is selected", () => {
    const result = projectWorkflowRecommendations(
      facts({
        arrangementSelected: false,
        arrangementCount: 2,
        restoreAvailable: false,
      }),
    );
    expect(workflowRecommendationIds(result)).toContain("preview_unavailable");
    const preview = result.find((item) => item.id === "preview_unavailable");
    expect(preview?.action).toBe("Preview unavailable");
    expect(preview?.because).toBe("no Arrangement is selected");
    expect(preview?.line).toContain("Owner: Interaction");
  });

  it("projects Capture / Update / Restore / Preview from existing facts", () => {
    const noneSaved = projectWorkflowRecommendations(
      facts({
        arrangementCount: 0,
        arrangementSelected: false,
      }),
    );
    expect(workflowRecommendationIds(noneSaved)).toContain(
      "capture_recommended",
    );

    const differs = projectWorkflowRecommendations(
      facts({
        desktopDiffersFromArrangement: true,
        newWindowsDetected: true,
        restoreAvailable: true,
      }),
    );
    const ids = workflowRecommendationIds(differs);
    expect(ids).toContain("update_recommended");
    expect(ids).toContain("capture_recommended");
    expect(ids).toContain("preview_recommended");
    expect(ids).toContain("restore_available");
    expect(
      differs.find((item) => item.id === "update_recommended")?.owner,
    ).toBe("Arrangement Comparison");
    expect(
      differs.find((item) => item.id === "restore_available")?.owner,
    ).toBe("Restore Projection");
  });

  it("keeps recommendation wording stable for identical facts", () => {
    const input = facts({
      desktopDiffersFromArrangement: true,
      restoreAvailable: true,
    });
    const a = projectWorkflowRecommendations(input);
    const b = projectWorkflowRecommendations(input);
    expect(a).toEqual(b);
    expect(a.map((item) => item.line)).toEqual(b.map((item) => item.line));
  });

  it("uses fixed declaration order, not scored ranking", () => {
    const ids = workflowRecommendationIds(
      projectWorkflowRecommendations(
        facts({
          arrangementSelected: false,
          arrangementCount: 0,
          observedWindowCount: 2,
        }),
      ),
    );
    expect(ids.indexOf("preview_unavailable")).toBeLessThan(
      ids.indexOf("capture_recommended"),
    );
  });

  it("suppresses recommendations while an operation is in flight", () => {
    expect(
      projectWorkflowRecommendations(
        facts({
          operationInFlight: true,
          restoreAvailable: true,
          desktopDiffersFromArrangement: true,
        }),
      ),
    ).toEqual([]);
  });
});
