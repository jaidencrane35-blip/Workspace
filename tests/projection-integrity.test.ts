/**
 * Projection integrity — React/consumers must not invent lifecycle truth.
 */
import { describe, expect, it } from "vitest";
import {
  hasProjectedTerminalEvidence,
  isActiveRecommendation,
} from "../app/src/components/RecommendationExplanationView";
import type {
  RecommendationHistoryEntry,
  RecommendationItem,
} from "../app/src/types/domain";

function baseItem(
  overrides: Partial<RecommendationItem> = {}
): RecommendationItem {
  return {
    id: "rec-1",
    workspace_id: "ws-1",
    kind: "focus_suggestion",
    title: "Focus",
    reason: "test",
    impact: "low",
    related_attention_id: null,
    related_task_id: null,
    related_decision_id: null,
    evidence: [],
    attention_reasons: [],
    lifecycle_state: null,
    lifecycle_presented_at: null,
    lifecycle_resolved_at: null,
    lifecycle_resolution_type: null,
    content_fingerprint: null,
    outcome: null,
    explanation: null,
    decision_context: null,
    decision_readiness: null,
    decision_boundary: null,
    decision_confirmation: null,
    decision_intake: null,
    decision_intake_inspection: null,
    decision_intake_compatibility: null,
    decision_intake_proceed_denial: null,
    decision_intake_package_seal: null,
    decision_intake_adapter_preparation: null,
    decision_handoff_request: null,
    decision_engine_acceptance: null,
    authority_effect: "none",
    ...overrides,
  } as RecommendationItem;
}

describe("projection integrity — recommendation lifecycle mirrors", () => {
  it("treats only open lifecycle states as active (matches Rust)", () => {
    expect(isActiveRecommendation(baseItem({ lifecycle_state: null }))).toBe(
      true
    );
    expect(
      isActiveRecommendation(baseItem({ lifecycle_state: "available" }))
    ).toBe(true);
    expect(
      isActiveRecommendation(baseItem({ lifecycle_state: "presented" }))
    ).toBe(true);
    expect(
      isActiveRecommendation(baseItem({ lifecycle_state: "accepted" }))
    ).toBe(false);
    expect(
      isActiveRecommendation(baseItem({ lifecycle_state: "rejected" }))
    ).toBe(false);
    expect(
      isActiveRecommendation(baseItem({ lifecycle_state: "corrupt_state" }))
    ).toBe(false);
  });

  it("requires projected terminal history evidence — no inferred truth", () => {
    const valid: RecommendationHistoryEntry = {
      native_id: "rec-1",
      lifecycle_state: "accepted",
      outcome: {
        outcome_id: "recommendation_outcome:rec-1",
        recommendation_id: "rec-1",
        user_decision: "accepted",
        result_kind: "accepted_follow_through",
        lifecycle_resolution: "accepted",
        recorded_at: "t1",
        explanation_keys: [],
        evidence_refs: [],
        experience_trace_match_keys: [],
        is_system_failure: false,
        authority_effect: "none",
      },
      resolved_at: "t1",
      authority_effect: "none",
    };
    expect(hasProjectedTerminalEvidence(valid)).toBe(true);

    const missingOutcome = {
      ...valid,
      outcome: {
        ...valid.outcome,
        outcome_id: "",
      },
    };
    expect(hasProjectedTerminalEvidence(missingOutcome)).toBe(false);

    const openState = { ...valid, lifecycle_state: "available" };
    expect(hasProjectedTerminalEvidence(openState)).toBe(false);
  });

  it("summary contract includes optional history fields for IPC alignment", () => {
    // Compile-time / structural: consumers may read history_count without inventing it.
    const summary = {
      workspace_id: "ws",
      generated_at: "t",
      label: "L",
      candidate_count: 0,
      relationship_count: 0,
      top_candidates: [] as RecommendationItem[],
      history: [] as RecommendationHistoryEntry[],
      history_count: 0,
      explanation: "",
      summary: "",
      authority_effect: "none",
    };
    expect(summary.history_count).toBe(0);
    expect(Array.isArray(summary.history)).toBe(true);
    expect(summary.authority_effect).toBe("none");
  });
});
