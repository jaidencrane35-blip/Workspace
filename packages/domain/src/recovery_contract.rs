//! Operational recovery contract helpers — detector predicates only.
//!
//! Does not own lifecycle. Services remain authorities; these helpers encode
//! shared fail-closed recovery expectations for tests, documentation, and
//! governance detectors. No state mutation occurs here.

use crate::audit::AuditEvent;
use crate::execution_reconciliation::{ExecutionLifecycleRecord, ExecutionState};
use crate::workspace_recommendation::RecommendationHistoryEntry;
use crate::workspace_cognitive_graph::{CognitiveGraphHistoryEntry, CognitiveGraphSnapshot};
use crate::workspace_cognitive_orchestration::{
    OrchestrationHistoryEntry, WorkspaceOrchestrationSnapshot,
};
use crate::workspace_learning_adaptation::{LearningHistoryEntry, LearningSnapshot};
use crate::workspace_cognitive_agent_cast::{
    CognitiveAgentCastHistoryEntry, CognitiveAgentCastSnapshot,
};
use crate::workspace_cognitive_autonomy::{
    CognitiveAutonomyHistoryEntry, CognitiveAutonomySnapshot,
};
use crate::workspace_state_envelope::{
    WorkspaceStateHistoryEntry, WorkspaceStateSnapshot,
};
use crate::policy_governance::{
    PolicyGovernanceHistoryEntry, PolicyGovernanceSnapshot,
};
use crate::workspace_historical_reconstruction::{
    HistoricalReconstructionHistoryEntry, HistoricalReconstructionSnapshot,
};
use crate::workspace_temporal_intelligence::{
    TemporalIntelligenceHistoryEntry, TemporalIntelligenceSnapshot,
};
use crate::workspace_explanation::{
    WorkspaceExplanationHistoryEntry, WorkspaceExplanationSnapshot,
};
use crate::workspace_contextual_understanding::{
    ContextualUnderstandingHistoryEntry, ContextualUnderstandingProjection,
};
use crate::workspace_knowledge_synthesis::{
    KnowledgeSynthesisHistoryEntry, KnowledgeSynthesisProjection,
};
use crate::workspace_knowledge_integration::{
    KnowledgeIntegrationHistoryEntry, KnowledgeIntegrationProjection,
};
use crate::workspace_insight_coordination::{
    InsightCoordinationHistoryEntry, InsightCoordinationProjection,
};
use crate::workspace_cross_intelligence::{
    CrossWorkspaceIntelligenceHistoryEntry, CrossWorkspaceIntelligenceProjection,
};
use crate::workspace_decision_support::{
    DecisionSupportHistoryEntry, WorkspaceDecisionSupportProjection,
};
use crate::workspace_intelligence_hub::{
    IntelligenceHubHistoryEntry, WorkspaceIntelligenceHubProjection,
};
use crate::workspace_semantic_query::{
    SemanticQueryHistoryEntry, WorkspaceSemanticQueryProjection,
};
use crate::workspace_evidence_navigation::{
    EvidenceNavigationHistoryEntry, WorkspaceEvidenceNavigationProjection,
};
use crate::workspace_evidence_trace::{
    EvidenceTraceHistoryEntry, WorkspaceEvidenceTraceProjection,
};
use crate::workspace_evidence_coverage::{
    EvidenceCoverageHistoryEntry, WorkspaceEvidenceCoverageProjection,
};
use crate::workspace_evidence_consistency::{
    EvidenceConsistencyHistoryEntry, WorkspaceEvidenceConsistencyProjection,
};
use crate::workspace_evidence_dependency::{
    EvidenceDependencyHistoryEntry, WorkspaceEvidenceDependencyProjection,
};
use crate::workspace_reasoning_memory::{ReasoningHistoryEntry, ReasoningSnapshot};

/// Documented startup in-progress sweep cap (must match
/// `ExecutionLifecycleService::STARTUP_IN_PROGRESS_SWEEP_LIMIT`).
/// Remaining stale rows reconcile lazily via existing get/list paths.
pub const STARTUP_IN_PROGRESS_SWEEP_LIMIT: usize = 500;

