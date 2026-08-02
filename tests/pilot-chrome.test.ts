/**
 * PP-P01D — Pilot-safe primary chrome.
 *
 * Default navigation must be Save / Resume / Help. Engine tabs (Canvas, Work,
 * Assistant, Diagnostic) must not appear as primary pilot surfaces.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  PILOT_HIDDEN_ENGINE_TAB_LABELS,
  PILOT_PRIMARY_TAB_LABELS,
  PILOT_PRIMARY_VIEWS,
} from "../app/src/lib/pilotChrome";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

const appSource = fs.readFileSync(path.join(root, "app/src/App.tsx"), "utf8");
const helpSource = fs.readFileSync(
  path.join(root, "app/src/components/PilotHelpPanel.tsx"),
  "utf8",
);
const saveSource = fs.readFileSync(
  path.join(root, "app/src/components/SaveContextPanel.tsx"),
  "utf8",
);
const resumeSource = fs.readFileSync(
  path.join(root, "app/src/components/ResumeContextPanel.tsx"),
  "utf8",
);

describe("PP-P01D pilot-safe chrome", () => {
  it("defines Save, Resume, and Help as the only primary pilot views", () => {
    expect([...PILOT_PRIMARY_VIEWS]).toEqual(["save", "resume", "help"]);
    expect([...PILOT_PRIMARY_TAB_LABELS]).toEqual(["Save", "Resume", "Help"]);
    for (const label of PILOT_PRIMARY_TAB_LABELS) {
      expect(appSource).toContain(`>\n            ${label}\n          </button>`);
    }
  });

  it("keeps engine tabs out of the default pilot chrome", () => {
    for (const label of PILOT_HIDDEN_ENGINE_TAB_LABELS) {
      expect(appSource).not.toContain(`>\n            ${label}\n          </button>`);
      expect(appSource).not.toContain(`>${label}</button>`);
    }
    expect(appSource).not.toContain("OperatorConsole");
    expect(appSource).not.toContain("CanvasShell");
    expect(appSource).not.toContain("WorkspaceIntelligencePanel");
    expect(appSource).not.toContain("AssistantPanel");
  });

  it("preserves Product Proof Save and Resume surfaces", () => {
    expect(appSource).toContain("SaveContextPanel");
    expect(appSource).toContain("ResumeContextPanel");
    expect(appSource).toContain("PilotHelpPanel");
    expect(saveSource).toContain("RestoreLimitsNotice");
    expect(resumeSource).toContain("RestoreLimitsNotice");
    expect(resumeSource).toContain("delete_saved_context");
    expect(resumeSource).toContain("get_saved_context");
  });

  it("creates a workspace from Save without routing through Canvas", () => {
    expect(saveSource).toContain("onCreateWorkspace");
    expect(saveSource).toContain("Create a workspace");
    expect(saveSource).not.toContain("Go to Canvas");
    expect(saveSource).not.toContain("onGoToCanvas");
    expect(appSource).toContain("create_workspace");
    expect(appSource).toContain("onCreateWorkspace={createWorkspace}");
  });

  it("offers minimal help that restates trust limits without AI claims", () => {
    expect(helpSource).toContain("RESTORE_LIMITS_SUMMARY");
    expect(helpSource).toContain("How this pilot works");
    expect(helpSource).toContain("You approve every restore plan");
    expect(helpSource).not.toMatch(/\bAI\b|generated summary|engine console/i);
  });
});
