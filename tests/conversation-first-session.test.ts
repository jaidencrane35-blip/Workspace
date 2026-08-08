import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P20.S1 Empty Conversation first-session cue", () => {
  const operator = readFileSync(
    path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
    "utf8",
  );
  const voice = readFileSync(
    path.join(root, "app/src/components/operator/VoiceMicButton.tsx"),
    "utf8",
  );

  it("invites action only on genuinely empty Conversation", () => {
    expect(operator).toContain("showFirstSessionCue");
    expect(operator).toContain("data-first-session");
    expect(operator).toContain("messages.length === 0");
    expect(operator).toMatch(/Say or type what you need/);
  });

  it("presents a calm composer placeholder", () => {
    expect(operator).not.toMatch(/placeholder=""/);
    expect(operator).toMatch(/COMPOSER_PLACEHOLDER|Say or type…/);
  });

  it("suppresses the cue during busy, Moments, voice, and draft", () => {
    expect(operator).toContain("!composerBusy");
    expect(operator).toContain("momentsActive");
    expect(operator).toContain("!voiceCapturing");
    expect(operator).toContain("!draft.trim()");
    expect(voice).toContain("onCaptureActiveChange");
  });

  it("does not introduce onboarding or catalogues", () => {
    expect(operator).not.toMatch(/OnboardingWizard|CapabilityCatalog|TutorialModal/);
    expect(operator).toContain('data-gravity="conversation"');
    expect(operator).toContain("WORKING_ACK");
  });
});
