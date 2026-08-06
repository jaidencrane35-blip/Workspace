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

  it("routes application provider operations", () => {
    expect(resolveIntent("open notepad")).toMatchObject({
      kind: "appOpen",
      query: "notepad",
    });
    expect(resolveIntent("launch chrome")).toMatchObject({
      kind: "appLaunch",
      query: "chrome",
    });
    expect(resolveIntent("switch to Chrome")).toMatchObject({
      kind: "appFocus",
      query: "Chrome",
    });
    expect(resolveIntent("close Spotify")).toMatchObject({
      kind: "appClose",
      query: "Spotify",
    });
    expect(resolveIntent("list apps").kind).toBe("appEnumerate");
  });

  it("routes window provider operations", () => {
    expect(resolveIntent("list windows").kind).toBe("winEnumerate");
    expect(resolveIntent("snap Chrome left")).toMatchObject({
      kind: "winSnap",
      query: "Chrome",
      snap: "left",
    });
    expect(resolveIntent("maximize notepad")).toMatchObject({
      kind: "winMaximize",
      query: "notepad",
    });
    expect(resolveIntent("move Cursor to monitor 1")).toMatchObject({
      kind: "winMoveMonitor",
      monitorIndex: 1,
    });
  });
});
