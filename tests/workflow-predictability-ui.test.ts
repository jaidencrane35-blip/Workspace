/**
 * Purpose: Programme V IC4 — workflow predictability projections (stable, fidelity-limited).
 */

import { describe, expect, it } from "vitest";
import {
  projectWorkflowPredictability,
  workflowPredictabilityIds,
  type WorkflowPredictabilityFacts,
} from "../app/src/lib/workflowPredictabilityUi";

function facts(
  overrides: Partial<WorkflowPredictabilityFacts> = {},
): WorkflowPredictabilityFacts {
  return {
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
    ...overrides,
  };
}

describe("workflowPredictabilityUi", () => {
  it("projects Restore will move / leave / skip from existing comparison counts", () => {
    const result = projectWorkflowPredictability(facts());
    expect(workflowPredictabilityIds(result)).toContain("restore_outcome");
    const restore = result.find((item) => item.id === "restore_outcome");
    expect(restore?.what).toBe("Restore will");
    expect(restore?.effects).toContain("move 12 windows");
    expect(restore?.unchanged).toContain("leave 3 windows unchanged");
    expect(restore?.unavailable).toContain("skip 1 unavailable window");
    expect(restore?.because).toBe(
      "current comparison indicates these differences",
    );
    expect(restore?.owner).toBe("Restore Projection");
    expect(restore?.line).toContain("Owner: Restore Projection");
    expect(restore?.effects.join(" ")).not.toMatch(/perfectly|recreate/i);
  });

  it("projects Update affected vs unchanged from change-diff counts", () => {
    const result = projectWorkflowPredictability(
      facts({
        updateAddedCount: 1,
        updateRemovedCount: 1,
        updateBoundsChangedCount: 2,
        updateUnchangedCount: 4,
      }),
    );
    const update = result.find((item) => item.id === "update_outcome");
    expect(update?.what).toBe("Update will");
    expect(update?.effects).toEqual(
      expect.arrayContaining([
        "add 1 window to the Arrangement",
        "remove 1 window from the Arrangement",
        "update bounds for 2 windows",
      ]),
    );
    expect(update?.unchanged).toContain(
      "leave 4 windows unchanged in the Arrangement",
    );
    expect(update?.owner).toBe("Arrangement Comparison");
    expect(update?.because).toBe(
      "the saved Arrangement differs from the current desktop",
    );
  });

  it("projects Update noop when the desktop matches the Arrangement", () => {
    const result = projectWorkflowPredictability(
      facts({
        desktopDiffersFromArrangement: false,
        restoreMoveCount: 0,
        restoreUnchangedCount: 5,
        restoreUnavailableCount: 0,
        updateUnchangedCount: 5,
      }),
    );
    expect(workflowPredictabilityIds(result)).toContain("update_noop");
    expect(workflowPredictabilityIds(result)).not.toContain("update_outcome");
    const noop = result.find((item) => item.id === "update_noop");
    expect(noop?.effects).toContain("make no effective Arrangement changes");
    expect(noop?.owner).toBe("Arrangement Comparison");
  });

  it("explains limited Restore outcomes when facts are insufficient", () => {
    const result = projectWorkflowPredictability(
      facts({
        restoreAvailable: false,
        restoreMoveCount: 0,
        restoreUnchangedCount: 0,
        restoreUnavailableCount: 2,
        restoreWithoutBoundsCount: 1,
        desktopDiffersFromArrangement: false,
      }),
    );
    const limited = result.find((item) => item.id === "restore_outcome_limited");
    expect(limited?.informationGaps.length).toBeGreaterThan(0);
    expect(limited?.informationGaps.join(" ")).toMatch(/cannot be projected/i);
    expect(limited?.owner).toBe("Restore Projection");
  });

  it("projects Capture and Preview without claiming desktop recreation", () => {
    const result = projectWorkflowPredictability(
      facts({
        selectedWindowCount: 2,
        previewBoundsEntryCount: 4,
      }),
    );
    const capture = result.find((item) => item.id === "capture_outcome");
    expect(capture?.effects).toContain(
      "save 2 selected windows as a new Arrangement",
    );
    expect(capture?.unchanged).toContain(
      "leave desktop window positions unchanged",
    );
    expect(capture?.owner).toBe("Arrangement");

    const preview = result.find((item) => item.id === "preview_outcome");
    expect(preview?.effects).toEqual(
      expect.arrayContaining([
        "show saved Arrangement bounds as overlays",
        "represent up to 4 saved bounds entries",
      ]),
    );
    expect(preview?.unchanged).toContain(
      "leave desktop window positions unchanged",
    );
    expect(preview?.owner).toBe("Interaction");
  });

  it("keeps predictability wording stable for identical facts", () => {
    const input = facts();
    const a = projectWorkflowPredictability(input);
    const b = projectWorkflowPredictability(input);
    expect(a).toEqual(b);
    expect(a.map((item) => item.line)).toEqual(b.map((item) => item.line));
  });

  it("uses fixed declaration order, not scored ranking", () => {
    const ids = workflowPredictabilityIds(projectWorkflowPredictability(facts()));
    expect(ids.indexOf("restore_outcome")).toBeLessThan(
      ids.indexOf("update_outcome"),
    );
    expect(ids.indexOf("update_outcome")).toBeLessThan(
      ids.indexOf("preview_outcome"),
    );
  });

  it("suppresses projections while an operation is in flight", () => {
    expect(
      projectWorkflowPredictability(facts({ operationInFlight: true })),
    ).toEqual([]);
  });

  it("projects outcome unavailable when observation is not ready", () => {
    expect(
      workflowPredictabilityIds(
        projectWorkflowPredictability(
          facts({ loadState: "runtime_unavailable" }),
        ),
      ),
    ).toEqual(["outcome_unavailable"]);
    expect(
      projectWorkflowPredictability(facts({ loadState: "loading" }))[0]
        ?.informationGaps[0],
    ).toMatch(/cannot be projected/i);
  });
});
