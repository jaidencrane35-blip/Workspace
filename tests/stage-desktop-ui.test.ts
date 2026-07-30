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
  sortStageTilesByZOrder,
  toggleStageSelection,
} from "../app/src/lib/stageDesktopUi";
import type {
  DesktopWindowGroup,
  WorkspaceStateWindow,
} from "../app/src/types/domain";

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
    z_order: null,
    x: 0,
    y: 0,
    width: 800,
    height: 600,
    monitor_index: 0,
    monitor_name: "Display 1",
    first_seen_at: null,
    last_seen_at: null,
    identity_confidence: null,
    ...overrides,
  };
}

function sampleGroup(
  overrides: Partial<DesktopWindowGroup> &
    Pick<DesktopWindowGroup, "criterion" | "member_ids">,
): DesktopWindowGroup {
  return {
    id: `group:${overrides.criterion}:${overrides.member_ids.join("-")}`,
    fact_key: overrides.fact_key ?? overrides.member_ids.join(","),
    label: overrides.label ?? overrides.criterion,
    evidence_count: 1,
    confidence: "structural",
    authority_effect: "none",
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

  it("uses observed monitors as the Stage plane when present", () => {
    const tiles = layoutStageDesktopWindows(
      [
        sampleWindow({
          stable_window_id: "a",
          x: 100,
          y: 100,
          width: 200,
          height: 200,
        }),
      ],
      [
        {
          monitor_index: 0,
          name: "Primary",
          x: 0,
          y: 0,
          width: 1000,
          height: 1000,
          work_x: 0,
          work_y: 0,
          work_w: 1000,
          work_h: 960,
          is_primary: true,
        },
      ],
    );
    expect(tiles[0]?.leftPct).toBe(10);
    expect(tiles[0]?.topPct).toBe(10);
    expect(tiles[0]?.widthPct).toBe(20);
  });

  it("paints lower z_order tiles later (foreground on top)", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({
        stable_window_id: "back",
        hwnd: "0x2",
        z_order: 2,
        x: 0,
        y: 0,
      }),
      sampleWindow({
        stable_window_id: "front",
        hwnd: "0x1",
        z_order: 0,
        x: 10,
        y: 10,
      }),
    ]);
    const ordered = sortStageTilesByZOrder(tiles);
    expect(ordered.map((tile) => tile.key)).toEqual(["back", "front"]);
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
    const focus = organiseStageForWorkMode(windows, "focus", null, [
      sampleGroup({
        criterion: "process_id",
        fact_key: "10",
        label: "code.exe",
        member_ids: ["a"],
      }),
      sampleGroup({
        criterion: "process_id",
        fact_key: "20",
        label: "chrome.exe",
        member_ids: ["b", "c"],
      }),
    ]);
    expect(focus.mapWindows).toHaveLength(1);
    expect(focus.mapWindows[0]?.process_name).toBe("code.exe");
    expect(focus.dockEntries).toHaveLength(2);
    const chrome = focus.dockEntries.find((entry) => entry.processId === 20);
    expect(chrome?.windowCount).toBe(2);

    const flow = organiseStageForWorkMode(windows, "flow", null, []);
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
    const groups = [
      sampleGroup({
        criterion: "process_id",
        fact_key: "10",
        label: "code.exe",
        member_ids: ["a", "b"],
      }),
      sampleGroup({
        criterion: "monitor_index",
        fact_key: "0",
        label: "monitor:0",
        member_ids: ["a", "c"],
      }),
    ];
    const focusRelated = relatedStageObjectKeys(tiles, "a", "focus", groups);
    expect(focusRelated.has("a")).toBe(true);
    expect(focusRelated.has("b")).toBe(true);
    expect(focusRelated.has("c")).toBe(false);

    const flowRelated = relatedStageObjectKeys(tiles, "a", "flow", groups);
    expect(flowRelated.has("c")).toBe(true);
  });

  it("does not invent relationships without authoritative groups", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({ stable_window_id: "a", process_id: 10 }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 10,
      }),
    ]);
    const related = relatedStageObjectKeys(tiles, "a", "flow", []);
    expect([...related]).toEqual(["a"]);
  });

  it("Flow includes arrangement-membership groups", () => {
    const tiles = layoutStageDesktopWindows([
      sampleWindow({ stable_window_id: "a", process_id: 10 }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 99,
        process_name: "notes.exe",
      }),
    ]);
    const groups = [
      sampleGroup({
        criterion: "arrangement_membership",
        fact_key: "arr-1",
        label: "Working set",
        member_ids: ["a", "b"],
      }),
    ];
    expect(relatedStageObjectKeys(tiles, "a", "focus", groups).has("b")).toBe(
      false,
    );
    expect(relatedStageObjectKeys(tiles, "a", "flow", groups).has("b")).toBe(
      true,
    );
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
    const focus = organiseStageForWorkMode(windows, "focus", "b", [
      sampleGroup({
        criterion: "process_id",
        fact_key: "20",
        label: "chrome.exe",
        member_ids: ["b"],
      }),
      sampleGroup({
        criterion: "process_id",
        fact_key: "10",
        label: "code.exe",
        member_ids: ["a"],
      }),
    ]);
    expect(focus.mapWindows).toHaveLength(1);
    expect(focus.mapWindows[0]?.process_name).toBe("chrome.exe");
    expect(focus.dockEntries[0]?.processId).toBe(10);
  });

  it("surfaces attention and semantic awareness for Stage coherence", async () => {
    const {
      organiseStageForWorkMode,
      stageAttentionAwarenessLine,
      stageAttentionPrimaryKeys,
      stageFocusPreferredKeys,
      stageSemanticRoleByKey,
    } = await import("../app/src/lib/stageDesktopUi");
    const attention = {
      primary_item_id: "att-1",
      authority_effect: "none",
      items: [
        {
          id: "att-1",
          kind: "resume",
          summary: "Resume the editor",
          explanation: "Interrupted working object",
          lifecycle: "stable",
          strength: 0.9,
          confidence: "high",
          time_sensitivity: "near_term",
          entity_ids: ["b"],
          evidence: [],
          supporting_planes: ["semantics"],
          source_decision_id: null,
          authority_effect: "none",
        },
      ],
    };
    const semantics = {
      objects: [
        {
          stable_window_id: "b",
          hwnd: "0x2",
          title: "Browser",
          role: "working",
          importance: "important",
          confidence: "high",
          evidence_score: 1,
          authority_effect: "none",
        },
        {
          stable_window_id: "a",
          hwnd: "0x1",
          title: "Editor",
          role: "companion",
          importance: "routine",
          confidence: "medium",
          evidence_score: 0.5,
          authority_effect: "none",
        },
      ],
      relationships: [
        {
          from_stable_window_id: "b",
          to_stable_window_id: "a",
          kind: "works_with",
          evidence_count: 3,
          session_count: 2,
          confidence: "high",
          authority_effect: "none",
        },
      ],
      activities: [],
      graph: { nodes: [], edges: [] },
      authority_effect: "none",
    };
    expect(stageAttentionAwarenessLine(attention)).toBe("Resume the editor");
    expect([...stageAttentionPrimaryKeys(attention)]).toEqual(["b"]);
    expect(stageSemanticRoleByKey(semantics).get("b")).toBe("working");
    expect(stageFocusPreferredKeys(attention, semantics)[0]).toBe("b");

    const tiles = layoutStageDesktopWindows([
      sampleWindow({ stable_window_id: "a", process_id: 10, focused: true }),
      sampleWindow({
        stable_window_id: "b",
        hwnd: "0x2",
        process_id: 20,
        process_name: "chrome.exe",
        title: "Browser",
      }),
    ]);
    const flowRelated = relatedStageObjectKeys(
      tiles,
      "b",
      "flow",
      [],
      semantics,
    );
    expect(flowRelated.has("a")).toBe(true);

    const focus = organiseStageForWorkMode(
      [
        sampleWindow({
          stable_window_id: "a",
          hwnd: "0x1",
          focused: true,
          process_id: 10,
        }),
        sampleWindow({
          stable_window_id: "b",
          hwnd: "0x2",
          process_id: 20,
          process_name: "chrome.exe",
          title: "Browser",
        }),
      ],
      "focus",
      null,
      [
        sampleGroup({
          criterion: "process_id",
          fact_key: "20",
          label: "chrome.exe",
          member_ids: ["b"],
        }),
        sampleGroup({
          criterion: "process_id",
          fact_key: "10",
          label: "code.exe",
          member_ids: ["a"],
        }),
      ],
      [],
      stageFocusPreferredKeys(attention, semantics),
    );
    expect(focus.mapWindows[0]?.stable_window_id).toBe("b");
  });

  it("includes awareness line in Stage meta without diagnostic jargon", () => {
    const line = stageDesktopMetaLine({
      windowCount: 2,
      monitorCount: 1,
      focusedTitle: "Editor",
      awarenessLine: "Resume the editor",
    });
    expect(line).toMatch(/Resume the editor/);
    expect(line).not.toMatch(/attention_item|entity_ids|AI/i);
  });
});
