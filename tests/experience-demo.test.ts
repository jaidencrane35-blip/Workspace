/**
 * Experience demo dataset + IPC provider — DEV fixture for populated UI.
 */
import { beforeEach, describe, expect, it } from "vitest";
import { demoInvoke, resetExperienceDemoState } from "../app/src/demo/demoIpc";
import {
  DEMO_SAVED_CONTEXTS,
  DEMO_WORKSPACE,
} from "../app/src/demo/experienceDemoDataset";
import type { ResumePlanPreview, SavedContext } from "../app/src/types/domain";

describe("experience demo dataset", () => {
  beforeEach(() => {
    resetExperienceDemoState();
  });

  it("ships multiple distinct moments with unique handoffs", () => {
    expect(DEMO_SAVED_CONTEXTS.length).toBeGreaterThanOrEqual(5);
    const names = new Set(DEMO_SAVED_CONTEXTS.map((c) => c.name));
    const notes = new Set(DEMO_SAVED_CONTEXTS.map((c) => c.handoff_note));
    expect(names.size).toBe(DEMO_SAVED_CONTEXTS.length);
    expect(notes.size).toBe(DEMO_SAVED_CONTEXTS.length);
    for (const context of DEMO_SAVED_CONTEXTS) {
      expect(context.name).toMatch(/·/);
      expect(context.handoff_note.length).toBeGreaterThan(20);
      expect(context.windows.length).toBeGreaterThan(0);
    }
  });

  it("lists contexts and resolves a restore preview with confidence items", async () => {
    const listed = await demoInvoke<SavedContext[]>("list_saved_contexts", {
      workspaceId: DEMO_WORKSPACE.id,
    });
    expect(listed[0]?.id).toBe("demo-ctx-northwind");

    const preview = await demoInvoke<ResumePlanPreview>("resolve_resume_plan", {
      savedContextId: listed[0]!.id,
    });
    expect(preview.handoff_note).toContain("pricing");
    expect(preview.plan.items.length).toBeGreaterThan(1);
    expect(
      preview.plan.items.some((i) => i.projected_disposition === "will_attempt"),
    ).toBe(true);
  });

  it("returns consented pilot snapshot with leave→resume history", async () => {
    const snapshot = await demoInvoke<{
      consent: { scope_id: string } | null;
      leave_resume: unknown[];
      median_return_minutes: number | null;
    }>("get_pilot_measurement_snapshot");
    expect(snapshot.consent).not.toBeNull();
    expect(snapshot.leave_resume.length).toBeGreaterThanOrEqual(4);
    expect(snapshot.median_return_minutes).toBeTruthy();
  });
});
