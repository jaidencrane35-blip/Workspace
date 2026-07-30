/**
 * Purpose: Programme I IC3/IC4 layout editing helpers — lifecycle, diff, preview.
 */

import { describe, expect, it } from "vitest";
import {
  diffLayoutEditingChanges,
  layoutArrangementPreviewGhosts,
  layoutEditingBanner,
  layoutEditingHint,
  layoutEditingPhase,
  layoutEditingPhaseLabel,
  layoutEditingProposedWindows,
  layoutEditingStatusMeta,
  layoutEditingUpdateConfirmation,
  layoutEditingUpdatedLabel,
  layoutEditingWorkflowLine,
  stagePlaneBoundsFromRects,
} from "../app/src/lib/desktopLayoutEditing";
import { stageDesktopWindowKey } from "../app/src/lib/stageDesktopUi";
import type { DesktopArrangementEntry } from "../app/src/types/desktopArrangement";
import type {
  WorkspaceStateMonitor,
  WorkspaceStateWindow,
} from "../app/src/types/domain";

function sampleEntry(
  overrides: Partial<DesktopArrangementEntry> = {},
): DesktopArrangementEntry {
  return {
    id: "e1",
    arrangement_id: "arr",
    stable_window_id: "a",
    hwnd: "0x1",
    process_id: 10,
    process_name: "code.exe",
    title_fingerprint: null,
    label: "Editor",
    sort_order: 0,
    x: 0,
    y: 0,
    width: 500,
    height: 400,
    authority_effect: "none",
    ...overrides,
  };
}

function sampleMonitor(
  overrides: Partial<WorkspaceStateMonitor> = {},
): WorkspaceStateMonitor {
  return {
    monitor_index: 0,
    name: "Main",
    x: 0,
    y: 0,
    width: 1920,
    height: 1080,
    work_x: 0,
    work_y: 0,
    work_w: 1920,
    work_h: 1040,
    is_primary: true,
    ...overrides,
  };
}

function sampleWindow(
  overrides: Partial<WorkspaceStateWindow> = {},
): WorkspaceStateWindow {
  return {
    hwnd: "0x1",
    stable_window_id: "a",
    title: "Editor",
    process_id: 10,
    process_name: "code.exe",
    x: 0,
    y: 0,
    width: 500,
    height: 400,
    visible: true,
    focused: false,
    minimized: false,
    z_order: 1,
    monitor_index: 0,
    monitor_name: "Main",
    first_seen_at: null,
    last_seen_at: null,
    identity_confidence: null,
    ...overrides,
  };
}

describe("desktopLayoutEditing", () => {
  it("builds editing banner and phase-aware workflow copy", () => {
    expect(layoutEditingBanner("Focus coding")).toBe("Editing · Focus coding");
    expect(layoutEditingBanner("  ")).toBe("Editing · Arrangement");
    expect(layoutEditingWorkflowLine("editing")).toContain("No changes");
    expect(layoutEditingWorkflowLine("changes_pending")).toContain(
      "Changes pending",
    );
    expect(layoutEditingHint("changes_pending")).toContain("Update");
  });

  it("derives explicit editing lifecycle phases", () => {
    expect(layoutEditingPhase(false, false)).toBe("idle");
    expect(layoutEditingPhase(false, true)).toBe("idle");
    expect(layoutEditingPhase(true, false)).toBe("editing");
    expect(layoutEditingPhase(true, true)).toBe("changes_pending");
    expect(layoutEditingPhaseLabel("editing")).toBe("No changes");
    expect(layoutEditingPhaseLabel("changes_pending")).toBe("Changes pending");
  });

  it("computes plane bounds from monitors", () => {
    const plane = stagePlaneBoundsFromRects([sampleMonitor()], []);
    expect(plane).toEqual({
      minX: 0,
      minY: 0,
      spanX: 1920,
      spanY: 1080,
    });
  });

  it("projects arrangement entry bounds as preview ghosts", () => {
    const ghosts = layoutArrangementPreviewGhosts(
      [
        sampleEntry({
          id: "e1",
          label: "Editor",
          x: 0,
          y: 0,
          width: 960,
          height: 540,
        }),
        sampleEntry({
          id: "e2",
          label: "Browser",
          hwnd: "0x2",
          x: 960,
          y: 0,
          width: 960,
          height: 540,
        }),
      ],
      [sampleMonitor()],
      [],
    );
    expect(ghosts).toHaveLength(2);
    expect(ghosts[0]?.label).toBe("Editor");
    expect(ghosts[0]?.leftPct).toBeCloseTo(0);
    expect(ghosts[0]?.widthPct).toBeCloseTo(50);
    expect(ghosts[1]?.leftPct).toBeCloseTo(50);
  });

  it("skips entries without stored bounds", () => {
    const ghosts = layoutArrangementPreviewGhosts(
      [
        sampleEntry({ x: null, y: null, width: null, height: null }),
        sampleEntry({
          id: "e2",
          label: "Ok",
          x: 10,
          y: 10,
          width: 100,
          height: 80,
        }),
      ],
      [],
      [sampleWindow()],
    );
    expect(ghosts).toHaveLength(1);
    expect(ghosts[0]?.label).toBe("Ok");
  });

  it("returns empty when no entries have geometry", () => {
    expect(
      layoutArrangementPreviewGhosts([
        sampleEntry({ x: null, y: null, width: null, height: null }),
      ]),
    ).toEqual([]);
  });

  it("proposes selected windows or all when selection is empty", () => {
    const windows = [
      sampleWindow({ stable_window_id: "a" }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 20,
      }),
    ];
    expect(
      layoutEditingProposedWindows(windows, [], stageDesktopWindowKey),
    ).toHaveLength(2);
    expect(
      layoutEditingProposedWindows(windows, ["a"], stageDesktopWindowKey),
    ).toHaveLength(1);
  });

  it("detects added, removed, moved, and no-change diffs", () => {
    const unchanged = diffLayoutEditingChanges(
      [sampleEntry()],
      [sampleWindow()],
    );
    expect(unchanged.hasChanges).toBe(false);
    expect(unchanged.unchanged).toBe(1);
    expect(layoutEditingUpdateConfirmation(unchanged)).toContain(
      "no effective changes",
    );

    const moved = diffLayoutEditingChanges(
      [sampleEntry()],
      [sampleWindow({ x: 40, y: 20 })],
    );
    expect(moved.hasChanges).toBe(true);
    expect(moved.boundsChanged).toBe(1);

    const membership = diffLayoutEditingChanges(
      [
        sampleEntry(),
        sampleEntry({
          id: "e2",
          stable_window_id: "b",
          hwnd: "0x2",
          label: "Browser",
        }),
      ],
      [
        sampleWindow(),
        sampleWindow({
          stable_window_id: "c",
          hwnd: "0x3",
          process_id: 30,
        }),
      ],
    );
    expect(membership.added).toBe(1);
    expect(membership.removed).toBe(1);
    expect(layoutEditingUpdateConfirmation(membership)).toContain("affected");
  });

  it("formats status meta from existing arrangement fields", () => {
    const diff = diffLayoutEditingChanges([sampleEntry()], [sampleWindow()]);
    const meta = layoutEditingStatusMeta({
      phase: "editing",
      previewEnabled: true,
      updatedAt: "2026-07-30T12:00:00.000Z",
      diff,
    });
    expect(meta).toContain("No changes");
    expect(meta).toContain("Preview on");
    expect(meta).toContain("Updated ·");
    expect(layoutEditingUpdatedLabel("")).toBe("Updated · unknown");
  });
});
