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
import { isTaskHistoryNonActionable } from "../app/src/components/taskGraphProjection";
import { isExecutionHistoryNonActionable } from "../app/src/components/executionLifecycleProjection";
import {
  isPlanningHistoryNonActionable,
  isPlanningSnapshotNonCommandable,
  planningHistoryCountIsAuthoritative,
} from "../app/src/components/planningProjection";
import type {
  DecisionArtifactHistoryEntry,
  DecisionEngineSummary,
  DecisionOverlayHistoryEntry,
  DecisionQueueSummary,
  ExecutionLifecycleHistoryEntry,
  ExecutionLifecycleProjection,
  PlanningHistoryEntry,
  PlanningSnapshot,
  PlanningSummary,
  RecommendationHistoryEntry,
  RecommendationItem,
  TaskGraphSummary,
  TaskHistoryEntry,
  WorkspaceRecommendationEngineSummary,
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
      terminal: true,
      actionable: false,
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

    const actionableFlag = { ...valid, actionable: true };
    expect(hasProjectedTerminalEvidence(actionableFlag)).toBe(false);
  });

  it("summary contract requires history fields (strict IPC alignment)", () => {
    const summary: WorkspaceRecommendationEngineSummary = {
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

  it("treats history_count as authoritative over history.length for RE", () => {
    const summary: WorkspaceRecommendationEngineSummary = {
      workspace_id: "ws",
      generated_at: "t",
      label: "L",
      candidate_count: 0,
      relationship_count: 0,
      top_candidates: [],
      history: [
        {
          native_id: "rec-x",
          lifecycle_state: "accepted",
          outcome: {
            outcome_id: "recommendation_outcome:rec-x",
            recommendation_id: "rec-x",
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
          terminal: true,
          actionable: false,
          authority_effect: "none",
        },
      ],
      history_count: 4,
      explanation: "",
      summary: "none open",
      authority_effect: "none",
    };
    expect(summary.history.length).toBe(1);
    expect(summary.history_count).toBe(4);
    expect(summary.top_candidates.length === 0 && summary.history_count > 0).toBe(
      true
    );
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

describe("projection integrity — task graph terminal history", () => {
  function taskHistory(
    overrides: Partial<TaskHistoryEntry> = {}
  ): TaskHistoryEntry {
    return {
      task_id: "task:1",
      title: "Ship",
      status: "completed",
      progress_percent: 100,
      explanation: "done",
      updated_at: "t1",
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    };
  }

  it("treats task history as non-actionable", () => {
    expect(isTaskHistoryNonActionable(taskHistory())).toBe(true);
    expect(
      isTaskHistoryNonActionable(taskHistory({ status: "cancelled" }))
    ).toBe(true);
    expect(isTaskHistoryNonActionable(taskHistory({ actionable: true }))).toBe(
      false
    );
    expect(
      isTaskHistoryNonActionable(taskHistory({ status: "in_progress" }))
    ).toBe(false);
  });

  it("keeps actionable top_nodes distinct from history with count authority", () => {
    const summary: TaskGraphSummary = {
      workspace_id: "ws",
      generated_at: "t",
      node_count: 1,
      relationship_count: 0,
      active_count: 1,
      blocked_count: 0,
      waiting_count: 0,
      completed_count: 2,
      progress_percent: 50,
      top_nodes: [],
      history: [taskHistory({ task_id: "task:done" })],
      history_count: 3,
      summary: "mixed",
      integrity_ok: true,
      authority_effect: "none",
    };
    expect(summary.history.length).toBe(1);
    expect(summary.history_count).toBe(3);
    expect(summary.history.every(isTaskHistoryNonActionable)).toBe(true);
    expect(summary.history[0]).not.toHaveProperty("blocker_ids");
    expect(summary.history[0]).not.toHaveProperty("handoff_command");
  });
});

describe("projection integrity — execution lifecycle dual channel", () => {
  function execHistory(
    overrides: Partial<ExecutionLifecycleHistoryEntry> = {}
  ): ExecutionLifecycleHistoryEntry {
    return {
      execution_request_id: "execution:1",
      suggestion_id: "s-1",
      intent_id: null,
      state: "failed",
      retry_allowed: true,
      failure_reason: "gateway_denied",
      claimed_at: "t0",
      completed_at: null,
      updated_at: "t1",
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    };
  }

  it("treats execution history as non-actionable and preserves failure facts", () => {
    const entry = execHistory();
    expect(isExecutionHistoryNonActionable(entry)).toBe(true);
    expect(entry.retry_allowed).toBe(true);
    expect(entry.failure_reason).toBe("gateway_denied");
    expect(entry).not.toHaveProperty("dispatch_allowed");
    expect(entry).not.toHaveProperty("cancellation_allowed");
  });

  it("separates actionable in-flight from terminal history with count authority", () => {
    const projection: ExecutionLifecycleProjection = {
      actionable: [
        {
          execution_request_id: "execution:live",
          suggestion_id: "s-live",
          intent_id: null,
          state: "in_progress",
          claimed_at: "t0",
          updated_at: "t1",
          cancellation_allowed: false,
          authority_effect: "none",
        },
      ],
      history: [execHistory({ execution_request_id: "execution:fail" })],
      history_count: 5,
      authority_effect: "none",
    };
    expect(projection.actionable.every((e) => e.state === "in_progress")).toBe(
      true
    );
    expect(projection.history.every(isExecutionHistoryNonActionable)).toBe(
      true
    );
    expect(projection.history.length).toBe(1);
    expect(projection.history_count).toBe(5);
    expect(
      projection.actionable.some(
        (e) => e.execution_request_id === "execution:fail"
      )
    ).toBe(false);
  });

  it("never treats unknown as a known terminal", () => {
    expect(
      isExecutionHistoryNonActionable(execHistory({ state: "unknown" }))
    ).toBe(false);
  });
});

describe("projection contract — shared cross-domain invariants", () => {
  function assertCountAuthority(windowLen: number, count: number) {
    expect(count).toBeGreaterThanOrEqual(windowLen);
  }

  function assertNoCommandKeys(entry: Record<string, unknown>) {
    for (const key of [
      "execute",
      "handoff_command",
      "dispatch_allowed",
      "cancellation_allowed",
      "recommended_action",
    ]) {
      expect(entry).not.toHaveProperty(key);
    }
    if ("actionable" in entry) {
      expect(entry.actionable).toBe(false);
    }
  }

  it("enforces history_count >= history.length for every surface", () => {
    assertCountAuthority(1, 4); // RE-style truncation
    assertCountAuthority(1, 5); // DQ
    assertCountAuthority(1, 4); // DE
    assertCountAuthority(1, 3); // Task
    assertCountAuthority(1, 5); // Execution
  });

  it("treats history DTOs as non-commandable across domains", () => {
    assertNoCommandKeys({
      native_id: "r",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    });
    assertNoCommandKeys({
      decision_item_id: "d",
      actionable: false,
      authority_effect: "none",
    });
    assertNoCommandKeys({
      artifact_id: "a",
      actionable: false,
      terminal: true,
      origin: "unknown",
    });
    assertNoCommandKeys({
      task_id: "t",
      actionable: false,
      terminal: true,
    });
    assertNoCommandKeys({
      execution_request_id: "e",
      actionable: false,
      terminal: true,
      retry_allowed: true,
    });
    assertNoCommandKeys({
      plan_id: "p",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    });
  });
});

describe("projection integrity — planning engine", () => {
  const historyEntry = (
    overrides: Partial<PlanningHistoryEntry> = {}
  ): PlanningHistoryEntry => ({
    plan_id: "planning_plan:1",
    title: "Prior plan",
    status: "superseded",
    confidence: 70,
    uncertainty: 30,
    generated_at: "t0",
    superseded_at: "t1",
    terminal: true,
    actionable: false,
    authority_effect: "none",
    ...overrides,
  });

  it("marks planning history as non-actionable", () => {
    expect(isPlanningHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isPlanningHistoryNonActionable(
        historyEntry({ actionable: true, status: "superseded" })
      )
    ).toBe(false);
    expect(
      isPlanningHistoryNonActionable(historyEntry({ status: "active" }))
    ).toBe(false);
  });

  it("keeps planning snapshots non-commandable with authoritative history_count", () => {
    const snapshot: PlanningSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isPlanningSnapshotNonCommandable(snapshot)).toBe(true);
    const summary: PlanningSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_active_plan: false,
      active_plan_id: null,
      active_title: null,
      step_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(planningHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);
  });
});

describe("projection integrity — unknown provenance", () => {
  it("keeps unknown provenance as unknown (never invent native)", () => {
    const orphan = {
      origin: "unknown",
      recommendation_id: null as string | null,
    };
    expect(orphan.origin).toBe("unknown");
    expect(orphan.origin).not.toBe("native");
    expect(orphan.recommendation_id).toBeNull();
  });
});
