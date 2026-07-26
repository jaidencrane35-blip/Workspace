/**
 * Experience translation trace contract tests (Sprint 135).
 */
import { describe, expect, it } from "vitest";
import {
  formatResolverPathLabel,
  resolveAttentionReason,
  resolveAttentionReasonTraced,
  resolveDecisionReasonTraced,
} from "../app/src/lib/experienceTranslation";
import type {
  AttentionReason,
  DecisionReason,
} from "../app/src/types/domain";

const attentionReason = (
  explanationKey: string,
  weight = 40,
): AttentionReason => ({
  source: "task_graph",
  signal: "blocked_task",
  weight,
  explanation_key: explanationKey,
});

describe("Experience translation traces", () => {
  it("does not mutate domain reasoning", () => {
    const reason = attentionReason("task.base.blocked", 40);
    const snapshot = structuredClone(reason);
    resolveAttentionReasonTraced(reason, "test");
    expect(reason).toEqual(snapshot);
  });

  it("produces deterministic traces for the same input", () => {
    const reason = attentionReason(
      "purpose.obstacle.composition:missing_application",
      36,
    );
    expect(resolveAttentionReasonTraced(reason, "operator")).toEqual(
      resolveAttentionReasonTraced(reason, "operator"),
    );
  });

  it("records unknown path safely", () => {
    const reason = attentionReason("future.debug.unknown", 7);
    const trace = resolveAttentionReasonTraced(reason);
    expect(trace.resolver_path.kind).toBe("unknown");
    expect(trace.resolver_path.match_key).toBe("unknown:future.debug.unknown");
    expect(trace.display.known).toBe(false);
    expect(trace.display.description).toContain("future.debug.unknown");
  });

  it("records resolver path correctly across tiers", () => {
    expect(
      resolveAttentionReasonTraced(
        attentionReason("decision.base.outstanding", 55),
      ).resolver_path,
    ).toEqual({
      kind: "exact",
      match_key: "exact:decision.base.outstanding",
    });

    expect(
      resolveAttentionReasonTraced(attentionReason("task.base.blocked")).resolver_path,
    ).toEqual({
      kind: "prefix_suffix",
      match_key: "prefix_suffix:task.base.blocked",
    });

    expect(
      resolveAttentionReasonTraced(
        attentionReason("purpose.obstacle.unlisted_kind", 30),
      ).resolver_path.match_key,
    ).toBe("prefix_fallback:purpose.obstacle.*");

    const decision: DecisionReason = {
      kind: "goal_alignment",
      summary: "Active goal still needs progress",
      evidence_ref: null,
      attention_reason: null,
    };
    expect(resolveDecisionReasonTraced(decision).resolver_path).toEqual({
      kind: "decision_native",
      match_key: "decision_native:decision.goal_alignment",
    });
  });

  it("DisplayReason matches trace.display", () => {
    const reason = attentionReason("continuity.interrupted", 60);
    const display = resolveAttentionReason(reason);
    const trace = resolveAttentionReasonTraced(reason);
    expect(trace.display).toEqual(display);
    expect(formatResolverPathLabel(trace.resolver_path)).toContain("exact:");
  });
});
