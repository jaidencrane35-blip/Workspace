import { describe, expect, it } from "vitest";
import {
  SHELL_EXITS,
  SHELL_TRANSITIONS,
  canTransition,
  verifyZeroTrapGraph,
} from "../app/src/lib/shellStateMachine";

describe("Zero-Trap two-form shell", () => {
  it("has no dead ends between Operator and Conversation", () => {
    const result = verifyZeroTrapGraph();
    expect(result.issues).toEqual([]);
    expect(result.ok).toBe(true);
  });

  it("allows Operator ↔ Conversation only", () => {
    expect(canTransition(0, 1)).toBe(true);
    expect(canTransition(1, 0)).toBe(true);
    expect(SHELL_TRANSITIONS[0]).toEqual([1]);
    expect(SHELL_TRANSITIONS[1]).toEqual([0]);
  });

  it("restores Conversation via click path, not Expand/Settings/Hide", () => {
    const labels = SHELL_EXITS[0].map((e) => e.label);
    expect(labels).toContain("Open Conversation");
    expect(labels).toContain("Exit Workspace");
    expect(labels).not.toContain("Expand Workspace");
    expect(labels).not.toContain("Settings");
    expect(labels).not.toContain("Hide");
    expect(SHELL_EXITS[0].some((e) => e.id === "click")).toBe(true);
  });

  it("exposes Collapse / Exit from Conversation", () => {
    const labels = SHELL_EXITS[1].map((e) => e.label);
    expect(labels).toContain("Collapse");
    expect(labels).toContain("Exit Workspace");
  });
});
