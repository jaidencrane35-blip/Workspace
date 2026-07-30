import { describe, expect, it } from "vitest";
import {
  layoutStageDesktopWindows,
  nextStageSelectionKey,
  relatedStageObjectKeys,
  stageArrangementMemberKeys,
  stageDesktopMetaLine,
  stageDesktopPlaneMessage,
  stageDesktopWindowKey,
  stageDesktopWindowTitle,
  toggleStageSelection,
} from "../app/src/lib/stageDesktopUi";
import type { WorkspaceStateWindow } from "../app/src/types/domain";

function sampleWindow(
  overrides: Partial<WorkspaceStateWindow> = {},
): WorkspaceStateWindow {
  return {
    stable_window_id: "stable-1",
    hwnd: "0x1",
    title: "Editor",
    process_id: 10,
    process_name: "code.exe",
    visible: true,
    focused: false,
    minimized: false,
    x: 0,
    y: 0,
    width: 800,
    height: 600,
    monitor_index: 0,
    monitor_name: "Display 1",
    ...overrides,
  };
}

describe("stage desktop UI helpers", () => {
  it("prefers stable window identity for keys", () => {
    expect(stageDesktopWindowKey(sampleWindow())).toBe("stable-1");
    expect(
      stageDesktopWindowKey(
        sampleWindow({ stable_window_id: null, hwnd: "0xABC" }),
      ),
    ).toBe("0xABC");
  });

  it("does not invent titles when metadata exists", () => {
    expect(stageDesktopWindowTitle(sampleWindow())).toBe("Editor");
    expect(
      stageDesktopWindowTitle(
        sampleWindow({ title: "  ", process_name: "chrome.exe" }),
      ),
    ).toBe("chrome.exe");
  });

  it("projects runtime object fields from observation", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({
        stable_window_id: "a",
        x: 0,
        y: 0,
        width: 500,
        height: 500,
        focused: true,
        process_id: 10,
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        title: "Browser",
        process_name: "chrome.exe",
        process_id: 20,
        x: 500,
        y: 0,
        width: 500,
        height: 500,
      }),
      sampleWindow({
        stable_window_id: "c",
        hwnd: "0x3",
        title: "Docs",
        process_name: "chrome.exe",
        process_id: 20,
        x: 0,
        y: 500,
        width: 500,
        height: 500,
      }),
    ]);
    expect(tiles).toHaveLength(3);
    expect(tiles[0]?.hwnd).toBe("0x1");
    expect(tiles[0]?.processId).toBe(10);
    expect(tiles[0]?.stableWindowId).toBe("a");
    expect(tiles[0]?.visible).toBe(true);
    expect(tiles[0]?.processKey).toBe("pid:10");
    expect(tiles[1]?.processKey).toBe(tiles[2]?.processKey);
    expect(tiles[1]?.relationIndex).toBe(tiles[2]?.relationIndex);
    expect(tiles[0]?.leftPct).toBe(0);
    expect(tiles[1]?.leftPct).toBe(50);
  });

  it("organises Focus mode as primary process + process dock", async () => {
    const { organiseStageForWorkMode } = await import(
      "../app/src/lib/stageDesktopUi"
    );
    const windows = [
      sampleWindow({
        stable_window_id: "a",
        hwnd: "0x1",
        focused: true,
        process_id: 10,
        process_name: "code.exe",
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        title: "Tab 1",
        process_id: 20,
        process_name: "chrome.exe",
      }),
      sampleWindow({
        stable_window_id: "c",
        hwnd: "0x3",
        title: "Tab 2",
        process_id: 20,
        process_name: "chrome.exe",
      }),
      sampleWindow({
        stable_window_id: "d",
        hwnd: "0x4",
        title: "Notes",
        process_id: 30,
        process_name: "notepad.exe",
      }),
    ];
    const focus = organiseStageForWorkMode(windows, "focus", null);
    expect(focus.mapWindows).toHaveLength(1);
    expect(focus.mapWindows[0]?.process_name).toBe("code.exe");
    expect(focus.dockEntries).toHaveLength(2);
    const chrome = focus.dockEntries.find((entry) => entry.processId === 20);
    expect(chrome?.windowCount).toBe(2);

    const flow = organiseStageForWorkMode(windows, "flow", null);
    expect(flow.mapWindows).toHaveLength(4);
    expect(flow.dockEntries).toHaveLength(0);
  });

  it("exposes Flow relationships across process and monitor", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({
        stable_window_id: "a",
        process_id: 10,
        monitor_index: 0,
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 10,
        title: "Other",
        monitor_index: 1,
        monitor_name: "Display 2",
      }),
      sampleWindow({
        stable_window_id: "c",
        hwnd: "0x3",
        process_id: 20,
        process_name: "chrome.exe",
        title: "Browser",
        monitor_index: 0,
      }),
    ]);
    const focusRelated = relatedStageObjectKeys(tiles, "a", "focus");
    expect(focusRelated.has("a")).toBe(true);
    expect(focusRelated.has("b")).toBe(true);
    expect(focusRelated.has("c")).toBe(false);

    const flowRelated = relatedStageObjectKeys(tiles, "a", "flow");
    expect(flowRelated.has("c")).toBe(true);
  });

  it("matches arrangement working-set membership by identity", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({ stable_window_id: "a", hwnd: "0x1" }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 20,
        process_name: "chrome.exe",
      }),
    ]);
    const members = stageArrangementMemberKeys(tiles, [
      {
        id: "e1",
        arrangement_id: "arr",
        stable_window_id: "a",
        hwnd: null,
        process_id: 10,
        process_name: "code.exe",
        title_fingerprint: null,
        label: "Editor",
        sort_order: 0,
        x: null,
        y: null,
        width: null,
        height: null,
        authority_effect: "none",
      },
    ]);
    expect(members.has("a")).toBe(true);
    expect(members.has("b")).toBe(false);
  });

  it("supports selection toggle and keyboard neighbour keys", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({ stable_window_id: "a", x: 0, y: 0 }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 20,
        x: 100,
        y: 0,
      }),
    ]);
    expect(toggleStageSelection(["a"], "b")).toEqual(["a", "b"]);
    expect(toggleStageSelection(["a", "b"], "a")).toEqual(["b"]);
    expect(nextStageSelectionKey(tiles, "a", "next")).toBe("b");
    expect(nextStageSelectionKey(tiles, "b", "previous")).toBe("a");
    expect(nextStageSelectionKey(tiles, null, "home")).toBe("a");
  });

  it("uses one short plane message without setup language", () => {
    expect(stageDesktopPlaneMessage("ready")).toBe("No windows open.");
    expect(stageDesktopPlaneMessage("runtime_unavailable")).toMatch(
      /desktop app/i,
    );
    expect(stageDesktopPlaneMessage("runtime_unavailable")).not.toMatch(
      /create/i,
    );
    expect(stageDesktopPlaneMessage("error")).toMatch(/Could not read/i);
  });

  it("summarises observation metadata without AI framing", () => {
    const line = stageDesktopMetaLine({
      windowCount: 2,
      monitorCount: 1,
      focusedTitle: "Editor",
      selectedCount: 2,
    });
    expect(line).toMatch(/2 windows/);
    expect(line).toMatch(/2 selected/);
    expect(line).toMatch(/Editor/);
    expect(line).not.toMatch(/AI/i);
  });

  it("prefers selected process for Focus organisation", async () => {
    const { organiseStageForWorkMode } = await import(
      "../app/src/lib/stageDesktopUi"
    );
    const windows = [
      sampleWindow({
        stable_window_id: "a",
        hwnd: "0x1",
        focused: true,
        process_id: 10,
        process_name: "code.exe",
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        title: "Browser",
        process_id: 20,
        process_name: "chrome.exe",
        focused: false,
      }),
    ];
    const focus = organiseStageForWorkMode(windows, "focus", "b");
    expect(focus.mapWindows).toHaveLength(1);
    expect(focus.mapWindows[0]?.process_name).toBe("chrome.exe");
    expect(focus.dockEntries[0]?.processId).toBe(10);
  });
});
