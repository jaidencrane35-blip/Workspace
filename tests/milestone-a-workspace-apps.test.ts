import { describe, expect, it } from "vitest";
import {
  activeApplicationLabel,
  applicationIdentityLine,
  applicationsEmptyCopy,
  canLaunchApplication,
} from "../app/src/lib/applicationsUi";
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

  it("explains empty registry states", () => {
    expect(applicationsEmptyCopy(false).title).toMatch(/workspace/i);
    expect(applicationsEmptyCopy(true).body).toMatch(/Register/i);
  });
});
