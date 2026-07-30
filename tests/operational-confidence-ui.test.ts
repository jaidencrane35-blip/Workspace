/**
 * Purpose: Programme I IC6 — operational confidence explanations (derived only).
 */

import { describe, expect, it } from "vitest";
import {
  arrangementRestoredExplanationMessage,
  explainArrangementCurrency,
  explainOperationalActivity,
  explainPostRestore,
  explainPreRestore,
  formatChangeSinceCapture,
} from "../app/src/lib/operationalConfidenceUi";
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

describe("operationalConfidenceUi", () => {
  it("explains current activity with ownership why", () => {
    const observing = explainOperationalActivity({
      loadState: "loading",
      hasProfile: true,
      arrangementSelected: false,
      layoutEditing: false,
      layoutPreview: false,
      inFlight: "observe",
      hasLastRestoreResult: false,
    });
    expect(observing.activity).toBe("Observing desktop");
    expect(observing.line).toContain("because");
    expect(observing.owner).toBe("Observation");

    const selected = explainOperationalActivity({
      loadState: "ready",
      hasProfile: true,
      arrangementSelected: true,
      arrangementName: "Focus coding",
      layoutEditing: false,
      layoutPreview: false,
      inFlight: null,
      hasLastRestoreResult: false,
    });
    expect(selected.activity).toBe("Arrangement selected");
    expect(selected.line).toContain("Focus coding");
  });

  it("derives Arrangement currency Current / Out of date / Partial / Unavailable", () => {
    const arrangement = sampleArrangement([
      sampleEntry(),
      sampleEntry({
        id: "e2",
        stable_window_id: "b",
        hwnd: "0x2",
      }),
    ]);

    expect(
      explainArrangementCurrency(arrangement, [
        sampleWindow(),
        sampleWindow({ stable_window_id: "b", hwnd: "0x2", process_id: 20 }),
      ]).currency,
    ).toBe("current");

    expect(
      explainArrangementCurrency(arrangement, [
        sampleWindow({ x: 40 }),
        sampleWindow({
          stable_window_id: "b",
          hwnd: "0x2",
          process_id: 20,
        }),
      ]).currency,
    ).toBe("out_of_date");

    expect(
      explainArrangementCurrency(arrangement, [sampleWindow()]).currency,
    ).toBe("partial");

    expect(
      explainArrangementCurrency(arrangement, []).currency,
    ).toBe("unavailable");
  });

  it("formats what changed since capture", () => {
    const currency = explainArrangementCurrency(
      sampleArrangement([
        sampleEntry(),
        sampleEntry({ id: "e2", stable_window_id: "b", hwnd: "0x2" }),
      ]),
      [
        sampleWindow({ x: 10 }),
        sampleWindow({
          stable_window_id: "c",
          hwnd: "0x3",
          process_id: 30,
        }),
      ],
    );
    expect(formatChangeSinceCapture(currency.changeDiff)).toMatch(/\+/);
    expect(currency.changeLine).toMatch(/moved|window/);
  });

  it("builds pre-Restore explanation without applying bounds", () => {
    const pre = explainPreRestore(sampleArrangement([sampleEntry()]), [
      sampleWindow({ x: 50 }),
    ]);
    expect(pre.available).toBe(true);
    expect(pre.line).toContain("because");
    expect(pre.bullets.some((item) => item.includes("move"))).toBe(true);
    expect(pre.summaryLine).toContain("Restore will");
  });

  it("interprets post-Restore DTOs with why ownership", () => {
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
        {
          entry_id: "e2",
          label: "Chat",
          hwnd: "0x2",
          status: "skipped",
          detail: "already",
          simulated: false,
        },
      ],
      applied_count: 12,
      gap_count: 1,
      failed_count: 0,
    };
    const explained = explainPostRestore("Focus coding", result);
    expect(explained.title).toContain("Restore completed");
    expect(explained.line).toContain("because");
    expect(arrangementRestoredExplanationMessage("Focus coding", result)).toContain(
      "12 windows restored",
    );
  });
});
