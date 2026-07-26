/**
 * UI Experience import boundary tests (Sprint 134).
 */
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { auditUiExperienceBoundary } from "../scripts/ui-experience-boundary-lib.mjs";

const componentsDir = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  "../app/src/components",
);

describe("UI Experience import boundary", () => {
  it("normal UI surfaces do not bypass Experience translation imports", () => {
    const violations = auditUiExperienceBoundary(componentsDir);
    expect(violations).toEqual([]);
  });

  it("allows DisplayReasonList as the sole reasoning-type consumer in components", () => {
    const violations = auditUiExperienceBoundary(componentsDir);
    const reasonTypeViolations = violations.filter((v) =>
      v.includes("AttentionReason/DecisionReason"),
    );
    expect(reasonTypeViolations).toEqual([]);
  });

  it("diagnostic Operator surface is exempt from user-boundary rules", () => {
    const violations = auditUiExperienceBoundary(componentsDir);
    expect(violations.some((v) => v.startsWith("OperatorConsole.tsx"))).toBe(
      false,
    );
  });
});
