import { describe, expect, it } from "vitest";
import {
  activeApplicationLabel,
  applicationIdentityLine,
  applicationsEmptyCopy,
  applicationsLayoutsRelationCopy,
  canLaunchApplication,
} from "../app/src/lib/applicationsUi";
import {
  classifyBanner,
  monogramFromName,
} from "../app/src/lib/productShellUi";
import { IpcRuntimeUnavailableError } from "../app/src/lib/ipc";
import {
  formatWorkspaceMeta,
  workspaceRowLabel,
  workspaceSwitcherEmptyCopy,
} from "../app/src/lib/workspaceSwitcherUi";
import type {
  ApplicationReference,
  Workspace,
  WorkspaceActiveApplication,
} from "../app/src/types/domain";

const workspace: Workspace = {
  id: "ws-1",
  name: "Deep work",
  created_at: "2026-07-29T10:00:00Z",
  updated_at: "2026-07-29T12:00:00Z",
};

const app: ApplicationReference = {
  id: "app-1",
  workspace_id: "ws-1",
  name: "Code",
  identifier: "editor",
  executable_path: "C:\\\\Apps\\\\code.exe",
};

describe("workspace switcher UI helpers", () => {
  it("marks the current workspace in list labels", () => {
    expect(workspaceRowLabel(workspace, "ws-1")).toContain("(current)");
    expect(workspaceRowLabel(workspace, "other")).toBe("Deep work");
    expect(formatWorkspaceMeta(workspace)).toContain("Updated");
  });

  it("provides empty-state copy for runtime and empty lists", () => {
    expect(workspaceSwitcherEmptyCopy(false).title).toMatch(/runtime/i);
    expect(workspaceSwitcherEmptyCopy(true).body).toMatch(/Optional/i);
    expect(workspaceSwitcherEmptyCopy(true).title).toMatch(/profile/i);
  });
});

describe("applications UI helpers", () => {
  it("summarizes identity and launch readiness", () => {
    expect(applicationIdentityLine(app)).toContain("editor");
    expect(canLaunchApplication(app)).toBe(true);
    expect(
      canLaunchApplication({ ...app, executable_path: null }),
    ).toBe(false);
  });

  it("labels observed active applications", () => {
    const active: WorkspaceActiveApplication = {
      process_id: 42,
      process_name: "code.exe",
      window_count: 2,
    };
    expect(activeApplicationLabel(active)).toContain("code.exe");
    expect(activeApplicationLabel(active)).toContain("2 windows");
    expect(activeApplicationLabel(active)).not.toMatch(/open/i);
  });

  it("explains empty registry states and layout relationship", () => {
    expect(applicationsEmptyCopy(false).title).toMatch(/profile/i);
    expect(applicationsEmptyCopy(true).body).toMatch(/Stage/i);
    expect(applicationsLayoutsRelationCopy()).toMatch(/Running/i);
    expect(applicationsEmptyCopy(false).body).toMatch(/Stage/i);
  });
});

describe("product shell UI helpers", () => {
  it("softens desktop-runtime unavailable into a runtime banner", () => {
    const banner = classifyBanner(new IpcRuntimeUnavailableError());
    expect(banner.kind).toBe("runtime");
    expect(banner.text).toMatch(/preview/i);
  });

  it("builds monograms for cards", () => {
    expect(monogramFromName("Deep work")).toBe("DW");
    expect(monogramFromName("Code")).toBe("CO");
  });
});

describe("layouts stage UI helpers", () => {
  it("keeps stage copy desktop-first and minimal", async () => {
    const {
      layoutsStageEmptyAppsCopy,
      layoutsStageTitle,
      layoutsStageRegistryHeading,
    } = await import("../app/src/lib/layoutsStageUi");
    expect(layoutsStageTitle("Deep work")).toBe("Deep work");
    expect(layoutsStageTitle(null)).toBe("Desktop");
    expect(layoutsStageEmptyAppsCopy().title).toMatch(/library/i);
    expect(layoutsStageRegistryHeading()).toMatch(/Library/i);
  });
});

