//! Operational recovery contract helpers — detector predicates only.
//!
//! Does not own lifecycle. Services remain authorities; these helpers encode
//! shared fail-closed recovery expectations for tests, documentation, and
//! governance detectors. No state mutation occurs here.

use crate::audit::AuditEvent;
use crate::execution_reconciliation::{ExecutionLifecycleRecord, ExecutionState};
use crate::workspace_recommendation::RecommendationHistoryEntry;
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
}
