/**
 * P22.S1 — Intelligence Kind Routing & Reasoning Provider.
 */
import { beforeEach, describe, expect, it } from "vitest";
import {
  chatgptReasoningUrl,
  classifyIntelligenceKind,
  resolveIntelligenceRoute,
} from "../app/src/lib/intelligenceRouting";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { resetConversationGuidanceState } from "../app/src/lib/conversationGuidance";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

describe("P22.S1 Intelligence Kind Routing", () => {
  beforeEach(() => {
    resetConversationGuidanceState();
    resetWorkspaceContext();
  });

  it("answers arithmetic locally (not desktop refusal)", () => {
    expect(classifyIntelligenceKind("what is 2 + 2")).toBe("REASONING_LOCAL");
    const action = resolveIntent("what is 2 + 2");
    expect(action.kind).toBe("unknown");
    expect(action.reply).toBe("4");
    expect(action.reply.toLowerCase()).not.toMatch(/outside|desktop refusal|not set up/);
  });

  it("answers percentages and unit conversions locally", () => {
    expect(resolveIntent("what is 15% of 80").reply).toBe("12");
    expect(resolveIntent("convert 100 c to f").reply).toMatch(/212/);
    expect(resolveIntent("10 km to miles").reply).toMatch(/6\.21371/);
  });

  it("answers local time/date; clarifies ambiguous WA", () => {
    const local = resolveIntent("what time is it");
    expect(local.kind).toBe("unknown");
    expect(local.reply.toLowerCase()).toMatch(/it’?s|its/);
    expect(local.reply.toLowerCase()).not.toMatch(/outside what i can do on the desktop/);

    const wa = resolveIntent("What time is it in WA?");
    expect(classifyIntelligenceKind("What time is it in WA?")).toBe(
      "CLARIFICATION",
    );
    expect(wa.reply.toLowerCase()).toMatch(/western australia|washington/);

    const perth = resolveIntent("what time is it in Perth");
    expect(perth.reply.toLowerCase()).toMatch(/perth/);
  });

  it("explains desktop capabilities via Registry", () => {
    const action = resolveIntent("Can you manage windows?");
    expect(action.kind).toBe("capabilityExplain");
    expect(action.reply.toLowerCase()).toMatch(/window/);
    expect(action.reply.toLowerCase()).not.toMatch(
      /outside what i can do on the desktop/,
    );
  });

  it("hands world knowledge to ChatGPT without inventing", () => {
    expect(
      classifyIntelligenceKind("How long from Rockhampton to Gladstone?"),
    ).toBe("REASONING_PROVIDER");
    const action = resolveIntent("How long from Rockhampton to Gladstone?");
    expect(action.kind).toBe("browserOpen");
    if (action.kind !== "browserOpen") throw new Error("expected browserOpen");
    expect(action.url).toBe(
      chatgptReasoningUrl("How long from Rockhampton to Gladstone?"),
    );
    expect(action.reply.toLowerCase()).toMatch(/chatgpt/);
    expect(action.reply.toLowerCase()).toMatch(/won.?t invent/);
  });

  it("keeps desktop commands unchanged", () => {
    expect(resolveIntent("Open Notepad").kind).toMatch(/appOpen|appLaunch/);
    expect(resolveIntent("Take a screenshot.").kind).toBe("screenshotDesktop");
    expect(resolveIntent("Open ChatGPT.")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
  });

  it("preserves invent-exe and file walls", () => {
    const invent = resolveIntent("Open foobarbazqux");
    expect(invent.kind).toBe("unknown");
    expect(invent.reply.toLowerCase()).toMatch(/don.?t recognize|can.?t/);

    const files = resolveIntent("organize my downloads folder");
    expect(files.kind).toBe("unknown");
    expect(files.reply.toLowerCase()).toMatch(/files and folders/);
    expect(classifyIntelligenceKind("organize my downloads folder")).toBe(
      "CAPABILITY_LIMIT",
    );
  });

  it("supports hybrid local math then open", () => {
    expect(
      classifyIntelligenceKind("calculate 15% of 80 then open Calculator"),
    ).toBe("HYBRID");
    const action = resolveIntent("calculate 15% of 80 then open Calculator");
    expect(action.kind).toBe("appOpen");
    if (action.kind !== "appOpen") throw new Error("expected appOpen");
    expect(action.query.toLowerCase()).toMatch(/calculator/);
    expect(action.reply).toMatch(/^12\b/);
  });

  it("resolveIntelligenceRoute never fabricates knowledge answers", () => {
    const route = resolveIntelligenceRoute("Who wrote Hamlet?");
    expect(route?.kind).toBe("browserOpen");
    expect(route && "reply" in route ? route.reply : "").not.toMatch(/shakespeare/i);
  });
});
