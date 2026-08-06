/**
 * PP-P01D — Pilot-safe primary chrome (Experience presentation).
 *
 * Default navigation must remain Product Proof surfaces without engine tabs.
 * Labels may be product-oriented (Home / Save / Continue / Check-in / Guide).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  PILOT_HIDDEN_ENGINE_TAB_LABELS,
  PILOT_PRIMARY_TAB_LABELS,
  PILOT_PRIMARY_VIEWS,
  PILOT_VIEW_LABELS,
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
const homeSource = fs.readFileSync(
  path.join(root, "app/src/components/HomeWorkspacePanel.tsx"),
  "utf8",
);

describe("PP-P01D pilot-safe chrome", () => {
  it("defines product-oriented primary pilot views without engine tabs", () => {
    expect([...PILOT_PRIMARY_VIEWS]).toEqual([
      "home",
      "save",
      "resume",
      "pilot",
      "help",
    ]);
    expect([...PILOT_PRIMARY_TAB_LABELS]).toEqual([
      "Home",
      "Save",
      "Continue",
      "Check-in",
      "Guide",
    ]);
    expect(appSource).toContain("PILOT_PRIMARY_VIEWS");
    // View labels remain in pilotChrome; App no longer mounts a labeled dock.
    expect(Object.keys(PILOT_VIEW_LABELS).length).toBe(PILOT_PRIMARY_VIEWS.length);
    for (const label of Object.values(PILOT_VIEW_LABELS)) {
      expect(PILOT_PRIMARY_TAB_LABELS).toContain(label);
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

  it("preserves Save/Resume as specialized tools without Home onboarding identity", () => {
    expect(appSource).toContain("SaveContextPanel");
    expect(appSource).toContain("ResumeContextPanel");
    expect(appSource).toContain("PilotHelpPanel");
    expect(appSource).toContain("HomeWorkspacePanel");
    expect(appSource).toContain("OperatorRoot");
    expect(saveSource).toContain("RestoreLimitsNotice");
    expect(resumeSource).toContain("RestoreLimitsNotice");
    expect(resumeSource).toContain("delete_saved_context");
    expect(resumeSource).toContain("get_saved_context");
    // Moments tool — not Product Proof onboarding theatre
    expect(homeSource).toContain("Moments");
    expect(homeSource).toContain("Say “save this” in conversation");
    expect(homeSource).not.toContain("Save your first moment");
    expect(homeSource).not.toContain("This is your Workspace");
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