describe("application launch helpers", () => {
  it("blocks launch without an executable path", async () => {
    const { launchBlockedReason, launchSuccessMessage } = await import(
      "../app/src/lib/applicationLaunch"
    );
    expect(
      launchBlockedReason({
        id: "a",
        workspace_id: "w",
        name: "Notes",
        identifier: null,
        executable_path: null,
      }),
    ).toMatch(/executable path/i);
    expect(
      launchBlockedReason({
        id: "a",
        workspace_id: "w",
        name: "Notes",
        identifier: null,
        executable_path: "C:\\\\app.exe",
      }),
    ).toBeNull();
    expect(
      launchSuccessMessage({
        application_id: "a",
        name: "Notes",
        executable_path: "C:\\\\app.exe",
        process_id: null,
        simulated: true,
      }),
    ).toMatch(/simulated/i);
  });
});

describe("work mode helpers", () => {
  it("parses and labels Flow/Focus chrome modes", async () => {
    const {
      parseWorkMode,
      workModeLabel,
      workModeDescription,
      FOCUS_PRIMARY_APP_COUNT,
      DEFAULT_WORK_MODE,
    } = await import("../app/src/lib/workMode");
    expect(parseWorkMode("focus")).toBe("focus");
    expect(parseWorkMode("nope")).toBe(DEFAULT_WORK_MODE);
    expect(workModeLabel("flow")).toBe("Flow");
    expect(workModeDescription("focus")).toMatch(/Same desktop/i);
    expect(FOCUS_PRIMARY_APP_COUNT).toBe(1);
  });

  it("partitions Focus primary vs supporting with a shared rule", async () => {
    const { partitionFocusApplications } = await import(
      "../app/src/lib/workMode"
    );
    const apps = [
      { id: "a", name: "Alpha" },
      { id: "b", name: "Beta" },
      { id: "c", name: "Gamma" },
    ];
    expect(partitionFocusApplications([], null)).toEqual({
      primary: null,
      supporting: [],
    });
    expect(partitionFocusApplications(apps, null).primary?.id).toBe("a");
    expect(partitionFocusApplications(apps, "b").primary?.id).toBe("b");
    expect(
      partitionFocusApplications(apps, "b").supporting.map((app) => app.id),
    ).toEqual(["a", "c"]);
    expect(partitionFocusApplications(apps, "missing").primary?.id).toBe("a");
  });
});

describe("product shell banner helpers", () => {
  it("classifies string and runtime errors without wrapping", async () => {
    const { classifyBanner, DESKTOP_PREVIEW_BANNER } = await import(
      "../app/src/lib/productShellUi"
    );
    expect(classifyBanner("plain failure").kind).toBe("error");
    expect(classifyBanner("Desktop runtime is unavailable").kind).toBe(
      "runtime",
    );
    expect(classifyBanner("Desktop runtime is unavailable").text).toBe(
      DESKTOP_PREVIEW_BANNER,
    );
  });
});

describe("assistant companion rail helpers", () => {
  it("parses open/collapsed preference with a safe default", async () => {
    const {
      parseAssistantRailOpen,
      assistantRailToggleLabel,
      DEFAULT_ASSISTANT_RAIL_OPEN,
    } = await import("../app/src/lib/assistantRail");
    expect(DEFAULT_ASSISTANT_RAIL_OPEN).toBe(false);
    expect(parseAssistantRailOpen("0")).toBe(false);
    expect(parseAssistantRailOpen("1")).toBe(true);
    expect(parseAssistantRailOpen("nope")).toBe(false);
    expect(assistantRailToggleLabel(true)).toMatch(/Hide/i);
    expect(assistantRailToggleLabel(false)).toMatch(/Show/i);
  });
  it("exports a stable companion rail DOM id for aria-controls", async () => {
    const { ASSISTANT_COMPANION_RAIL_ID } = await import(
      "../app/src/lib/assistantRail"
    );
    expect(ASSISTANT_COMPANION_RAIL_ID).toBe(
      "workspace-assistant-companion-rail",
    );
  });
});

