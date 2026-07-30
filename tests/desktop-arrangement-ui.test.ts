import { describe, expect, it } from "vitest";
import {
  applyStatusLabel,
  arrangementStatusLabel,
  arrangementSummaryLine,
  availabilityLabel,
  emptyArrangementsCopy,
  entryHasBounds,
  entryIdentitySummary,
  gapDiagnostics,
  permissionHintForError,
  restoreSummaryCopy,
} from "../app/src/lib/desktopArrangementUi";
import type {
  DesktopArrangement,
  DesktopArrangementEntry,
  DesktopArrangementRestoreResult,
} from "../app/src/types/desktopArrangement";

function sampleEntry(
  overrides: Partial<DesktopArrangementEntry> = {},
): DesktopArrangementEntry {
  return {
    id: "e1",
    arrangement_id: "arr-1",
    stable_window_id: "stable-1",
    hwnd: "0xAA",
    process_id: 10,
    process_name: "code.exe",
    title_fingerprint: "Code",
    label: "Editor",
    sort_order: 0,
    x: 10,
    y: 20,
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
    name: "Focus",
    description: "",
    status: "active",
    entries,
    created_at: "2026-07-29T12:00:00Z",
    updated_at: "2026-07-29T12:00:00Z",
    authority_effect: "none",
  };
}

describe("desktop arrangement UI helpers", () => {
  it("renders human-readable status and availability labels", () => {
    expect(arrangementStatusLabel("active")).toBe("Active");
    expect(availabilityLabel("identity_known_window_missing")).toBe(
      "Known, not open",
    );
    expect(applyStatusLabel("failed")).toBe("Failed");
  });

  it("summarizes arrangements and entries for list/details", () => {
    const arrangement = sampleArrangement([sampleEntry()]);
    expect(arrangementSummaryLine(arrangement)).toBe("Active · 1 window");
    expect(entryHasBounds(sampleEntry())).toBe(true);
    expect(entryHasBounds(sampleEntry({ width: null }))).toBe(false);
    expect(entryIdentitySummary(sampleEntry())).toContain("code.exe");
    expect(entryIdentitySummary(sampleEntry({ width: null }))).toContain(
      "no stored bounds",
    );
  });

  it("provides empty states for missing workspace and empty lists", () => {
    expect(emptyArrangementsCopy(false).title).toMatch(/profile/i);
    expect(emptyArrangementsCopy(false).body).toMatch(/Profiles/i);
    expect(emptyArrangementsCopy(true).body).toMatch(/Save open windows/i);
  });

  it("surfaces unavailable diagnostics without fabricating success", () => {
    const result: DesktopArrangementRestoreResult = {
      arrangement_id: "arr-1",
      diagnostics: [
        {
          entry_id: "e1",
          label: "Editor",
          stable_window_id: "stable-1",
          hwnd: "0xAA",
          availability: "available",
          detail: "ok",
          restore_gap: false,
          gap_reason: null,
        },
        {
          entry_id: "e2",
          label: "Chat",
          stable_window_id: "stable-gone",
          hwnd: "0xBB",
          availability: "unavailable",
          detail: "missing",
          restore_gap: true,
          gap_reason: "unavailable",
        },
      ],
      outcomes: [
        {
          entry_id: "e1",
          label: "Editor",
          hwnd: "0xAA",
          status: "applied",
          detail: "set_bounds applied",
          simulated: true,
        },
      ],
      applied_count: 1,
      gap_count: 1,
      failed_count: 0,
    };

    expect(restoreSummaryCopy(result)).toBe("1 applied · 1 gap · simulated");
    expect(gapDiagnostics(result.diagnostics)).toHaveLength(1);
    expect(gapDiagnostics(result.diagnostics)[0]?.label).toBe("Chat");
  });

  it("explains permission failures for restore interaction states", () => {
    expect(permissionHintForError("Permission denied")).toMatch(
      /desktop\.restore/i,
    );
    expect(permissionHintForError("Approval required")).toMatch(/approval/i);
    expect(permissionHintForError("not found")).toBeNull();
  });
});
