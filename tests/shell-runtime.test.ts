import { describe, expect, it } from "vitest";
import {
  CONVERSATION_SIZE,
  OPERATOR_SIZE,
  SHELL_MODE_LABEL,
  isShellMode,
  normalizeShellMode,
} from "../app/src/lib/shellRuntime";
import { resolveIntent } from "../app/src/lib/intentBridge";

describe("shell runtime two-form model", () => {
  it("defines Form A Operator and Form B Conversation only", () => {
    expect(SHELL_MODE_LABEL[0]).toMatch(/Desktop Operator/i);
    expect(SHELL_MODE_LABEL[1]).toMatch(/Conversation/i);
    expect(isShellMode(0)).toBe(true);
    expect(isShellMode(1)).toBe(true);
    expect(isShellMode(2)).toBe(false);
    expect(isShellMode(3)).toBe(false);
  });

  it("migrates legacy expanded/specialized modes to Conversation", () => {
    expect(normalizeShellMode(2)).toBe(1);
    expect(normalizeShellMode(3)).toBe(1);
    expect(normalizeShellMode(0)).toBe(0);
  });

  it("keeps operator tiny relative to conversation default", () => {
    expect(OPERATOR_SIZE.width).toBeLessThan(CONVERSATION_SIZE.width);
  });

  it("routes collapse / expand intents without inventing shell forms", () => {
    expect(resolveIntent("collapse").kind).toBe("collapse");
    expect(resolveIntent("expand").kind).toBe("expand");
  });
});
