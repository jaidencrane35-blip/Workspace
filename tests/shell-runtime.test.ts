import { describe, expect, it } from "vitest";
import {
  COMPACT_SIZE,
  EXPANDED_SIZE,
  SHELL_MODE_LABEL,
  isShellMode,
} from "../app/src/lib/shellRuntime";
import { resolveIntent } from "../app/src/lib/intentBridge";

describe("shell runtime modes", () => {
  it("defines Modes 0–3 labels", () => {
    expect(SHELL_MODE_LABEL[0]).toMatch(/Desktop Operator/i);
    expect(SHELL_MODE_LABEL[1]).toMatch(/Compact/i);
    expect(SHELL_MODE_LABEL[2]).toMatch(/Expanded/i);
    expect(SHELL_MODE_LABEL[3]).toMatch(/Specialized/i);
    expect(isShellMode(0)).toBe(true);
    expect(isShellMode(4)).toBe(false);
  });

  it("keeps compact smaller than expanded", () => {
    expect(COMPACT_SIZE.width).toBeLessThan(EXPANDED_SIZE.width);
  });

  it("routes collapse / expand as shell intents", () => {
    expect(resolveIntent("collapse").kind).toBe("collapse");
    expect(resolveIntent("expand").kind).toBe("expand");
  });
});
