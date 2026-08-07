import { beforeEach, describe, expect, it } from "vitest";
import {
  isVoiceCheckUtterance,
  resetConversationGuidanceState,
  resolveUnknownGuidance,
  softenUtterance,
} from "../app/src/lib/conversationGuidance";
import { resolveIntent } from "../app/src/lib/intentBridge";

describe("conversation quality (P16.6)", () => {
  beforeEach(() => {
    resetConversationGuidanceState();
  });

  it("softens polite wrappers without AI guessing", () => {
    expect(softenUtterance("could you please take a screenshot")).toBe(
      "take a screenshot",
    );
    expect(softenUtterance("can you open chatgpt")).toBe("open chatgpt");
  });

  it("treats mic-check phrases as heard, not desktop failure", () => {
    expect(isVoiceCheckUtterance("testing testing 123")).toBe(true);
    expect(
      isVoiceCheckUtterance("sally sells seashells by the seashore"),
    ).toBe(true);

    const a = resolveIntent("Testing testing 123");
    expect(a.kind).toBe("unknown");
    expect(a.reply.toLowerCase()).toMatch(/heard you/);
    expect(a.reply.toLowerCase()).not.toMatch(/don.?t have that yet/);

    const b = resolveIntent("Sally sells seashells by the seashore");
    expect(b.reply.toLowerCase()).toMatch(/heard you/);
  });

  it("rotates unsupported replies instead of identical churn", () => {
    const first = resolveUnknownGuidance("teleport my windows to mars");
    const second = resolveUnknownGuidance("invent a flying car");
    expect(first.reply).not.toBe(second.reply);
    expect(first.reply.toLowerCase()).not.toMatch(/don.?t have that yet/);
    expect(second.suggestion).toBeTruthy();
  });

  it("guides near-miss desktop topics truthfully", () => {
    const files = resolveIntent("please organize my downloads folder");
    expect(files.kind).toBe("unknown");
    expect(files.reply.toLowerCase()).toMatch(/files?|folders?/);
    expect(files.suggestion?.toLowerCase()).toMatch(/open|windows|screenshot/);

    const chat = resolveIntent("what's the weather in paris today");
    expect(chat.kind).toBe("unknown");
    expect(chat.reply.toLowerCase()).toMatch(/desktop/);
    expect(chat.reply.toLowerCase()).not.toMatch(/provider|runtime|winrt/);
  });

  it("accepts ordinary wording for Product Proof voice paths", () => {
    expect(resolveIntent("Could you take a screenshot?").kind).toBe(
      "screenshotDesktop",
    );
    expect(resolveIntent("Please open ChatGPT.")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("Open my browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("Open GitHub beside Cursor.")).toMatchObject({
      kind: "browserOpenBeside",
      beside: "Cursor",
    });
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
    expect(resolveIntent("Bring Chrome to the front.")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    });
    expect(resolveIntent("Put Chrome in front.")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    });
  });

  it("does not send casual how-to chat into Guide", () => {
    const pasta = resolveIntent("how do I cook pasta");
    expect(pasta.kind).toBe("unknown");
    expect(pasta).not.toMatchObject({ view: "help" });
  });
});
