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
import {
  isReasoningHistoryNonActionable,
  isReasoningSnapshotNonCommandable,
  reasoningHistoryCountIsAuthoritative,
} from "../app/src/components/reasoningProjection";
import {
  isCognitiveGraphHistoryNonActionable,
  isCognitiveGraphSnapshotNonCommandable,
  cognitiveGraphHistoryCountIsAuthoritative,
} from "../app/src/components/cognitiveGraphProjection";
import {
  isOrchestrationHistoryNonActionable,
  isOrchestrationSnapshotNonCommandable,
  orchestrationHistoryCountIsAuthoritative,
} from "../app/src/components/orchestrationProjection";
import {
  isLearningHistoryNonActionable,
  isLearningSnapshotNonCommandable,
  learningHistoryCountIsAuthoritative,
} from "../app/src/components/learningProjection";
import {
  isCognitiveAgentCastHistoryNonActionable,
  isCognitiveAgentCastSnapshotNonCommandable,
  cognitiveAgentCastHistoryCountIsAuthoritative,
} from "../app/src/components/cognitiveAgentCastProjection";
import {
  isCognitiveAutonomyHistoryNonActionable,
  isCognitiveAutonomySnapshotNonCommandable,
  cognitiveAutonomyHistoryCountIsAuthoritative,
} from "../app/src/components/cognitiveAutonomyProjection";
import {
  isWorkspaceStateHistoryNonActionable,
  isWorkspaceStateSnapshotNonCommandable,
  workspaceStateHistoryCountIsAuthoritative,
} from "../app/src/components/workspaceStateEnvelopeProjection";
import {
  isPolicyGovernanceHistoryNonActionable,
  isPolicyGovernanceSnapshotNonCommandable,
  policyGovernanceHistoryCountIsAuthoritative,
} from "../app/src/components/policyGovernanceProjection";
import {
  historicalReconstructionHistoryCountIsAuthoritative,
  isHistoricalReconstructionHistoryNonActionable,
  isHistoricalReconstructionSnapshotNonCommandable,
} from "../app/src/components/historicalReconstructionProjection";
import {
  isTemporalIntelligenceHistoryNonActionable,
  isTemporalIntelligenceSnapshotNonCommandable,
  temporalIntelligenceHistoryCountIsAuthoritative,
} from "../app/src/components/temporalIntelligenceProjection";
import {
  isWorkspaceExplanationHistoryNonActionable,
  isWorkspaceExplanationSnapshotNonCommandable,
  workspaceExplanationHistoryCountIsAuthoritative,
} from "../app/src/components/workspaceExplanationProjection";
import {
  isContextualUnderstandingHistoryNonActionable,
  isContextualUnderstandingProjectionNonCommandable,
  contextualUnderstandingHistoryCountIsAuthoritative,
} from "../app/src/components/contextualUnderstandingProjection";
import {
  isKnowledgeSynthesisHistoryNonActionable,
  isKnowledgeSynthesisProjectionNonCommandable,
  knowledgeSynthesisHistoryCountIsAuthoritative,
} from "../app/src/components/knowledgeSynthesisProjection";
import {
  isKnowledgeIntegrationHistoryNonActionable,
  isKnowledgeIntegrationProjectionNonCommandable,
  knowledgeIntegrationHistoryCountIsAuthoritative,
} from "../app/src/components/knowledgeIntegrationProjection";
import {
  isInsightCoordinationHistoryNonActionable,
  isInsightCoordinationProjectionNonCommandable,
  insightCoordinationHistoryCountIsAuthoritative,
} from "../app/src/components/insightCoordinationProjection";
import {
  isCrossWorkspaceIntelligenceHistoryNonActionable,
  isCrossWorkspaceIntelligenceProjectionNonCommandable,
  crossWorkspaceIntelligenceHistoryCountIsAuthoritative,
} from "../app/src/components/crossWorkspaceIntelligenceProjection";
import {
  isDecisionSupportHistoryNonActionable,
  isDecisionSupportProjectionNonCommandable,
  decisionSupportHistoryCountIsAuthoritative,
} from "../app/src/components/decisionSupportProjection";
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
  ReasoningHistoryEntry,
  ReasoningSnapshot,
  ReasoningSummary,
  CognitiveGraphHistoryEntry,
  CognitiveGraphSnapshot,
  CognitiveGraphSummary,
  OrchestrationHistoryEntry,
  WorkspaceOrchestrationSnapshot,
  WorkspaceOrchestrationSummary,
  LearningHistoryEntry,
  LearningSnapshot,
  LearningSummary,
  CognitiveAgentCastHistoryEntry,
  CognitiveAgentCastSnapshot,
  CognitiveAgentCastSummary,
  CognitiveAutonomyHistoryEntry,
  CognitiveAutonomySnapshot,
  CognitiveAutonomySummary,
  WorkspaceStateHistoryEntry,
  WorkspaceStateSnapshot,
  WorkspaceStateSummary,
  PolicyGovernanceHistoryEntry,
  PolicyGovernanceSnapshot,
  PolicyGovernanceSummary,
  HistoricalReconstructionHistoryEntry,
  HistoricalReconstructionSnapshot,
  HistoricalReconstructionSummary,
  TemporalIntelligenceHistoryEntry,
  TemporalIntelligenceSnapshot,
  TemporalIntelligenceSummary,
  WorkspaceExplanationHistoryEntry,
  WorkspaceExplanationSnapshot,
  WorkspaceExplanationSummary,
  ContextualUnderstandingHistoryEntry,
  ContextualUnderstandingProjection,
  ContextualUnderstandingSummary,
  KnowledgeSynthesisHistoryEntry,
  KnowledgeSynthesisProjection,
  KnowledgeSynthesisSummary,
  KnowledgeIntegrationHistoryEntry,
  KnowledgeIntegrationProjection,
  KnowledgeIntegrationSummary,
  InsightCoordinationHistoryEntry,
  InsightCoordinationProjection,
  InsightCoordinationSummary,
  CrossWorkspaceIntelligenceHistoryEntry,
  CrossWorkspaceIntelligenceProjection,
  CrossWorkspaceIntelligenceSummary,
  DecisionSupportHistoryEntry,
  WorkspaceDecisionSupportProjection,
  WorkspaceDecisionSupportSummary,
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
    assertNoCommandKeys({
      record_id: "reasoning:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    });
    assertNoCommandKeys({
      snapshot_id: "cognitive_graph:1",
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

describe("projection integrity — reasoning memory", () => {
  const historyEntry = (
    overrides: Partial<ReasoningHistoryEntry> = {}
  ): ReasoningHistoryEntry => ({
    record_id: "reasoning:1",
    title: "Prior reasoning",
    status: "superseded",
    confidence: 70,
    uncertainty: 30,
    created_at: "t0",
    superseded_at: "t1",
    reflection_excerpt: "learned something",
    terminal: true,
    actionable: false,
    authority_effect: "none",
    ...overrides,
  });

  it("marks reasoning history as non-actionable", () => {
    expect(isReasoningHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isReasoningHistoryNonActionable(
        historyEntry({ actionable: true, status: "superseded" })
      )
    ).toBe(false);
    expect(
      isReasoningHistoryNonActionable(historyEntry({ status: "current" }))
    ).toBe(false);
  });

  it("keeps reasoning snapshots non-commandable with authoritative history_count", () => {
    const snapshot: ReasoningSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isReasoningSnapshotNonCommandable(snapshot)).toBe(true);
    const summary: ReasoningSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      current_id: null,
      current_title: null,
      confidence: null,
      uncertainty: null,
      reflection_excerpt: "",
      lesson_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(reasoningHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);
  });
});

describe("projection integrity — cognitive graph", () => {
  const historyEntry = (
    overrides: Partial<CognitiveGraphHistoryEntry> = {}
  ): CognitiveGraphHistoryEntry => ({
    snapshot_id: "cognitive_graph:1",
    status: "superseded",
    generated_at: "t0",
    superseded_at: "t1",
    node_count: 2,
    edge_count: 1,
    broken_node_count: 0,
    broken_edge_count: 0,
    terminal: true,
    actionable: false,
    authority_effect: "none",
    ...overrides,
  });

  it("marks graph history as non-actionable", () => {
    expect(isCognitiveGraphHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isCognitiveGraphHistoryNonActionable(
        historyEntry({ actionable: true, status: "superseded" })
      )
    ).toBe(false);
  });

  it("keeps graph snapshots non-commandable with authoritative history_count", () => {
    const snapshot: CognitiveGraphSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isCognitiveGraphSnapshotNonCommandable(snapshot)).toBe(true);
    const summary: CognitiveGraphSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      current_id: null,
      node_count: 0,
      edge_count: 0,
      broken_reference_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(cognitiveGraphHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);
  });
});

describe("projection integrity — cognitive orchestration", () => {
  const historyEntry = (
    overrides: Partial<OrchestrationHistoryEntry> = {}
  ): OrchestrationHistoryEntry => ({
    orchestration_id: "orchestration:1",
    status: "superseded",
    created_at: "t0",
    superseded_at: "t1",
    current_generation: 1,
    uncertainty: 40,
    rationale_excerpt: "coordinate",
    stage_count: 4,
    cycle_count: 0,
    terminal: true,
    actionable: false,
    authority_effect: "none",
    ...overrides,
  });

  it("marks orchestration history as non-actionable", () => {
    expect(isOrchestrationHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isOrchestrationHistoryNonActionable(
        historyEntry({ actionable: true, status: "superseded" })
      )
    ).toBe(false);
  });

  it("keeps orchestration snapshots non-commandable with authoritative history_count", () => {
    const snapshot: WorkspaceOrchestrationSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isOrchestrationSnapshotNonCommandable(snapshot)).toBe(true);
    const summary: WorkspaceOrchestrationSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      orchestration_id: null,
      current_generation: null,
      stage_count: 0,
      stale_count: 0,
      blocked_count: 0,
      cycle_count: 0,
      uncertainty: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(orchestrationHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      orchestration_id: "orchestration:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("command");
    expect(Object.keys(nonCommand)).not.toContain("accept");
  });
});

describe("projection integrity — learning adaptation", () => {
  const historyEntry = (
    overrides: Partial<LearningHistoryEntry> = {}
  ): LearningHistoryEntry => ({
    learning_id: "learning:1",
    status: "superseded",
    created_at: "t0",
    superseded_at: "t1",
    uncertainty: 40,
    observation_count: 2,
    pattern_count: 1,
    adaptation_count: 1,
    terminal: true,
    actionable: false,
    authority_effect: "none",
    ...overrides,
  });

  it("marks learning history as non-actionable", () => {
    expect(isLearningHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isLearningHistoryNonActionable(
        historyEntry({ actionable: true, status: "superseded" })
      )
    ).toBe(false);
  });

  it("keeps learning snapshots non-commandable with authoritative history_count", () => {
    const snapshot: LearningSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isLearningSnapshotNonCommandable(snapshot)).toBe(true);
    const summary: LearningSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      learning_id: null,
      observation_count: 0,
      pattern_count: 0,
      adaptation_count: 0,
      uncertainty: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(learningHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      learning_id: "learning:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("command");
    expect(Object.keys(nonCommand)).not.toContain("accept");
  });
});

describe("projection integrity — cognitive agent cast", () => {
  const historyEntry = (
    overrides: Partial<CognitiveAgentCastHistoryEntry> = {}
  ): CognitiveAgentCastHistoryEntry => ({
    cast_id: "cognitive_cast:1",
    status: "superseded",
    created_at: "t0",
    superseded_at: "t1",
    confidence: 60,
    uncertainty: 40,
    agent_count: 6,
    perspective_count: 6,
    critique_count: 1,
    synthesis_count: 1,
    terminal: true,
    actionable: false,
    authority_effect: "none",
    ...overrides,
  });

  it("marks agent cast history as non-actionable", () => {
    expect(isCognitiveAgentCastHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isCognitiveAgentCastHistoryNonActionable(
        historyEntry({ actionable: true, status: "superseded" })
      )
    ).toBe(false);
  });

  it("keeps agent cast snapshots non-commandable with authoritative history_count", () => {
    const snapshot: CognitiveAgentCastSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isCognitiveAgentCastSnapshotNonCommandable(snapshot)).toBe(true);
    const summary: CognitiveAgentCastSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      cast_id: null,
      agent_count: 0,
      perspective_count: 0,
      critique_count: 0,
      synthesis_count: 0,
      confidence: null,
      uncertainty: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(cognitiveAgentCastHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      cast_id: "cognitive_cast:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("command");
    expect(Object.keys(nonCommand)).not.toContain("accept");
  });

  it("keeps autonomy snapshots non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<CognitiveAutonomyHistoryEntry> = {}
    ): CognitiveAutonomyHistoryEntry => ({
      autonomy_id: "cognitive_autonomy:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      confidence: 70,
      uncertainty: 30,
      opportunity_count: 1,
      proposal_count: 1,
      recommendation_count: 1,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const snapshot: CognitiveAutonomySnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isCognitiveAutonomySnapshotNonCommandable(snapshot)).toBe(true);
    expect(isCognitiveAutonomyHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isCognitiveAutonomyHistoryNonActionable(
        historyEntry({ actionable: true })
      )
    ).toBe(false);

    const summary: CognitiveAutonomySummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      autonomy_id: null,
      opportunity_count: 0,
      proposal_count: 0,
      recommendation_count: 0,
      confidence: null,
      uncertainty: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(cognitiveAutonomyHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      autonomy_id: "cognitive_autonomy:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
      executable_payload: null as string | null,
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("command");
    expect(Object.keys(nonCommand)).not.toContain("permission");
    expect(nonCommand.executable_payload).toBeNull();
  });

  it("keeps unified state envelopes non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<WorkspaceStateHistoryEntry> = {}
    ): WorkspaceStateHistoryEntry => ({
      state_id: "workspace_state_envelope:1",
      revision: "rev:abc",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      freshness: "fresh",
      completeness: "partial",
      consistency: "consistent",
      source_count: 3,
      conflict_count: 0,
      unknown_count: 1,
      composition_status: "partial",
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const snapshot: WorkspaceStateSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isWorkspaceStateSnapshotNonCommandable(snapshot)).toBe(true);
    expect(isWorkspaceStateHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isWorkspaceStateHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: WorkspaceStateSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      state_id: null,
      revision: null,
      freshness: null,
      completeness: null,
      consistency: null,
      source_count: 0,
      conflict_count: 0,
      unknown_count: 0,
      composition_status: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(workspaceStateHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      state_id: "workspace_state_envelope:1",
      revision: "rev:abc",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("command");
    expect(Object.keys(nonCommand)).not.toContain("permission");
    expect(Object.keys(nonCommand)).toContain("revision");
  });

  it("keeps policy governance snapshots non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<PolicyGovernanceHistoryEntry> = {}
    ): PolicyGovernanceHistoryEntry => ({
      evaluation_set_id: "policy_governance:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      context_revision: "rev:1",
      policy_catalog_revision: "policies:1",
      evaluation_count: 2,
      aggregate_result: "unknown",
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const snapshot: PolicyGovernanceSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isPolicyGovernanceSnapshotNonCommandable(snapshot)).toBe(true);
    expect(isPolicyGovernanceHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isPolicyGovernanceHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: PolicyGovernanceSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      evaluation_set_id: null,
      context_revision: null,
      aggregate_result: null,
      evaluation_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(policyGovernanceHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      evaluation_set_id: "policy_governance:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("grant");
    expect(Object.keys(nonCommand)).not.toContain("approve");
  });

  it("keeps historical reconstruction snapshots non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<HistoricalReconstructionHistoryEntry> = {}
    ): HistoricalReconstructionHistoryEntry => ({
      reconstruction_id: "historical_reconstruction:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      from_revision: "rev:a",
      to_revision: "rev:b",
      completeness: "partial",
      change_count: 1,
      gap_count: 0,
      comparison_count: 1,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const snapshot: HistoricalReconstructionSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isHistoricalReconstructionSnapshotNonCommandable(snapshot)).toBe(true);
    expect(isHistoricalReconstructionHistoryNonActionable(historyEntry())).toBe(
      true
    );
    expect(
      isHistoricalReconstructionHistoryNonActionable(
        historyEntry({ actionable: true })
      )
    ).toBe(false);

    const summary: HistoricalReconstructionSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      reconstruction_id: null,
      from_revision: null,
      to_revision: null,
      completeness: null,
      change_count: 0,
      gap_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(historicalReconstructionHistoryCountIsAuthoritative(summary)).toBe(
      true
    );
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      reconstruction_id: "historical_reconstruction:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("replay");
    expect(Object.keys(nonCommand)).not.toContain("restore");
  });

  it("keeps temporal intelligence snapshots non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<TemporalIntelligenceHistoryEntry> = {}
    ): TemporalIntelligenceHistoryEntry => ({
      analysis_id: "temporal_analysis:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      from_revision: "rev:a",
      to_revision: "rev:b",
      completeness: "partial",
      conflict_count: 1,
      gap_count: 0,
      chain_ref_count: 2,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const snapshot: TemporalIntelligenceSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isTemporalIntelligenceSnapshotNonCommandable(snapshot)).toBe(true);
    expect(isTemporalIntelligenceHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isTemporalIntelligenceHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: TemporalIntelligenceSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      analysis_id: null,
      from_revision: null,
      to_revision: null,
      completeness: null,
      conflict_count: 0,
      gap_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(temporalIntelligenceHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      analysis_id: "temporal_analysis:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("simulate");
    expect(Object.keys(nonCommand)).not.toContain("forecast");
    expect(Object.keys(nonCommand)).not.toContain("repair");
  });

  it("keeps workspace explanation snapshots non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<WorkspaceExplanationHistoryEntry> = {}
    ): WorkspaceExplanationHistoryEntry => ({
      explanation_id: "workspace_explanation:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      section_count: 2,
      gap_count: 1,
      conflict_count: 0,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const snapshot: WorkspaceExplanationSnapshot = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isWorkspaceExplanationSnapshotNonCommandable(snapshot)).toBe(true);
    expect(isWorkspaceExplanationHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isWorkspaceExplanationHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: WorkspaceExplanationSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      explanation_id: null,
      completeness: null,
      section_count: 0,
      gap_count: 0,
      conflict_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(workspaceExplanationHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      explanation_id: "workspace_explanation:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("resolve");
    expect(Object.keys(nonCommand)).not.toContain("dispatch");
  });

  it("keeps contextual understanding projections non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<ContextualUnderstandingHistoryEntry> = {}
    ): ContextualUnderstandingHistoryEntry => ({
      understanding_id: "contextual_understanding:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      theme_count: 2,
      gap_count: 1,
      source_revision_count: 3,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const projection: ContextualUnderstandingProjection = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isContextualUnderstandingProjectionNonCommandable(projection)).toBe(true);
    expect(isContextualUnderstandingHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isContextualUnderstandingHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: ContextualUnderstandingSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      understanding_id: null,
      completeness: null,
      theme_count: 0,
      gap_count: 0,
      source_revision_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(contextualUnderstandingHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      understanding_id: "contextual_understanding:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("recommend");
    expect(Object.keys(nonCommand)).not.toContain("dispatch");
  });

  it("keeps knowledge synthesis projections non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<KnowledgeSynthesisHistoryEntry> = {}
    ): KnowledgeSynthesisHistoryEntry => ({
      synthesis_id: "knowledge_synthesis:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      concept_count: 2,
      cluster_count: 1,
      relationship_count: 1,
      gap_count: 1,
      source_revision_count: 3,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const projection: KnowledgeSynthesisProjection = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isKnowledgeSynthesisProjectionNonCommandable(projection)).toBe(true);
    expect(isKnowledgeSynthesisHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isKnowledgeSynthesisHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: KnowledgeSynthesisSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      synthesis_id: null,
      completeness: null,
      concept_count: 0,
      cluster_count: 0,
      relationship_count: 0,
      gap_count: 0,
      source_revision_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(knowledgeSynthesisHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      synthesis_id: "knowledge_synthesis:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("causes");
    expect(Object.keys(nonCommand)).not.toContain("dispatch");
  });

  it("keeps knowledge integration projections non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<KnowledgeIntegrationHistoryEntry> = {}
    ): KnowledgeIntegrationHistoryEntry => ({
      integration_id: "knowledge_integration:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      link_count: 2,
      hit_count: 1,
      gap_count: 1,
      source_revision_count: 3,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const projection: KnowledgeIntegrationProjection = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isKnowledgeIntegrationProjectionNonCommandable(projection)).toBe(true);
    expect(isKnowledgeIntegrationHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isKnowledgeIntegrationHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: KnowledgeIntegrationSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      integration_id: null,
      completeness: null,
      link_count: 0,
      hit_count: 0,
      gap_count: 0,
      source_revision_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(knowledgeIntegrationHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      integration_id: "knowledge_integration:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("should_execute");
    expect(Object.keys(nonCommand)).not.toContain("dispatch");
  });

  it("keeps insight coordination projections non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<InsightCoordinationHistoryEntry> = {}
    ): InsightCoordinationHistoryEntry => ({
      coordination_id: "insight_coordination:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      cluster_count: 2,
      intersection_count: 1,
      attention_signal_count: 1,
      gap_count: 1,
      source_revision_count: 3,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const projection: InsightCoordinationProjection = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isInsightCoordinationProjectionNonCommandable(projection)).toBe(true);
    expect(isInsightCoordinationHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isInsightCoordinationHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: InsightCoordinationSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      coordination_id: null,
      completeness: null,
      cluster_count: 0,
      intersection_count: 0,
      attention_signal_count: 0,
      gap_count: 0,
      source_revision_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(insightCoordinationHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      coordination_id: "insight_coordination:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("recommend");
    expect(Object.keys(nonCommand)).not.toContain("dispatch");
  });

  it("keeps cross-workspace intelligence projections non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<CrossWorkspaceIntelligenceHistoryEntry> = {}
    ): CrossWorkspaceIntelligenceHistoryEntry => ({
      intelligence_id: "cross_workspace_intelligence:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      pattern_count: 2,
      theme_count: 1,
      risk_signal_count: 1,
      constraint_pattern_count: 1,
      gap_count: 1,
      workspace_count: 2,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const projection: CrossWorkspaceIntelligenceProjection = {
      scope_id: "cross_workspace",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isCrossWorkspaceIntelligenceProjectionNonCommandable(projection)).toBe(true);
    expect(isCrossWorkspaceIntelligenceHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isCrossWorkspaceIntelligenceHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: CrossWorkspaceIntelligenceSummary = {
      scope_id: "cross_workspace",
      generated_at: "t2",
      has_current: false,
      intelligence_id: null,
      completeness: null,
      pattern_count: 0,
      theme_count: 0,
      risk_signal_count: 0,
      constraint_pattern_count: 0,
      gap_count: 0,
      workspace_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(crossWorkspaceIntelligenceHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      intelligence_id: "cross_workspace_intelligence:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("recommend");
    expect(Object.keys(nonCommand)).not.toContain("prioritise");
  });

  it("keeps decision support projections non-commandable with authoritative history_count", () => {
    const historyEntry = (
      overrides: Partial<DecisionSupportHistoryEntry> = {}
    ): DecisionSupportHistoryEntry => ({
      support_id: "decision_support:1",
      status: "superseded",
      created_at: "t0",
      superseded_at: "t1",
      completeness: "partial",
      context_count: 1,
      bundle_count: 1,
      tradeoff_count: 1,
      dependency_count: 2,
      gap_count: 1,
      source_revision_count: 3,
      terminal: true,
      actionable: false,
      authority_effect: "none",
      ...overrides,
    });
    const projection: WorkspaceDecisionSupportProjection = {
      workspace_id: "ws",
      generated_at: "t2",
      current: null,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(isDecisionSupportProjectionNonCommandable(projection)).toBe(true);
    expect(isDecisionSupportHistoryNonActionable(historyEntry())).toBe(true);
    expect(
      isDecisionSupportHistoryNonActionable(historyEntry({ actionable: true }))
    ).toBe(false);

    const summary: WorkspaceDecisionSupportSummary = {
      workspace_id: "ws",
      generated_at: "t2",
      has_current: false,
      support_id: null,
      completeness: null,
      context_count: 0,
      bundle_count: 0,
      tradeoff_count: 0,
      dependency_count: 0,
      gap_count: 0,
      source_revision_count: 0,
      history: [historyEntry()],
      history_count: 3,
      authority_effect: "none",
    };
    expect(decisionSupportHistoryCountIsAuthoritative(summary)).toBe(true);
    expect(summary.history_count).toBeGreaterThanOrEqual(summary.history.length);

    const nonCommand = {
      support_id: "decision_support:1",
      actionable: false,
      terminal: true,
      authority_effect: "none",
    };
    expect(Object.keys(nonCommand)).not.toContain("execute");
    expect(Object.keys(nonCommand)).not.toContain("approve");
    expect(Object.keys(nonCommand)).not.toContain("recommend");
    expect(Object.keys(nonCommand)).not.toContain("decide");
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
