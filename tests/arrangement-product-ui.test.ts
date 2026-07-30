/**
 * Purpose: Programme I IC5 — derived Arrangement metadata and product feedback.
 */

import { describe, expect, it } from "vitest";
import {
  arrangementRestoredFeedback,
  arrangementSavedFeedback,
  arrangementUpdatedFeedback,
  deriveArrangementProductMeta,
  desktopFirstUseGuidance,
} from "../app/src/lib/arrangementProductUi";
import type {
  DesktopArrangement,
  DesktopArrangementEntry,
  DesktopArrangementRestoreResult,
} from "../app/src/types/desktopArrangement";
import type { WorkspaceStateWindow } from "../app/src/types/domain";

function sampleEntry(
  overrides: Partial<DesktopArrangementEntry> = {},
): DesktopArrangementEntry {
  return {
    id: "e1",
    arrangement_id: "arr-1",
    stable_window_id: "a",
    hwnd: "0x1",
    process_id: 10,
    process_name: "code.exe",
    title_fingerprint: null,
    label: "Editor",
    sort_order: 0,
    x: 0,
    y: 0,
    width: 800,
    height: 600,
    authority_effect: "none",
    ...overrides,
  };
}

function sampleArrangement(
  entries: DesktopArrangementEntry[],
): DesktopArrangement {
  return {
    id: "arr-1",
    workspace_id: "ws-1",
    name: "Focus coding",
    description: "",
    status: "active",
    entries,
    created_at: "2026-07-30T10:00:00.000Z",
    updated_at: "2026-07-30T12:00:00.000Z",
    authority_effect: "none",
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
    width: 800,
    height: 600,
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

describe("arrangementProductUi", () => {
  it("derives window count, overlap, completeness, and restore readiness", () => {
    const arrangement = sampleArrangement([
      sampleEntry(),
      sampleEntry({
        id: "e2",
        stable_window_id: "b",
        hwnd: "0x2",
        label: "Browser",
      }),
    ]);
    const meta = deriveArrangementProductMeta(arrangement, [
      sampleWindow(),
    ]);
    expect(meta.windowCount).toBe(2);
    expect(meta.presentCount).toBe(1);
    expect(meta.missingCount).toBe(1);
    expect(meta.completeness).toBe("complete");
    expect(meta.restoreReadiness).toBe("partial");
    expect(meta.summaryLine).toContain("2 windows");
    expect(meta.summaryLine).toContain("1/2 open");
    expect(meta.readinessLine).toContain("Restore partial");
  });

  it("marks restore ready when all members are open with bounds", () => {
    const arrangement = sampleArrangement([sampleEntry()]);
    const meta = deriveArrangementProductMeta(arrangement, [sampleWindow()]);
    expect(meta.restoreReadiness).toBe("ready");
    expect(meta.readinessLine).toContain("Restore ready");
  });

  it("builds Save / Update / Restore feedback from existing DTOs", () => {
    const arrangement = sampleArrangement([sampleEntry()]);
    expect(arrangementSavedFeedback(arrangement)).toContain(
      "Arrangement saved",
    );
    expect(arrangementSavedFeedback(arrangement)).toContain(
      "completed successfully",
    );
    expect(
      arrangementUpdatedFeedback(arrangement, "2 windows · 1 affected · 1 unchanged"),
    ).toContain("Arrangement updated");

    const result: DesktopArrangementRestoreResult = {
      arrangement_id: "arr-1",
      diagnostics: [],
      outcomes: [
        {
          entry_id: "e1",
          label: "Editor",
          hwnd: "0x1",
          status: "applied",
          detail: "ok",
          simulated: false,
        },
      ],
      applied_count: 1,
      gap_count: 0,
      failed_count: 0,
    };
    expect(arrangementRestoredFeedback("Focus coding", result)).toBe(
      "Arrangement restored · “Focus coding” · 1 applied · 0 gaps · completed successfully",
    );
  });

  it("shows first-use guidance only when no Arrangements exist", () => {
    expect(
      desktopFirstUseGuidance({
        hasProfile: true,
        arrangementCount: 0,
        desktopReady: true,
      })?.title,
    ).toMatch(/first Arrangement/i);
    expect(
      desktopFirstUseGuidance({
        hasProfile: true,
        arrangementCount: 2,
        desktopReady: true,
      }),
    ).toBeNull();
    expect(
      desktopFirstUseGuidance({
        hasProfile: false,
        arrangementCount: 0,
        desktopReady: false,
      })?.body,
    ).toMatch(/Profile/i);
  });
});
