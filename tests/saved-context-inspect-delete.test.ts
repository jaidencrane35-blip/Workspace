/**
 * PP-P01C — Inspect and delete retained saved contexts in the product UI.
 *
 * Users must be able to inspect truthful saved-context metadata and delete only
 * after an explicit confirmation. Deletion is user-initiated; nothing is removed
 * in the background.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

const panel = fs.readFileSync(
  path.join(root, "app/src/components/ResumeContextPanel.tsx"),
  "utf8",
);
const tauriResume = fs.readFileSync(
  path.join(root, "app/src-tauri/src/commands/resume.rs"),
  "utf8",
);
const tauriLib = fs.readFileSync(path.join(root, "app/src-tauri/src/lib.rs"), "utf8");
const kernelResume = fs.readFileSync(
  path.join(root, "packages/kernel/src/commands/resume.rs"),
  "utf8",
);

describe("PP-P01C inspect and delete", () => {
  it("loads a saved context for inspection via get_saved_context", () => {
    expect(panel).toContain('invokeIpc<SavedContext>("get_saved_context"');
    expect(panel).toContain('setStep("inspect")');
    expect(panel).toContain("Inspect");
  });

  it("presents retained metadata on inspect without inventing content", () => {
    expect(panel).toContain("inspected.handoff_note");
    expect(panel).toContain("inspected.windows.map");
    expect(panel).toContain("inspected.monitors.map");
    expect(panel).toContain("approved_scope");
    expect(panel).not.toMatch(/generat(?:e|ed).*handoff|summariz/i);
  });

  it("requires an explicit confirmation step before delete", () => {
    expect(panel).toContain('setStep("confirm_delete")');
    expect(panel).toContain("Delete permanently");
    expect(panel).toContain("cannot be undone");
    // Delete IPC is only reached from confirmDelete after the confirm step.
    expect(panel).toMatch(/const confirmDelete = \(\) => \{/);
    expect(panel).toMatch(
      /confirmDelete = \(\) => \{[\s\S]*delete_saved_context/,
    );
    expect(panel).toContain('onClick={() => setStep("confirm_delete")}');
    expect(panel).toContain("onClick={confirmDelete}");
  });

  it("wires delete through Tauri IPC to the kernel mutation", () => {
    expect(panel).toContain('invokeIpc<null>("delete_saved_context"');
    expect(tauriResume).toContain("pub fn delete_saved_context");
    expect(tauriLib).toContain("delete_saved_context");
    expect(kernelResume).toContain("struct DeleteSavedContext");
    expect(kernelResume).toContain("Capability::workspace_write()");
  });

  it("does not introduce background cleanup or retention policies", () => {
    expect(panel).not.toMatch(/setInterval|retention|auto.?delet|expire.*delet/i);
    expect(kernelResume).not.toMatch(/retention|auto.?delet|background.?clean/i);
  });
});
