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
    expect(workspaceSwitcherEmptyCopy(true).body).toMatch(/named workspace/i);
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
  });

  it("explains empty registry states and layout relationship", () => {
    expect(applicationsEmptyCopy(false).title).toMatch(/workspace/i);
    expect(applicationsEmptyCopy(true).body).toMatch(/Layouts/i);
    expect(applicationsLayoutsRelationCopy()).toMatch(/Layouts/i);
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
  it("keeps stage copy honest about modes and OS windows", async () => {
    const {
      layoutsStageLede,
      layoutsStageEmptyAppsCopy,
      layoutsStageCanvasNote,
    } = await import("../app/src/lib/layoutsStageUi");
    expect(layoutsStageLede()).toMatch(/Flow/i);
    expect(layoutsStageEmptyAppsCopy().body).toMatch(/OS windows/i);
    expect(layoutsStageCanvasNote(0)).toMatch(/Companion canvas/i);
    expect(layoutsStageCanvasNote(2)).toMatch(/2 companion/i);
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
    expect(workModeDescription("focus")).toMatch(/OS windows/i);
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
    expect(DEFAULT_ASSISTANT_RAIL_OPEN).toBe(true);
    expect(parseAssistantRailOpen("0")).toBe(false);
    expect(parseAssistantRailOpen("1")).toBe(true);
    expect(parseAssistantRailOpen("nope")).toBe(true);
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

describe("chrome preference hooks", () => {
  it("exposes work-mode and assistant-rail helpers used by App", async () => {
    const workMode = await import("../app/src/lib/useWorkMode");
    const rail = await import("../app/src/lib/useAssistantRail");
    expect(typeof workMode.useWorkMode).toBe("function");
    expect(typeof rail.useAssistantRail).toBe("function");
  });
});
