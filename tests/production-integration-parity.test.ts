/**
 * Production integration parity — demo adapter and Tauri command surface
 * must stay aligned for the frozen experience catalog.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it } from "vitest";
import {
  EXPERIENCE_IPC_COMMANDS,
  EXPERIENCE_SCREEN_COMMANDS,
  isExperienceIpcCommand,
} from "../app/src/demo/experienceIpcCatalog";
import { demoInvoke, resetExperienceDemoState } from "../app/src/demo/demoIpc";
import {
  DEMO_CAPTURE_SCOPE,
  DEMO_PILOT_SCOPE,
  DEMO_WORKSPACE,
} from "../app/src/demo/experienceDemoDataset";
import type {
  ActionOperationResult,
  ResumePlanPreview,
  SavedContext,
} from "../app/src/types/domain";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

const tauriLib = fs.readFileSync(
  path.join(root, "app/src-tauri/src/lib.rs"),
  "utf8",
);

const frozenScreens = [
  "home-desktop.png",
  "continue-desktop.png",
  "checkin-desktop.png",
  "guide-desktop.png",
].map((name) =>
  path.join(
    root,
    "architecture/research/experience/screenshots/convergence-pass-07",
    name,
  ),
);

describe("experience IPC catalog ↔ Tauri registration", () => {
  it("registers every catalog command on the Tauri invoke handler", () => {
    for (const command of EXPERIENCE_IPC_COMMANDS) {
      expect(tauriLib).toContain(command);
    }
  });

  it("covers every frozen pilot screen in the screen matrix", () => {
    expect(EXPERIENCE_SCREEN_COMMANDS.home.length).toBeGreaterThan(0);
    expect(EXPERIENCE_SCREEN_COMMANDS.save.length).toBeGreaterThan(0);
    expect(EXPERIENCE_SCREEN_COMMANDS.continue.length).toBeGreaterThan(0);
    expect(EXPERIENCE_SCREEN_COMMANDS.checkin.length).toBeGreaterThan(0);
    expect(EXPERIENCE_SCREEN_COMMANDS.guide).toEqual([]);
  });
});

describe("demo adapter ↔ production command parity (behaviour)", () => {
  beforeEach(() => {
    resetExperienceDemoState();
  });

  it("rejects unknown commands (adapter boundary)", async () => {
    await expect(demoInvoke("not_an_experience_command")).rejects.toThrow(
      /does not implement/,
    );
    expect(isExperienceIpcCommand("list_saved_contexts")).toBe(true);
    expect(isExperienceIpcCommand("generate_decision_queue")).toBe(false);
  });

  it("Home: lists saved contexts for the active workspace", async () => {
    const listed = await demoInvoke<SavedContext[]>("list_saved_contexts", {
      workspaceId: DEMO_WORKSPACE.id,
    });
    expect(listed.length).toBeGreaterThanOrEqual(5);
    expect(listed.every((c) => c.workspace_id === DEMO_WORKSPACE.id)).toBe(
      true,
    );
  });

  it("Save: capture scope then save yields a Moment listable on Home", async () => {
    const scope = await demoInvoke<{ id: string }>(
      "get_saved_context_capture_scope",
    );
    expect(scope.id).toBe(DEMO_CAPTURE_SCOPE.id);

    const saved = await demoInvoke<SavedContext>("save_workspace_context", {
      workspaceId: DEMO_WORKSPACE.id,
      name: "Parity moment · Test",
      approvedScope: scope.id,
      handoffNote: "Confirm adapter save matches production command shape.",
    });
    expect(saved.name).toContain("Parity moment");
    expect(saved.handoff_note).toContain("Confirm adapter");

    const listed = await demoInvoke<SavedContext[]>("list_saved_contexts", {
      workspaceId: DEMO_WORKSPACE.id,
    });
    expect(listed.some((c) => c.id === saved.id)).toBe(true);
  });

  it("Continue: resolve → execute preserve plan digest contract", async () => {
    const listed = await demoInvoke<SavedContext[]>("list_saved_contexts", {
      workspaceId: DEMO_WORKSPACE.id,
    });
    const preview = await demoInvoke<ResumePlanPreview>("resolve_resume_plan", {
      savedContextId: listed[0]!.id,
    });
    expect(preview.plan.plan_digest.length).toBeGreaterThan(0);
    expect(preview.compatibility?.confidence_band).toBeTruthy();

    const result = await demoInvoke<ActionOperationResult>(
      "execute_resume_plan",
      {
        plan: preview.plan,
        approvedPlanDigest: preview.plan.plan_digest,
      },
    );
    expect(result.outcome).toMatch(/completed|partial|failed/);
    expect(result.summary).toBeDefined();
    expect(
      result.summary!.restored_windows +
        result.summary!.skipped_windows +
        result.summary!.failed_operations,
    ).toBe(result.items.length);
    expect(typeof result.summary!.duration_ms).toBe("number");
  });

  it("Check-in: scope + snapshot + consent record path", async () => {
    const scope = await demoInvoke<{ id: string }>(
      "get_pilot_measurement_scope",
    );
    expect(scope.id).toBe(DEMO_PILOT_SCOPE.id);

    const before = await demoInvoke<{
      leave_resume: { id: string }[];
    }>("get_pilot_measurement_snapshot");
    const prior = before.leave_resume.length;

    await demoInvoke("record_pilot_leave_resume", {
      returnMinutes: 7,
      correctionNeeded: false,
      correctionNote: "",
      localDay: "2026-08-03",
    });

    const after = await demoInvoke<{
      leave_resume: { return_minutes: number }[];
    }>("get_pilot_measurement_snapshot");
    expect(after.leave_resume.length).toBe(prior + 1);
    expect(after.leave_resume[0]?.return_minutes).toBe(7);
  });
});

describe("frozen experience visual anchors", () => {
  it("keeps convergence-pass-07 screenshots as regression baselines", () => {
    for (const file of frozenScreens) {
      expect(fs.existsSync(file), file).toBe(true);
      expect(fs.statSync(file).size).toBeGreaterThan(10_000);
    }
  });

  it("keeps the experience freeze authority document", () => {
    const freeze = path.join(
      root,
      "architecture/research/experience/experience-freeze.md",
    );
    expect(fs.existsSync(freeze)).toBe(true);
    expect(fs.readFileSync(freeze, "utf8")).toContain("Experience freeze");
  });
});
