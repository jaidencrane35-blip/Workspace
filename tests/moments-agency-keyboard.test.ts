import { describe, expect, it } from "vitest";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P17.S3 Moments agency keyboard continuity", () => {
  const hook = fs.readFileSync(
    path.join(root, "app/src/hooks/useAgencyKeyboard.ts"),
    "utf8",
  );
  const preview = fs.readFileSync(
    path.join(root, "app/src/components/objects/ContinuePreviewObject.tsx"),
    "utf8",
  );
  const resume = fs.readFileSync(
    path.join(root, "app/src/components/ResumeContextPanel.tsx"),
    "utf8",
  );
  const operator = fs.readFileSync(
    path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
    "utf8",
  );

  it("shares one agency keyboard hook for Esc dismiss and Enter primary", () => {
    expect(hook).toContain("export function useAgencyKeyboard");
    expect(hook).toContain('event.key === "Escape"');
    expect(hook).toContain('event.key === "Enter"');
    expect(hook).toContain("focusConversationInput");
    expect(hook).toContain("isTextEntryTarget");
  });

  it("wires preview and delete confirm agency cards", () => {
    expect(preview).toContain("useAgencyKeyboard");
    expect(preview).toContain('data-agency-card="preview"');
    expect(resume).toContain("useAgencyKeyboard");
    expect(resume).toContain('data-agency-card="confirm_delete"');
  });

  it("keeps Conversation focus return id and Soft Send", () => {
    expect(operator).toContain('id="workspace-conversation-input"');
    expect(operator).toContain("data-voice-ready");
  });

  it("renders preview without expandHost so agency is reachable on App path", () => {
    expect(resume).toContain("!expandHost");
    expect(resume).toContain("ContinuePreviewBody");
  });
});
