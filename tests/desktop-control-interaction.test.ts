/**
 * C-ACT-004 / C-ACT-005 — Desktop control interaction (click + type).
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";

describe("C-ACT-004 / C-ACT-005 Desktop Control Interaction", () => {
  it("clicks a named control", () => {
    const action = resolveIntent("Click the Save button in Notepad");
    expect(action.kind).toBe("winClickControl");
    if (action.kind !== "winClickControl") return;
    expect(action.control.toLowerCase()).toBe("save");
    expect(action.query.toLowerCase()).toContain("notepad");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "click_control",
      text: action.control,
      query: action.query,
    });
  });

  it("types into a named field", () => {
    const action = resolveIntent("Type hello into Edit in Notepad");
    expect(action.kind).toBe("winTypeControl");
    if (action.kind !== "winTypeControl") return;
    expect(action.text).toBe("hello");
    expect(action.control.toLowerCase()).toBe("edit");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "type_control",
      title: action.control,
      text: "hello",
    });
  });

  it("accepts quoted type phrasing", () => {
    const action = resolveIntent("Type 'test note' into Edit in Notepad");
    expect(action.kind).toBe("winTypeControl");
    if (action.kind !== "winTypeControl") return;
    expect(action.text).toBe("test note");
  });

  it("keeps discovery on find path", () => {
    expect(resolveIntent("Find the Save button in Notepad").kind).toBe(
      "winFindControl",
    );
  });

  it("keeps list controls on tree path", () => {
    expect(resolveIntent("What controls are in Notepad?").kind).toBe(
      "winEnumerateControls",
    );
  });
});
