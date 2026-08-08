/**
 * C-OBS-003 — Desktop UI Tree (UIA control enumeration; observation only).
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";

describe("C-OBS-003 Desktop UI Tree", () => {
  it("lists controls in a named window", () => {
    const action = resolveIntent("What controls are in Notepad?");
    expect(action.kind).toBe("winEnumerateControls");
    if (action.kind !== "winEnumerateControls") return;
    expect(action.query.toLowerCase()).toContain("notepad");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "enumerate_controls",
      query: action.query,
    });
  });

  it("lists controls for the active window", () => {
    const action = resolveIntent("List controls in this window");
    expect(action.kind).toBe("winEnumerateControls");
    if (action.kind !== "winEnumerateControls") return;
    expect(action.query).toBe("this");
  });

  it("accepts desktop ui tree phrasing", () => {
    const action = resolveIntent("Show desktop ui tree in Notepad");
    expect(action.kind).toBe("winEnumerateControls");
  });

  it("does not claim click or type (observation only)", () => {
    const click = resolveIntent("Click the Save button in Notepad");
    expect(click.kind).not.toBe("winEnumerateControls");
  });

  it("leaves window enumeration on the enumerate path", () => {
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
  });
});