describe("assistant companion chat helpers", () => {
  it("prefers utterance body for the visible answer", async () => {
    const { companionAnswerFromSurface } = await import(
      "../app/src/lib/assistantCompanion"
    );
    expect(
      companionAnswerFromSurface({
        workspace_id: "ws-1",
        current: {
          surface_id: "s1",
          workspace_id: "ws-1",
          generated_at: "2026-07-30T00:00:00Z",
          status: "ready",
          superseded_at: null,
          human_ask: "What is open?",
          scope: {} as never,
          utterance: {
            utterance_id: "u1",
            role: "assistant",
            body: "Editor and browser are open.",
            citations: [],
            authority_effect: "none",
            actionable: false,
          },
          lineage: {} as never,
          gaps: [],
          diagnostics: {
            consulted_surfaces: [],
            unavailable_surfaces: [],
            notes: [],
            authority_effect: "none",
            actionable: false,
          },
          narrative_summary: "Summary only",
          narrative: "Long narrative",
          limitations: [],
          authority_effect: "none",
          actionable: false,
          terminal: true,
        },
        history: [],
        history_count: 0,
        projected_at: "2026-07-30T00:00:00Z",
        authority_effect: "none",
      }),
    ).toBe("Editor and browser are open.");
  });

  it("keeps recent turns bounded in session storage", async () => {
    const store = new Map<string, string>();
    const memoryStorage = {
      getItem: (key: string) => store.get(key) ?? null,
      setItem: (key: string, value: string) => {
        store.set(key, value);
      },
      removeItem: (key: string) => {
        store.delete(key);
      },
    };
    Object.defineProperty(globalThis, "sessionStorage", {
      configurable: true,
      value: memoryStorage,
    });
    const {
      ASSISTANT_COMPANION_RECENT_KEY,
      ASSISTANT_COMPANION_RECENT_LIMIT,
      appendCompanionRecentTurn,
      companionThreadTurns,
      enrichAskWithDesktopObservation,
      loadCompanionRecentTurns,
    } = await import("../app/src/lib/assistantCompanion");
    memoryStorage.removeItem(ASSISTANT_COMPANION_RECENT_KEY);
    for (let i = 0; i < ASSISTANT_COMPANION_RECENT_LIMIT + 3; i += 1) {
      appendCompanionRecentTurn({
        id: `t-${i}`,
        ask: `Ask ${i}`,
        answer: `Answer ${i}`,
        at: "2026-07-30T00:00:00Z",
      });
    }
    const recent = loadCompanionRecentTurns();
    expect(recent).toHaveLength(ASSISTANT_COMPANION_RECENT_LIMIT);
    expect(recent[0]?.id).toBe(`t-${ASSISTANT_COMPANION_RECENT_LIMIT + 2}`);
    const thread = companionThreadTurns(recent);
    expect(thread[0]?.id).toBe(recent[recent.length - 1]?.id);
    expect(
      enrichAskWithDesktopObservation("What is open?", {
        metadata: {
          state_id: "s1",
          created_at: "2026-07-30T00:00:00Z",
          observation_pass_id: "pass-1",
          latest_delta_reference: null,
          window_count: 1,
          monitor_count: 1,
          has_changes: false,
          authority_effect: "none",
        },
        focused_window: {
          stable_window_id: "a",
          hwnd: "0x1",
          title: "Editor",
          process_id: 1,
        },
        active_applications: [],
        windows: [
          {
            stable_window_id: "a",
            hwnd: "0x1",
            title: "Editor",
            process_id: 1,
            process_name: "code.exe",
            visible: true,
            focused: true,
            minimized: false,
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            monitor_index: 0,
            monitor_name: "Display 1",
          },
        ],
        authority_effect: "none",
      }),
    ).toMatch(/Observed desktop[\s\S]*What is open\?/);
    memoryStorage.removeItem(ASSISTANT_COMPANION_RECENT_KEY);
  });
});

describe("chrome preference hooks", () => {
  it("exposes work-mode and assistant-rail helpers used by App", async () => {
    const workMode = await import("../app/src/lib/useWorkMode");
    const rail = await import("../app/src/lib/useAssistantRail");
    expect(typeof workMode.useWorkMode).toBe("function");
    expect(typeof rail.useAssistantRail).toBe("function");
  });
});
