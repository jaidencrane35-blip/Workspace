import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";

describe("intent bridge", () => {
  it("routes save / continue / expand without inventing capabilities", () => {
    expect(resolveIntent("Save this").kind).toBe("navigate");
    expect(resolveIntent("Save this")).toMatchObject({ view: "save" });
    expect(resolveIntent("Continue yesterday")).toMatchObject({
      kind: "navigate",
      view: "resume",
    });
    expect(resolveIntent("expand").kind).toBe("expand");
    expect(resolveIntent("collapse").kind).toBe("collapse");
  });

  it("refuses unknown desktop claims honestly", () => {
    const unknown = resolveIntent("teleport my windows to Mars");
    expect(unknown.kind).toBe("unknown");
    expect(unknown.reply.toLowerCase()).toMatch(/don.t have that yet/);
  });

  it("opens repository health as a secondary surface", () => {
    expect(resolveIntent("repository health").kind).toBe("health");
  });

  it("routes clipboard intents through Capability Runtime kinds", () => {
    expect(resolveIntent("what's on my clipboard")).toMatchObject({
      kind: "clipboardRead",
    });
    expect(resolveIntent("copy to clipboard: hello p10")).toMatchObject({
      kind: "clipboardWrite",
      text: "hello p10",
    });
  });
});