/// Recovery diagnostic event types — append-only audit evidence, never commands.
pub const RECOVERY_DIAGNOSTIC_ATTEMPTED: &str = "system.recovery.startup.attempted";
pub const RECOVERY_DIAGNOSTIC_COMPLETED: &str = "system.recovery.startup.completed";
pub const RECOVERY_DIAGNOSTIC_FAILED: &str = "system.recovery.startup.failed";

pub const RECOVERY_DIAGNOSTIC_EVENT_TYPES: &[&str] = &[
    RECOVERY_DIAGNOSTIC_ATTEMPTED,
    RECOVERY_DIAGNOSTIC_COMPLETED,
    RECOVERY_DIAGNOSTIC_FAILED,
];

pub const RECOVERY_SUBSYSTEM_EXECUTION_LIFECYCLE: &str = "execution_lifecycle";

/// Stale in-progress claims must never advertise retry after fail-closed recovery.
pub fn recovered_stale_claim_is_non_retryable(record: &ExecutionLifecycleRecord) -> bool {
    record.state == ExecutionState::Failed && !record.retry_allowed
}

/// Recovery must not invent terminal history from an in-progress claim.
pub fn in_progress_claim_is_not_terminal_evidence(record: &ExecutionLifecycleRecord) -> bool {
    record.state == ExecutionState::InProgress
}

/// Missing / incomplete history evidence must not pass as terminal proof.
pub fn history_evidence_is_complete(entry: &RecommendationHistoryEntry) -> bool {
    entry.is_non_actionable()
}

/// Recovery repairs must go through service/command paths — never fabricate
/// an actionable history row.
pub fn recovery_must_not_fabricate_actionable_history(entry: &RecommendationHistoryEntry) -> bool {
    !entry.actionable
}

/// Recovery must never invent Completed terminal from a stale claim.
pub fn recovery_must_not_invent_completed(record: &ExecutionLifecycleRecord) -> bool {
    record.state != ExecutionState::Completed
        || record.completed_at.as_ref().is_some_and(|t| !t.trim().is_empty())
}

