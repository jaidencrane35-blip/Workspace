/**
 * Projection integrity — React/consumers must not invent lifecycle truth.
 */
import { describe, expect, it } from "vitest";
import {
  hasProjectedTerminalEvidence,
  isActiveRecommendation,
} from "../app/src/components/RecommendationExplanationView";
import {
  decisionOverlayHistoryRetentionLabel,
  isDecisionOverlayHistoryNonActionable,
} from "../app/src/components/decisionQueueProjection";
import { isDecisionArtifactHistoryNonActionable } from "../app/src/components/decisionEngineProjection";
import type {
  DecisionArtifactHistoryEntry,
  DecisionEngineSummary,
  DecisionOverlayHistoryEntry,
  DecisionQueueSummary,
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

describe("projection integrity — decision queue terminal overlay history", () => {
  function historyEntry(
    overrides: Partial<DecisionOverlayHistoryEntry> = {}
  ): DecisionOverlayHistoryEntry {
    return {
      decision_item_id: "decision:intent_proposal:x",
      source_type: "intent_proposal",
      source_id: "x",
      decision_state: "dismissed",
      orphaned: false,
      updated_at: "t1",
      actor_id: "local-user",
      actionable: false,
      authority_effect: "none",
      ...overrides,
    };
  }

  it("treats projected history as non-actionable (matches Rust)", () => {
    expect(isDecisionOverlayHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isDecisionOverlayHistoryNonActionable(
        historyEntry({ decision_state: "expired", orphaned: true })
      )
    ).toBe(true);
    expect(
      isDecisionOverlayHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);
    expect(
      isDecisionOverlayHistoryNonActionable(
        historyEntry({ decision_state: "pending" })
      )
    ).toBe(false);
  });

  it("distinguishes expired vs orphaned retention labels without inference", () => {
    expect(
      decisionOverlayHistoryRetentionLabel(
        historyEntry({ decision_state: "dismissed", orphaned: false })
      )
    ).toBe("dismissed");
    expect(
      decisionOverlayHistoryRetentionLabel(
        historyEntry({ decision_state: "dismissed", orphaned: true })
      )
    ).toBe("orphaned_dismissed");
    expect(
      decisionOverlayHistoryRetentionLabel(
        historyEntry({ decision_state: "expired", orphaned: true })
      )
    ).toBe("orphaned_expired");
  });

  it("IPC summary shape keeps history distinct from actionable items", () => {
    const summary: DecisionQueueSummary = {
      workspace_id: "ws",
      generated_at: "t",
      pending_count: 1,
      high_priority_count: 0,
      items: [
        {
          id: "decision:intent_proposal:live",
          workspace_id: "ws",
          source_type: "intent_proposal",
          source_id: "live",
          category: "planning",
          title: "Live",
          summary: "s",
          explanation: "e",
          recommended_action: "a",
          decision_state: "pending",
          priority: "normal",
          created_at: "t0",
          expires_at: null,
          actor_id: "local-user",
          actor_type: "user",
          project_id: null,
          required_capabilities: [],
          handoff_command: null,
          authority_effect: "none",
        },
      ],
      history: [
        historyEntry({
          source_id: "orphan",
          decision_state: "expired",
          orphaned: true,
        }),
        historyEntry({
          source_id: "gone",
          decision_state: "dismissed",
          orphaned: true,
        }),
      ],
      history_count: 2,
      authority_effect: "none",
    };

    expect(summary.items.every((i) => i.decision_state === "pending")).toBe(
      true
    );
    expect(summary.history.every(isDecisionOverlayHistoryNonActionable)).toBe(
      true
    );
    expect(
      summary.history.some((h) => h.orphaned && h.decision_state === "expired")
    ).toBe(true);
    expect(summary.history_count).toBe(2);
    expect(summary.items.some((i) => i.source_id === "orphan")).toBe(false);
  });

  it("treats history_count as authoritative over history.length window", () => {
    const summary: DecisionQueueSummary = {
      workspace_id: "ws",
      generated_at: "t",
      pending_count: 0,
      high_priority_count: 0,
      items: [],
      history: [historyEntry({ source_id: "window-only" })],
      history_count: 5,
      authority_effect: "none",
    };
    expect(summary.history.length).toBe(1);
    expect(summary.history_count).toBe(5);
    expect(summary.items.length === 0 && summary.history_count > 0).toBe(true);
  });

  it("does not convert history entries into DecisionItems", () => {
    const entry = historyEntry({ orphaned: true, decision_state: "expired" });
    expect(isDecisionOverlayHistoryNonActionable(entry)).toBe(true);
    expect(entry).not.toHaveProperty("recommended_action");
    expect(entry).not.toHaveProperty("handoff_command");
    expect(entry.actionable).toBe(false);
  });
});

describe("projection integrity — decision engine terminal artifact history", () => {
  function deHistory(
    overrides: Partial<DecisionArtifactHistoryEntry> = {}
  ): DecisionArtifactHistoryEntry {
    return {
      artifact_id: "engine_decision:attention:x",
      candidate_id: "engine_decision:attention:x",
      candidate_key: "attention:x",
      decision_state: "dismissed",
      created_at: "t0",
      updated_at: "t1",
      resolution_type: "dismissed",
      origin: "native",
      recommendation_id: null,
      intake_candidate_id: null,
      package_seal_digest: null,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    };
  }

  it("treats projected DE history as non-actionable", () => {
    expect(isDecisionArtifactHistoryNonActionable(deHistory())).toBe(true);
    expect(
      isDecisionArtifactHistoryNonActionable(
        deHistory({ decision_state: "expired", resolution_type: "expired" })
      )
    ).toBe(true);
    expect(
      isDecisionArtifactHistoryNonActionable(deHistory({ actionable: true }))
    ).toBe(false);
    expect(
      isDecisionArtifactHistoryNonActionable(
        deHistory({ decision_state: "open", terminal: false })
      )
    ).toBe(false);
  });

  it("uses history_count as authoritative evidence count", () => {
    const summary: DecisionEngineSummary = {
      workspace_id: "ws",
      generated_at: "t",
      candidate_count: 0,
      open_count: 0,
      top_candidates: [],
      history: [deHistory()],
      history_count: 4,
      summary: "none open",
      authority_effect: "none",
    };
    expect(summary.top_candidates.length).toBe(0);
    expect(summary.history.length).toBe(1);
    expect(summary.history_count).toBe(4);
    expect(summary.history.every(isDecisionArtifactHistoryNonActionable)).toBe(
      true
    );
  });

  it("does not convert DE history into DecisionCandidates", () => {
    const entry = deHistory({ decision_state: "selected" });
    expect(isDecisionArtifactHistoryNonActionable(entry)).toBe(true);
    expect(entry).not.toHaveProperty("goal_statement");
    expect(entry).not.toHaveProperty("handoff_command");
    expect(entry).not.toHaveProperty("score");
    expect(entry.actionable).toBe(false);
  });

  it("treats orphan overlay provenance as unknown — never invented native", () => {
    const orphan = deHistory({
      origin: "unknown",
      recommendation_id: null,
      intake_candidate_id: null,
      package_seal_digest: null,
    });
    expect(isDecisionArtifactHistoryNonActionable(orphan)).toBe(true);
    expect(orphan.origin).toBe("unknown");
    expect(orphan.origin).not.toBe("native");
  });
});
