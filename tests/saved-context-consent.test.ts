/**
 * Consent invariants for the Save flow (PP-M1-01).
 *
 * The kernel refuses a save whose approved scope is not the scope this build
 * would capture. That check is only meaningful if the interface forwards the
 * scope it was served rather than a value of its own, and if it shows the user
 * the whole scope rather than the reassuring half. Neither property can be
 * enforced from the Rust side, so it is audited here.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

const panel = fs.readFileSync(
  path.join(root, "app/src/components/SaveContextPanel.tsx"),
  "utf8",
);

const scopeDeclaration = fs.readFileSync(
  path.join(root, "packages/domain/src/saved_context/mod.rs"),
  "utf8",
);

describe("Save flow consent", () => {
  it("takes the capture scope from the kernel", () => {
    expect(panel).toMatch(
      /invokeIpc<SavedContextCaptureScope>\(\s*"get_saved_context_capture_scope"/,
    );
  });

  it("forwards the scope it was served instead of asserting one of its own", () => {
    expect(panel).toContain("approvedScope: scope.id");

    // A scope identifier written into the interface would let a stale preview
    // satisfy the kernel's staleness check.
    const declaredScopeId = /SAVED_CONTEXT_SCOPE_ID: &str = "([^"]+)"/
      .exec(scopeDeclaration)?.[1];
    expect(declaredScopeId).toBeTruthy();
    expect(panel).not.toContain(declaredScopeId as string);
    expect(panel).not.toContain("saved-context-scope");
  });

  it("requires a user-authored handoff and forwards it unchanged", () => {
    expect(panel).toContain("handoffNote: trimmedHandoff");
    expect(panel).toMatch(/trimmedHandoff\.length > 0/);
    expect(panel).toContain("You write this");
    expect(panel).not.toMatch(/generate.*handoff|summariz/i);
  });

  it("shows what will not be saved alongside what will", () => {
    expect(panel).toContain("scope.captured.map");
    expect(panel).toContain("scope.excluded.map");
  });

  it("cannot save before the scope has been served", () => {
    // `canReview` gates the step that leads to saving, and `save` returns early
    // without a scope, so an unreviewed capture has no route to the kernel.
    expect(panel).toContain("const canReview =");
    expect(panel).toContain("scope !== null");
    expect(panel).toMatch(
      /if \(!workspace \|\| !scope \|\| !trimmedHandoff\) \{\s*return;\s*\}/,
    );
  });
});
