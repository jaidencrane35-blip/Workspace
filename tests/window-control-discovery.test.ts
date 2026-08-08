/**
 * C-OBS-004 — Window Control Discovery (locate named control; observation only).
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";

describe("C-OBS-004 Window Control Discovery", () => {
  it("finds a named control in a window", () => {
    const action = resolveIntent("Find the Save button in Notepad");
    expect(action.kind).toBe("winFindControl");
    if (action.kind !== "winFindControl") return;
    expect(action.control.toLowerCase()).toBe("save");
    expect(action.query.toLowerCase()).toContain("notepad");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "find_control",
      text: action.control,
      query: action.query,
    });
  });

  it("answers where-is phrasing", () => {
    const action = resolveIntent("Where is Edit in Notepad?");
    expect(action.kind).toBe("winFindControl");
    if (action.kind !== "winFindControl") return;
    expect(action.control.toLowerCase()).toBe("edit");
  });

  it("answers does-have phrasing", () => {
    const action = resolveIntent("Does Notepad have a File menu?");
    expect(action.kind).toBe("winFindControl");
    if (action.kind !== "winFindControl") return;
    expect(action.control.toLowerCase()).toBe("file");
    expect(action.query.toLowerCase()).toContain("notepad");
  });

  it("does not claim click or type", () => {
    const click = resolveIntent("Click the Save button in Notepad");
    expect(click.kind).not.toBe("winFindControl");
  });

  it("leaves full control list on C-OBS-003 path", () => {
    expect(resolveIntent("What controls are in Notepad?").kind).toBe(
      "winEnumerateControls",
    );
  });
});
