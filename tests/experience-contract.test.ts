/**
 * Experience translation contract tests (Sprint 133).
 */
import { describe, expect, it } from "vitest";
import {
  resolveAttentionReason,
  resolveAttentionReasons,
  resolveDecisionReason,
  resolveDecisionReasons,
} from "../app/src/lib/experienceTranslation";
import type { DecisionReason } from "../app/src/types/domain";
import { attentionReason } from "./fixtures/attentionReason";

describe("Experience translation contract", () => {
  it("produces deterministic DisplayReason output", () => {
    const reason = attentionReason("task.base.blocked");
    expect(resolveAttentionReason(reason)).toEqual(
      resolveAttentionReason(reason),
    );
  });

  it("preserves structured identity on DisplayReason", () => {
    const reasons = [
      attentionReason("decision.base.outstanding", 55),
      attentionReason("continuity.interrupted", 60),
    ];
    const displayed = resolveAttentionReasons(reasons);
    expect(displayed[0].explanation_key).toBe("decision.base.outstanding");
    expect(displayed[0].weight).toBe(55);
    expect(displayed[1].signal).toBe("blocked_task");
  });

  it("degrades unknown meaning safely", () => {
    const display = resolveAttentionReason(
      attentionReason("future.contract.unknown", 33),
    );
    expect(display.known).toBe(false);
    expect(display.description).toContain("future.contract.unknown");
    expect(display.title).toBe("Blocked task needs attention");
  });

  it("routes Decision reasons through Attention when present", () => {
    const reason: DecisionReason = {
      kind: "attention",
      summary: "ignored",
      evidence_ref: null,
      attention_reason: attentionReason("decision.priority.high", 12),
    };
    const display = resolveDecisionReason(reason);
    expect(display.title).toBe("High-priority decision raises focus");
    expect(display.explanation_key).toBe("decision.priority.high");
  });

  it("translates Decision-native reasons without Attention keys", () => {
    const reason: DecisionReason = {
      kind: "goal_alignment",
      summary: "Active goal still needs progress",
      evidence_ref: "goal:1",
      attention_reason: null,
    };
    const displays = resolveDecisionReasons([reason]);
    expect(displays[0].title).toBe("Active goal still needs progress");
    expect(displays[0].description).toBe("goal_alignment");
    expect(displays[0].known).toBe(true);
  });
});
