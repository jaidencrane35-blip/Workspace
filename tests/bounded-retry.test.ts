/**
 * C-VER-002 — Bounded Retry (Intent paths stay click/type; Operator owns retry).
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";
import { buildExecutionPlan } from "../app/src/lib/executionPlanner";

describe("C-VER-002 Bounded Retry", () => {
  it("keeps click intent on the authorized click path (no new goal)", () => {
    const action = resolveIntent("Click the Save button in Notepad");
    expect(action.kind).toBe("winClickControl");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "click_control",
    });
    const plan = buildExecutionPlan(
      "Click the Save button in Notepad",
      action,
    );
    expect(plan.capabilities).toContain("bounded-retry");
  });

  it("keeps type intent on the authorized type path", () => {
    const action = resolveIntent("Type hello into Edit in Notepad");
    expect(action.kind).toBe("winTypeControl");
    expect(toCapabilityIntent(action)).toMatchObject({
      domain: "window",
      operation: "type_control",
    });
    const plan = buildExecutionPlan("Type hello into Edit in Notepad", action);
    expect(plan.capabilities).toContain("bounded-retry");
  });

  it("keeps wait conditions intact", () => {
    expect(resolveIntent("Wait for Save in Notepad").kind).toBe(
      "winWaitCondition",
    );
  });

  it("does not invent a standalone retry operation from ordinary phrasing", () => {
    const action = resolveIntent("Click NoSuchControlZZZ in Notepad");
    expect(action.kind).toBe("winClickControl");
    if (action.kind !== "winClickControl") return;
    expect(action.control.toLowerCase()).toContain("nosuch");
  });
});
