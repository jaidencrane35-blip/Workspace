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
    expect(softenUtterance("Would you open Chrome for me?")).toBe(
      "open Chrome",
    );
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
    expect(resolveIntent("Open a new GPT tab")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("Open my browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("Open another browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("Open a new browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("Open a browser beside Cursor.")).toMatchObject({
      kind: "browserOpenBeside",
      beside: "Cursor",
    });
    expect(resolveIntent("Open Git.")).toMatchObject({
      kind: "browserOpen",
      url: "https://github.com",
    });
    expect(resolveIntent("Open GitHub beside Cursor.")).toMatchObject({
      kind: "browserOpenBeside",
      beside: "Cursor",
    });
    expect(resolveIntent("Take a screenshot and copy it.").kind).toBe(
      "screenshotCaptureAndCopy",
    );
    expect(resolveIntent("Capture this window.").kind).toBe("screenshotWindow");
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
    expect(resolveIntent("Bring Chrome to the front.")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    });
    expect(resolveIntent("Bring Cursor forward.")).toMatchObject({
      kind: "winFocus",
      query: "Cursor",
    });
    expect(resolveIntent("Put Chrome in front.")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    }); // Semantic entity focusQuery for Chrome
    expect(resolveIntent("Would you open Chrome for me?")).toMatchObject({
      kind: "appOpen",
      query: "Google Chrome",
    });
    expect(resolveIntent("Open Edge.")).toMatchObject({
      kind: "appOpen",
      query: "Microsoft Edge",
    });
    expect(resolveIntent("Launch GPT")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("What can you do for me?")).toMatchObject({
      kind: "capabilityExplain",
    });
    expect(resolveIntent("Please what can you do")).toMatchObject({
      kind: "capabilityExplain",
    });
    expect(resolveIntent("Please open Settings")).toMatchObject({
      kind: "appLaunch",
      query: "ms-settings:",
    });
    expect(resolveIntent("Open Chrome browser.")).toMatchObject({
      kind: "appOpen",
      query: "Google Chrome",
    });
    expect(resolveIntent("Launch browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("Can you hear me?").kind).toBe("voiceStatus");
    expect(resolveIntent("Could you hear me?").kind).toBe("voiceStatus");
    expect(resolveIntent("Would you hear me?").kind).toBe("voiceStatus");
    expect(resolveIntent("Do you hear me?").kind).toBe("voiceStatus");
    expect(resolveIntent("Are you listening?").kind).toBe("voiceStatus");
    expect(resolveIntent("Open GPT")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("Open YouTube beside ChatGPT.")).toMatchObject({
      kind: "browserOpenBeside",
    });
    expect(resolveIntent("Open YouTube beside Cursor.")).toMatchObject({
      kind: "browserOpenBeside",
      beside: "Cursor",
    });
  });

  it("does not send casual how-to chat into Guide", () => {
    const pasta = resolveIntent("how do I cook pasta");
    expect(pasta.kind).toBe("unknown");
    expect(pasta).not.toMatchObject({ view: "help" });
  });
});
