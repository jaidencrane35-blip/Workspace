import { describe, expect, it } from "vitest";
import {
  SHELL_EXITS,
  SHELL_TRANSITIONS,
  canTransition,
  verifyZeroTrapGraph,
} from "../app/src/lib/shellStateMachine";

describe("Zero-Trap shell state machine", () => {
  it("has no dead ends and is fully reachable from Compact", () => {
    const result = verifyZeroTrapGraph();
    expect(result.issues).toEqual([]);
    expect(result.ok).toBe(true);
  });

  it("allows floating ↔ compact ↔ expand recovery paths", () => {
    expect(canTransition(0, 1)).toBe(true);
    expect(canTransition(0, 2)).toBe(true);
    expect(canTransition(1, 0)).toBe(true);
    expect(canTransition(2, 0)).toBe(true);
    expect(canTransition(2, 1)).toBe(true);
    expect(canTransition(1, 2)).toBe(true);
  });

  it("exposes Open / Expand / Hide / Exit from floating mode", () => {
    const labels = SHELL_EXITS[0].map((e) => e.label);
    expect(labels).toContain("Open Workspace");
    expect(labels).toContain("Expand Workspace");
    expect(labels).toContain("Hide");
    expect(labels).toContain("Exit Workspace");
  });

  it("keeps specialized mode recoverable", () => {
    expect(SHELL_TRANSITIONS[3]).toEqual(expect.arrayContaining([0, 1, 2]));
  });
});