/// Missing reasoning remains missing — never fabricate a current record on restart.
pub fn recovery_must_not_fabricate_reasoning(snapshot: &ReasoningSnapshot) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable reasoning history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_reasoning_history(
    entry: &ReasoningHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing graph remains missing — never invent nodes/edges on restart.
pub fn recovery_must_not_fabricate_cognitive_graph(snapshot: &CognitiveGraphSnapshot) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable graph history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_graph_history(
    entry: &CognitiveGraphHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing orchestration remains missing — never invent refresh plans on restart.
pub fn recovery_must_not_fabricate_orchestration(
    snapshot: &WorkspaceOrchestrationSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable orchestration history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_orchestration_history(
    entry: &OrchestrationHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing learning remains missing — never invent lessons/success/failure on restart.
pub fn recovery_must_not_fabricate_learning(snapshot: &LearningSnapshot) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable learning history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_learning_history(
    entry: &LearningHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing agent cast remains missing — never invent agents/perspectives on restart.
pub fn recovery_must_not_fabricate_cognitive_agent_cast(
    snapshot: &CognitiveAgentCastSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable agent cast history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_agent_cast_history(
    entry: &CognitiveAgentCastHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing autonomy remains missing — never invent opportunities/confidence/approvals on restart.
pub fn recovery_must_not_fabricate_cognitive_autonomy(
    snapshot: &CognitiveAutonomySnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable autonomy history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_autonomy_history(
    entry: &CognitiveAutonomyHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing unified state remains missing — never invent revisions/freshness on restart.
pub fn recovery_must_not_fabricate_workspace_state_envelope(
    snapshot: &WorkspaceStateSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable unified-state history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_workspace_state_history(
    entry: &WorkspaceStateHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing policy evaluation remains missing — never invent Compliant on restart.
pub fn recovery_must_not_fabricate_policy_governance(
    snapshot: &PolicyGovernanceSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable policy-governance history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_policy_governance_history(
    entry: &PolicyGovernanceHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing historical reconstruction remains missing — never invent transitions on restart.
pub fn recovery_must_not_fabricate_historical_reconstruction(
    snapshot: &HistoricalReconstructionSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable historical-reconstruction history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_historical_history(
    entry: &HistoricalReconstructionHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing temporal analysis remains missing — never invent causes or forecasts on restart.
pub fn recovery_must_not_fabricate_temporal_intelligence(
    snapshot: &TemporalIntelligenceSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable temporal-intelligence history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_temporal_history(
    entry: &TemporalIntelligenceHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing explanation remains missing — never invent sections or evidence on restart.
pub fn recovery_must_not_fabricate_workspace_explanation(
    snapshot: &WorkspaceExplanationSnapshot,
) -> bool {
    snapshot.is_non_commandable()
        && snapshot.history.iter().all(|h| h.is_non_actionable())
        && snapshot
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable explanation history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_explanation_history(
    entry: &WorkspaceExplanationHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing contextual understanding remains missing — never invent themes on restart.
pub fn recovery_must_not_fabricate_contextual_understanding(
    projection: &ContextualUnderstandingProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable contextual-understanding history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_contextual_understanding_history(
    entry: &ContextualUnderstandingHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing knowledge synthesis remains missing — never invent concepts on restart.
pub fn recovery_must_not_fabricate_knowledge_synthesis(
    projection: &KnowledgeSynthesisProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable knowledge-synthesis history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_knowledge_synthesis_history(
    entry: &KnowledgeSynthesisHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing knowledge integration remains missing — never invent hits on restart.
pub fn recovery_must_not_fabricate_knowledge_integration(
    projection: &KnowledgeIntegrationProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable knowledge-integration history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_knowledge_integration_history(
    entry: &KnowledgeIntegrationHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing insight coordination remains missing — never invent clusters on restart.
pub fn recovery_must_not_fabricate_insight_coordination(
    projection: &InsightCoordinationProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable insight-coordination history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_insight_coordination_history(
    entry: &InsightCoordinationHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing cross-workspace intelligence remains missing — never invent global patterns.
pub fn recovery_must_not_fabricate_cross_workspace_intelligence(
    projection: &CrossWorkspaceIntelligenceProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable cross-workspace history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_cross_workspace_intelligence_history(
    entry: &CrossWorkspaceIntelligenceHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing decision support remains missing — never invent decision outcomes on restart.
pub fn recovery_must_not_fabricate_decision_support(
    projection: &WorkspaceDecisionSupportProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable decision-support history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_decision_support_history(
    entry: &DecisionSupportHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing intelligence hub remains missing — never invent intelligence packages on restart.
pub fn recovery_must_not_fabricate_intelligence_hub(
    projection: &WorkspaceIntelligenceHubProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable intelligence-hub history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_intelligence_hub_history(
    entry: &IntelligenceHubHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Missing semantic query remains missing — never invent matches, relevance, or lineage on restart.
pub fn recovery_must_not_fabricate_semantic_query(
    projection: &WorkspaceSemanticQueryProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable semantic-query history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_semantic_query_history(
    entry: &SemanticQueryHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}


/// Missing evidence navigation remains missing — never invent paths or continuity on restart.
pub fn recovery_must_not_fabricate_evidence_navigation(
    projection: &WorkspaceEvidenceNavigationProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable evidence-navigation history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_evidence_navigation_history(
    entry: &EvidenceNavigationHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}


/// Missing evidence trace remains missing — never invent lineage or bridge history on restart.
pub fn recovery_must_not_fabricate_evidence_trace(
    projection: &WorkspaceEvidenceTraceProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable evidence-trace history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_evidence_trace_history(
    entry: &EvidenceTraceHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Recovery must not invent missing evidence, fill unavailable sources, or estimate coverage.
pub fn recovery_must_not_fabricate_evidence_coverage(
    projection: &WorkspaceEvidenceCoverageProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable evidence-coverage history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_evidence_coverage_history(
    entry: &EvidenceCoverageHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Recovery must not repair conflicts, invent consistency/inconsistency, or assume agreement.
pub fn recovery_must_not_fabricate_evidence_consistency(
    projection: &WorkspaceEvidenceConsistencyProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable evidence-consistency history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_evidence_consistency_history(
    entry: &EvidenceConsistencyHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Recovery must not invent dependency, repair links, bridge broken structure, or estimate missing structure.
pub fn recovery_must_not_fabricate_evidence_dependency(
    projection: &WorkspaceEvidenceDependencyProjection,
) -> bool {
    projection.is_non_commandable()
        && projection.history.iter().all(|h| h.is_non_actionable())
        && projection
            .current
            .as_ref()
            .map(|c| c.is_non_executing())
            .unwrap_or(true)
}

/// Fabricated actionable evidence-dependency history must fail the recovery contract.
pub fn recovery_must_not_fabricate_actionable_evidence_dependency_history(
    entry: &EvidenceDependencyHistoryEntry,
) -> bool {
    entry.is_non_actionable()
}

/// Recovery diagnostics are observational audit evidence only.
pub fn recovery_diagnostic_is_evidence_only(event: &AuditEvent) -> bool {
    RECOVERY_DIAGNOSTIC_EVENT_TYPES.contains(&event.event_type.as_str())
        && event.command_name.is_none()
        && metadata_authority_effect_is_none(event.metadata.as_deref())
}

/// Diagnostic event types must never look like mutation / retry commands.
pub fn recovery_diagnostic_event_type_is_non_commandable(event_type: &str) -> bool {
    RECOVERY_DIAGNOSTIC_EVENT_TYPES.contains(&event_type)
        && !event_type.contains("execute")
        && !event_type.contains("retry")
        && !event_type.contains("dispatch")
        && !event_type.contains("mutate")
}

fn metadata_authority_effect_is_none(metadata: Option<&str>) -> bool {
    let Some(raw) = metadata else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) else {
        return false;
    };
    value
        .get("authority_effect")
        .and_then(|v| v.as_str())
        == Some("none")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::Actor;
    use crate::execution_reconciliation::ExecutionLifecycleRecord;
    use crate::workspace_recommendation::{RecommendationHistoryEntry, RecommendationOutcomeView};

    #[test]
    fn stale_recovery_contract_rejects_retryable_failed_claims() {
        let record = ExecutionLifecycleRecord {
            execution_request_id: "e1".into(),
            suggestion_id: "s1".into(),
            intent_id: None,
            state: ExecutionState::Failed,
            retry_allowed: true,
            failure_reason: Some("stale".into()),
            claimed_at: "t0".into(),
            completed_at: Some("t1".into()),
            updated_at: "t1".into(),
        };
        assert!(!recovered_stale_claim_is_non_retryable(&record));
    }

    #[test]
    fn incomplete_history_is_not_complete_evidence() {
        let entry = RecommendationHistoryEntry {
            native_id: "r".into(),
            lifecycle_state: "accepted".into(),
            outcome: RecommendationOutcomeView {
                outcome_id: "".into(),
                recommendation_id: "r".into(),
                user_decision: "accepted".into(),
                result_kind: "accepted_follow_through".into(),
                lifecycle_resolution: Some("accepted".into()),
                recorded_at: "t".into(),
                explanation_keys: vec![],
                evidence_refs: vec![],
                experience_trace_match_keys: vec![],
                is_system_failure: false,
                authority_effect: "none".into(),
            },
            resolved_at: Some("t".into()),
            terminal: true,
            actionable: false,
            authority_effect: "none".into(),
        };
        assert!(!history_evidence_is_complete(&entry));
        assert!(recovery_must_not_fabricate_actionable_history(&entry));
    }

    #[test]
    fn recovery_diagnostic_events_are_evidence_only() {
        for event_type in RECOVERY_DIAGNOSTIC_EVENT_TYPES {
            assert!(recovery_diagnostic_event_type_is_non_commandable(event_type));
            let event = AuditEvent::from_actor(*event_type, &Actor::system(), false)
                .with_metadata(r#"{"authority_effect":"none","subsystem":"execution_lifecycle"}"#);
            assert!(recovery_diagnostic_is_evidence_only(&event));
        }
    }

    #[test]
    fn recovery_diagnostic_with_command_name_is_not_evidence_only() {
        let event = AuditEvent::from_actor(RECOVERY_DIAGNOSTIC_FAILED, &Actor::system(), false)
            .with_command_name("ExecuteIntentRequest")
            .with_metadata(r#"{"authority_effect":"none"}"#);
        assert!(!recovery_diagnostic_is_evidence_only(&event));
    }

    #[test]
    fn recovery_never_fabricates_reasoning_on_empty_snapshot() {
        let empty = ReasoningSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_reasoning(&empty));
        assert!(empty.current.is_none());
        assert_eq!(empty.history_count, 0);
    }

    #[test]
    fn recovery_rejects_actionable_reasoning_history() {
        let bad = ReasoningHistoryEntry {
            record_id: "reasoning:1".into(),
            title: "x".into(),
            status: "superseded".into(),
            confidence: 50,
            uncertainty: 50,
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            reflection_excerpt: "".into(),
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_reasoning_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_cognitive_graph_on_empty_snapshot() {
        let empty = CognitiveGraphSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_cognitive_graph(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_graph_history() {
        let bad = CognitiveGraphHistoryEntry {
            snapshot_id: "cognitive_graph:1".into(),
            status: "superseded".into(),
            generated_at: "t0".into(),
            superseded_at: Some("t1".into()),
            node_count: 1,
            edge_count: 0,
            broken_node_count: 0,
            broken_edge_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_graph_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_orchestration_on_empty_snapshot() {
        let empty = WorkspaceOrchestrationSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_orchestration(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_orchestration_history() {
        let bad = OrchestrationHistoryEntry {
            orchestration_id: "orchestration:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            current_generation: 1,
            uncertainty: 40,
            rationale_excerpt: "".into(),
            stage_count: 0,
            cycle_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_orchestration_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_learning_on_empty_snapshot() {
        let empty = LearningSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_learning(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_learning_history() {
        let bad = LearningHistoryEntry {
            learning_id: "learning:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            uncertainty: 40,
            observation_count: 0,
            pattern_count: 0,
            adaptation_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_learning_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_agent_cast_on_empty_snapshot() {
        let empty = CognitiveAgentCastSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_cognitive_agent_cast(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_agent_cast_history() {
        let bad = CognitiveAgentCastHistoryEntry {
            cast_id: "cognitive_cast:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            confidence: 50,
            uncertainty: 40,
            agent_count: 0,
            perspective_count: 0,
            critique_count: 0,
            synthesis_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_agent_cast_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_autonomy_on_empty_snapshot() {
        let empty = CognitiveAutonomySnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_cognitive_autonomy(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_autonomy_history() {
        let bad = CognitiveAutonomyHistoryEntry {
            autonomy_id: "cognitive_autonomy:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            confidence: 50,
            uncertainty: 40,
            opportunity_count: 0,
            proposal_count: 0,
            recommendation_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_autonomy_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_workspace_state_on_empty_snapshot() {
        let empty = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_workspace_state_envelope(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_workspace_state_history() {
        let bad = WorkspaceStateHistoryEntry {
            state_id: "workspace_state_envelope:1".into(),
            revision: "rev:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            freshness: "fresh".into(),
            completeness: "complete".into(),
            consistency: "consistent".into(),
            source_count: 0,
            conflict_count: 0,
            unknown_count: 0,
            composition_status: "composed".into(),
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_workspace_state_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_policy_governance_on_empty_snapshot() {
        let empty = PolicyGovernanceSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_policy_governance(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_policy_governance_history() {
        let bad = PolicyGovernanceHistoryEntry {
            evaluation_set_id: "policy_governance:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            context_revision: None,
            policy_catalog_revision: "policies:1".into(),
            evaluation_count: 0,
            aggregate_result: "unknown".into(),
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_policy_governance_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_historical_reconstruction_on_empty_snapshot() {
        let empty = HistoricalReconstructionSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_historical_reconstruction(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_historical_history() {
        let bad = HistoricalReconstructionHistoryEntry {
            reconstruction_id: "historical_reconstruction:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            from_revision: None,
            to_revision: None,
            completeness: "unavailable".into(),
            change_count: 0,
            gap_count: 0,
            comparison_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_historical_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_temporal_intelligence_on_empty_snapshot() {
        let empty = TemporalIntelligenceSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_temporal_intelligence(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_temporal_history() {
        let bad = TemporalIntelligenceHistoryEntry {
            analysis_id: "temporal_analysis:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            from_revision: None,
            to_revision: None,
            completeness: "unavailable".into(),
            conflict_count: 0,
            gap_count: 0,
            chain_ref_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_temporal_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_workspace_explanation_on_empty_snapshot() {
        let empty = WorkspaceExplanationSnapshot::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_workspace_explanation(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_explanation_history() {
        let bad = WorkspaceExplanationHistoryEntry {
            explanation_id: "workspace_explanation:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            section_count: 0,
            gap_count: 0,
            conflict_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_explanation_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_contextual_understanding_on_empty_projection() {
        let empty = ContextualUnderstandingProjection::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_contextual_understanding(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_contextual_understanding_history() {
        let bad = ContextualUnderstandingHistoryEntry {
            understanding_id: "contextual_understanding:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            theme_count: 0,
            gap_count: 0,
            source_revision_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_contextual_understanding_history(
            &bad
        ));
    }

    #[test]
    fn recovery_never_fabricates_knowledge_synthesis_on_empty_projection() {
        let empty = KnowledgeSynthesisProjection::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_knowledge_synthesis(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_knowledge_synthesis_history() {
        let bad = KnowledgeSynthesisHistoryEntry {
            synthesis_id: "knowledge_synthesis:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            concept_count: 0,
            cluster_count: 0,
            relationship_count: 0,
            gap_count: 0,
            source_revision_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_knowledge_synthesis_history(
            &bad
        ));
    }

    #[test]
    fn recovery_never_fabricates_knowledge_integration_on_empty_projection() {
        let empty = KnowledgeIntegrationProjection::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_knowledge_integration(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_knowledge_integration_history() {
        let bad = KnowledgeIntegrationHistoryEntry {
            integration_id: "knowledge_integration:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            link_count: 0,
            hit_count: 0,
            gap_count: 0,
            source_revision_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_knowledge_integration_history(
            &bad
        ));
    }

    #[test]
    fn recovery_never_fabricates_insight_coordination_on_empty_projection() {
        let empty = InsightCoordinationProjection::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_insight_coordination(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_insight_coordination_history() {
        let bad = InsightCoordinationHistoryEntry {
            coordination_id: "insight_coordination:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            cluster_count: 0,
            intersection_count: 0,
            attention_signal_count: 0,
            gap_count: 0,
            source_revision_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_insight_coordination_history(
            &bad
        ));
    }

    #[test]
    fn recovery_never_fabricates_cross_workspace_on_empty_projection() {
        let empty = CrossWorkspaceIntelligenceProjection::assemble(None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_cross_workspace_intelligence(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_cross_workspace_history() {
        let bad = CrossWorkspaceIntelligenceHistoryEntry {
            intelligence_id: "cross_workspace_intelligence:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            pattern_count: 0,
            theme_count: 0,
            risk_signal_count: 0,
            constraint_pattern_count: 0,
            gap_count: 0,
            workspace_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_cross_workspace_intelligence_history(
            &bad
        ));
    }

    #[test]
    fn recovery_never_fabricates_decision_support_on_empty_projection() {
        let empty = WorkspaceDecisionSupportProjection::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_decision_support(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_decision_support_history() {
        let bad = DecisionSupportHistoryEntry {
            support_id: "decision_support:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            context_count: 0,
            bundle_count: 0,
            tradeoff_count: 0,
            dependency_count: 0,
            gap_count: 0,
            source_revision_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_decision_support_history(&bad));
    }

    #[test]
    fn recovery_never_fabricates_intelligence_hub_on_empty_projection() {
        let empty = WorkspaceIntelligenceHubProjection::assemble("ws", None, vec![], 0, "t0");
        assert!(recovery_must_not_fabricate_intelligence_hub(&empty));
        assert!(empty.current.is_none());
    }

    #[test]
    fn recovery_rejects_actionable_intelligence_hub_history() {
        let bad = IntelligenceHubHistoryEntry {
            hub_id: "intelligence_hub:1".into(),
            status: "superseded".into(),
            created_at: "t0".into(),
            superseded_at: Some("t1".into()),
            completeness: "unavailable".into(),
            package_count: 0,
            gap_count: 0,
            conflict_count: 0,
            source_revision_count: 0,
            terminal: true,
            actionable: true,
            authority_effect: "none".into(),
        };
        assert!(!recovery_must_not_fabricate_actionable_intelligence_hub_history(&bad));
    }
}
