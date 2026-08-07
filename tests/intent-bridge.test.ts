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

  it("refuses unknown desktop claims honestly without mechanical churn", () => {
    const unknown = resolveIntent("teleport my windows to Mars");
    expect(unknown.kind).toBe("unknown");
    expect(unknown.reply.toLowerCase()).toMatch(/window|can.?t|won.?t invent/);
    expect(unknown.suggestion).toBeTruthy();
    expect(unknown.suggestion?.toLowerCase()).toMatch(
      /what windows are open|bring chrome|snap/,
    );
    expect(unknown.reply.toLowerCase()).not.toMatch(
      /provider|runtime|winrt|kernel|don.?t have that yet/,
    );
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
      kind: "winFocus",
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
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
    expect(resolveIntent("Show me my open windows.").kind).toBe("winEnumerate");
    expect(resolveIntent("Which window is active?").kind).toBe("winActive");
    expect(resolveIntent("snap Chrome left")).toMatchObject({
      kind: "winSnap",
      query: "Chrome",
      snap: "left",
    });
    expect(resolveIntent("Move this window to the left.")).toMatchObject({
      kind: "winSnap",
      query: "this",
      snap: "left",
    });
    expect(resolveIntent("maximize notepad")).toMatchObject({
      kind: "winMaximize",
      query: "notepad",
    });
    expect(resolveIntent("Restore Chrome.")).toMatchObject({
      kind: "winRestore",
      query: "Chrome",
    });
    expect(resolveIntent("Bring Chrome to the front.")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    });
    expect(resolveIntent("move Cursor to monitor 1")).toMatchObject({
      kind: "winMoveMonitor",
      monitorIndex: 1,
    });
    expect(resolveIntent("Move Chrome to monitor two.")).toMatchObject({
      kind: "winMoveMonitor",
      query: "Chrome",
      monitorIndex: 2,
    });
    expect(resolveIntent("Move this window.").kind).toBe("unknown");
  });

  it("routes notifications provider operations", () => {
    expect(resolveIntent("Can you send notifications?").kind).toBe(
      "notifyStatus",
    );
    expect(resolveIntent("Can you send me a notification?").kind).toBe(
      "notifyStatus",
    );
    expect(resolveIntent("Show me a desktop notification.")).toMatchObject({
      kind: "notifyShow",
      text: "Notification from Workspace.",
    });
    expect(
      resolveIntent("Notify me that the build finished."),
    ).toMatchObject({
      kind: "notifyShow",
      text: "the build finished",
    });
    expect(resolveIntent("Dismiss that notification.").kind).toBe(
      "notifyDismiss",
    );
    expect(resolveIntent("Notify me when Cursor finishes.").kind).toBe(
      "unknown",
    );
  });

  it("routes browser provider operations", () => {
    expect(resolveIntent("Which browsers are available?").kind).toBe(
      "browserStatus",
    );
    expect(resolveIntent("Open ChatGPT.")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("Open Google.")).toMatchObject({
      kind: "browserOpen",
      url: "https://www.google.com",
    });
    expect(resolveIntent("Open ChatGPT beside Cursor.")).toMatchObject({
      kind: "browserOpenBeside",
      url: "https://chatgpt.com",
      beside: "Cursor",
    });
    expect(resolveIntent("Open this website.").kind).toBe("unknown");
    expect(resolveIntent("Open Notepad.").kind).not.toBe("browserOpen");
  });

  it("routes GPT / tab phrasing to browser — never gpt tab.exe", () => {
    for (const utterance of [
      "Open a new GPT tab",
      "Open GPT",
      "Open GPT in a new tab",
      "Open ChatGPT in a new tab",
      "open gpt",
    ]) {
      const action = resolveIntent(utterance);
      expect(action.kind, utterance).toBe("browserOpen");
      expect(action).toMatchObject({ url: "https://chatgpt.com" });
      expect(JSON.stringify(action).toLowerCase()).not.toMatch(/gpt tab\.exe/);
    }
    expect(resolveIntent("Open Chrome.")).toMatchObject({
      kind: "appOpen",
      query: "Chrome",
    });
    expect(resolveIntent("Open my recent browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("What can you do with browsers?").kind).toBe(
      "browserExplain",
    );
  });

  it("expands natural desktop intents for Product Proof remediation", () => {
    expect(resolveIntent("Open YouTube beside ChatGPT")).toMatchObject({
      kind: "browserOpenBeside",
      url: "https://www.youtube.com",
      beside: "ChatGPT",
    });
    expect(
      resolveIntent("Open ChatGPT in another browser window"),
    ).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
    expect(resolveIntent("Close YouTube")).toMatchObject({
      kind: "appClose",
      query: "YouTube",
    });
    expect(resolveIntent("Bring Chrome forward")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    });
    expect(resolveIntent("Open Settings")).toMatchObject({
      kind: "appLaunch",
      query: "ms-settings:",
    });
    expect(resolveIntent("Take a capture of our chat")).toMatchObject({
      kind: "screenshotWindow",
      query: "this",
    });
    expect(resolveIntent("Close this browser tab").kind).toBe("unknown");
    expect(resolveIntent("Minimize all applications").kind).toBe("unknown");
    expect(resolveIntent("Set speaker volume to 50%").kind).toBe("unknown");
    expect(resolveIntent("Transcribe this conversation").kind).toBe("unknown");
  });
});
