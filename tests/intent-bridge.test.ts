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
      query: "Notepad",
    });
    expect(resolveIntent("launch chrome")).toMatchObject({
      kind: "appOpen",
      query: "Google Chrome",
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
      query: "Notepad",
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
    expect(resolveIntent("Open a browser beside Cursor.")).toMatchObject({
      kind: "browserOpenBeside",
      url: "https://www.google.com",
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
      "Please open GPT",
      "Launch GPT",
      "Start YT",
      "Can you open Git for me",
    ]) {
      const action = resolveIntent(utterance);
      expect(action.kind, utterance).toBe("browserOpen");
      expect(JSON.stringify(action).toLowerCase()).not.toMatch(/gpt tab\.exe/);
      expect(action.kind === "browserOpen" && "url" in action ? action.url : "").toMatch(
        /chatgpt\.com|youtube\.com|github\.com/,
      );
    }
    expect(resolveIntent("Open Chrome.")).toMatchObject({
      kind: "appOpen",
      query: "Google Chrome",
    });
    expect(resolveIntent("Open Git")).toMatchObject({
      kind: "browserOpen",
      url: "https://github.com",
    });
    expect(resolveIntent("Open YT")).toMatchObject({
      kind: "browserOpen",
      url: "https://www.youtube.com",
    });
    expect(resolveIntent("Open VSCode")).toMatchObject({
      kind: "appOpen",
      query: "Visual Studio Code",
    });
    expect(resolveIntent("Open YouTube beside GPT")).toMatchObject({
      kind: "browserOpenBeside",
      url: "https://www.youtube.com",
      beside: "ChatGPT",
    });
    expect(resolveIntent("Open my recent browser.")).toMatchObject({
      kind: "browserOpen",
    });
    expect(resolveIntent("What can you do with browsers?").kind).toBe(
      "browserExplain",
    );
  });

  it("P16.31 Semantic Intent Engine — Store, window compose, discovery", () => {
    expect(resolveIntent("Open Microsoft Store")).toMatchObject({
      kind: "appLaunch",
      query: "ms-windows-store:",
    });
    expect(resolveIntent("Bring GPT to the front")).toMatchObject({
      kind: "winFocus",
      query: "ChatGPT",
    });
    expect(
      resolveIntent("Locate the browser with YouTube open"),
    ).toMatchObject({
      kind: "winFocus",
      query: "YouTube",
    });
    expect(
      resolveIntent(
        "Locate the application with ChatGPT open and minimise it",
      ),
    ).toMatchObject({
      kind: "winFocusMinimize",
      query: "ChatGPT",
    });
    expect(resolveIntent("Restore Cursor")).toMatchObject({
      kind: "winRestore",
      query: "Cursor",
    });
    expect(resolveIntent("Maximise Cursor")).toMatchObject({
      kind: "winMaximize",
      query: "Cursor",
    });
    expect(resolveIntent("Focus Chrome")).toMatchObject({
      kind: "winFocus",
      query: "Chrome",
    });
    expect(resolveIntent("Focus Edge")).toMatchObject({
      kind: "winFocus",
      query: "Edge",
    });
    expect(resolveIntent("What can you do?")).toMatchObject({
      kind: "capabilityExplain",
    });
    expect(resolveIntent("Show me your capabilities")).toMatchObject({
      kind: "capabilityExplain",
    });
    expect(resolveIntent("List desktop actions")).toMatchObject({
      kind: "capabilityExplain",
    });
    const discovery = resolveIntent("What can you do?");
    expect(discovery.kind).toBe("capabilityExplain");
    if (discovery.kind === "capabilityExplain") {
      expect(discovery.reply.toLowerCase()).toMatch(/applications|windows|browser/);
      expect(discovery.reply.toLowerCase()).not.toMatch(/provider|registry|kernel/);
    }
  });

  it("P16.30 Intent Grammar — F11 compounds never become executable names", () => {
    expect(resolveIntent("Open GPT and bring to the front")).toMatchObject({
      kind: "browserOpenFocus",
      url: "https://chatgpt.com",
      focusQuery: "ChatGPT",
    });
    expect(
      resolveIntent("Open GPT and bring it to the front"),
    ).toMatchObject({
      kind: "browserOpenFocus",
      url: "https://chatgpt.com",
      focusQuery: "ChatGPT",
    });
    expect(resolveIntent("Open Cursor to full size")).toMatchObject({
      kind: "appOpenMaximize",
      query: "Cursor",
    });
    expect(
      resolveIntent("Locate the application with ChatGPT"),
    ).toMatchObject({
      kind: "winFocus",
      query: "ChatGPT",
    });
    expect(
      resolveIntent("Open File Explorer and locate Pictures"),
    ).toMatchObject({
      kind: "appLaunch",
      query: "shell:My Pictures",
    });
    for (const utterance of [
      "Open GPT and bring to the front",
      "Open Cursor to full size",
      "Locate the application with ChatGPT",
      "Open File Explorer and locate Pictures",
    ]) {
      const action = resolveIntent(utterance);
      expect(action.kind, utterance).not.toBe("unknown");
      expect(action.kind, utterance).not.toBe("appOpen");
      // Executable / launch query must never be the raw compound transcript.
      if ("query" in action && typeof action.query === "string") {
        expect(action.query.toLowerCase(), utterance).not.toMatch(
          / and |to full|locate pictures|application with/i,
        );
      }
    }
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
    expect(resolveIntent("Bring Cursor forward")).toMatchObject({
      kind: "winFocus",
      query: "Cursor",
    });
    expect(resolveIntent("Maximize Cursor")).toMatchObject({
      kind: "winMaximize",
      query: "Cursor",
    });
    expect(resolveIntent("Close Settings")).toMatchObject({
      kind: "appClose",
      query: "Windows Settings",
    });
    expect(resolveIntent("Open Settings")).toMatchObject({
      kind: "appLaunch",
      query: "ms-settings:",
    });
    expect(resolveIntent("Can you hear me?").kind).toBe("voiceStatus");
    expect(resolveIntent("What can you do with voice?").kind).toBe(
      "voiceExplain",
    );
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
