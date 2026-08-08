/**
 * C-VER-003 — Wait Conditions (Intent + composition wiring).
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";

describe("C-VER-003 Wait Conditions", () => {
  it("waits for a control to become available", () => {
    const action = resolveIntent("Wait for Save in Notepad");
    expect(action.kind).toBe("winWaitCondition");
    if (action.kind !== "winWaitCondition") return;
    expect(action.condition).toBe("control_available");
    expect(action.control?.toLowerCase()).toBe("save");
    expect(action.query.toLowerCase()).toContain("notepad");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "wait_condition",
      category: "control_available",
      text: action.control,
      query: action.query,
    });
  });

  it("accepts appear phrasing", () => {
    const action = resolveIntent("Wait until Save appears in Notepad");
    expect(action.kind).toBe("winWaitCondition");
    if (action.kind !== "winWaitCondition") return;
    expect(action.condition).toBe("control_available");
  });

  it("waits for a control to disappear", () => {
    const action = resolveIntent("Wait until Save disappears in Notepad");
    expect(action.kind).toBe("winWaitCondition");
    if (action.kind !== "winWaitCondition") return;
    expect(action.condition).toBe("control_gone");
  });

  it("waits for a window to become active", () => {
    const action = resolveIntent("Wait until Notepad is active");
    expect(action.kind).toBe("winWaitCondition");
    if (action.kind !== "winWaitCondition") return;
    expect(action.condition).toBe("window_active");
    expect(action.query.toLowerCase()).toContain("notepad");
  });

  it("keeps click/type paths intact", () => {
    expect(resolveIntent("Click the Save button in Notepad").kind).toBe(
      "winClickControl",
    );
    expect(resolveIntent("Type hello into Edit in Notepad").kind).toBe(
      "winTypeControl",
    );
  });
});
