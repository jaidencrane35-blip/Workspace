import { describe, expect, it } from "vitest";
import {
  layoutStageDesktopWindows,
  stageDesktopEmptyCopy,
  stageDesktopMetaLine,
  stageDesktopPlaneMessage,
  stageDesktopWindowKey,
  stageDesktopWindowTitle,
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

    it("lays out windows spatially from real bounds", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({
        stable_window_id: "a",
        x: 0,
        y: 0,
        width: 500,
        height: 500,
        focused: true,
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        title: "Browser",
        process_name: "chrome.exe",
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
        x: 0,
        y: 500,
        width: 500,
        height: 500,
      }),
    ]);
    expect(tiles).toHaveLength(3);
    expect(tiles[0]?.hwnd).toBe("0x1");
    expect(tiles[0]?.leftPct).toBe(0);
    expect(tiles[1]?.leftPct).toBe(50);
    expect(tiles[0]?.focused).toBe(true);
    expect(tiles[0]?.title).toBe("code.exe");
    expect(tiles[0]?.processLabel).toBe("Editor");
    expect(tiles[1]?.title).toBe("chrome.exe");
    expect(tiles[1]?.processLabel).toBe("Browser");
    expect(tiles[1]?.relationIndex).toBe(tiles[2]?.relationIndex);
    expect(tiles[1]?.processKey).toBe(tiles[2]?.processKey);
    expect(tiles.every((tile) => tile.widthPct > 0 && tile.heightPct > 0)).toBe(
      true,
    );
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
        process_name: "code.exe",
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        title: "Tab 1",
        process_name: "chrome.exe",
      }),
      sampleWindow({
        stable_window_id: "c",
        hwnd: "0x3",
        title: "Tab 2",
        process_name: "chrome.exe",
      }),
      sampleWindow({
        stable_window_id: "d",
        hwnd: "0x4",
        title: "Notes",
        process_name: "notepad.exe",
      }),
    ];
    const focus = organiseStageForWorkMode(windows, "focus", null);
    expect(focus.mapWindows).toHaveLength(1);
    expect(focus.mapWindows[0]?.process_name).toBe("code.exe");
    expect(focus.dockEntries).toHaveLength(2);
    const chrome = focus.dockEntries.find(
      (entry) => entry.processKey === "chrome.exe",
    );
    expect(chrome?.windowCount).toBe(2);

    const flow = organiseStageForWorkMode(windows, "flow", null);
    expect(flow.mapWindows).toHaveLength(4);
    expect(flow.dockEntries).toHaveLength(0);
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
    expect(stageDesktopEmptyCopy("ready").body).toBe("");
  });

  it("summarises observation metadata without AI framing", () => {
    const line = stageDesktopMetaLine({
      windowCount: 2,
      monitorCount: 1,
      focusedTitle: "Editor",
    });
    expect(line).toMatch(/2 windows/);
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
        process_name: "code.exe",
      }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        title: "Browser",
        process_name: "chrome.exe",
        focused: false,
      }),
    ];
    const focus = organiseStageForWorkMode(windows, "focus", "b");
    expect(focus.mapWindows).toHaveLength(1);
    expect(focus.mapWindows[0]?.process_name).toBe("chrome.exe");
    expect(focus.dockEntries[0]?.processKey).toBe("code.exe");
  });
});
