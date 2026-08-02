/**
 * Observation pipeline regression — demo Moments and restore previews must
 * expose the same logical fields the frozen Continue UI reads from production.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it } from "vitest";
import { DEMO_WORKSPACE } from "../app/src/demo/experienceDemoDataset";
import { demoInvoke, resetExperienceDemoState } from "../app/src/demo/demoIpc";
import type { ResumePlanPreview, SavedContext } from "../app/src/types/domain";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

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

describe("observation pipeline — capture → persist → restore preview", () => {
  beforeEach(() => {
    resetExperienceDemoState();
  });

  it("save → list → resolve yields compatibility bands the UI understands", async () => {
    const scope = await demoInvoke<{ id: string }>(
      "get_saved_context_capture_scope",
    );
    const saved = await demoInvoke<SavedContext>("save_workspace_context", {
      workspaceId: DEMO_WORKSPACE.id,
      name: "Pipeline Moment",
      approvedScope: scope.id,
      handoffNote: "Finish the outline",
    });

    expect(saved.windows.length).toBeGreaterThan(0);
    expect(saved.monitors.length).toBeGreaterThan(0);
    expect(saved.captured_at).toBeTruthy();
    expect(saved.windows.every((w) => typeof w.process_id === "number")).toBe(
      true,
    );

    const listed = await demoInvoke<SavedContext[]>("list_saved_contexts", {
      workspaceId: DEMO_WORKSPACE.id,
    });
    expect(listed.some((c) => c.id === saved.id)).toBe(true);

    const preview = await demoInvoke<ResumePlanPreview>("resolve_resume_plan", {
      savedContextId: saved.id,
    });
    expect(preview.plan.items.length).toBeGreaterThan(0);
    expect(preview.compatibility).toBeDefined();
    expect(["high", "steady", "limited", "empty"]).toContain(
      preview.compatibility?.confidence_band,
    );

    const willAttempt = preview.plan.items.filter(
      (i) => i.projected_disposition === "will_attempt",
    ).length;
    expect(preview.compatibility?.will_attempt).toBe(willAttempt);
    expect(preview.compatibility?.restore_eligible).toBe(willAttempt > 0);
  });

  it("missing / skipped dispositions still render Continue quality copy inputs", async () => {
    const listed = await demoInvoke<SavedContext[]>("list_saved_contexts", {
      workspaceId: DEMO_WORKSPACE.id,
    });
    const withMinimized = listed.find((c) =>
      c.windows.some((w) => w.minimized),
    );
    expect(withMinimized).toBeTruthy();

    const preview = await demoInvoke<ResumePlanPreview>("resolve_resume_plan", {
      savedContextId: withMinimized!.id,
    });
    const skips = preview.plan.items.filter(
      (i) => i.projected_disposition !== "will_attempt",
    );
    expect(skips.length).toBeGreaterThan(0);
    expect(preview.compatibility?.will_skip_unresolvable).toBeGreaterThan(0);
  });
});

describe("observation pipeline — frozen screenshot anchors", () => {
  it("keeps pass-07 desktop baselines present and non-trivial", () => {
    for (const file of frozenScreens) {
      expect(fs.existsSync(file), file).toBe(true);
      expect(fs.statSync(file).size).toBeGreaterThan(8_000);
    }
  });
});

describe("observation model docs", () => {
  it("documents canonical Snapshot → Moment → Plan → Execution", () => {
    const doc = fs.readFileSync(
      path.join(root, "architecture/24_Runtime_Observation_Model.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceObservationSnapshot");
    expect(doc).toContain("SavedContext");
    expect(doc).toContain("ActionPlan");
    expect(doc).toContain("ActionOperationResult");
    expect(doc).toContain("save_workspace_context");
    expect(doc).toContain("resolve_resume_plan");
  });

  it("documents restore executor authority", () => {
    const doc = fs.readFileSync(
      path.join(root, "architecture/25_Restore_Execution_Model.md"),
      "utf8",
    );
    expect(doc).toContain("RestoreExecutor");
    expect(doc).toContain("RestoreExecutionSummary");
    expect(doc).toContain("SWP_NOZORDER");
  });

  it("documents canonical WorkspaceRuntimeState ownership", () => {
    const doc = fs.readFileSync(
      path.join(root, "architecture/26_Workspace_Runtime_State.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceRuntimeState");
    expect(doc).toContain("ObservationCachePhase");
    expect(doc).toContain("RestoreExecutionPhase");
    expect(doc).toContain("get_workspace_runtime_state");
  });

  it("documents persistent session field classification", () => {
    const doc = fs.readFileSync(
      path.join(root, "architecture/27_Workspace_Session_Persistence.md"),
      "utf8",
    );
    expect(doc).toContain("PersistentWorkspaceSession");
    expect(doc).toContain("WorkspaceSessionStore");
    expect(doc).toContain("Ephemeral");
    expect(doc).toContain("Derived");
    expect(doc).toContain("schema_version");
  });
});
