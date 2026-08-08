/**
 * T1 — Companion Conversation Behavior.
 * Conversation voice + lightweight continuity (no capability execution changes).
 */
import { beforeEach, describe, expect, it } from "vitest";
import {
  companionGreetingReply,
  resetConversationGuidanceState,
  resolveUnknownGuidance,
} from "../app/src/lib/conversationGuidance";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

describe("T1 Companion Conversation Behavior", () => {
  beforeEach(() => {
    resetConversationGuidanceState();
    resetWorkspaceContext();
  });

  it("greets calmly without a feature catalogue", () => {
    const g = companionGreetingReply();
    expect(g.suggestion).toBeUndefined();
    expect(g.reply.toLowerCase()).not.toMatch(/try “|try "/);
    expect(resolveIntent("hey").suggestion).toBeUndefined();
  });

  it("keeps recovery truthful without invent/pretend chorus", () => {
    const samples = [
      "teleport my windows to mars",
      "organize my downloads folder",
      "run a powershell script",
      "mute the volume",
    ];
    for (const u of samples) {
      const action = resolveIntent(u);
      expect(action.kind).toBe("unknown");
      expect(action.reply.toLowerCase()).not.toMatch(
        /won.?t invent|won.?t pretend|won.?t fake/,
      );
    }
  });

  it("does not append the same recovery suggestion on consecutive soft misses", () => {
    const a = resolveUnknownGuidance("organize my downloads folder");
    const b = resolveUnknownGuidance("delete my documents folder");
    expect(a.suggestion).toBeTruthy();
    // Same near-miss family twice → rotate; no repeated suggestion line.
    expect(b.suggestion).toBeUndefined();
  });

  it("continues pronouns when session context already exists", () => {
    expect(resolveIntent("Open Notepad").kind).toMatch(/appOpen|appLaunch/);
    expect(resolveIntent("Close that")).toMatchObject({
      kind: "appClose",
      query: expect.stringMatching(/notepad/i),
    });
    expect(resolveIntent("Open it")).toMatchObject({
      kind: "appOpen",
      query: expect.stringMatching(/notepad/i),
    });
    expect(resolveIntent("Minimize that").kind).toBe("winMinimize");
    expect(resolveIntent("Do that again").kind).toBe("winMinimize");
  });

  it("preserves ordinary desktop actions", () => {
    expect(resolveIntent("Take a screenshot.").kind).toBe("screenshotDesktop");
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
    expect(resolveIntent("Open ChatGPT.")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
  });
});
